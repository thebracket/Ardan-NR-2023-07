# Handling Files in Your Service

There are three major categories of file-handling inside a server service:

* Processing the file, as fast as possible.
* Sending the file somewhere else---streaming it to maximize I/O efficiency.
* Sending the file somewhere else---and doing something with it on the way through.

These tasks can be orthogonal, and are often best handled in chunks. Sometimes, you need to combine the two.

## Regular File I/O

Let's start by making sure we're comfortable with regular file input/output (non-async). Let's count the lines in *War and Peace*. An inefficient but fast way to do this is to read the entire file into memory and process it:

```rust
use std::fs::read_to_string;

fn main() {
    let now = std::time::Instant::now();
    let war_and_peace = read_to_string("../warandpeace.txt").unwrap();
    println!("Line count: {}", war_and_peace.lines().count());
    println!("Completed in {} ms", now.elapsed().as_millis());
}
```

This is really fast (3 ms on my development machine)---but you are going to have a copy of War and Peace in memory while the function runs. That's not great if you're working with an enormous file!

So let's use Rust's `BufReader` to buffer the read, and only keep one line in memory at a time.

```rust
use std::{io::{BufRead, BufReader}, fs::File};

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("../warandpeace.txt").unwrap();
    let buffered_reader = BufReader::new(file);
    println!("Line count: {}", buffered_reader.lines().count());
    println!("Completed in {} ms", now.elapsed().as_millis());
}
```

Using the buffer presents a trade-off. You only use a tiny amount of memory, but execution time is up to 8-9ms on my development machine. That's still pretty fast---but there's no such thing as a free lunch, so you have to decide between memory and raw speed.

### Let's go a little crazy

`mmap` system calls provide a fast way to treat a file as an area of memory, with loading handled by the operating system's really efficient page loading. Let's use `memmap2` to count the lines in War and Peace.

We'll add the `mmap` crate with `cargo add memmap2`.

```rust
use std::{io::{BufRead, BufReader}, fs::File};
use memmap2::MmapOptions;

fn main() {
    let now = std::time::Instant::now();
    let file = File::open("../warandpeace.txt").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let buffered_reader = BufReader::new(&mmap[..]);
    println!("Line count: {}", buffered_reader.lines().count());
    println!("Completed in {} ms", now.elapsed().as_millis());
}
```

This provides a small improvement for *War and Peace* (1-2 ms). For really big files, the improvement can be *huge*. For a randomly generated 10,000,000 line file (1 gb) the timings come in as follows:

| Method | Time |
| --- | --- |
| Read to String | 2,311 ms |
| BufReader | 853 ms |
| MMap | 778 ms |

You can generate a file on *NIX with:

```
tr -dc "A-Za-z 0-9" < /dev/urandom | fold -w100|head -n 1000000 > bigfile.txt
```

## So What is a Stream?

Streams are a lot like Rust iterators, but operate in an asynchronous context. Instead of just calling `next()` on each item, you call `poll_next()`---and `await` the result. This allows the stream to yield control back to the executor, and allows other tasks to run. When the stream has a new item, it will resume execution.

## Streaming Files

Streaming files is a great way to maximize I/O efficiency. It's also a great way to minimize memory usage. If you're sending a 1GB file to a client, you don't want to load the entire file into memory before sending it. You want to load a chunk, send it, load the next chunk, send it, and so on. You also want your async tasks to yield frequently---you don't want to tie up your executor with lots of waiting for I/O.

### Streaming Files with Axum

If you're working with one of the built-in response types provided by Axum.

> Code example: `code/axum-filestream`. You'll need to run `cargo add tokio-util -F io` to use the example.

You need to set a header to indicate how browsers (or other clients) should handle the file, and then use `tokio_util`'s `io::ReaderStream` to stream the file to the client.

```rust
use axum::{
    body::StreamBody,
    http::{HeaderMap, header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> impl IntoResponse {
    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/toml; charset=utf-8"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str("attachment; filename=\"Cargo.toml\"").unwrap()
    );

    Ok((headers, body))
}
```

This allows you to stream files from disk to the client, without loading the entire file into memory. It maximizes *throughput*, not latency. The task will be yielding frequently, allowing other tasks to run. This has the advantage of making the best use of your I/O resources (disk and network bandwidth).

### Modifying Files as They Stream

Sometimes, you need to modify a file as it streams. Let's expand on the previous version, and asynchronously read the file line-by-line. Let's read War and Peace, and stream it in upper case.

Chaining streams together is called "using an adapter". Streams can be nested on top of one another. Because each stream only handles input from the previous stream, it remains reasonably efficient (but yielding frequently). This is great for maximizing I/O, especially if you are reading from a fast source (disk) and sending to a slower one (the network). Each "chunk" of the stream only remains in memory while it is in-flight, so you minimize RAM requirements.

The downside: writing an adapter is a bit tricky.

You'll need to add the following dependencies:

```bash
cargo add axum
cargo add tokio -F full
cargo add tokio-stream -F full
cargo add tokio-util -F io
cargo add pin-project-lite
```

The basic handler is very similar:

```rust
async fn handler() -> impl IntoResponse {
    use tokio::io::AsyncBufReadExt;

    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a buffered reader, then a line stream, then your adapter
    let stream = BufReader::new(file).lines();
    let stream = tokio_stream::wrappers::LinesStream::new(stream);
    let stream = ToUpper::new(stream);

    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/toml; charset=utf-8"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str("attachment; filename=\"Cargo.toml\"").unwrap()
    );

    Ok((headers, body))
}
```

Instead of just loading the file as a stream, and passing it along - we are building a chain of adapters. Tokio and the `tokio_util` and `tokio_stream` crates provide some very helpful adapters to work with. Let's break down what we're doing:

* We open file file, using Tokio's file handling (async) routines (`tokio::fs::File`).
* We create `BufReader` - but it's a `tokio::io::BufReader` not the standard one.
* We use `tokio_stream`'s `LinesStream` adapter to turn the `.lines()` function into a stream.
* We pass the stream into a new adapter we're going to create called `ToUpper` - which takes a stream of strings, and emits a stream of upper case strings.

Now we run headlong into one of Rust's memory management issues. When you want to stream data between asynchronous objects in memory, you need to ensure that Rust won't relocate any of the data. This is called *pinning*. The `pin-project-lite` crate provides the easiest way to do this; you can wrap your project in a macro, and it does the work for you. So let's create a `ToUpper` type, and use the macro to ensure that the underlying stream is *pinned* in memory---it won't move:

```rust
pin_project! {
    struct ToUpper {
        #[pin]
        stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>,
    }
}
```

Note that I've spelled out the whole "tokio_stream..." - normally you'd `use` the types. I wanted it to be obvious how everything is wrapped.

Now we'll implement a constructor:

```rust
impl ToUpper {
    fn new(stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>) -> Self {
        Self { stream }
    }
}
```

Straightforward - it takes a stream and puts it in the structure. The pin macro will take care of the rest. Finally, we need to implement `Stream`:

```rust
impl tokio_stream::Stream for ToUpper {
    type Item = std::io::Result<String>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx).map(|opt| {
            opt.map(|res| {
                res.map(|line| {
                    line.to_uppercase() + "\n"
                })
            })
        })
    }
}
```

Implementing a stream is just like implementing an iterator: you specify the type with `type Item` for each entry. Then `poll_next` works like `next`, but accepts an async context. So `project()` is created by the `pin_project` macro - and provides easy access to the pinned data structure. We use that to access the underlying stream, and call `poll_next` - to get the next item in the stream (this is an async call). Then we `map` the result to be uppercase (the outer map handles the option, `Some` or `None`).

You can easily modify this to work as a stream adapter for anything you need to be streaming.

## Processing Files

Sometimes, you need to process a file as fast as possible. This is a great use case for system threads. You can either spawn a system thread with Tokio's `spawn_blocking` and wait for the result, or you can submit your task to a thread pool and wait for the result. This allows you to build a hybrid model and get the best of both worlds: you can process files as fast as you can, but retain network performance and responsiveness with async.

## Notify When Files Change

Just a quick note that if you need to receive file change events, the [notify](https://docs.rs/notify/latest/notify/) crate provides everything you need. Implementations tend to become platform specific.

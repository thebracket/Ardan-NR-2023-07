# External Programs

You quite often want to call an external program, and handle its results. I didn't want to try and find a cross-platform program that would be certain to work on whatever platform you're using - so in the repo there's a Rust project named "thumbnailer". It's very simple: it loads a picture, and emits a thumbnail.

You can find the project in `code/thumbnailer`. Compile it with `cargo build --release`, and copy the resulting `target/release/thumbnailer` program into the code for this example.

## Calling Programs with Command

The simplest way to call another process is with the `Command` type from the standard library:

```rust
use std::process::Command;

fn main() {
    let result = Command::new("../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .spawn();
    println!("{result:?}");
}
```

This isn't ideal - it doesn't capture input or output, so the program's result message is echoed to your screen. So let's capture `stdout`:

```rust
use std::process::Command;

fn main() {
    let result = Command::new("../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .output();

    if let Ok(output) = result {
        let returned_text = String::from_utf8(output.stdout).unwrap();
        println!("Process returned: {returned_text}");
    }
}
```

### Sending Input to a Program

Sometimes, sending arguments isn't enough. You actually want to control the input (it might even be output from another program). If you only need to read the output once, you can do this as follows:

```rust
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut echo = Command::new("../target/debug/echo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let echo_stdin = echo.stdin.as_mut().unwrap();

    echo_stdin.write_all(b"Hello, world!\n").unwrap();
    let output = echo.wait_with_output().unwrap();
    println!("output = {:?}", output);
}
```

If you are emitting a lot of data, you can use `echo.stdout.take()` to take ownership of the `stdout` handle, and then read from it as you would any other file. Linking it to a `BufReader` like you did with files gives a nice line-by-line option.

## Asynchronously Calling Programs

Within an async program, you have two options for calling external programs:

* Use `spawn_blocking` and use the regular Rust interface.
* Use Tokio's `Command` alternative.

Tokio's `Command` works almost exactly like the regular one---but it's async. With normal usage, you can use it almost identically:

```rust
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let result = Command::new("../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .output()        
        .await;

    if let Ok(output) = result {
        let returned_text = String::from_utf8(output.stdout).unwrap();
        println!("Process returned: {returned_text}");
    }
}
```

Internally, your command will be spawned in a blocking thread - and the async queue remains responsive.

Using async (with Tokio) also offers you a handy "timeout" option:

```rust
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let future = Command::new("../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .output();

    if let Ok(Ok(output)) = timeout(Duration::from_secs(1), future).await {
        let returned_text = String::from_utf8(output.stdout).unwrap();
        println!("Process returned: {returned_text}");
    }
}
```

## Remember the Foreign Function Interface

Calling processes with `Command` is fast, but does come with overhead (the same overhead as calling them from a command-line script). If you have the source code to the command you want to call, and you are making enough calls to make reducing the overhead worthwhile, Rust's FFI interface can be a great option. It can connect to any language that emits a C-based output.

FFI wasn't included in the scope of this training, but you can find material [here](https://github.com/thebracket/ArdanUltimateRust-5Days/blob/main/04-Memory/FFI.md).
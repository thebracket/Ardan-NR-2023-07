# System Threads

We've used threads a little bit already, but we've glossed over the details. You've learned how to share data safely between threads, but you haven't learned how to create threads, how to join them, or how to use them effectively.

> **What is a thread?** A thread is an operating-system supported structure that allows you to run multiple tasks at the same time. Threads are a parallel structure, meaning that they run on different CPU cores at the same time. Threads are scheduled by the operating system, in the same way as processes. Threads have their own stack, share memory with the parent process, and have their own instruction pointer. They are relatively heavyweight; creating a thread requires that the thread create its own execution context. Switching threads requires that the CPU freeze execution status for the previous entity, restore the previous context for the next thread and run. The OS will interrupt execution when it chooses---although you can give it hints. Other than protecting synchronized data, you don't need to write anything esoteric to use threads. You're also limited in how many threads you can create; my system reports about 60,000. Performance will really suffer if I try to do that, though.

## Create Your First System Thread

Let's make a very simple program that creates a thread and waits for it to finish:

```rust
fn hello_thread() {
    println!("Hello from thread!");
}

fn main() {
    println!("Hello from main thread!");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap();
}
```

Now run the program:

```bash
Hello from main thread!
Hello from thread!
```

So what's going on here? Let's break it down:

1. The program starts in the main thread.
2. The main thread prints a message.
3. We create a thread using `std::thread::spawn` and tell it to run the function `hello_thread`.
4. The return value is a "thread handle". You can use these to "join" threads---wait for them to finish.
5. We call `join` on the thread handle, which waits for the thread to finish.

### What happens if we don't join the thread?

Run the program a few times. Sometimes the secondary thread finishes, sometimes it doesn't. Threads don't outlive the main program, so if the main program exits before the thread finishes, the thread is killed.

## Spawning Threads with Parameters

The `spawn` function takes a function without parameters. What if we want to pass parameters to the thread? We can use a closure:

```rust
fn hello_thread(n: u32) {
    println!("Hello from thread {n}!");
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0 .. 5 {
        let thread_handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(thread_handle);
    }
    thread_handles.into_iter().for_each(|h| h.join().unwrap());
}
```

Notice three things:

* We're using a *closure*---an inline function that can capture variables from the surrounding scope.
* We've used the shorthand format for closure: `|| code` - parameters live in the `||` (there aren't any), and a single statement goes after the `||`. You can use complex closures with a scope: `|x,y| { code block }`.
* The closure says `move`. Remember when we talked about ownership? You have to *move* variables into the closure, so the closure gains ownership of them. The ownership is then passed to the thread. Otherwise, you have to use some form of synchronization to ensure that data is independently accessed---to avoid race conditions.

The output will look something like this (the order of the threads will vary):

```
Hello from thread 0!
Hello from thread 2!
Hello from thread 1!
Hello from thread 4!
Hello from thread 3!
```

 ## Returning Data from Threads

The thread handle will return any value returned by the thread. It's generic, so it can be of any type (that supports sync+send; we'll cover that later). Each thread has its own stack, and can make normal variables inside the thread---and they won't be affected by other threads.

Let's build an example:

```rust
fn do_math(i: u32) -> u32 {
    let mut n = i+1;
    for _ in 0 .. 10 {
        n *= 2;
    }
    n
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0..10 {
        thread_handles.push(std::thread::spawn(move || {
            do_math(i)
        }));
    }

    for handle in thread_handles {
        println!("Thread returned: {}", handle.join().unwrap());
    }
}
```

This returns:

```
Thread returned: 1024
Thread returned: 2048
Thread returned: 3072
Thread returned: 4096
Thread returned: 5120
Thread returned: 6144
Thread returned: 7168
Thread returned: 8192
Thread returned: 9216
Thread returned: 10240
```

Notice that each thread is doing its own math, and returning its own value. The `join` function waits for the thread to finish, and returns the value from the thread.

## Dividing Workloads

We can use threads to divide up a workload. Let's say we have a vector of numbers, and we want to add them all up. We can divide the vector into chunks, and have each thread add up its own chunk. Then we can add up the results from each thread.

```rust
fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect(); // Shorthand for building a vector [0,1,2 .. 4999]
    let mut thread_handles = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    // Notice that each chunk is a *slice* - a reference - to part of the array.    
    for chunk in chunks {
        // So we *move* the chunk into its own vector, taking ownership and
        // passing that ownership to the thread. This adds a `memcpy` call
        // to your code, but avoids ownership issues.
        let my_chunk = chunk.to_owned();

        // Each thread sums its own chunk. You could use .sum() for this!
        thread_handles.push(std::thread::spawn(move || {
            let mut sum = 0;
            for i in my_chunk {
                sum += i;
            }
            sum
        }));
    }

    // Sum the sums from each thread.
    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }
    println!("Sum is {sum}");
}
```

There's a lot to unpack here, so I've added comments:

1. We use a constant to define how many threads we want to use. This is a good idea, because it makes it easy to change the number of threads later. We'll use 8 threads, because my laptop happens to have 8 cores.
2. We create a vector of numbers to add up. We use the `collect` function to build a vector from an iterator. We'll cover iterators later, but for now, just know that `collect` builds a vector from a range. This is a handy shorthand for turning any range into a vector.
3. We create a vector of thread handles. We'll use this to join the threads later.
4. We use the `chunks` function to divide the vector into chunks. This returns an iterator, so we can use it in a `for` loop. Chunks aren't guaranteed to be of equal size, but they're guaranteed to be as close to equal as possible. The last chunk will be smaller than the others.
5. Now we hit a problem:
    * `chunks` is a vector owned by the main thread.
    * Each chunk is a slice --- a borrowed reference --- to part of the vector.
    * We can't pass a borrowed reference to a thread, because the thread might outlive the main thread. There's no guarantee that the order of execution will ensure that the data is destroyed in a safe order.
    * Instead, we use `to_owned` which creates an owned copy of each chunk. This is a `memcpy` operation, so it's not free, but it's safe.

This is a common pattern when working with threads. You'll often need to move data into the thread, rather than passing references.

Moving chunks like this works fine, but if you are using threads to divide up a heavy workload with a single answer --- there's an easier way!

## Scoped Threads

In the previous example we divided our workload into chunks and then took a copy of each chunk. That works, but it adds some overhead. Rust has a mechanism to assist with this pattern (it's a very common pattern): scoped threads.

Let's build an example:

```rust
use std::thread;

fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();
    let chunks = to_add.chunks(N_THREADS);
    let sum = thread::scope(|s| {
        let mut thread_handles = Vec::new();

        for chunk in chunks {
            let thread_handle = s.spawn(move || {
                let mut sum = 0;
                for i in chunk {
                    sum += i;
                }
                sum
            });
            thread_handles.push(thread_handle);
        }

        thread_handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .sum::<u32>()
    });
    println!("Sum is {sum}");
}
```

This is quite similar to the previous example, but we're using *scoped threads*. When you use `thread::scope` you are creating a *thread scope*. Any threads you spawn with the `s` parameter are *guaranteed* to end when the scope ends. You can still treat each scope just like a thread.

Because the threads are *guaranteed* to terminate, you can safely borrow data from the parent scope. This is a *lifetime* issue: a normal thread could keep running for a long time, past the time the scope that launched it ends---so borrowing data from that scope would be a bug (and a common cause of crashes and data corruption in other languages). Rust won't let you do that. But since you have the guarantee of lifetime, you can borrow data from the parent scope without having to worry about it.

This pattern is perfect for when you want to fan out a workload to a set of calculation threads, and wait to combine them into an answer.

## Sending Data Between Threads with Channels

If you're used to Go, channels should sound familiar. They are very similar to Go's channels. A few differences:

* Rust Channels are strongly typed. So you can use a sum type/enum to act like a command pattern.
* Rust Channels are bounded by size, and will block if you try to send data to a full channel.
* Rust Channels are unidirectional. You can't send data back to the sender. (You can make another channel)
* You can't forget to close a channel. Once a channel is out of scope, the "drop" system (we'll talk about that in a couple of weeks) will close the channel for you.

### Multi-Producer, Single Consumer Channels

The most basic type of channel is the MPSC channel: any number of producers can send a message to a single consumer. Let's build a simple example:

```rust
use std::sync::mpsc;

enum Command {
    SayHello, Quit
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello"),
                Command::Quit => {
                    println!("Quitting now");
                    break;
                }
            }
        }
    });

    for _ in 0 .. 10 {
        tx.send(Command::SayHello).unwrap();
    }
    println!("Sending quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
```

This is a relatively simple example. We're only sending messages to one thread, and not trying to send anything back. We're also not trying to send anything beyond a simple command. But this is a great pattern---you can extend the `Command` to include lots of operations, and you can send data along with the command. Threads can send to other threads, and you can `clone` the `tx` handle to have as many writers as you want.

> You don't have to send an enum. You can send a boxed trait, a function pointer, an `Arc` wrapped in synchronization primitives.

## Hello Async World


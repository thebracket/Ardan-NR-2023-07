# Async/Await, Green Threads and the Tokio Executor

In the last set of examples, we worked with *threads*---heavyweight operating system scheduled entities. Threads are great for pushing through computational workloads, but aren't great for I/O dependent workloads.

System threads are managed by the operating system and are *preemptively* multi-tasked. What does that really mean?

* A thread can be interrupted at any time by the OS scheduler.
* The OS scheduler is relatively heavy-weight: it has to save the state of the current thread, load the state of the next thread, and then resume execution.
* You have limited control over when tasks are scheduled.

An *async* model is *cooperatively* multi-tasked, and may run in just one thread---or it may split tasks across many threads. What does that really mean?

* A task can only be interrupted when it yields control. (The executor process might still be interrupted by the OS scheduler.)
* Tasks are really light-weight: they just contain the execution stack (local variables and function calls), and any data indicating how to resume the task (for example---when a network operation completes).

## When to use Async or System Threads?

| **System Threads** | **Async** |
| --- | --- |
| Long-running tasks | Short-running tasks |
| CPU-bound tasks | I/O-bound tasks |
| Tasks that need to run in parallel | Tasks that need to run concurrently |
| Tasks that need minimal, predictable latency | Tasks that can take advantage of latency to do other things - and improve thoughput |

As Bill puts it: "Async takes advantage of latency". When you have a lot to do, and much of it involves waiting for something else to finish---a database operation, the network, a file system operation---then async is a great fit. It's also a great fit for short-running tasks that need to run concurrently.

**Don't** use Async when your task will consume lots of CPU and there may be a long pause between logical points to yield to other async tasks. You'll slow everything down.

**Don't** use System Threads to spawn one per network client and spend most of the time waiting for I/O. You'll run out of memory or thread allocations.

**Do** mix and match the two to really unlock the power of Rust's concurrency model.

## Rust and Async/Await

NodeJS, Go, C#, Python and others all implement an *opinionated* and *batteries included* approach to Async/Await. You `await` a "future" or "promise"---depending upon the language---and the language framework handles the rest.

C++ and Rust both took a more agnostic approach. They provide the building blocks, but leave it up to you to assemble them into a framework. This is a good thing, because it means you can build a framework that fits your needs, and not the needs of the language designers. It also allows for competition between frameworks, driving innovation and fitness for purpose.

The downside is that it's a bit more work to get started. But that's what this course is for!

## Hello Async/Await

There's a simple rule to remember in async/await land:

* Async functions *can* execute non-async functions (and do all the time).
* Non-async functions *cannot* execute async functions, except with the help of an executor.

### Futures

> The code for this is in `code/03_async/hello_async_futures`.

Let's build a really simple example:

```rust
async fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    let x = say_hello();
}
```

This doesn't do anything. Even though `say_hello()` *looks* like it's calling the "say_hello" function---it's not. The type hint in Visual Studio Code gives it away: `impl Future<Output = ()>`. This is a *future*. It represents a task that hasn't been executed yet. You can pass it to an executor to run it, but you can't run it directly.

So let's add an executor. We'll start by using one of the simplest executors out there---a proof of concept more than a real executor. It's called `block_on` and it's in the `futures` crate. We'll start by adding the crate:

```bash
cargo add futures
```

Now, we'll use the simplest bridge between synchronous and asynchronous code: `block_on`. This is a function that takes a future and runs it to completion. It's not a real executor, but it's good enough for our purposes:

```rust
use futures::executor::block_on;

async fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    let _x = say_hello();
    block_on(say_hello());
}
```

The `futures` crate has implemented a simple *executor*, which provides the ability to "block" on an async function. It runs the function---and any async functions it calls---to completion.

Let's add a second async function, and call it from the first:

```rust
use futures::executor::block_on;

async fn say_hello() {
    println!("Hello, world!");
    second_fn().await;
}

async fn second_fn() {
    println!("Second function");
}

fn main() {
    let _x = say_hello();
    block_on(say_hello());
}
```

Notice that once you are *inside* an `async` context, it's easier to call the next async function. You just call it and add `.await` at the end. No need to block again. The "await" keyword tells the executor to run an async task (returned as a future) and wait until it's done.

This is the building block of async/await. You can call async functions from other async functions, and the executor will run them to completion.

### What's Actually Happening Here?

When you call `block_on`, the `futures` crate sets up an execution context. It's basically a list of tasks. The first async function is added to the list and runs until it awaits. Then it moves to the back of the list, and a new task is added to the list. Once the second function completes, it is removed from the task list---and execution returns to the first task. Once there are no more tasks, this simple executor exits.

In other words, you have *cooperative* multitasking. You can await as many things as you want. This particular executor doesn't implement a threaded task pool (unless you ask for it)---it's a single threaded job.

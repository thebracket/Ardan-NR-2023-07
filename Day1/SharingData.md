# Safely Sharing Data

We've seen that the *borrow checker* is your friend, even if its sometimes an irritating friend! Eliminating whole classes of bugs is worth the initial mental gymnastics required to adopt the ownership model.

To recap: think about ownership. Clear ownership prevents bugs, and makes your code easier to read:

* If you don't need a variable anymore, move it to the new owner (or let it drop).
* If you need to retain ownership, lend/borrow with a reference.
* Absolutely ensure that your variable lives long enough for all borrowers.
* Keep reference lifetimes as short as possible.

Sometimes, you can't do that. You might need to share data between threads, or you might have an unclear picture of exactly how long a variable will live.

## Rc - Reference Counting

Early garbage collectors often worked with reference counting: when a reference to a variable is created, the "reference count" --- how many things are looking at the variable --- is incremented. When the reference drops, the "reference count" is decremented. When nothing refers to a variable, it is dropped.

Rust supports reference counting with `Rc`. This appends a reference counter to your type as a tuple. You can increment the reference count with `clone()`, and decrement it with `drop()`. When the reference count reaches zero, the variable is dropped. This allows you to worry less about the lifetime of your variables, at the small cost of a few extra CPU cycles (it's still very fast).

You can *wrap* a type in `Rc` to make it reference counted:

```rust
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0)
    }
}

fn main() {
    let s = Rc::new(Data("hello".to_string()));
}
```

Because you have the `Rc` wrapper, you can now `clone` the variable as often as you like---and it'll still remain unique, only being dropped once it is no longer used:

```rust
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0)
    }
}

fn main() {
    let s = Rc::new(Data("hello".to_string()));
    {
        let t = s.clone();
        println!("I have a {t:?}");
    }
}
```

So you can create functions that accept `Rc<Data>` and pass clones, and all of the clones will point to the same data. You've traded a quick add/compare operation for re-use---and memory is preserved by only allocating once.

## Arc - Atomic Reference Counting

`Rc` is a single-threaded construct. You can't pass an `Rc` between threads, because nothing is protecting the reference count itself from being modified by multiple threads. `Arc` is the same as `Rc`, but uses atomic types---which we'll cover shortly for the reference counter.

You can use `Arc` just like an `Rc`. The only difference is slightly slower performance, because reference count updates are atomic operations. Here's an example:

```rust
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0)
    }
}

fn main() {
    let s = Arc::new(Data("hello".to_string()));
    std::thread::scope(|scope| {
        for _ in 0..10 {
            let my_s = s.clone();
            scope.spawn(move || println!("{my_s:?}"));
        }
    });
}
```

We've used scoped threads, which we'll talk about tomorrow. For now, just know that the `scope` function creates a scope for the threads to run in, and that the `move` keyword moves the variable into the thread. Notice that this is the exception to the rule: `move` is explicit, and by default closures will *refer* to parent variables.

For immutable data that you need to share around your program, `Arc` is a great choice. It's fast, it's safe, and it's easy to use.

How about mutable data? Let's start with a broader view of mutable shared state.

## Mutable Global Variables

Global variables are often a design problem---but they are also sometimes a necessary evil. Over the years, singletons, shared data tables, and other global constructs have evolved to safely share data and resources between threads. Rust `static` variables can be used for this. An immutable, global static is easy:

```rust
static SHARED: i32 = 6;

fn main() {
    println!("{SHARED}");
}
```

What if we want to make `SHARED` mutable?

```rust
static mut SHARED: i32 = 6;

fn main() {
    SHARED += 1;
    println!("{SHARED}");
}
```

Fails spectacularly to compile. Rust cannot guarantee that `SHARED` will retain single ownership, that you won't change it inside threads and potentially overwrite the data. You *can* do this, but *shouldn't*:

```rust
static mut SHARED: i32 = 6;

fn main() {
    unsafe { 
        SHARED += 1; 
        println!("{SHARED}");
    }
}
```

The `unsafe` keyword is telling Rust that you know what you're doing---hold my beer. If bad things happen, by marking the block as `unsafe`---you're accepting responsibility. There's a time and a place for unsafe code, but this is a terrible idea unless you can absolutely, 100% prove that you won't shoot yourself in the foot! Take the following unsafe example:

```rust
static mut SHARED: i32 = 0;

fn main() {
    std::thread::scope(|scope| {
        for _ in 0..1000 {
            scope.spawn(|| unsafe {
                for _ in 0 .. 1000 {
                    SHARED += 1;
                }
            });
        }
    });
    unsafe {
        println!("{SHARED}");
    }
}
```

Run this a few times, and you get a different result each time! This is a *data-race*, and is one of the primary things that Rust protects you against (using `unsafe` to opt-out of safety isn't a great idea, much of the time!).

## Atomic Types

Let's use an *atomic* to solve the problem. We used an atomic inside an `Arc`. Atomics are typically CPU accelerated, and guaranty that operations will be thread-safe. Let's change the example to use atomics:

```rust
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

static SHARED: AtomicI32 = AtomicI32::new(0);

fn main() {
    std::thread::scope(|scope| {
        for _ in 0..1000 {
            scope.spawn(|| {
                for _ in 0 .. 1000 {
                    SHARED.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    println!("{}", SHARED.load(Ordering::Relaxed));
}
```

And now, no matter how many times you run the program - you get the same result. The `Ordering` parameter is a hint to the CPU about how to handle the operation. `Relaxed` is the fastest, but doesn't guarantee that the operation will be atomic. `Acquire` and `Release` are slower, but guarantee that the operation will be atomic. `SeqCst` is the slowest, but guarantees that the operation will be atomic and that the CPU will not reorder the operation. `Relaxed` is just fine for a simple increment like this.

So now the bad news: there isn't an atomic available for every type. You only get atomics for most of the numeric primitives (bools, integers, pointers, etc.). So for more complex types, you need something else.

## Mutexes

The easiest synchronization primitive is the `Mutex`. A mutex works like a stop-light: if the mutex is free, it allows `lock` to give access to the contents. If it is already locked, the `lock` call waits until a lock is available. Mutexes are quite a bit slower than atomics. The following works:

```rust
use std::sync::Mutex;

static SHARED: Mutex<i32> = Mutex::new(0);

fn main() {
    std::thread::scope(|scope| {
        for _ in 0..1000 {
            scope.spawn(|| {
                for _ in 0 .. 1000 {
                    let mut lock = SHARED.lock().unwrap();
                    *lock += 1;
                }
            });
        }
    });
    let lock = SHARED.lock().unwrap();
    println!("{}", *lock);
}
```

There's also `RwLock` which allows you to have unlimited `read` locks, and only one active `write` lock---which waits for all readers and writers to exit. Note that you're not unlocking the mutex---`Drop` does that for you.

### Warning: Deadlocks!

The following WILL compile:

```rust
let lock = SHARED.lock().unwrap();
println!("{}", *lock);
let lock2 = SHARED.lock().unwrap();
```

It will also hang forever. Rust does NOT protect you from deadlocks! You can manually drop `lock` if you wish:

```rust
let lock = SHARED.lock().unwrap();
println!("{}", *lock);
std::mem::drop(lock);
let lock2 = SHARED.lock().unwrap();
```

You can also use a scope:

```rust
{
    let lock = SHARED.lock().unwrap();
    println!("{}", *lock);
}
let lock2 = SHARED.lock().unwrap();
```

So be careful. Normal use doesn't make it easy to deadlock, but it can bite you hard!

### Poisoning

Notice that you called `unwrap` on the Mutex? Mutex acquisition returns a `Result` type. If a thread crashes while a lock is held, that lock is *poisoned*---unwrapping it gives you an error:

```rust
use std::sync::Mutex;

static DATA: Mutex<u32> = Mutex::new(0);

fn poisoner() {
    let mut lock = DATA.lock().unwrap();
    *lock += 1;
    panic!("And poisoner crashed horribly");
}

fn main() {
    let handle = std::thread::spawn(poisoner);
    println!("Trying to return from the thread:");
    println!("{:?}", handle.join());
    println!("Locking the Mutex after the crash:");
    let lock = DATA.lock();
    println!("{lock:?}");
}
```

You can try to get your data back, but it's better to avoid poisoning if you can:

```rust
// Let's try to save the day by recovering the data from the Mutex
let recovered_data = lock.unwrap_or_else(|poisoned| {
    println!("Mutex was poisoned, recovering data...");
    poisoned.into_inner()
});
println!("Recovered data: {recovered_data:?}");
```

## Initialization

One of the problems with global/static variables is "when are they initialized?" If they are more complicated than a simple constant, it can be an open question---when, and in what order do your globals get setup? Rust won't let you assign a non-constant to a static, without jumping through a few hoops that make initialization somewhat explicit. Take the following code:

```rust
static SHARED: String = "Hello".to_string();

fn main() {
    println!("{SHARED}");
}
```

This won't compile. The compiler suggestion is to use the `once_cell` crate (which is gradually becoming part of the standard library).

The easiest option is to use `Lazy`:

```rust
use once_cell::sync::Lazy;

static SHARED: Lazy<String> = Lazy::new(|| "Hello".to_string());

fn main() {
    println!("{}", *SHARED);
}
```

The `Lazy` type stores a function (or function pointer) that will run the first time the static is accessed.

## Interior Mutability

Notice that neither `static DATA: Atomic...` or `static DATA: Mutex` mark the static as `mut`---mutable. Rust allows for *interior mutability*. This is provided with the `sync` trait; a structure is `sync` if it provides synchronization primitives that are safe across threads. Atomics, mutexes and similar provide this.

Remember how we talked about `Arc` being great for sharing *immutable* data across threads? `Arc` provides `send`---a trait that says "this is safe to send across threads", but doesn't provide any locking for the shared data---so it doesn't provide `sync`. If you want to use a reference counted object across threads, the data you are sharing *also* needs to support synchronization. You can do this in two ways.

Let's start with a type:

```rust
use once_cell::sync::Lazy;

#[derive(Debug)]
struct Data (String, String);

impl Data {
    fn new() -> Self {
        Self("Hello".to_string(), "World".to_string())
    }
}

static SHARED: Lazy<Data> = Lazy::new(Data::new);

fn main() {
    println!("{:?}", *SHARED);
}
```

The first pattern you can use is *exterior mutability*:

```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
struct Data (String, String);

impl Data {
    fn new() -> Self {
        Self("Hello".to_string(), "World".to_string())
    }
}

static SHARED: Lazy<Mutex<Data>> = Lazy::new(|| Mutex::new(Data::new()));

fn main() {
    let lock = SHARED.lock().unwrap();
    println!("{:?}", lock);
}
```

This is the pattern we used before: you wrap the entire contents in a `Mutex`, providing one-at-a-time access to the entire structure. It's also potentially the slowest pattern, because *any* access to the variable requires that you wait for the mutex.

Let's rewrite this so that `SHARED` is no longer a global (it will be dropped), and a `Mutex` protects the entire structure. You have a thread-safe, reference-counted variable---just like garbage collection, but entirely deterministic.

```rust
use std::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
struct Data (String, String);

impl Data {
    fn new() -> Self {
        Self("Hello".to_string(), "World".to_string())
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Data::new()));
    std::thread::scope(|scope| {
        for i in 0..10 {
            let my_shared = shared.clone();
            scope.spawn(move || {
                let mut lock = my_shared.lock().unwrap();
                lock.0 = format!("{}{i}", lock.0);
            });
        }
    });
    let lock = shared.lock().unwrap();
    println!("{:?}", lock);
}
```

Now let's do it again, but with *interior mutability*. It gets messy wrapping everything in a `Mutex`, and maybe the entire structure doesn't need protecting? Or maybe different threads will change different parts of the structure, and you'd like to keep locking to a minimum. Let's implement interior mutability:

```rust
use std::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
struct Data (Mutex<String>, String);

impl Data {
    fn new() -> Self {
        Self(Mutex::new("Hello".to_string()), "World".to_string())
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let shared = Arc::new(Data::new());
    std::thread::scope(|scope| {
        for i in 0..10 {
            let my_shared = shared.clone();
            scope.spawn(move || {
                let mut lock = my_shared.0.lock().unwrap();
                *lock = format!("{}{i}", lock);
            });
        }
    });
    let lock = shared.0.lock().unwrap();
    println!("{}, {}", *lock, shared.1);
}
```

Notice how the `Data` structure contains the `Mutex`, and the second string---which never changes---is untouched. You no longer need to declare an outer wrapper, you have to lock each of the fields you are planning on mutating externally.\

## Summary

Rust doesn't have garbage collection, but if you need it---you can get very close, without sacrificing speed. The trick is modeling your data and understanding:

* Ownership - single or shared, and who is responsible for the data (and making sure it eventually goes away)
* Lifetimes - making sure data lives exactly as long as you need it
* Borrowing - don't keep references longer than you need them. Model your data to ensure that a reference is sane. That may mean a parent/child object relationship.

And once you have this: enjoy the performance.

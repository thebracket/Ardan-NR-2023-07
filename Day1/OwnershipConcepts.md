# Ownership & Borrowing - Concepts

The first thing I hear from a lot of people starting with Rust is:

> I tried, but the borrow checker made life impossible.

The borrow checker is a core feature of Rust---without it, you lose most of the safety benefits. C++ has even considered adding one! But it *can* require that you unlearn some concepts from other languages, and it can be a bit of a pain to work with at first. Talking to the same people a few months later, they say:

1. "At first, I hated the borrow checker. I had to change everything"
2. "After a while, I started writing code that worked with the borrow checker"
3. "I started to realize that the borrow checker was helping me write better code"
4. "I realized that when I went back to other languages, I was writing more robust code there, too"

The reason for this cycle is that the borrow checker is preventing you from making some really easy-to-miss mistakes. It eliminates:

* "Use after free" --- you *can't* accidentally use a variable after it's been freed.
* "Double free" --- you *can't* accidentally free a variable twice.
* "Dangling pointers" --- you *can't* accidentally create a pointer to a variable that's been freed.
* "Data Races" --- you can't accidentally share a variable without protecting it against data races.

> Note that you *can* do all of these, but you'll need the `unsafe` tag to do it!

The borrow checker also ties in to resource acquisition/destruction. You can make memory leaks in Rust, but you have to work to make it happen.

Let's take a look at some of the concepts that make up the borrow checker.

## Ownership and "Move By Default"

Let's start with a simple example:

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2 = {s2}");
    println!("s1 = {s1}");
}
```

This *won't* compile. The error message looks like this:

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:20
  |
2 |     let s1 = String::from("Hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("s2 = {s2}");
5 |     println!("s1 = {s1}");
  |                    ^^^^ value borrowed here after move
```

Because `String` isn't a `Copy` type (the primitives such as `i32`, `usize` etc. support trivial copying---types made of of other data usually doesn't), this fails:

1. You create `s1`.
2. When you assign `s1` to `s2`, you are *moving* it. Rust is *move by default*.
3. You can print `s2`, because it is valid.
4. You can't print `s1`, because when you *moved out of it*---it ceased to exist.

Let's expand this a little and use a function:

```rust
fn print(s: String) {
    println!("{s}");
}

fn main() {
    let s = "Hello".to_string();
    print(s);
    println!("{s}");
}
```

When you call the `print` function, you *move* `s` into the function. The `print` function now *owns* the variable---and `s` in `main` is no longer valid. This is the same as the previous example, but illustrates what's really happening. When you *move* a variable, you are transferring ownership---giving it away. It's no longer yours to worry about.

## Dropping - RAII (Resource Acquisition is Initialization)

If you're used to a language such as C, you may be wondering why we're not causing a memory leak here. Rust supports RAII by default. RAII originated in the early days of Object Oriented Programming - when a type falls out of scope, it's destructor is called. Rust uses this everywhere to ensure that you don't leak resources.

Destructors fire as soon as a variable leaves scope and is eligible for destruction. This contrasts with some managed languages such as Java, in which exactly when the destructor fires can be a bit nebulous.

Let's build a quick example:

```rust
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn main() {
    let s = Data("Hello".to_string());
}
```

Running this shows "Hello was dropped"---Rust automatically calls the destructor when `s` goes out of scope. How about when we move `s`?

```rust
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: Data) {
    println!("Hello {}", s.0);
}

fn main() {
    let s = Data("Hello".to_string());
    print(s);
    println!("And we're back");
}
```

This produces:

```
Hello Hello
Hello was dropped
And we're back
```

When you *move* s into `print`, it isn't dropped - it remains valid, it just passes ownership. The function ends, dropping the variable - and then `main` continues---but `s` is gone.

This pattern holds for moving data around repeatedly. Let's return `s` from `print` and see how this affects the move/drop:

```rust
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: Data) -> Data {
    println!("Hello {}", s.0);
    return s;
}

fn main() {
    let s = Data("Hello".to_string());
    let _t = print(s);
    println!("And we're back");
}
```

This yields:

```
Hello Hello
And we're back
Hello was dropped
```

So the lesson here is: consider ownership. If you don't need a variable again, give ownership to the new function that is using it. If you do need it, either return it (giving you a single variable that is being moved around)---or borrow it, which we'll talk about in a moment.

### Let's talk about Clone

Sometimes, you want to pass a variable somewhere and keep the original. `clone` does this. Let's look at an example:

```rust
#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: Data) {
    println!("Hello {}", s.0);
}

fn main() {
    let s = Data("Hello".to_string());
    print(s.clone());
    println!("{s:?} is still valid");
}
```

This gives us:

```
Hello Hello
Hello was dropped
Data("Hello") is still valid
Hello was dropped
```

So your clone made an entirely new copy the data (which can be slow), causing both to be dropped. You'll sometimes find yourself cloning when you really don't want to move something.

## Borrowing

A more obvious approach is to *borrow* the data. Borrowing doesn't transfer ownership, it creates a reference to it---a pointer to the original. Let's look at an example:

```rust
#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: &Data) {
    println!("Hello {}", s.0);
}

fn main() {
    let s = Data("Hello".to_string());
    print(&s);
    println!("{s:?} is still valid");
}
```

The output shows:

```
Hello Hello
Data("Hello") is still valid
Hello was dropped
```

`s` is created once, and then shared with your function. There's no move, and you retain ownership of the data---it's your responsibility (fortunately, `Drop` handles the cleanup for you---so it's a light responsibility).

How about *mutable borrowing*? Let's change `s` to be mutable, and allow `print` to change it:

```rust
#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: &mut Data) {
    println!("Original {}", s.0);
    s.0 += "World";
}

fn main() {
    let mut s = Data("Hello".to_string());
    print(&mut s);
    println!("{s:?} is still valid");
}
```

You had to mark your data as mutable, and explicitly lend it as mutable with `&mut`. Since Rust is immutable by default, you always have to tell it that you a) expect to be able to make changes, and b) want to allow the function to make changes. No surprises! This gives you:

```
Original Hello
Data("HelloWorld") is still valid
HelloWorld was dropped
```

When you start working across threads, you need to remember the golden rule:

* You can have any number of immutable borrows---the underlying data won't change.
* You can only have ONE mutable borrow (and no immutable borrows) at a time.

This is enforced by the compiler. It prevents data-races, which we'll talk about soon.

## Lifetimes

There's some syntax sugar going on! When you borrow a variable, you're actually borrowing it for a *lifetime*. The lifetime is the scope of the borrow. Let's look at an example:

```rust
#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print<'a>(s: &'a mut Data) {
    println!("Original {}", s.0);
    s.0 += "World";
}

fn main() {
    let mut s = Data("Hello".to_string());
    print(&mut s);
    println!("{s:?} is still valid");
}
```

Putting lifetimes in by hand everywhere made Rust really messy, so "lifetime elision" was added for the easy cases. Most of the time, the compiler can figure out the lifetime for you. Lifetimes are important, because the compiler has to check that a borrow will remain valid for the lifetime of the borrow; if you were to store a reference, you have to be sure that the underlying variable to which the borrow refers will remain valid for as long as you keep the reference.

Let's look at a function that returns a reference:

```rust
#[derive(Debug, Clone)]
struct Data(String);

fn get_ref(s: &Data) -> &Data {
    s
}

fn main() {
    let s = Data("Hello".to_string());
    let t = get_ref(&s);
}
```

So this is fine: there's only one variable entering the function, so it's obvious to the compiler that the lifetime of the borrow is the same as the lifetime of the variable. But what if we do this:

```rust
#[derive(Debug, Clone)]
struct Data(String);

fn get_ref(s: &Data, t: &Data) -> &Data {
    s
}

fn main() {
    let s = Data("Hello".to_string());
    let t = get_ref(&s);
}
```

This fails to compile, demanding that we name the lifetimes. We can fix that:

```rust
#[derive(Debug, Clone)]
struct Data(String);

fn get_ref<'a, 'b>(s: &'a Data, t: &'b Data) -> &'a Data {
    s
}

fn main() {
    let s = Data("Hello".to_string());
    let t = get_ref(&s, &s);
}
```

You can get by with using one lifetime for both, if you are sure that they will be the same---but it's better to be explicit. You also don't have to use `'a`---you can use a long name if you prefer.

Why does this help? Let's try to create a dangling pointer:

```rust
#[derive(Debug, Clone)]
struct Data(String);

fn get_ref<'a, 'b>(s: &'a Data, t: &'b Data) -> &'a Data {
    s
}

fn main() {
    let s = Data("Hello".to_string());
    let t = get_ref(&s, &s);
    println!("{t:?}");
    std::mem::drop(s);
    println!("{t:?}");
}
```

Calling `std::mem::drop` explicitly destroys (and calls the destructor) of any variable. It's the same as `delete` in C++ or `free` in C---the variable is gone. C++ would let you run this, and crash on execution. Rust won't allow it to compile:

```
 |
4 | fn get_ref<'a, 'b>(s: &'a Data, t: &'b Data) -> &'a Data {
  |                                 ^ help: if this is intentional, prefix it with an underscore: `_t`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0505]: cannot move out of `s` because it is borrowed
  --> src/main.rs:12:20
   |
9  |     let s = Data("Hello".to_string());
   |         - binding `s` declared here
10 |     let t = get_ref(&s, &s);
   |                     -- borrow of `s` occurs here
11 |     println!("{t:?}");
12 |     std::mem::drop(s);
   |                    ^ move out of `s` occurs here
13 |     println!("{t:?}");
   |               ----- borrow later used here
```

So, we've eliminated an entire class of bugs---without resorting to a garbage collector. This is a *good thing*. Unfortunately, we've also made working with references a bit harder.

## Copyable Types

Just a quick note that if your type is composed entirely of primitives (`i32`, `usize` etc.) it is *copyable*. Any type you create that contains just these can be annotated with `#[derive(Copy)]` - and sending it no longer moves it, it takes a copy. For primitives smaller than your platforms pointer size, this is faster---so it's an optimization.

## Summary

We've covered the basics of ownership and the borrow checker:

* Only one part of a program *owns* a variable.
* By default, when you pass a variable to a function, you *move* it.
    * You can always move it back!
    * If you want to copy it, you can use `clone()`
    * If you want to retain ownership, you can borrow with `&`
* Every borrow has a *lifetime*, even if you can't see it.
* The compiler will prevent you from making use-after-move, use-after-free, dangling pointer errors. That's whole classes of CVEs from C++ eliminated, without losing performance by using a garbage collector.
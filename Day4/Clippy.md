# Clippy - the Linter

Clippy isn't actually an annoying paperclip in Rust.

The `clippy` linter is installed by default with Rust. It's a great tool that will help you find common mistakes and bad practices in your code. It's also a great way to learn more about Rust.

## Clippy Basics

Let's start with some innocuous code:

```rust
fn main() {
    let numbers = (0..100).collect::<Vec<i32>>();
    for i in 0 .. numbers.len() {
        println!("{}", numbers[i]);
    }
}
```

The code works. Clippy will happily tell you why it's not idiomatic Rust:

```bash
cargo clippy
```

```
warning: the loop variable `i` is only used to index `numbers`
 --> clippy_test\src\main.rs:3:14
  |
3 |     for i in 0 .. numbers.len() {
  |              ^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
3 |     for <item> in &numbers {
  |         ~~~~~~    ~~~~~~~~
```

The code `for i in 0 .. numbers.len()` is a common pattern in other languages, but it's not idiomatic Rust. The idiomatic way to do this is to use an iterator:

```rust
for i in &numbers {
    println!("{i}");
}
```

> We'll talk about iterators in a minute, let's stay focused on Clippy for now!

## Pedantic Clippy

Clippy has a "pedantic mode". You probably don't want to use it all the time---it will drive you nuts! It is worth periodically enabling it, and looking for the deeper problems it finds. Pedantic Clippy is slower than regular Clippy.

Add the following to your top-level file:

```rust
#![warn(clippy::pedantic)]
```

The `!` means "apply globally". You'll see a *lot* of warnings on a large code-base. They may not all be correct (pedantic mode contains a few work-in-progress items), but they often give great hints as to where you can improve your code.

## Ignoring Warnings

Sometimes, you want to ignore a warning. You can do this with a `#[allow(...)]` attribute. For example, if you want to ignore the warning about `for i in 0 .. numbers.len()`, you can do this:

```rust
#[allow(dead_code)]
fn nobody_called_me() {
    // Code
}
```

Warnings exist for a reason: they often indicate a problem. If you are going to ignore a warning, make sure you understand why it's there, and that you are ignoring it for a good reason. It's often a good idea to put a comment in place to explain why you are ignoring it.

## Replace `cargo check` with `cargo clippy` in VSCode

1. Open your settings (`ctrl` + `comma`)
2. Search for "cargo check"
3. Change "Rust Analyzer > Check Command" to "clippy"

This is a bit slower, but will run the linter every time you save a file. It will also show you the linter results in the "Problems" tab.

![](/images/RustAnalyzerClippy.png)
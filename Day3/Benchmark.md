# Benchmarking

Rust has a skeleton for benchmarking built-in. It can be combined with helper crates to provide a very detailed benchmarking system.

## Quick and Dirty

A quick and dirty way to benchmark operations is to use the built in `Instant` and `Duration` types. For example:

[Playground Link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=52dedaf0c6963c7deb6a2728425b78c5)

```rust
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut i = 0;
    for j in 0 .. 1_000 {
        i += j*j;
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {} nanos", elapsed.as_nanos());
    println!("{i}");
}
```

This is handy when you just want to get a handle on how long something takes to run. It's not 100% accurate, because reading the clock isn't instantaneous.

## More Complicated Benchmarks with Criterion

Recommended crate:

* [Criterion](https://github.com/bheisler/criterion.rs)

## Setting Up Criterion

In `Cargo.toml`, add:

```toml
[dev-dependencies]
criterion = { version = "0.4", features = [ "html_reports" ] }

[[bench]]
name = "my_benchmark"
harness = false
```

Create `benchmark2/benches/my_benchmark.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

> Taken from the Criterion demo page. The "Optimizing & Debugging Rust" class goes into a lot more detail.

Run `cargo bench` and see the result.

Go to `target/criterion` and you have a full HTML report with statistics.
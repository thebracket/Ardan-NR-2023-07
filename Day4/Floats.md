# Floating Point Numbers

Rust's floating point types are full IEEE 754 floating-point representations---including inaccuracy. It's very easy to get a nearly-right but not accurate number. Checking equality is dangerous!

**In particular, don't ever store money in a floating point number. Your accountants won't like it.**

If this matters to your code, you have a few options:

* You can use the [approx crate](https://docs.rs/approx/latest/approx/) for some helpful macros to help check approximate equality.
* You can use the [bigdecimal](https://docs.rs/bigdecimal/latest/bigdecimal/) to store large decimals without inequality. It's quite fast (not as fast as CPU-enhanced floating point operations), and is supported by Serde, Postgres and other crates.

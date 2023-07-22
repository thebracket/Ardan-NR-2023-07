# Clever Code

The Go language likes to say "Don't be Clever". It's a good rule of thumb, but can be better expressed as "Write Code you can Explain". If you can't explain it, you probably shouldn't be writing it.

Rust is a language that can be very clever. It's a language that can be very expressive. It's a language that can be very terse. It's a language that can be very confusing. Since Rust gives you the power to be really confusing, it's up to you to tame the complexity.

In some cases, you *need* clever code---you want to take full advantage of Rust's performance capabilities! I recommend taking the same approach as the Rust standard library: have some "internal" types that contain the cleverness, and wrap them in a user-friendly facade that is easy to understand.

## How This Works in Practice

Let's say that you are creating a system that memory maps files as needed, based on the requested data. Multiple requests may be coming in from other systems, and you want to make sure that you don't map the same file twice. You also want to make sure that you don't map too many files at once, and run out of paged memory. The system has a relatively simple interface to the outside world: given a set of coordinates, the system transforms the coordinates into the associated filename, accesses it and returns data about those coordinates.

Start by creating a directory module (a directory containing a `mod.rs`) file. In that module, create some stub functions (or types) describing the interface you wish to offer. This might be as simple as:

```rust
fn get_coordinate_data(position: LatLon) -> Option<MyLidarData> {
    todo!("Implement this");
}
```

Then for each part of the complicated system, make another file (I prefer one per type) in the directory - and link them to the module with `mod my_module;`. No `pub mod` - you're hiding the internals.

So for this example, you might have:
* A type representing each file (containing a memory map structure, and access logic)
* A type that converts `LatLon` into the required filename
* A cache type that keeps a least-recently-used cache (with a maximum size) of the file types.
* Lots of tests!

You can then link those to the facade functions. From the outside, you're offering just the one function. Internally, you're doing a lot of clever things. Other teams don't need to see the cleverness, they just need to see the interface.

## When to be Clever

Knuth famously said that premature optimization is the root of all evil. That doesn't mean you should write really terrible code; if there's an obvious performance "win", take it---but not at the expense of readability. Then, profile or trace your code with real data and see where the bottlenecks are. The bottlenecks are where it's worth being clever.

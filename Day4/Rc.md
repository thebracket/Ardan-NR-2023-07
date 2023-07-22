# Don't Reference Count Everything

It's really tempting when you find out that `Rc` and `Arc` can give you a simplified form of garbage collection at very little cost to make *everything* a reference and let Rust sort it out. It'll probably even work.

BUT - you're throwing away some potential performance, and complicating things.

Instead:

* Think about ownership. 
    * If data is taking a clear path, move it along each element of the path.
    * If data is predominantly owned inside a function, and "fans out" to functions that operate on it - before coming back, then references make sense.
    * If data is genuinely shared between entities, then reference counting makes good sense.


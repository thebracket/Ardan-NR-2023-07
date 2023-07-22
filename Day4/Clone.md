# Minimize Cloning

When you're first getting into Rust, it's really easy to abuse `clone()`. It's pretty fast (slowing down the more complex your structure is). With the `move` semantics and the borrow checker, it's very tempting to clone a LOT. The optimizer will minimize the overhead quite a bit, but when you can avoid cloning - it's worth it.

> The exception being types that are *designed* to be cloned, such as `Rc` or connection pools!

If you find yourself cloning things a lot, so you can fan data out in lots of directions---it's usually a sign that your design needs some work. Should you be destructuring and moving the relevant data? Should you be using a shared type (like an `Rc`/`Arc`) and sharing the data? Maybe you should look at reducing the number of `&mut` and use references?
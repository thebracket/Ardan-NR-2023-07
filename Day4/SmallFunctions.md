# Favor Small Functions

Rust is *really* good at inlining---eliminating the cost of calling a function by embedding it in the caller. This gives you no real downside to writing small functions.

Small functions:
* Are easier to read---if you can see the whole function at once, it's easier to understand.
* Are easier to test---you can test each function in isolation.
* Are easier to compose in functional/iterator pipelines.
* Are easier to optimize---the compiler can inline them, and optimize them individually.

Along the same lines, "pure" functions have a performance and clarity advantage. A "pure" function doesn't mutate any external state, but operates entirely on its parameters. This makes it easier to reason about the function, and easier to optimize.

Name your functions well, and you'll have a program that's easy to understand.

> Sometimes your function is long because you're doing something complicated. That's fine! But if you can break it down into smaller functions, you should.
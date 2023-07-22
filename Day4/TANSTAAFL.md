# TANSTAAFL

> There Aint No Such Thing As A Free Lunch

This is something I like to remind everyone of, no matter what language you are using.

Rust is fast, and you often get performance benefits relative to other languages just for using it. But *everything* is a trade-off:

* You can trade developer time optimizing vs. execution time and server cost.
* You can trade code readability vs. performance in some cases.
* You can trade compilation time vs. execution time.

## Generics

Every time you make a generic type or function, the compiler will replace it at compile time with concrete implementations matching what you actually used. This adds to your compile times, and can make for bigger binaries.

## Macros

Macros are slow to compile, but can make coding much more pleasant. Use macros sparingly. Don't redefine large swathes of syntax because you can---do it when it makes the interface objectively better.
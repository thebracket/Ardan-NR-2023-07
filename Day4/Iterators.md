# Favor Iterators

You often find yourself using `for` loops to iterate over collections. Rust's iterator system can transform many of these operations into a functional style. The function style is often more concise, and can result in faster code---the optimizer handles iterators *really* well. It also opens up some possibilities for parallelization.

Here's a type and function to generate some test data:

```rust
struct Row {
    language: String,
    message: String,
}

fn get_rows() -> Vec<Row> {
    vec![
        Row { language : "English".to_string(), message : "Hello".to_string() },
        Row { language : "French".to_string(), message : "Bonjour".to_string() },
        Row { language : "Spanish".to_string(), message : "Hola".to_string() },
        Row { language : "Russian".to_string(), message : "Zdravstvuyte".to_string() },
        Row { language : "Chinese".to_string(), message : "Nǐn hǎo".to_string() },
        Row { language : "Italian".to_string(), message : "Salve".to_string() },
        Row { language : "Japanese".to_string(), message : "Konnichiwa".to_string() },
        Row { language : "German".to_string(), message : "Guten Tag".to_string() },
        Row { language : "Portuguese".to_string(), message : "Olá".to_string() },
        Row { language : "Korean".to_string(), message : "Anyoung haseyo".to_string() },
        Row { language : "Arabic".to_string(), message : "Asalaam alaikum".to_string() },
        Row { language : "Danish".to_string(), message : "Goddag".to_string() },
        Row { language : "Swahili".to_string(), message : "Shikamoo".to_string() },
        Row { language : "Dutch".to_string(), message : "Goedendag".to_string() },
        Row { language : "Greek".to_string(), message : "Yassas".to_string() },
        Row { language : "Polish".to_string(), message : "Dzień dobry".to_string() },
        Row { language : "Indonesian".to_string(), message : "Selamat siang".to_string() },
        Row { language : "Hindi".to_string(), message : "Namaste, Namaskar".to_string() },
        Row { language : "Norwegian".to_string(), message : "God dag".to_string() },
        Row { language : "Turkish".to_string(), message : "Merhaba".to_string() },
        Row { language : "Hebrew".to_string(), message : "Shalom".to_string() },
        Row { language : "Swedish".to_string(), message : "God dag".to_string() },
                
    ]
}
```

A naieve `for` loop to find a language looks like this:

```rust
fn main() {
    let rows = get_rows();
    for row in rows.iter() {
        if row.language == "French" {
            println!("{}", row.message);
            break; // Stop looping
        }
    }
}
```

That works just fine, and isn't too bad. `.iter()` transforms the rows into an iterator (receiving references to each entry, no copying). You can do the same thing with the following:

```rust
rows.iter()
    .filter(|r| r.language == "French")
    .for_each(|r| println!("{}", r.message));
```

We can add some timing to the code:

```rust
fn main() {
    let now = std::time::Instant::now();
    let rows = get_rows();
    for row in rows.iter() {
        if row.language == "French" {
            println!("{}", row.message);
            break;
        }
    }
    println!("Elapsed: {} nanos", now.elapsed().as_nanos());

    let now = std::time::Instant::now();
    rows.iter()
        .filter(|r| r.language == "French")
        .for_each(|r| println!("{}", r.message));
    println!("Elapsed: {} nanos", now.elapsed().as_nanos());
}
```

In debug mode, I get:

```
Bonjour
Elapsed: 187500 nanos
Bonjour
Elapsed: 62200 nano
```

In release mode, I get:

```
Bonjour
Elapsed: 132200 nanos
Bonjour
Elapsed: 57900 nanos
```

This isn't a great benchmark, but the iterator version is faster. The iterator is able to elide some of the range checks (since the size of the iterator is known at compile time).

## Working with Data

Iterators could be a class unto themselves. It's always worth looking at the operations offered by iterators. `map` can be used to transform data on its way through the pipeline. `filter_map` can combine filtering and mapping into a single operation. `all`, `any` can be used to see if a predicate matches all or any element. `skip` and `nth` let you navigate within the iterator. `fold` can apply an accumulator, `reduce` can shrink your data. With `chain` and `zip` you can combine iterators.

In some cases, it's worth learning to make your own iterators. It's relatively simple (very similar to the stream we made).

Remember, iterators don't yield. You can turn an iterator into a stream with a helper function from `tokio-streams` (and also `futures`) if you do need to yield at each step in an async program.

Let's transform a program into iterators.

We'll use a really inefficient prime factoring function:

```rust
fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
}
```

And some code to iterate through the first range of numbers and count the primes we find:

```rust
let now = std::time::Instant::now();
const MAX:u32 = 200000;
let mut count = 0;
for n in 2 .. MAX {
    if is_prime(n) {
        count+=1;
    }
}
println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());
```

On my development workstation, I found 17,984 primes in 1.09 seconds.

Let's write the same code, as an iterator:

```rust
let now = std::time::Instant::now();
let count = (2..MAX)
    .filter(|n| is_prime(*n))
    .count();
println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());
```

There's no speedup, but we have less code --- making it easier to read. We've also opened ourselves up for a really easy parallelization. Add `rayon` to the crate (`cargo add rayon`) and we can use all of our CPU cores with just two lines of code:

```rust
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
let now = std::time::Instant::now();
let count = (2..MAX)
    .into_par_iter()
    .filter(|n| is_prime(*n))
    .count();
println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());
```

The result I get shows that we found the same number of primes in 0.10 seconds.

So not only are iterators more idiomatic, they open up a world of possibilities.

## Understanding .iter() vs .into_iter()

This is a common mistake when you're getting started, and understanding the difference can make a big performance difference sometimes.

`.iter()` returns an iterator that yields references to the data. `.into_iter()` returns an iterator that yields the data itself. This is a subtle difference, but it can make a big difference.

Take the following code:

```rust
let mut v = vec!["one".to_string(), "two".to_string()];
v.iter().for_each(|v| do_something(v));
println!("{v:?}");
```

`v`---your vector---is still valid after the `iter()` call. Each iteration receives a reference to the original data. If you `collect` it into another vector, you get a vector of `&String` types.

However:

```rust
let mut v = vec!["one".to_string(), "two".to_string()];
v.into_iter().for_each(|v| do_something(v));
println!("{v:?}");
```

Won't compile! `v` is *destroyed* by the conversion into an iterator---and each pass is receiving the *actual* `String`, not a reference. If you `collect` it into another vector, you get a vector of `String` types.

* Use `iter()` when you are just referencing the data, and want to retain ownership of it.
* Use `into_iter()` when you will never use the data again. You *move* the data out of the vector, and send it to its new owner.
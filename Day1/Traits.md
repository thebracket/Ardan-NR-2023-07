# Traits

You've used traits a lot---they are an important part of Rust. But we haven't really talked about them.

A trait is effectively an *interface* from other languages. A trait offers a guarantee: if a type implements this trait, it also implements these functions. Much of Rust uses traits (traits, generics and RAII are the pillars on which much of Rust is built).

## Using Traits with Derive

You've used traits a lot. Whenever you've used `#[derive(..)]`, you're using a short-hand procedural macro that implements a trait for you. This:

```rust
#[derive(Debug)]
struct Data(i32);

fn main() {
    let data = Data(12);
    println!("{data:?}");
}
```

is the same as this:

```rust
struct Data(i32);

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Data").field(&self.0).finish()
    }
}

fn main() {
    let data = Data(12);
    println!("{data:?}");
}
```

> It's also a whole lot less typing!

If you look at the `Debug` trait in Rust, it's defined as a trait:

```rust
pub trait Debug {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

## Make a Simple Trait

Let's create a simple trait to explore how they work:

```rust
trait Animal {
    fn speak(&self);
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

fn main() {
    let cat = Cat;
    cat.speak();
}
```

When you run this, unsurprisingly the cat says "meow".

You can add a second implementation just to show that it works:

```rust
trait Animal {
    fn speak(&self);
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

fn main() {
    let cat = Cat;
    cat.speak();
    let dog = Dog;
    dog.speak();
}
```

Now you get "Meow" and "Woof".

## Traits as Function Parameters

You can create a function that requires a parameter implement a trait, rather than a concrete type:

```rust
fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}
```

This function takes a reference to a type that implements the `Animal` trait. It doesn't care what type it is, as long as it implements the trait. It runs the `speak` function twice.

## Traits as Return Types

```rust
fn get_animal() -> impl Animal {
    Cat
}
```

This function returns a type that implements the `Animal` trait. It doesn't care what type it is, as long as it implements the trait. In this case, it returns a `Cat`.

## Traits that Require Other Traits

You can create a trait that requires another trait:

```rust
trait Animal: Debug {
    fn speak(&self);
}
```

Now, the program won't compile until `Cat` and `Dog` both implement `Debug` as well as `Animal`.

You can keep piling on the requirements:

```rust
trait Animal: Debug+Clone {
    fn speak(&self);
}
```

## Dynamic Dispatch

All of the examples above can be resolved at *compile time*. The compiler knows the concrete type of the trait, and can generate the code for it. But what if you want to store a bunch of different types in a collection, and call a trait function on all of them?

You might want to try this:

```rust
let animals: Vec<impl Animal> = vec![Cat, Dog];
```

And it won't work. The *reason* it won't work is that `Vec` stores identical entries for each record. That means it needs to know the size of the entry. Since cats and dogs might be of different sizes, `Vec` can't store them.

You can get around this with *dynamic dispatch*. You've seen this once before, with `type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;`. The `dyn` keyword means that the type is *dynamic*---it can be different sizes.

Now think about boxes. Boxes are a *smart-pointer*. That means they occupy the size of a *pointer* in memory, and that pointer tells you where the data actually is in the heap. So you *can* make a vector of dynamic, boxed traits:

```rust
let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
```

Each vector entry is a pointer (with a type hint) to a trait. The trait itself is stored in the heap. Accessing each entry requires a pointer dereference and a virtual function call. (A `vtable` will be implemented, but often optimized away---LLVM is very good at avoiding making vtables when it can).

> This works with other pointer types like `Rc`, and `Arc`, too. You can have a reference-counted, dynamic dispatch pointer to a trait.

Using dynamic dispatch won't perform as well as static dispatch, because of pointer chasing (which reduces the likelihood of a memory cache hit).

## The `Any` Type

If you really, *really* need to find out the concrete type of a dynamically dispatched trait, you can use the `std::any::Any` trait. It's not the most efficient design, but it's there if you *really* need it.

The easiest way to "downcast" is to require `Any` in your type and an `as_any` function:

```rust
struct Tortoise;

impl Animal for Tortoise {
    fn speak(&self) {
        println!("What noise does a tortoise make anyway?");
    }
}

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

Then you can "downcast" to the concrete type:

```rust
let more_animals : Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
for animal in more_animals.iter() {
    if let Some(cat) = animal.as_any().downcast_ref::<Tortoise>() {
        println!("We have access to the tortoise");
    }
    animal.speak();
}
```

If you can avoid this pattern, you should. It's not very Rusty---it's pretending to be an object-oriented language. But it's there if you need it.

## Generics and Traits

Generics are very closely tied to traits. "Generics" are meta-programming: a way to write "generic" code that works for multiple types. Traits are a way to specify the requirements for a generic type.

The simplest generic is a function that takes a generic type. Who'se sick of typing `to_string()` all the time? I am! You can write a generic function that accepts any type that implements `ToString`---even `&str` (bare strings) implement `ToString`:

```rust
fn print_it<T: ToString>(x: T) {
    println!("{}", x.to_string());
}
```

So now you can call `print_it` with `print_it("Hello")`, `print_it(my_string)` or even `print_it(42)` (because integers implement `ToString`).

There's a second format for generics that's a bit longer but more readable when you start piling on the requirements:

```rust
fn print_it<T>(x: T)
where
    T: ToString,
{
    println!("{}", x.to_string());
}
```

You can combine requirements with `+`:

```rust
fn print_it<T>(x: T)
where
    T: ToString + Debug,
{
    println!("{:?}", x);
    println!("{}", x.to_string());
}
```

You can have multiple generic types:

```rust
fn print_it<T, U>(x: T, y: U)
where
    T: ToString + Debug,
    U: ToString + Debug,
{
    println!("{:?}", x);
    println!("{}", x.to_string());
    println!("{:?}", y);
    println!("{}", y.to_string());
}
```

The generics system is almost a programming language in and of itself---you really can build most things with it.

## Traits with Generics

> See the `04_mem/trait_generic` project.

Some traits use generics in their implementation. The `From` trait is particularly useful, so let's take a look at it:

```rust
struct Degrees(f32);
struct Radians(f32);

impl From<Radians> for Degrees {
    fn from(rad: Radians) -> Self {
        Degrees(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Degrees> for Radians {
    fn from(deg: Degrees) -> Self {
        Radians(deg.0 * std::f32::consts::PI / 180.0)
    }
}
```

Here we've defined a type for Degrees, and a type for Radians. Then we've implemented `From` for each of them, allowing them to be converted from the other. This is a very common pattern in Rust. `From` is also one of the few surprises in `Rust`, because it *also* implements `Into` for you. So you can use any of the following:

```rust
let behind_you = Degrees(180.0);
let behind_you_radians = Radians::from(behind_you);
let behind_you_radians2: Radians = Degrees(180.0).into();
```

You can even define a function that requires that an argument be convertible to a type:

```rust
fn sin(angle: impl Into<Radians>) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}
```

And you've just made it impossible to accidentally use degrees for a calculation that requires Radians. This is called a "new type" pattern, and it's a great way to add constraints to prevent bugs.

You can *also* make the `sin` function with generics:

```rust
fn sin<T: Into<Radians>>(angle: T) -> f32 {
    let angle: Radians = angle.into();
    angle.0.sin()
}
```

The `impl` syntax is a bit newer, so you'll see the generic syntax more often.

## Generics and Structs

You can make generic structs and enums, too. In fact, you've seen lots of generic `enum` types already: `Option<T>`, `Result<T, E>`. You've seen plenty of generic structs, too: `Vec<T>`, `HashMap<K,V>` etc.

Let's build a useful example. How often have you wanted to add entries to a `HashMap`, and instead of replacing whatever was there, you wanted to keep a list of *all* of the provided values that match a key.

> The code for this is in `04_mem/hashmap_bucket`.

Let's start by defining the basic type:

```rust
use std::collections::HashMap;

struct HashMapBucket<K,V>
{
    map: HashMap<K, Vec<V>>
}
```

The type contains a `HashMap`, each key (of type `K`) referencing a vector of values (of type `V`). Let's make a constructor:

```rust
impl <K,V> HashMapBucket<K,V> 
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }
}

So far, so good. Let's add an `insert` function (inside the implementation block):

```rust
fn insert(&mut self, key: K, value: V) {
    let values = self.map.entry(key).or_insert(Vec::new());
    values.push(value);
}
```

Uh oh, that shows us an error. Fortunately, the error tells us exactly what to do---the key has to support `Eq` (for comparison) and `Hash` (for hashing). Let's add those requirements to the struct:

```rust
impl <K,V> HashMapBucket<K,V> 
where K: Eq + std::hash::Hash
{
    fn new() -> Self {
        HashMapBucket {
            map: HashMap::new()
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}
```

So now we can insert into the map and print the results:

```rust
fn main() {
    let mut my_buckets = HashMapBucket::new();
    my_buckets.insert("hello", 1);
    my_buckets.insert("hello", 2);
    my_buckets.insert("goodbye", 3);
    println!("{:#?}", my_buckets.map);
}
```

In 21 lines of code, you've implemented a type that can store multiple values for a single key. That's pretty cool. Generics are a little tricky to get used to, but they can really supercharge your productivity.

## Mocking and Traits

Since traits are an interface, they provide an easy way to "mock" functionality. We'll dig more into this when we talk about unit testing.
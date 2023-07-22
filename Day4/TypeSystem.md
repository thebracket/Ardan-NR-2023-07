# Let the Type System Help You

Rust's type system is very powerful, and can help you write better code.

## Avoid Ambiguity with New Types

> Don't go too crazy with this. If it's obvious that `number_of_threads` is a `usize`, and what the parameter does, it doesn't need its own type!

### Ambiguous Units

We talked about a generic conversion between units in [Traits](../Day1/Traits.md). This is one of the easiest ways to avoid introducing bugs into your system. In the example, we created `Radians` and `Degrees` types and setup `Into` for converting between them. Now the user has to specify the units, and automatic conversion means that passing degrees into a Radians-based function won't cause a bug.

This applies to almost any unit of measure, and is a great place for "new types"---a type that wraps a value, specifying the type and optionally provides unit conversions.

For example, it's pretty common to count bytes. A `Bytes` type makes it obvious that you aren't actually expecting kilobytes, megabytes, etc. --- but for output, you probably want those types, too. You can create a `Bytes` type that implements `Into` for `Kilobytes`, `Megabytes`, etc. and then use `Bytes` internally. You could even provide some output/formatting options that checks the size of the contained value and returns an appropriately scaled value.

For example:

```rust
struct Bytes(usize);
struct Kilobytes(usize);
struct MegaBytes(usize);

impl From<Kilobytes> for Bytes {
    fn from(kb: Kilobytes) -> Self {
        Self(kb.0 * 1024)
    }
}

impl From<MegaBytes> for Bytes {
    fn from(mb: MegaBytes) -> Self {
        Self(mb.0 * 1024 * 1024)
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.0;
        let kb = bytes / 1024;
        let mb = kb / 1024;
        if mb > 0 {
            write!(f, "{} MB", mb)
        } else if kb > 0 {
            write!(f, "{} KB", kb)
        } else {
            write!(f, "{} B", bytes)
        }
    }
}

fn main() {
    let bytes: Bytes = MegaBytes(8).into();
    println!("{bytes}");
}
```

## Long Parameter Lists

It's a shame that Rust doesn't have a named parameter system. When you have a large number of parameters, it becomes very easy to get them in the wrong order. This is especially true if you have a lot of parameters of the same type.

For example, let's say that you have a function that takes a lot of parameters:

```rust
fn do_something(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    // etc
) {
    todo!("Implement this");
}
```

Obviously, you can help the situation by now naming them with one-letter names! Your IDE will show you the parameter list, making it easier. But what if you change a parameter? You have to change it everywhere, and if you happened to still have the same number of parameters of a similar type---you might not notice the error.

There are a couple of solutions here:

* Use a simple new type for some parameters. If `a` is actually always a count of rows you could create a `RowCount(pub usize)` type to make it obvious. That way, even though you are passing `usize`, you have to specify your intent. You've almost got named parameters that way!
* Create a structure containing the parameters and pass that. Now you *have* to name your parameters, and it's much harder to get it wrong.
* And if your structure is large, use a builder pattern.

## Builder Pattern

You've used the builder pattern---it's very common in Rust. It provides a great way to set defaults at the beginning, specify only the parameters you want to change, and then build the final structure.

For example:

```rust
struct ThingConfig {
    do_a: bool,
    do_b: bool,
    setting: usize,
    another_setting: usize,
}

impl ThingConfig {
    fn new() -> Self {
        ThingConfig {
            do_a: false,
            do_b: false,
            setting: 0,
            another_setting: 0,
        }
    }

    fn do_a(mut self) -> Self {
        self.do_a = true;
        self
    }

    fn do_b(mut self) -> Self {
        self.do_b = true;
        self
    }

    fn with_setting(mut self, setting: usize) -> Self {
        self.setting = setting;
        self
    }

    fn with_another_setting(mut self, setting: usize) -> Self {
        self.another_setting = setting;
        self
    }

    fn execute(&self) {
        if self.do_a {
            println!("Doing A");
        }
        if self.do_b {
            println!("Doing B");
        }
        println!("Setting: {}", self.setting);
        println!("Another Setting: {}", self.another_setting);
    }
}

fn main() {
    ThingConfig::new()
        .do_a()
        .with_setting(3)
        .execute();
}
```

Now you've tucked away the complexity, and made it much harder to get the parameters wrong. You can also add validation to the builder, and make sure that the parameters are valid before you execute the function.

You can combine this with the [Error Handling](../Day3/ErrorHandling.md) system to chain validation calls, and return an error if the parameters are invalid. For example:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ThingError {
    #[error("Setting must be between 0 and 10")]
    SettingOutOfRange,
}

type ThingResult<T> = Result<T, ThingError>;

struct ThingConfig {
    do_a: bool,
    do_b: bool,
    setting: usize,
    another_setting: usize,
}

#[allow(dead_code)]
impl ThingConfig {
    fn new() -> Self {
        ThingConfig {
            do_a: false,
            do_b: false,
            setting: 0,
            another_setting: 0,
        }
    }

    fn do_a(mut self) -> ThingResult<Self> {
        self.do_a = true;
        Ok(self)
    }

    fn do_b(mut self) -> ThingResult<Self> {
        self.do_b = true;
        Ok(self)
    }

    fn with_setting(mut self, setting: usize) -> ThingResult<Self> {
        if setting > 10 {
            Err(ThingError::SettingOutOfRange)
        } else {
            self.setting = setting;
            Ok(self)
        }
    }

    fn with_another_setting(mut self, setting: usize) -> ThingResult<Self> {
        self.another_setting = setting;
        Ok(self)
    }

    fn execute(&self) -> ThingResult<()> {
        if self.do_a {
            println!("Doing A");
        }
        if self.do_b {
            println!("Doing B");
        }
        println!("Setting: {}", self.setting);
        println!("Another Setting: {}", self.another_setting);
        Ok(())
    }
}

fn main() -> ThingResult<()> {
    ThingConfig::new()
        .do_a()?
        .with_setting(3)?
        .execute()?;

    Ok(())
}
```

## Defaults

Complex types should implement `Default`. This allows you to create a default instance of the type, and then override the parameters you want to change. For example:

```rust
pub struct MyType {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Default for MyType {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
        }
    }
}
```

You can use the shorthand:

```rust
#[derive(Default)]
pub struct MyType {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}
```

You can now instantiate the structure as `MyType::default()` or use a partial initialization:

```rust
fn main() {
    let t = MyType {
        a: 2,
        ..Default::default()
    };
}
```

You can set default values for enums with `Default`, too:

```rust
#[derive(Default)]
enum MyType {
    One,
    #[default]
    Two,
    Three,
}
```

## Partial Structure Assignment

Don't forget partial structure assignment. It's very helpful when you need to create a new value based mostly on a previous one:

```rust
struct MyType {
    a: i32,
    b: i32,
}

fn main() {
    let one = MyType { a: 3, b: 4 };
    let two = MyType {
        a: 4,
        ..one
    };
}
```

## Prefer Enums

Whenever possible, don't store a `String` with some preferences in it or an opaque integer where 3 means "do this". Use an enumeration. Rust's enumerations are very powerful, and you can add parameter data to options as needed. They also work *really* well with `match`, and there's no room for typos.

## New Types as Traits

Another way to represent unit types is with a trait. This has certain advantages - the trait defines the possible output types, and you are using a named function to retrieve what you want (no more `.0` and tuple syntax). You can also implement the trait for any type, allowing you to arbitrarily create a temperature. Here's an example of creating a temperature conversion with a trait, and then using that trait with an `enum` implementation as a means of applying user output preferences:

```rust
trait TemperatureConversion {
    fn as_celsius(&self) -> f32;
    fn as_farenheit(&self) -> f32;
}

struct Temperature {
    kelvin: f32
}

impl Temperature {
    fn with_celsius(celsius: f32) -> Self {
        Self { kelvin: celsius + 273.15 }
    }
    
    fn with_farenheit(farenheit: f32) -> Self {
        Self { kelvin: ((farenheit - 32.0) * 5.0 / 9.0) + 273.15 }
    }
}

impl TemperatureConversion for Temperature {
    fn as_celsius(&self) -> f32 {
        self.kelvin - 273.15
    }
    
    fn as_farenheit(&self) -> f32 {
        ((self.kelvin - 273.15) * 9.0/5.0) + 32.0
    }
}

enum TemperaturePreference {
    Celsius,
    Farenheit,
}

impl TemperaturePreference {
    fn display(&self, temperature: impl TemperatureConversion) -> String {
        match self {
            Self::Celsius => format!("{:.0}°C", temperature.as_celsius()),
            Self::Farenheit => format!("{:.0}°F", temperature.as_farenheit()),
        }
    }
}

fn main() {
    let temperature = Temperature::with_celsius(100.0);
    let preference = TemperaturePreference::Farenheit;
    println!("{}", preference.display(temperature));
}
```
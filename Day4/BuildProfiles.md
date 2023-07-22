# Build Profiles

You should already know that you can run your program in debug mode with `cargo run`, and release mode with `cargo run --release`. The latter enables `O3` optimization, and does a pretty decent job. You can customize the build optimization process.

## Faster Debugging

By default, the `debug` profile bounds checks everything and produces completely unoptimized code. That's generally a good thing---but in some cases it can make the debug profile too slow to run. You can customize the debug profile to be faster, at the expense of debuggers occasionally "skipping" optimized code. In your `Cargo.toml` file (the parent, for a workspace---all workspace elements share the same profile), you can add:

```toml
[profile.dev]
opt-level = 1
```

You can customize this further. If integer overflow checking is causing you problems, you can disable it:

```toml
[profile.dev]
overflow-checks = false
```

> I don't recommend this, most of the time!

## Link-Time Optimization

If you don't mind slower compiles for release mode, you can enable LTO. LTO extends "inlining" checks across crate boundaries, and can often produce smaller and faster code. You can enable it with:

```toml
[profile.release]
lto = true
```

The downside is that LTO can be quite slow. If you're doing a lot of development, you may want to disable it.

## Making Your Binaries Smaller

Sometimes, you need a smaller binary. You might be deploying to an embedded target and not have much space. You might be shipping a binary to customers and want to minimize download usage (and weight on their target). You might just be picky about saving space (I am!). Rust has a lot of options for this.

Let's establish a baseline by compiling the `axum_sqlx` binary in release mode with `cargo build --release` from the appropriate directory.

The optimized binary is 4,080,128 bytes. It's large because Rust has statically linked every dependency. This is a good thing, because it means that you can ship a single binary to your customers. But it's also a bad thing, because it means that you're shipping a large binary to your customers.

### LTO

Let's start by enabling LTO by adding this to `Cargo.toml`:

```toml
[profile.release]
lto = true
```

Now we rebuilt with `cargo build --release`---it takes quite a bit longer, especially the link portion. The binary has shrunk to 3,464,704 bytes in size. That's a decent improvement---but we can do better!

### Optimize for Size

We'll add another line to `Cargo.toml`:

```toml
[profile.release]
lto = true
opt-level = "s"
```

This tells the underlying compiler to optimize for a small binary---but not to great extremes. Now we rebuild it again. It actually compiles slightly faster, because it's skipping some of the more expensive optimizations. We're down to 2,277,888 bytes!

We can go a step further and replace the `opt-level` with `z`:

```toml
[profile.release]
lto = true
opt-level = "z"
```

`z` tells the compiler to optimize for size, and still perform performance optimizations---but always preferring size benefits. Compile again, and we're now 2,227,712 bytes. A very small improvement, but a lot smaller than our original 4mb.

Now we're going to "strip" the binary. Remove all debug symbols, nice naming hints and similar. This also anonymizes the binary a bit if you like to hide your secret sauce. Add this to `Cargo.toml`:

```toml
[profile.release]
lto = true
opt-level = "z"
strip = "debuginfo"
```

Once again we rebuild, we're at 2,226,176 bytes. A truly tiny improvement, because release builds already remove a lot of information. You've also just lost the ability to trace crashes to a line number. You can use `strip = "symbols"` to retain the debugger information.

Since we've killed debug information, we don't really need such nice handling of panics. Displaying nice panic handler messages is surprisingly expensive! You can turn it off as follows:

```toml
[profile.release]
lto = true
opt-level = "z"
strip = "debuginfo"
panic = "abort"
```

Rebuilding this way, we're down to 1,674,752 bytes. If the program crashes, the error message won't help you find the issue---you're relying on having properly handled errors and tracing. You should be doing that anyway!

When Rust compiles, it uses as many CPU cores as possible. This actually removes some optimization opportunities! If you don't mind a really slow compile time, you can disable this:

```toml
[profile.release]
lto = true
opt-level = "z"
strip = "debuginfo"
panic = "abort"
codegen-units = 1
```

You can now only compile one code unit at a time, but optimizations won't be skipped because the build was divided between cores. This results in a relatively tiny improvement: our binary is now 1,614,336 bytes. (1.53 Mb)

That's not bad for an application with full database support, migrations and a web server!

More extreme optimizations are possible if you use `no_std` mode---but then you're writing without the standard library, and will be doing a lot of things from scratch. That's a topic for another day!
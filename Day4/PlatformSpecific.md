# Platform and Feature Specific Code

It's quite common if you're shipping a binary to run into a situation in which you need to do something differently on different platforms. For example, you might want to use a different library on Windows than you do on Linux. You might want to use a different file path separator. You might want to use a different system call.

## Use Library Code

A lot of platform abstraction is done for you by the standard library. For example, prefer using `File` over the `nix` crate to obtain a file handle. For memory-mapping, the `memmap2` crate handles many of the platform issues for you.

## Use Config Directives

You can make blocks of code conditionally compile based on feature flags or platform. For example:

```rust
#[cfg(all(not(feature = "opengl"), feature = "cross_term"))]
```

Will only compile if the `opengl` feature is disabled, and the `cross_term` feature is enabled. You'll often need blobs combining feature combinations to determine what to compile. It gets messy fast.

To minimize the mess, define a common interface. It could be a trait, or it could be a structure that will always offer the same methods (the trait is cleaner). Put each platform/feature implementation in a separate module and make the compilation decision at *module inclusion time*. For example:

```rust
#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
mod native;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
pub use native::*;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
mod wasm;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
pub use wasm::*;
```

Now when you compile, it only includes the appropriate module and shares the common type defined in each of the modules. That's a great way to share functionality between platform-specific implementations (which can be managed by different teams, even) without resorting to dynamic dispatch.
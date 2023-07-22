# Use `cargo fmt`

`cargo fmt` is Rust's built-in code formatter. You can configure a style guide by setting up `rustfmt.toml` and following [this guide](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=). Staying close to the standard format is recommended.

## Format early, format often

You can dig yourself into a bit of a hole by only occasionally remembering to format your code. Take the following scenario:

1. Developer A writes some code and doesn't format it.
2. Developer B goes to fix a bug in the code, and *does* format it.
3. Developer B's patch now contains a lot of formatting changes, which makes it hard to review.

So: run `cargo fmt` often, before each commit.

## Checking that everyone remembered to format their code

You can use `cargo fmt -- --check` to check that all code is formatted. This is a good thing to add to your CI pipeline. In a workspace, you can use `cargo fmt -- --check --all` to check the entire workspace.

You can also add it to your git hooks (assuming you are using git). Add to or create `.git/hooks/pre-commit`:

```bash
#!/bin/bash

diff=$(cargo fmt -- --check)
result=$?

if [[ ${result} -ne 0 ]] ; then
    cat <<\EOF
There are some code style issues, run `cargo fmt` first.
EOF
    exit 1
fi

exit 0
```

> Don't forget to make it executable with `chmod u+x .git/hooks/pre-commit`!

## Excluding Some Code from Formatting

Sometimes, you like the way something is formatted---even though it's not standard. You can exclude it from formatting by adding a `#[rustfmt::skip]` attribute to the item. For example:

```rust
#[rustfmt::skip]
mod unformatted {
    pub fn add(a : i32, b : i32) -> i32 { a + b }
    pub fn sub(a : i32, b : i32) -> i32 { a - b }
}
```

This is particularly handy when you are working with a table and you like the tabular formatting, or generated code.

## Excluding Whole Files from Formatting

If you have generated code that winds up being committed (particularly common if you are generating bindings for FFI), you can add `ignore` to your `rustfmt.toml` file. For example:

```toml
ignore = [
    "src/my_c_bindings.rs", # Ignore a file
    "src/bindgen", # Ignore a directory
]
```

It uses the same rules as `.gitignore`.

## Automatically Formatting in VSCode

If you're using my favorite editor (VS Code), you can set a hook to run code formatting when you save a file. Edit `settings.json` and add the following:

```json
{
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust", // Makes the magic
        "editor.formatOnSave": true // Optional
    },
}
```

## Using Cargo Watch

You can install a tool named `cargo-watch` with `cargo install cargo-watch`. Once `cargo-watch` is installed, you can run:

```bash
cargo watch -x 'fmt'
```

Leave it running. Open a file, make a change, and save it. You'll see the formatting run.

> Cargo Watch is a very powerful command. You can use it to re-run servers on changes, run tests (this can be slow), etc. See [the documentation](https://github.com/watchexec/cargo-watch) for more information.
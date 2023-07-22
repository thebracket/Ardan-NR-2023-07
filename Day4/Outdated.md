# Checking for Outdated Dependencies

You can install the `cargo-outdated` tool with:

```bash
cargo install cargo-outdated
```

Then you can run it with:

```bash
cargo outdated
```

> Add the `-w` flag to check the whole workspace.

Hopefully, you'll see `All dependencies are up to date, yay!`

Sometimes, you'll see a list of dependencies that are out of date. You can *usually* update them with:

```bash
cargo update
```

When you're depending on a crate that in turn depends upon other crates, it's quite possible that the version number has been pinned somewhere down the chain (or you pinned it yourself).

> I do **not** recommend putting this into your CI pipeline. Rust crates update a LOT.
# Use `cargo fmt`

`cargo fmt` is Rust's built-in code formatter. You can configure a style guide by setting up `rustfmt.toml` and following [this guide](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=). Staying close to the standard format is recommended.

## Format early, format often

You can dig yourself into a bit of a hole by only occasionally remembering to format your code. Take the following scenario:

1. Developer A writes some code and doesn't format it.
2. Developer B goes to fix a bug in the code, and *does* format it.
3. Developer B's patch now contains a lot of formatting changes, which makes it hard to review.

So: run `cargo fmt` often, before each commit.

## Checking that everyone remembered to format their code

You can use `cargo fmt -- --check` to check that all code is formatted. This is a good thing to add to your CI pipeline.

You can also add it to your git hooks (assuming you are using git). On platforms that have bash, add to/create `.git/hooks/pre-commit`:

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
# Denying Dependencies

Sometimes, you *really* don't want to accidentally include some GPL code (and suffer the viral consequences of the license). Or you may want to apply other licensing restrictions---depending upon your company's legal department.

The [cargo deny](https://github.com/EmbarkStudios/cargo-deny) tool can help with this. You can install it with `cargo install cargo-deny`.

## Initial Setup

Setup with `cargo deny init`. This will make a (huge) `deny.toml` file that you can use to set your policies. Let's allow crates that didn't remember to specify a license (not a great idea), and specifically deny GPL:

```toml
[licenses]
# The lint level for crates which do not have a detectable license
unlicensed = "allow"
# List of explicitly allowed licenses
# See https://spdx.org/licenses/ for list of possible licenses
# [possible values: any SPDX 3.11 short identifier (+ optional exception)].
allow = [
    #"MIT",
    #"Apache-2.0",
    #"Apache-2.0 WITH LLVM-exception",
]
# List of explicitly disallowed licenses
# See https://spdx.org/licenses/ for list of possible licenses
# [possible values: any SPDX 3.11 short identifier (+ optional exception)].
deny = [
    #"Nokia",
    "GPL-1.0",
    "GPL-2.0",
]
# Lint level for licenses considered copyleft
copyleft = "warn"
```

## Checking Licenses

`cargo deny check licenses` will scan your entire workspace for licenses. With any luck, you'll see `licenses ok`. Revert `unlicensed` to `deny`, and you'll discover that `rustnr` forgot to specify a license. Wait! That's me! I didn't specify a license in the top-level of this project. Generally, you want to include a `license=` clause for your project licenses---unless you don't want one.

## Other Checks

Cargo Deny can also check:

* You can use `cargo deny check bans` to check for features or dependencies you've decided to ban.
* `cargo deny check advisories` will duplicate the functionality of `cargo audit` and check for CVEs. I do recommend `cargo audit` for CI use, it's a lot slimmer.
* `cargo deny check sources` allows you to ban importing code from specific sources.

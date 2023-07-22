# Checking for Vulnerabilities

Install the `cargo audit` tool with `cargo install cargo-audit`.

Now, at any time you can run `cargo audit` to check for vulnerabilities in your dependencies. This is a good thing to do periodically, and before you publish a crate. GitHub includes tooling for building this into your CI pipeline. Run it at the top-level of your workspace---it works by reading `Cargo.lock`.

For example, when I wrote this it warned me that the `memmap` crate we used is currently without a maintainer:

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 554 security advisories (from C:\Users\Herbert\.cargo\advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (284 crate dependencies)
Crate:     memmap
Version:   0.7.0
Warning:   unmaintained
Title:     memmap is unmaintained
Date:      2020-12-02
ID:        RUSTSEC-2020-0077
URL:       https://rustsec.org/advisories/RUSTSEC-2020-0077
Dependency tree:
memmap 0.7.0
└── count-lines-mmap 0.1.0

warning: 1 allowed warning found
```

This is an easy fix (`memmap` was replaced by `memmap2` which is almost identical).

This is a good tool to include in your CI pipeline. You may find that it's irritating---sometimes vulnerabilities don't affect what you're doing, sometimes it takes a little while for a fix to become available. This way, at least you know that there's action required!
# Understanding Dependencies

Rust has a great dependency management system in Cargo. It's easy to use, and it's easy to find and use libraries. On the other hand, it leads to a JavaScript/NPM like situation in which you can have a *lot* of dependencies. This can lead to difficulties auditing your code, accidental licensing issues (see [Denying Dependencies by License](./Deny.md)), and other problems. You can also run into the age-old problem of depending upon a crate, that crate updating, and your program breaking!

## Listing Your Dependencies

You can list your dependencies with:

```bash
cargo tree
```

Our `count-lines-mmap` project is nice and straightforward:

```
count-lines-mmap v0.1.0 (C:\Users\Herbert\Documents\Ardan\RustNR-2023-07\code\count-lines-mmap)
└── memmap2 v0.7.1
```

Our `axum_sqlx` project has a *lot* of dependencies:

```
axum_sqlx v0.1.0 (C:\Users\Herbert\Documents\Ardan\RustNR-2023-07\code\axum_sqlx)
├── axum v0.6.19
│   ├── async-trait v0.1.72 (proc-macro)
│   │   ├── proc-macro2 v1.0.66
│   │   │   └── unicode-ident v1.0.11
│   │   ├── quote v1.0.31
│   │   │   └── proc-macro2 v1.0.66 (*)
│   │   └── syn v2.0.27
│   │       ├── proc-macro2 v1.0.66 (*)
│   │       ├── quote v1.0.31 (*)
│   │       └── unicode-ident v1.0.11
│   ├── axum-core v0.3.4
│   │   ├── async-trait v0.1.72 (proc-macro) (*)
│   │   ├── bytes v1.4.0
│   │   ├── futures-util v0.3.28
│   │   │   ├── futures-core v0.3.28
│   │   │   ├── futures-io v0.3.28
│   │   │   ├── futures-sink v0.3.28
│   │   │   ├── futures-task v0.3.28
│   │   │   ├── memchr v2.5.0
│   │   │   ├── pin-project-lite v0.2.10
│   │   │   ├── pin-utils v0.1.0
│   │   │   └── slab v0.4.8
│   │   │       [build-dependencies]
│   │   │       └── autocfg v1.1.0
│   │   ├── http v0.2.9
│   │   │   ├── bytes v1.4.0
│   │   │   ├── fnv v1.0.7
│   │   │   └── itoa v1.0.9
│   │   ├── http-body v0.4.5
│   │   │   ├── bytes v1.4.0
│   │   │   ├── http v0.2.9 (*)
│   │   │   └── pin-project-lite v0.2.10
│   │   ├── mime v0.3.17
│   │   ├── tower-layer v0.3.2
│   │   └── tower-service v0.3.2
│   │   [build-dependencies]
│   │   └── rustversion v1.0.14 (proc-macro)
│   ├── bitflags v1.3.2
│   ├── bytes v1.4.0
│   ├── futures-util v0.3.28 (*)
│   ├── http v0.2.9 (*)
│   ├── http-body v0.4.5 (*)
│   ├── hyper v0.14.27
│   │   ├── bytes v1.4.0
│   │   ├── futures-channel v0.3.28
│   │   │   ├── futures-core v0.3.28
│   │   │   └── futures-sink v0.3.28
│   │   ├── futures-core v0.3.28
│   │   ├── futures-util v0.3.28 (*)
│   │   ├── http v0.2.9 (*)
│   │   ├── http-body v0.4.5 (*)
│   │   ├── httparse v1.8.0
│   │   ├── httpdate v1.0.2
│   │   ├── itoa v1.0.9
│   │   ├── pin-project-lite v0.2.10
│   │   ├── socket2 v0.4.9
│   │   │   └── winapi v0.3.9
│   │   ├── tokio v1.29.1
│   │   │   ├── bytes v1.4.0
│   │   │   ├── mio v0.8.8
│   │   │   │   └── windows-sys v0.48.0
│   │   │   │       └── windows-targets v0.48.1
│   │   │   │           └── windows_x86_64_msvc v0.48.0
│   │   │   ├── num_cpus v1.16.0
│   │   │   ├── parking_lot v0.12.1
│   │   │   │   ├── lock_api v0.4.10
│   │   │   │   │   └── scopeguard v1.2.0
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── autocfg v1.1.0
│   │   │   │   └── parking_lot_core v0.9.8
│   │   │   │       ├── cfg-if v1.0.0
│   │   │   │       ├── smallvec v1.11.0
│   │   │   │       └── windows-targets v0.48.1 (*)
│   │   │   ├── pin-project-lite v0.2.10
│   │   │   ├── socket2 v0.4.9 (*)
│   │   │   ├── tokio-macros v2.1.0 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.66 (*)
│   │   │   │   ├── quote v1.0.31 (*)
│   │   │   │   └── syn v2.0.27 (*)
│   │   │   └── windows-sys v0.48.0 (*)
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.1.0
│   │   ├── tower-service v0.3.2
│   │   ├── tracing v0.1.37
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── log v0.4.19
│   │   │   ├── pin-project-lite v0.2.10
│   │   │   ├── tracing-attributes v0.1.26 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.66 (*)
│   │   │   │   ├── quote v1.0.31 (*)
│   │   │   │   └── syn v2.0.27 (*)
│   │   │   └── tracing-core v0.1.31
│   │   │       └── once_cell v1.18.0
│   │   └── want v0.3.1
│   │       └── try-lock v0.2.4
│   ├── itoa v1.0.9
│   ├── matchit v0.7.0
│   ├── memchr v2.5.0
│   ├── mime v0.3.17
│   ├── percent-encoding v2.3.0
│   ├── pin-project-lite v0.2.10
│   ├── serde v1.0.174
│   │   └── serde_derive v1.0.174 (proc-macro)
│   │       ├── proc-macro2 v1.0.66 (*)
│   │       ├── quote v1.0.31 (*)
│   │       └── syn v2.0.27 (*)
│   ├── serde_json v1.0.103
│   │   ├── itoa v1.0.9
│   │   ├── ryu v1.0.15
│   │   └── serde v1.0.174 (*)
│   ├── serde_path_to_error v0.1.14
│   │   ├── itoa v1.0.9
│   │   └── serde v1.0.174 (*)
│   ├── serde_urlencoded v0.7.1
│   │   ├── form_urlencoded v1.2.0
│   │   │   └── percent-encoding v2.3.0
│   │   ├── itoa v1.0.9
│   │   ├── ryu v1.0.15
│   │   └── serde v1.0.174 (*)
│   ├── sync_wrapper v0.1.2
│   ├── tokio v1.29.1 (*)
│   ├── tower v0.4.13
│   │   ├── futures-core v0.3.28
│   │   ├── futures-util v0.3.28 (*)
│   │   ├── pin-project v1.1.2
│   │   │   └── pin-project-internal v1.1.2 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.66 (*)
│   │   │       ├── quote v1.0.31 (*)
│   │   │       └── syn v2.0.27 (*)
│   │   ├── pin-project-lite v0.2.10
│   │   ├── tokio v1.29.1 (*)
│   │   ├── tower-layer v0.3.2
│   │   ├── tower-service v0.3.2
│   │   └── tracing v0.1.37 (*)
│   ├── tower-layer v0.3.2
│   └── tower-service v0.3.2
│   [build-dependencies]
│   └── rustversion v1.0.14 (proc-macro)
├── serde v1.0.174 (*)
├── sqlx v0.7.1
│   ├── sqlx-core v0.7.1
│   │   ├── ahash v0.8.3
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── getrandom v0.2.10
│   │   │   │   └── cfg-if v1.0.0
│   │   │   └── once_cell v1.18.0
│   │   │   [build-dependencies]
│   │   │   └── version_check v0.9.4
│   │   ├── atoi v2.0.0
│   │   │   └── num-traits v0.2.16
│   │   │       [build-dependencies]
│   │   │       └── autocfg v1.1.0
│   │   ├── byteorder v1.4.3
│   │   ├── bytes v1.4.0
│   │   ├── crc v3.0.1
│   │   │   └── crc-catalog v2.2.0
│   │   ├── crossbeam-queue v0.3.8
│   │   │   ├── cfg-if v1.0.0
│   │   │   └── crossbeam-utils v0.8.16
│   │   │       └── cfg-if v1.0.0
│   │   ├── dotenvy v0.15.7
│   │   ├── either v1.8.1
│   │   │   └── serde v1.0.174 (*)
│   │   ├── event-listener v2.5.3
│   │   ├── futures-channel v0.3.28 (*)
│   │   ├── futures-core v0.3.28
│   │   ├── futures-intrusive v0.5.0
│   │   │   ├── futures-core v0.3.28
│   │   │   ├── lock_api v0.4.10 (*)
│   │   │   └── parking_lot v0.12.1 (*)
│   │   ├── futures-io v0.3.28
│   │   ├── futures-util v0.3.28 (*)
│   │   ├── hashlink v0.8.3
│   │   │   └── hashbrown v0.14.0
│   │   │       ├── ahash v0.8.3 (*)
│   │   │       └── allocator-api2 v0.2.16
│   │   ├── hex v0.4.3
│   │   ├── indexmap v2.0.0
│   │   │   ├── equivalent v1.0.1
│   │   │   └── hashbrown v0.14.0 (*)
│   │   ├── log v0.4.19
│   │   ├── memchr v2.5.0
│   │   ├── once_cell v1.18.0
│   │   ├── paste v1.0.14 (proc-macro)
│   │   ├── percent-encoding v2.3.0
│   │   ├── serde v1.0.174 (*)
│   │   ├── serde_json v1.0.103 (*)
│   │   ├── sha2 v0.10.7
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── cpufeatures v0.2.9
│   │   │   └── digest v0.10.7
│   │   │       ├── block-buffer v0.10.4
│   │   │       │   └── generic-array v0.14.7
│   │   │       │       └── typenum v1.16.0
│   │   │       │       [build-dependencies]
│   │   │       │       └── version_check v0.9.4
│   │   │       └── crypto-common v0.1.6
│   │   │           ├── generic-array v0.14.7 (*)
│   │   │           └── typenum v1.16.0
│   │   ├── smallvec v1.11.0
│   │   ├── sqlformat v0.2.1
│   │   │   ├── itertools v0.10.5
│   │   │   │   └── either v1.8.1 (*)
│   │   │   ├── nom v7.1.3
│   │   │   │   ├── memchr v2.5.0
│   │   │   │   └── minimal-lexical v0.2.1
│   │   │   └── unicode_categories v0.1.1
│   │   ├── thiserror v1.0.44
│   │   │   └── thiserror-impl v1.0.44 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.66 (*)
│   │   │       ├── quote v1.0.31 (*)
│   │   │       └── syn v2.0.27 (*)
│   │   ├── tokio v1.29.1 (*)
│   │   ├── tokio-stream v0.1.14
│   │   │   ├── futures-core v0.3.28
│   │   │   ├── pin-project-lite v0.2.10
│   │   │   └── tokio v1.29.1 (*)
│   │   ├── tracing v0.1.37 (*)
│   │   └── url v2.4.0
│   │       ├── form_urlencoded v1.2.0 (*)
│   │       ├── idna v0.4.0
│   │       │   ├── unicode-bidi v0.3.13
│   │       │   └── unicode-normalization v0.1.22
│   │       │       └── tinyvec v1.6.0
│   │       │           └── tinyvec_macros v0.1.1
│   │       └── percent-encoding v2.3.0
│   ├── sqlx-macros v0.7.1 (proc-macro)
│   │   ├── proc-macro2 v1.0.66 (*)
│   │   ├── quote v1.0.31 (*)
│   │   ├── sqlx-core v0.7.1 (*)
│   │   ├── sqlx-macros-core v0.7.1
│   │   │   ├── dotenvy v0.15.7
│   │   │   ├── either v1.8.1 (*)
│   │   │   ├── heck v0.4.1
│   │   │   │   └── unicode-segmentation v1.10.1
│   │   │   ├── hex v0.4.3
│   │   │   ├── once_cell v1.18.0
│   │   │   ├── proc-macro2 v1.0.66 (*)
│   │   │   ├── quote v1.0.31 (*)
│   │   │   ├── serde v1.0.174 (*)
│   │   │   ├── serde_json v1.0.103 (*)
│   │   │   ├── sha2 v0.10.7
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   ├── cpufeatures v0.2.9
│   │   │   │   └── digest v0.10.7
│   │   │   │       ├── block-buffer v0.10.4 (*)
│   │   │   │       └── crypto-common v0.1.6
│   │   │   │           ├── generic-array v0.14.7 (*)
│   │   │   │           └── typenum v1.16.0
│   │   │   ├── sqlx-core v0.7.1 (*)
│   │   │   ├── sqlx-sqlite v0.7.1
│   │   │   │   ├── atoi v2.0.0 (*)
│   │   │   │   ├── flume v0.10.14
│   │   │   │   │   ├── futures-core v0.3.28
│   │   │   │   │   ├── futures-sink v0.3.28
│   │   │   │   │   ├── pin-project v1.1.2 (*)
│   │   │   │   │   └── spin v0.9.8
│   │   │   │   │       └── lock_api v0.4.10 (*)
│   │   │   │   ├── futures-channel v0.3.28
│   │   │   │   │   ├── futures-core v0.3.28
│   │   │   │   │   └── futures-sink v0.3.28
│   │   │   │   ├── futures-core v0.3.28
│   │   │   │   ├── futures-executor v0.3.28
│   │   │   │   │   ├── futures-core v0.3.28
│   │   │   │   │   ├── futures-task v0.3.28
│   │   │   │   │   └── futures-util v0.3.28 (*)
│   │   │   │   ├── futures-intrusive v0.5.0 (*)
│   │   │   │   ├── futures-util v0.3.28 (*)
│   │   │   │   ├── libsqlite3-sys v0.26.0
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   ├── cc v1.0.79
│   │   │   │   │   ├── pkg-config v0.3.27
│   │   │   │   │   └── vcpkg v0.2.15
│   │   │   │   ├── log v0.4.19
│   │   │   │   ├── percent-encoding v2.3.0
│   │   │   │   ├── serde v1.0.174 (*)
│   │   │   │   ├── sqlx-core v0.7.1 (*)
│   │   │   │   ├── tracing v0.1.37 (*)
│   │   │   │   └── url v2.4.0 (*)
│   │   │   ├── syn v1.0.109
│   │   │   │   ├── proc-macro2 v1.0.66 (*)
│   │   │   │   ├── quote v1.0.31 (*)
│   │   │   │   └── unicode-ident v1.0.11
│   │   │   ├── tempfile v3.7.0
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   ├── fastrand v2.0.0
│   │   │   │   └── windows-sys v0.48.0
│   │   │   │       └── windows-targets v0.48.1 (*)
│   │   │   ├── tokio v1.29.1
│   │   │   │   ├── bytes v1.4.0
│   │   │   │   ├── mio v0.8.8 (*)
│   │   │   │   ├── pin-project-lite v0.2.10
│   │   │   │   ├── socket2 v0.4.9 (*)
│   │   │   │   └── windows-sys v0.48.0 (*)
│   │   │   │   [build-dependencies]
│   │   │   │   └── autocfg v1.1.0
│   │   │   └── url v2.4.0 (*)
│   │   └── syn v1.0.109 (*)
│   └── sqlx-sqlite v0.7.1
│       ├── atoi v2.0.0 (*)
│       ├── flume v0.10.14 (*)
│       ├── futures-channel v0.3.28 (*)
│       ├── futures-core v0.3.28
│       ├── futures-executor v0.3.28 (*)
│       ├── futures-intrusive v0.5.0 (*)
│       ├── futures-util v0.3.28 (*)
│       ├── libsqlite3-sys v0.26.0 (*)
│       ├── log v0.4.19
│       ├── percent-encoding v2.3.0
│       ├── serde v1.0.174 (*)
│       ├── sqlx-core v0.7.1 (*)
│       ├── tracing v0.1.37 (*)
│       └── url v2.4.0 (*)
└── tokio v1.29.1 (*)
```

In most cases, using `full` feature flags adds the kitchen sink to your project. If you're size conscious, or worried about dependencies, try to trim your feature flag usage.

In a highly secure environment, it's pretty unlikely that you can audit all of those. Ultimately, it's your decision as to how much you want to trust dependencies, verses bringing code "in house".

## Finding The Bloat

The `cargo bloat` (installed with `cargo install cargo-bloat`) command can "weigh" each of your dependencies and see how much space it is adding to your binary. For example, for the `axum-sqlx` project:

```
    Analyzing target\debug\axum_sqlx.exe

 File  .text    Size        Crate Name
 0.7%   0.9% 45.9KiB              sqlite3VdbeExec
 0.5%   0.7% 34.1KiB    sqlformat sqlformat::tokenizer::get_plain_reserved_token
 0.5%   0.6% 31.9KiB  sqlx_sqlite sqlx_sqlite::connection::explain::explain
 0.4%   0.5% 28.6KiB              yy_reduce
 0.3%   0.4% 22.3KiB    sqlx_core sqlx_core::logger::QueryLogger::finish
 0.3%   0.4% 22.1KiB              sqlite3Pragma
 0.3%   0.4% 20.0KiB enum2$<hyper enum2$<hyper::proto::h1::role::Server>::encode_headers<hyper::proto::h1::role::impl$1::encode_headers_with_orig...
 0.3%   0.4% 20.0KiB enum2$<hyper enum2$<hyper::proto::h1::role::Server>::encode_headers<hyper::proto::h1::role::impl$1::encode_headers_with_lowe...
 0.3%   0.3% 18.1KiB         http http::header::name::StandardHeader::from_bytes
 0.3%   0.3% 16.9KiB        hyper <hyper::proto::h1::role::Server as hyper::proto::h1::Http1Transaction>::parse
 0.2%   0.3% 15.7KiB  sqlx_sqlite sqlx_sqlite::logger::QueryPlanLogger<O,R,P>::finish
 0.2%   0.3% 14.0KiB              sqlite3WhereCodeOneLoopStart
 0.2%   0.2% 11.6KiB        hyper hyper::server::tcp::AddrIncoming::poll_next_
 0.2%   0.2% 11.6KiB              sqlite3Select
 0.2%   0.2% 11.0KiB        hyper hyper::proto::h1::dispatch::Dispatcher<hyper::proto::h1::dispatch::Server<axum::routing::Router<tuple$<>,hyper:...
 0.2%   0.2% 10.6KiB        hyper hyper::proto::h1::conn::Conn<I,B,T>::poll_read_body
 0.2%   0.2% 10.2KiB    sqlformat <(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U) as nom::branch::Alt<Input,Output,Error>>::choice
 0.2%   0.2% 10.2KiB    sqlformat <(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U) as nom::branch::Alt<Input,Output,Error>>::choice
 0.2%   0.2% 10.2KiB    sqlformat <(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U) as nom::branch::Alt<Input,Output,Error>>::choice
 0.2%   0.2% 10.2KiB    sqlformat <(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U) as nom::branch::Alt<Input,Output,Error>>::choice
69.6%  88.6%  4.5MiB              And 21717 smaller methods. Use -n N to show more.
78.6% 100.0%  5.1MiB              .text section size, the file size is 6.5MiB
```

If you're size conscious, you can use this to find the biggest offenders and either look for an alternative, or see which feature flag you can omit.

## Vendoring Dependencies

A consistent build is very important to shipping applications. You can run `cargo vendor` to download the current versions of all of your dependencies to a `vendor/` directory and build with those. This can take a while to run and use a lot of disk space.

As you approach release, you should `pin` your dependencies. So instead of:

```toml
[dependencies]
axum = "0.6.19"
```

Specify the EXACT version:

```toml
axum = "=0.6.19"
```
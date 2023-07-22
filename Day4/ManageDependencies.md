# Managing Your Own Dependencies

Cargo is pretty smart, and gives you some options for how you want dependencies to work within your own repo(s).

## Option 1: Publish Everything!

If you *really* want to, you can use `cargo publish` (after filling out all of the crate metadata) to make your crates public. You should only do this:

* If you actually want your crate to be public. The source code will be published, too.
* You have a license policy ready to go.
* Your crate offers something useful to the community. Let's not get into a `left-pad` situation.

For your company's pride and joy---a product you make money from---this is often not the best choice. In fact, your manager may be quite upset if you accidentally MIT license your core product.

## Option 2: Separate Git Repos Per Project

Cargo can handle dependencies directly from `git`. For example, you can use `bracket-lib` (a library I created for *Hands-on Rust*) from Github directly:

```toml
bracket_lib = { git = "https://github.com/amethyst/bracket-lib.git" }
```

You can tell it to use a specific branch:

```toml
bracket_lib = { git = "https://github.com/amethyst/bracket-lib.git", branch = "bevy" }
```

Now, when you run `cargo update`---Cargo pulls the latest version from the repo and uses it.

### Paths within Git Repos

Cargo supports committing a workspace or series of projects to the same repo. The committed repos can refer to one another with relative paths:

```toml
bracket_lib = { path = "../bracket_lib" }
```

If you then link to that git repo, dependencies within the repo are pulled correctly.

## Option 3: Mono-Repos

Using the `path =` system within the mono-repo works great. You can mix and match a bit, having some git repos, some cargo/crates.io repos, and some path repos.

## Option 4: File Server

If you all mount a directory you can share code that way. The downside is that git ownership becomes quite confusing.

## Overriding Dependencies

Let's say that Team B has created a crate, and you are happily depending upon it. Down the line, Team A want to test the dependency from Team B---but they want to replace a dependency throughout the project with a new version (maybe to see if a patch works).

You can do this with cargo patches. For example, if you decide to use your own `uuid` crate---you can add this to `Cargo.toml`:

```toml
[patch.crates-io]
uuid = { path = "../path/to/uuid" }
```

Be warned that you can't publish (to crates.io) any crates that specify a patch.
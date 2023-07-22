# Taming Compile Times

Rust is infamous for compilation times getting painful. It's improving rapidly, but there's still a lot you can do to help yourself.

* Get fast storage. Storage speed makes a huge difference!
* Use workspaces to share build artifacts. You don't need to rebuild `serde` every time you test part of your workspace!
* Break code into modules---in separate files/directories. Rust can compile modules concurrently, while a single large file has to be processed together.
* Divide into crates. Crates won't be recompiled at all if they didn't change.
* Use Macros Sparingly.
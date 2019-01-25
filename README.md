# guile-sys [![Docs](https://docs.rs/guile-sys-2/badge.svg)](https://docs.rs/guile-sys-2)

Low-level (bindgen) Rust bindings to [GNU Guile](https://www.gnu.org/software/guile/). 

[Another binding exists](https://github.com/shymega/guile-rs/tree/master/guile-sys). However:

1) It targets an older version of guile
2) It does not use `build.rs`. Beyond the [overall tradeoffs](https://rust-lang.github.io/rust-bindgen/library-usage.html), Guile provides a CLI for determining linker and compiler arguments. With `build.rs`, we can use this so that users don't have to manually specify, e.g. library and header paths.

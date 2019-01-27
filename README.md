# guile-sys [![Docs](https://docs.rs/guile-sys-2/badge.svg)](http://yusufsimonson.com/doc/guile/guile_sys)

Low-level (bindgen) Rust bindings to [GNU Guile](https://www.gnu.org/software/guile/). 

You probably don't want to use this or any guile bindings for rust. Guile liberally uses `setjmp`/`longjmp`, which breaks rust destructors.

[Documentation is available here](http://yusufsimonson.com/doc/guile/guile_sys). Note that it was generated on macOS. If you use a different platform, some of the things in the docs won't be available to you, and some things that you have access to will not be in the docs; e.g. you can disregard anything with darwin in its name. Generally these items should not be directly used anyways.

[Another binding exists](https://github.com/shymega/guile-rs/tree/master/guile-sys). However:

1) It targets an older version of guile.
2) It does not use `build.rs`. Beyond the [overall tradeoffs](https://rust-lang.github.io/rust-bindgen/library-usage.html), this uses `build.rs` to automatically determine linker and compiler arguments for ease of use.

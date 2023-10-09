- wait until `rustdoc_types::Type` support is added to [trustfall-rustdoc-adapter](https://github.com/obi1kenobi/trustfall-rustdoc-adapter) and replace getting constant type via ID with a direct query of a `type` property

Alternative ways to get all the contstants
- check out the way the Rust project itself is built via [bindgen](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen) from [WinMD](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen/default) windows metadata files and see if it's possible to tweak it to create a hashmap, check [winmd format](https://learn.microsoft.com/en-us/uwp/winrt-cref/winmd-files)
- maybe test `proc-macro2` to parse Windows_sys source files to get the missing constants in a crate that would dump the database?
- maybe test if `rust-analyzer` is able to parse all the contants?

# A library to build an elos (scanner) plugin in Rust

An elos plugin is a shared library implementing a struct with the symbol name `elosPluginConfig` with function pointers to load, start, stop & unload the plugin.
To implement this in Rust, set the crate-type to `cdynlib` in the Cargo.toml.
With this library the `Plugin` trait needs to be implemented and an `ElosPluginConfig` instance initialized with the `load_plugin!`, `start_plugin!`, `stop_plugin!` & `unload_plugin!` macros. Like The example here in the [src/lib.rs]

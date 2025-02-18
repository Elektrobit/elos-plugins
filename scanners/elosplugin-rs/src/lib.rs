//! To build an elos plugin with this library the crate type needs to be set to `cdylib` to get a
//! sharde object file and not a rust crate
//!
//! ```toml
//! crate-type = ["rlib", "cdylib"]
//! ```

pub mod event;
#[macro_use]
pub mod plugin;

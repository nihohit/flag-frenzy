//! Metadata loader and schema for `Cargo.toml` files.
//! 
//! `flag-frenzy` uses the [`cargo-metadata`](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)
//! command to discover the feature flags of packages in a workspace. Use [`load_metadata()`] to
//! load the [`Metadata`] of a `Cargo.toml` file.

mod loader;
mod schema;

pub use self::loader::load_metadata;
pub use self::schema::*;

//! rfesi - "Rust for ESI"
//!
//! [ESI] bindings in Rust.
//!
//! [ESI]: http://esi.evetech.net/

#![deny(clippy::all)]

mod builders;
pub mod client;
mod errors;
pub mod groups;
mod models;

pub use builders::EsiBuilder;
pub use client::{Esi, UrlBase};
pub use errors::EsiError;

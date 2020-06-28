//! rfesi - "Rust for ESI"
//!
//! [ESI] bindings in Rust.
//!
//! [ESI]: http://esi.evetech.net/

#![deny(clippy::all)]

mod builders;
mod client;
mod errors;

pub use builders::EsiBuilder;
pub use client::Esi;
pub use errors::EsiError;

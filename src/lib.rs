//! rfesi - "Rust for ESI"
//!
//! [ESI] bindings in Rust for [EVE Online].
//!
//! This crate provides helpful utilities and bindings for interacting
//! with ESI, including the SSO auth flow and making public and
//! authenticated calls to ESI without having to deal with JSON yourself.
//!
//! To make an EVE third-party application, visit the [developers site].
//!
//! # Example of the authorization flow
//!
//! ```rust,no_run
//! use rfesi::{Esi, EsiBuilder, EsiError, EsiResult};
//!
//! fn create_esi() -> EsiResult<Esi> {
//!     // Create a new struct from the builder. These parameters
//!     // all come from your third-party app on the developers site.
//!     EsiBuilder::new()
//!         .user_agent("some user agent")
//!         .client_id("your_client_id")
//!         .client_secret("your_client_secret")
//!         .callback_url("your_callback_url")
//!         .build()
//! }
//!
//! fn get_authorize_url(esi: &Esi) -> String {
//!     // Direct your user to this URL, and have a web service listening
//!     // at the callback URL that you specified in the EVE application.
//!     esi.get_authorize_url()
//! }
//!
//! async fn authenticate_user(esi: &mut Esi, code: &str) -> EsiResult<()> {
//!     // The `code` value here comes from the URL parameters your service
//!     // is sent following a user's successful SSO.
//!     //
//!     // Note that most functions in this crate are async, so you'll need
//!     // to handle those appropriately.
//!     //
//!     // Additionally, this function requires a mutable reference to the
//!     // struct, as the instance will self-mutate with the additional information
//!     // from ESI (assuming a successful authorization).
//!     //
//!     // Once the instance has the auth information, you can use it to make
//!     // authenticated requests to ESI for the user.
//!     esi.authenticate(code).await?;
//!     Ok(())
//! }
//! ```
//!
//! [ESI]: http://esi.evetech.net
//! [EVE Online]: https://www.eveonline.com
//! [developers site]: https://developers.eveonline.com

#![deny(clippy::all)]

mod builders;
mod client;
mod errors;
pub mod groups;

pub use builders::EsiBuilder;
pub use client::{Esi, RequestType};
pub use errors::{EsiError, EsiResult};

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
//! # Example of the authorization flow (https://docs.esi.evetech.net/docs/sso/web_based_sso_flow.html)
//!
//! ```rust,no_run
//! use rfesi::prelude::*;
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
//! fn get_authorize_url(esi: &Esi) -> AuthenticationInformation {
//!     // Direct your user to the tuple's first item, a URL, and have a web service listening
//!     // at the callback URL that you specified in the EVE application. The second item is
//!     // the random state variable, which is up to you to check.
//!     esi.get_authorize_url().unwrap()
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
//!     esi.authenticate(code, None).await?;
//!     Ok(())
//! }
//! ```
//! # Example of the authorization flow (Using PKCE/Application flow - See https://docs.esi.evetech.net/docs/sso/native_sso_flow.html)
//!
//! ```rust,no_run
//! use rfesi::prelude::*;
//!
//! fn create_esi() -> EsiResult<Esi> {
//!     // Create a new struct from the builder. These parameters
//!     // all come from your third-party app on the developers site.
//!     EsiBuilder::new()
//!         .user_agent("some user agent")
//!         .client_id("your_client_id")
//!         .callback_url("your_callback_url")
//!         .enable_application_authentication(true)
//!         .build()
//! }
//!
//! fn get_authorize_url(esi: &Esi) -> AuthenticationInformation {
//!     // Direct your user to the tuple's first item, a URL, and have a web service listening
//!     // at the callback URL that you specified in the EVE application. The second item is
//!     // the random state variable, which is up to you to check.
//!     esi.get_authorize_url().unwrap()
//! }
//!
//! async fn authenticate_user(esi: &mut Esi, code: &str, pkce_verifier: PkceVerifier) -> EsiResult<()> {
//!     // The `code` value here comes from the URL parameters your service
//!     // is sent following a user's successful SSO.
//!     // The 'pkce_verifier' is randomly generated and returned by the previously called
//!     // 'get_authorize_url()' method, in the AuthenticationInformation struct.
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
//!     esi.authenticate(code, Some(pkce_verifier)).await?;
//!     Ok(())
//! }
//! ```
//!
//! [ESI]: http://esi.evetech.net
//! [EVE Online]: https://www.eveonline.com
//! [developers site]: https://developers.eveonline.com

#![deny(clippy::all)]
#![deny(missing_docs)]

#[macro_use]
mod macros;

mod builders;
mod client;
mod errors;
pub mod groups;
#[cfg(feature = "validate_jwt")]
mod jwt_util;
mod pkce;
pub mod prelude;

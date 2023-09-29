//! Module for easy imports.

pub use crate::builders::EsiBuilder;
pub use crate::client::{Esi, RequestType};
pub use crate::errors::{EsiError, EsiResult};
pub(crate) use serde::Deserialize;

/// Access token (JWT) payload.
///
/// For more information on the content of this struct, see
/// the [ESI documentation].
///
/// [ESI documentation]: https://docs.esi.evetech.net/docs/sso/validating_eve_jwt.html
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct TokenClaims {
    pub aud: String,
    pub azp: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub jti: String,
    pub kid: String,
    pub name: String,
    pub owner: String,
    pub region: String,
    pub sub: String,
    pub tenant: String,
    pub tier: String,
}

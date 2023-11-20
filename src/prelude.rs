//! Module for easy imports.

pub use crate::builders::EsiBuilder;
pub use crate::client::{AuthenticationInformation, Esi, RequestType};
pub use crate::errors::{EsiError, EsiResult};
pub use crate::pkce::PkceVerifier;
pub(crate) use serde::{Deserialize, Serialize};

/// Access token (JWT) payload.
///
/// For more information on the content of this struct, see
/// the [ESI documentation].
///
/// [ESI documentation]: https://docs.esi.evetech.net/docs/sso/validating_eve_jwt.html
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(missing_docs)]
pub struct TokenClaims {
    pub aud: Vec<String>,
    pub azp: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub jti: String,
    pub kid: String,
    pub name: String,
    pub owner: String,
    pub region: String,
    pub scp: Option<serde_json::Value>,
    pub sub: String,
    pub tenant: String,
    pub tier: String,
}

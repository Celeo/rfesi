//! Errors

use thiserror::Error;

/// Errors that can occur when dealing with ESI.
#[derive(Debug, Error)]
pub enum EsiError {
    #[error("Missing `Esi` struct value '{0}'")]
    EmptyClientValue(String),
    #[error("Invalid HTTP status code received: {0}")]
    InvalidStatusCode(u16),
    #[error("Invalid HTTP header value")]
    InvalidUserAgentHeader(#[from] http::header::InvalidHeaderValue),
    #[error("Error constructing HTTP client")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Invalid HTTP method")]
    HttpMethodError(#[from] http::method::InvalidMethod),
}

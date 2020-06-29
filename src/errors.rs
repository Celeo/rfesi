//! Errors

use thiserror::Error;

/// Errors that can occur when dealing with ESI.
#[derive(Debug, Error)]
pub enum EsiError {
    #[error("Missing required builder struct value '{0}'")]
    EmptyClientValue(String),
    #[error("Invalid HTTP status code received: {0}")]
    InvalidStatusCode(u16),
    #[error("Invalid HTTP header value")]
    InvalidUserAgentHeader(#[from] http::header::InvalidHeaderValue),
    #[error("Error constructing HTTP client")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Invalid HTTP method")]
    HttpMethodError(#[from] http::method::InvalidMethod),
    #[error("This endpoint requires an access token")]
    MissingAuthentication,
    #[error("Could not resolve operationId '{0}' to a URL path")]
    UnknownOperationID(String),
    #[error("Error occurred while parsing the Swagger spec at: {0}")]
    FailedSpecParse(String),
}

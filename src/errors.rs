//! Errors

use thiserror::Error;

/// Errors that can occur when dealing with ESI.
#[derive(Debug, Error)]
pub enum EsiError {
    /// Error that can be thrown if the `EsiBuilder` struct is
    /// invalid when `.build()` is called.
    #[error("Missing required builder struct value '{0}'")]
    EmptyClientValue(String),
    /// You have to retrieve the ESI spec via `Esi::update_spec`
    /// before making this call.
    #[error("Missing spec")]
    EmptySpec,
    /// Error that could be thrown if the access token JWT from SSO
    /// is invalid, whether due to tampering or some other reason.
    #[error("Invalid JWT: {0}")]
    InvalidJWT(String),
    /// Validation of the JWT failed.
    #[error("JWT validation failed")]
    JwtValidationFailed(#[from] alcoholic_jwt::ValidationError),
    /// Error that can be thrown by any function that makes HTTP
    /// calls our to external resources for response codes that
    /// aren't valid as defined [by reqwest].
    /// [by reqwest]: https://docs.rs/reqwest/0.10.6/reqwest/struct.StatusCode.html#method.is_success
    #[error("Invalid HTTP status code received: {0}")]
    InvalidStatusCode(u16),
    /// Error for if the provided user-agent header value has invalid characters.
    #[error("Invalid HTTP header value")]
    InvalidUserAgentHeader(#[from] http::header::InvalidHeaderValue),
    /// Error for if the underlying `reqwest::Client` could not be constructed.
    #[error("Error constructing HTTP client")]
    ReqwestError(#[from] reqwest::Error),
    /// Error for if the String cannot be converted into a valid HTTP method.
    #[error("Invalid HTTP method")]
    HttpMethodError(#[from] http::method::InvalidMethod),
    /// Error for if a request is made to an endpoint that requires authentication,
    /// but no access token is present in the Esi struct.
    #[error("This endpoint requires an access token")]
    MissingAuthentication,
    /// Error for not finding the passed operationId in the ESI Swagger spec.
    #[error("Could not resolve operationId '{0}' to a URL path")]
    UnknownOperationID(String),
    /// Error for being unable to parse the Swagger spec from ESI.
    #[error("Error occurred while parsing the Swagger spec at: {0}")]
    FailedSpecParse(String),
    /// Error for being unable to parse JSON from anywhere.
    #[error("Failed to serialize/deserialize JSON")]
    FailedJsonParse(#[from] serde_json::Error),
    /// Error for being unable to get the current timestamp.
    #[error("Could not get current timestamp: {0}")]
    Timestamp(#[from] std::time::SystemTimeError),
    /// Error for the access token being used after expiring (and therefore
    /// being unable to be used for ESI) and no refresh token being present
    /// to fetch another access token.
    #[error("Access token is expired, and no refresh token is present")]
    AccessTokenExpired,
}

/// Crate `Result` wrapper.
pub type EsiResult<T> = Result<T, EsiError>;

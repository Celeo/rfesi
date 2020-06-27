//! rfesi - "Rust for ESI"
//!
//! [ESI] bindings in Rust.
//!
//! [ESI]: http://esi.evetech.net/
//!
//!

#![deny(clippy::all)]

use reqwest::{header, Client};
use std::time::Duration;
use thiserror::Error;

/// Struct to interact with the ESI.
#[derive(Debug)]
pub struct Esi {
    version: String,
    client_id: String,
    client_secret: String,
    callback_url: String,
    access_token: Option<String>,
    access_expiration: Option<u64>,
    refresh_token: Option<String>,

    client: Client,
}

/// Builder for the `Esi` struct.
#[derive(Clone, Debug, Default)]
pub struct EsiBuilder {
    version: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    callback_url: Option<String>,
    access_token: Option<String>,
    access_expiration: Option<u64>,
    refresh_token: Option<String>,
    user_agent: Option<String>,
    http_timeout: Option<u64>,
}

impl EsiBuilder {
    /// Start a new builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the version of the ESI to use.
    ///
    /// Will default to `"latest"` if not set.
    pub fn version(mut self, val: &str) -> Self {
        self.version = Some(val.to_owned());
        self
    }

    pub fn client_id(mut self, val: &str) -> Self {
        self.client_id = Some(val.to_owned());
        self
    }

    pub fn client_secret(mut self, val: &str) -> Self {
        self.client_secret = Some(val.to_owned());
        self
    }

    pub fn callback_url(mut self, val: &str) -> Self {
        self.callback_url = Some(val.to_owned());
        self
    }

    pub fn access_token(mut self, val: Option<&str>) -> Self {
        self.access_token = val.map(|v| v.to_owned());
        self
    }

    pub fn access_expiration(mut self, val: Option<u64>) -> Self {
        self.access_expiration = val;
        self
    }

    pub fn refresh_token(mut self, val: Option<&str>) -> Self {
        self.refresh_token = val.map(|v| v.to_owned());
        self
    }

    pub fn user_agent(mut self, val: &str) -> Self {
        self.user_agent = Some(val.to_owned());
        self
    }

    /// Set the timeout to use in millis when sending HTTP requests.
    ///
    /// Will default to 60,000 (1 minute) if not set.
    pub fn http_timeout(mut self, val: Option<u64>) -> Self {
        self.http_timeout = val;
        self
    }

    fn construct_client(&self) -> Result<Client, EsiError> {
        let http_timeout = self
            .http_timeout
            .map(|u| Duration::from_millis(u))
            .unwrap_or(Duration::from_secs(60));
        let headers = {
            let mut map = header::HeaderMap::new();
            let user_agent = &self
                .user_agent
                .as_ref()
                .ok_or(EsiError::EmptyClientValue("user_agent".to_owned()))?
                .to_owned();
            map.insert(
                header::USER_AGENT,
                header::HeaderValue::from_str(user_agent)?,
            );
            map.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("application/json"),
            );
            map
        };
        let client = Client::builder()
            .timeout(http_timeout)
            .default_headers(headers)
            .build()?;
        Ok(client)
    }

    /// Construct the `Esi` instance, consuming the builder.
    ///
    /// There are a few things that could go wrong:
    ///     - not setting one of the mandatory fields
    ///     - providing a user agent that is not a valid HTTP header value
    pub fn build(self) -> Result<Esi, EsiError> {
        let client = self.construct_client()?;
        let e = Esi {
            version: self.version.unwrap_or("latest".to_owned()),
            client_id: self
                .client_id
                .ok_or(EsiError::EmptyClientValue("client_id".to_owned()))?,
            client_secret: self
                .client_secret
                .ok_or(EsiError::EmptyClientValue("client_secret".to_owned()))?,
            callback_url: self
                .callback_url
                .ok_or(EsiError::EmptyClientValue("callback_url".to_owned()))?,
            access_token: self.access_token,
            access_expiration: self.access_expiration,
            refresh_token: self.refresh_token,
            client,
        };
        Ok(e)
    }
}

/// Errors that can occur when dealing with the ESI.
#[derive(Debug, Error)]
pub enum EsiError {
    #[error("Missing `Esi` struct value '{0}'")]
    EmptyClientValue(String),
    #[error("Invalid status code received: {0}")]
    InvalidStatusCode(String),
    #[error("Invalid HTTP header value")]
    InvalidUserAgentHeader(#[from] http::header::InvalidHeaderValue),
    #[error("Error constructing HTTP client")]
    ReqwestError(#[from] reqwest::Error),
}

#[allow(unused)]
const BASE_URL: &str = "https://esi.tech.ccp.is/";
#[allow(unused)]
const OAUTH_URL: &str = "https://loign.eveonline.com/oauth/";

#[cfg(test)]
mod tests {
    use super::EsiBuilder;

    #[test]
    fn test_builder_valid() {
        let b = EsiBuilder::new()
            .client_id("a")
            .client_secret("b")
            .callback_url("c")
            .user_agent("d")
            .build()
            .unwrap();

        assert_eq!(b.client_id, "a");
        assert_eq!(b.client_secret, "b");
        assert_eq!(b.callback_url, "c");
        assert_eq!(b.version, "latest");
        assert_eq!(b.access_token, None);
    }

    #[test]
    fn test_builder_missing_value() {
        let res = EsiBuilder::new().build();
        assert!(res.is_err());
        let s = format!("{}", res.unwrap_err());
        assert_eq!(s, "Missing `Esi` struct value 'user_agent'");
    }
}

//! Builders

use crate::prelude::*;
use reqwest::{header, Client};
use std::time::Duration;

/// Builder for the `Esi` struct.
///
/// # Example
/// ```rust
/// # use rfesi::EsiBuilder;
/// let mut esi = EsiBuilder::new()
///     .user_agent("some user agent")
///     .client_id("your_client_id")
///     .client_secret("your_client_secret")
///     .callback_url("your_callback_url")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug, Default)]
pub struct EsiBuilder {
    pub(crate) version: Option<String>,
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
    pub(crate) callback_url: Option<String>,
    pub(crate) scope: Option<String>,
    pub(crate) access_token: Option<String>,
    pub(crate) access_expiration: Option<u64>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) user_agent: Option<String>,
    pub(crate) http_timeout: Option<u64>,
}

impl EsiBuilder {
    /// Start a new builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the version of ESI to use.
    ///
    /// Will default to `"latest"` if not set.
    pub fn version(mut self, val: &str) -> Self {
        self.version = Some(val.to_owned());
        self
    }

    /// Set the client_id.
    pub fn client_id(mut self, val: &str) -> Self {
        self.client_id = Some(val.to_owned());
        self
    }

    /// Set the client_secret.
    pub fn client_secret(mut self, val: &str) -> Self {
        self.client_secret = Some(val.to_owned());
        self
    }

    /// Set the callback_url.
    pub fn callback_url(mut self, val: &str) -> Self {
        self.callback_url = Some(val.to_owned());
        self
    }

    /// Set the scope.
    pub fn scope(mut self, val: &str) -> Self {
        self.scope = Some(val.to_owned());
        self
    }

    /// Set the access_token.
    pub fn access_token(mut self, val: Option<&str>) -> Self {
        self.access_token = val.map(|v| v.to_owned());
        self
    }

    /// Set the access_expiration.
    pub fn access_expiration(mut self, val: Option<u64>) -> Self {
        self.access_expiration = val;
        self
    }

    /// Set the refresh_token.
    pub fn refresh_token(mut self, val: Option<&str>) -> Self {
        self.refresh_token = val.map(|v| v.to_owned());
        self
    }

    /// Set the user_agent.
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

    pub(crate) fn construct_client(&self) -> EsiResult<Client> {
        let http_timeout = self
            .http_timeout
            .map(Duration::from_millis)
            .unwrap_or_else(|| Duration::from_secs(60));
        let headers = {
            let mut map = header::HeaderMap::new();
            let user_agent = &self
                .user_agent
                .as_ref()
                .ok_or_else(|| EsiError::EmptyClientValue("user_agent".to_owned()))?
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

    /// Construct the `Esi` instance.
    ///
    /// There are a few things that could go wrong, like
    /// not setting one of the mandatory fields or providing a user
    /// agent that is not a valid HTTP header value.
    pub fn build(self) -> EsiResult<Esi> {
        Esi::from_builder(self)
    }
}

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
        assert_eq!(s, "Missing required builder struct value 'user_agent'");
    }
}

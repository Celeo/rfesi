//! Main logic

use crate::{models::auth, EsiBuilder, EsiError};
use log::debug;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Method,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::str::FromStr;

const BASE_URL: &str = "https://esi.evetech.net/";
const OAUTH_URL: &str = "https://login.eveonline.com/oauth/";
const AUTHORIZE_URL: &str = "https://login.eveonline.com/oauth/authorize";
const TOKEN_URL: &str = "https://login.eveonline.com/oauth/token";
const SPEC_URL_START: &str = "https://esi.evetech.net/_";
const SPEC_URL_END: &str = "/swagger.json";

/// Which base URL to start with - the public URL for unauthenticated
/// calls, or the authenticated URL for making calls to endpoints that
/// require an access token.
#[derive(Debug, PartialEq)]
pub enum UrlBase {
    Public,
    Authenticated,
}

/// Struct to interact with ESI.
///
/// Construct an instance of this struct using [`EsiBuilder`](./struct.EsiBuilder.html).
#[derive(Debug)]
pub struct Esi {
    pub(crate) version: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) callback_url: String,
    pub(crate) scope: String,
    pub(crate) access_token: Option<String>,
    pub(crate) access_expiration: Option<u64>,
    pub(crate) refresh_token: Option<String>,
    /// HTTP client
    pub(crate) client: Client,
    pub(crate) spec: Value,
}

impl Esi {
    /// Consume the builder, creating an instance of this struct.
    pub(crate) async fn from_builder(builder: EsiBuilder) -> Result<Self, EsiError> {
        let client = builder.construct_client()?;
        let version = builder.version.unwrap_or_else(|| "latest".to_owned());
        let spec = Esi::get_spec(&client, &version).await?;
        let e = Esi {
            version,
            client_id: builder
                .client_id
                .ok_or_else(|| EsiError::EmptyClientValue("client_id".to_owned()))?,
            client_secret: builder
                .client_secret
                .ok_or_else(|| EsiError::EmptyClientValue("client_secret".to_owned()))?,
            callback_url: builder
                .callback_url
                .ok_or_else(|| EsiError::EmptyClientValue("callback_url".to_owned()))?,
            scope: builder.scope.unwrap_or_else(|| "".to_owned()),
            access_token: builder.access_token,
            access_expiration: builder.access_expiration,
            refresh_token: builder.refresh_token,
            client,
            spec,
        };
        Ok(e)
    }

    /// Get the Swagger spec from ESI.
    pub(crate) async fn get_spec(client: &Client, version: &str) -> Result<Value, EsiError> {
        let resp = client
            .get(&format!("{}{}{}", SPEC_URL_START, version, SPEC_URL_END))
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
        }
        let data: Value = resp.json().await?;
        Ok(data)
    }

    pub fn get_authorize_url(&self) -> String {
        format!(
            "{}?response_type=code&redirect_uri={}&client_id={}&scope={}",
            AUTHORIZE_URL, self.callback_url, self.client_id, self.scope
        )
    }

    fn get_auth_headers(&self) -> Result<HeaderMap, EsiError> {
        let mut map = HeaderMap::new();
        let value = base64::encode(format!("{}:{}", self.client_id, self.client_secret))
            .replace("\n", "")
            .replace(" ", "");
        map.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", value))?,
        );
        Ok(map)
    }

    /// Authenticate with ESI, exchanging a code from the authorize flow
    /// for an access token that is used to make authenticated calls to ESI.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run() {
    /// # use rfesi::EsiBuilder;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .await
    /// #     .unwrap();
    /// esi.authenticate("abcdef").await.unwrap();
    /// # }
    /// ```
    pub async fn authenticate(&mut self, code: &str) -> Result<(), EsiError> {
        debug!("Authenticating with code {}", code);
        let resp = self
            .client
            .post(TOKEN_URL)
            .headers(self.get_auth_headers()?)
            .json(&json!({
                "grant_type": "authorization_code",
                "code": code,
            }))
            .send()
            .await?;
        if resp.status() != 200 {
            return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
        }
        let data: auth::AuthenticateResponse = resp.json().await?;
        self.access_token = Some(data.access_token);
        self.access_expiration = Some(data.expires_in + chrono::Utc::now().timestamp() as u64);
        self.refresh_token = data.refresh_token;
        Ok(())
    }

    /// Make a request to ESI.
    ///
    /// This is mainly used as the underlying function for this
    /// library when making calls to ESI; the other functions that
    /// you should primarily be using contain more functionality,
    /// including matching endpoint with struct.
    ///
    /// In the event that there is not a wrapper function for the
    /// endpoint that you want to use, this function is what you
    /// can use to make an API call without waiting for the library
    /// to be updated.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run() {
    /// # use serde::Deserialize;
    /// # use rfesi::{EsiBuilder, UrlBase};
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .await
    /// #     .unwrap();
    /// #[derive(Deserialize)]
    /// struct ReturnedData {}
    /// let data: ReturnedData = esi.query("GET", UrlBase::Public, "abc", None).await.unwrap();
    /// # }
    /// ```
    pub async fn query<T: DeserializeOwned>(
        &self,
        method: &str,
        url_base: UrlBase,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
    ) -> Result<T, EsiError> {
        debug!(
            "Making {} request to {:?}{} with query {:?}",
            method, url_base, endpoint, query
        );
        if url_base == UrlBase::Authenticated && self.access_token.is_none() {
            return Err(EsiError::MissingAuthentication);
        }
        let headers = {
            let mut map = HeaderMap::new();
            // The 'user-agent' and 'content-type' headers are set in the default headers
            // from the builder, so all that's required here is to set the authorization
            // header, if present in the struct's data.
            match &self.access_token {
                Some(at) => {
                    map.insert(
                        header::AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {}", at))?,
                    );
                }
                None => (),
            }
            map
        };
        let url = format!(
            "{}{}",
            match url_base {
                UrlBase::Public => BASE_URL,
                UrlBase::Authenticated => OAUTH_URL,
            },
            endpoint
        );
        let req = self
            .client
            .request(Method::from_str(method)?, &url)
            .headers(headers)
            .query(query.unwrap_or_else(|| &[]))
            .build()?;
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
        }
        let data: T = resp.json().await?;
        Ok(data)
    }

    /// Gets information on the currently-authenticated user.
    pub async fn get_whoami_info(&self) -> Result<auth::WhoAmIResponse, EsiError> {
        self.query("GET", UrlBase::Authenticated, "verify", None)
            .await
    }
}

//! Main logic

use crate::{models::auth::AuthenticateResponse, EsiError};
use reqwest::{header, Client};
use serde_json::json;

// const BASE_URL: &str = "https://esi.tech.ccp.is/";
// const OAUTH_URL: &str = "https://login.eveonline.com/oauth/";
const AUTHORIZE_URL: &str = "https://login.eveonline.com/oauth/authorize";
const TOKEN_URL: &str = "https://login.eveonline.com/oauth/token";

/// Struct to interact with the ESI.
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
}

impl Esi {
    pub fn get_authorize_url(&self) -> String {
        format!(
            "{}?response_type=code&redirect_uri={}&client_id={}&scope={}",
            AUTHORIZE_URL, self.callback_url, self.client_id, self.scope
        )
    }

    fn get_auth_headers(&self) -> Result<header::HeaderMap, EsiError> {
        let mut map = header::HeaderMap::new();
        let value = base64::encode(format!("{}:{}", self.client_id, self.client_secret))
            .replace("\n", "")
            .replace(" ", "");
        map.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Basic {}", value))?,
        );
        Ok(map)
    }

    /// Authenticate with the ESI, exchanging a code from the authorize flow
    /// for an access token that is used to make authenticated calls to the ESI.
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
    /// #     .unwrap();
    /// esi.authenticate("abcdef").await.unwrap();
    /// # }
    /// ```
    pub async fn authenticate(&mut self, code: &str) -> Result<(), EsiError> {
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
        let data: AuthenticateResponse = resp.json().await?;
        self.access_token = Some(data.access_token);
        self.access_expiration = Some(data.expires_in + chrono::Utc::now().timestamp() as u64);
        self.refresh_token = data.refresh_token;
        Ok(())
    }

    pub(crate) async fn query(&self) -> Result<(), EsiError> {
        unimplemented!()
    }
}

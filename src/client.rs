//! Main logic

use std::{
    collections::HashMap,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use base64::engine::{general_purpose::STANDARD as base64, Engine};
use log::{debug, error, warn};
#[cfg(feature = "random_state")]
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Method,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::pkce::PkceVerifier;
use crate::{groups::*, pkce, prelude::*};

const BASE_URL: &str = "https://esi.evetech.net/";
const AUTHORIZE_URL: &str = "https://login.eveonline.com/v2/oauth/authorize";
const TOKEN_URL: &str = "https://login.eveonline.com/v2/oauth/token";
const SPEC_URL_START: &str = "https://esi.evetech.net/_";
const SPEC_URL_END: &str = "/swagger.json";

/// Response from SSO when exchanging a SSO code for tokens.
#[derive(Debug, Deserialize)]
struct AuthenticateResponse {
    access_token: String,
    expires_in: u64,
    refresh_token: Option<String>,
}

/// Response from SSO when exchanging a refresh token for access token.
#[derive(Debug, Deserialize)]
struct RefreshTokenAuthenticateResponse {
    access_token: String,
    expires_in: u16,
    refresh_token: String,
}

/// Which base URL to start with - the public URL for unauthenticated
/// calls, or the authenticated URL for making calls to endpoints that
/// require an access token.
#[derive(Debug, PartialEq, Eq)]
pub enum RequestType {
    /// Endpoints that do not require authentication
    Public,
    /// Endpoints that require acting on behalf of an authenticated character
    Authenticated,
}

/// AuthenticationInformation contains data needed to complete the requested authentication flow.
pub struct AuthenticationInformation {
    /// URL to call/pass to users to initiate an authentication and get an auth code from ESI.
    pub authorization_url: String,
    /// If the default feature "random_state" is enabled, the returned state field string will be
    /// random; otherwise it'll be "rfesi_unused". The ESI docs link to
    /// [this auth0 page](https://auth0.com/docs/secure/attack-protection/state-parameters)
    /// to explain. You need to check the state yourself when the response from ESI is received.
    pub state: String,
    /// Filled if you've selected PKCE authentication for application.
    /// You will need it to authenticate using the code received from ESI.
    pub pkce_verifier: Option<PkceVerifier>,
}

/// Struct to interact with ESI.
///
/// Construct an instance of this struct using [`EsiBuilder`](./struct.EsiBuilder.html).
///
/// # Example
/// ```rust,no_run
/// use rfesi::prelude::EsiBuilder;
/// // the struct must be mutable for some functionality
/// let mut esi = EsiBuilder::new()
///     .user_agent("some user agent")
///     .client_id("your_client_id")
///     .client_secret("your_client_secret")
///     .callback_url("your_callback_url")
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Esi {
    pub(crate) version: String,
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
    pub(crate) callback_url: Option<String>,
    pub(crate) scope: String,
    pub(crate) application_auth: bool,
    /// The access token from ESI, if set.
    pub access_token: Option<String>,
    /// The millisecond unix timestamp after which the access token expires, if present.
    pub access_expiration: Option<u128>,
    /// The refresh token from ESI, if set.
    pub refresh_token: Option<String>,
    /// HTTP client
    pub(crate) client: Client,
    pub(crate) spec: Option<Value>,
}

impl Esi {
    /// Consume the builder, creating an instance of this struct.
    pub(crate) fn from_builder(builder: EsiBuilder) -> EsiResult<Self> {
        let client = builder.construct_client()?;
        let version = builder.version.unwrap_or_else(|| "latest".to_owned());
        let e = Esi {
            version,
            client_id: builder.client_id,
            client_secret: builder.client_secret,
            callback_url: builder.callback_url,
            scope: builder.scope.unwrap_or_else(|| "".to_owned()),
            application_auth: builder.application_auth.unwrap_or(false),
            access_token: builder.access_token,
            access_expiration: builder.access_expiration,
            refresh_token: builder.refresh_token,
            client,
            spec: builder.spec,
        };
        Ok(e)
    }

    /// Get the Swagger spec from ESI and store it in this struct.
    ///
    /// If you are making use of the `try_get_endpoint_for_op_id`,
    /// then this function will be called there when needed
    /// (which should only really be when the struct is
    /// constructed unless the struct is kept in memory for a very
    /// long time). When using `get_endpoint_for_op_id` however,
    /// you are responsible for calling this function beforehand.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run() {
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .unwrap();
    /// esi.update_spec().await.unwrap();
    /// # }
    pub async fn update_spec(&mut self) -> EsiResult<()> {
        debug!("Updating spec with version {}", self.version);
        let resp = self
            .client
            .get(&format!(
                "{}{}{}",
                SPEC_URL_START, self.version, SPEC_URL_END
            ))
            .send()
            .await?;
        if !resp.status().is_success() {
            error!("Got status {} when requesting spec", resp.status());
            return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
        }
        let data: Value = resp.json().await?;
        self.spec = Some(data);
        Ok(())
    }

    /// Ensure the user has specified all required EVE Developer App information.
    fn check_client_info(&self) -> EsiResult<()> {
        for (name, value) in &[
            ("client_id", &self.client_id),
            ("callback_url", &self.callback_url),
        ] {
            if value.is_none() {
                return Err(EsiError::EmptyClientValue(name.to_string()));
            }
        }

        if self.client_secret.is_none() {
            if !self.application_auth {
                return Err(EsiError::MissingAuthenticationFlowInformation);
            }
        } else if self.application_auth {
            return Err(EsiError::MissingAuthenticationFlowInformation);
        }

        Ok(())
    }

    /// Generate and return the URL required for the user to grant you an auth code, as wells as
    /// infos for future authentication request.
    ///
    /// You can inspect the URL returned by ESI to your web service to ensure it matches.
    /// No checking is done by `rfesi`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .unwrap();
    /// let auth_info = esi.get_authorize_url().unwrap();
    /// // then send your user to that URL
    /// let url = auth_info.authorization_url;
    /// ```
    ///
    /// If you opted to not include client information in
    /// the EsiBuilder flow, then this function will return
    /// an error instead.
    ///
    /// [this auth0 page]: https://auth0.com/docs/secure/attack-protection/state-parameters
    pub fn get_authorize_url(&self) -> EsiResult<AuthenticationInformation> {
        self.check_client_info()?;
        #[cfg(feature = "random_state")]
        let state = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        #[cfg(not(feature = "random_state"))]
        let state = "rfesi_unused".to_string();
        let mut url = format!(
            "{}?response_type=code&redirect_uri={}&client_id={}&scope={}&state={}",
            AUTHORIZE_URL,
            self.callback_url.as_ref().unwrap(),
            self.client_id.as_ref().unwrap(),
            self.scope,
            &state
        );
        let mut pkce_verifier = None;
        // PKCE can be theoretically combined with client secret, but not sure if ESI supports it
        if self.client_secret.is_none() && self.application_auth {
            let pkce = pkce::generate()?;
            pkce_verifier = Some(pkce.verifier);
            url = format!(
                "{}&code_challenge={}&code_challenge_method=S256",
                url, pkce.challenge
            )
        }
        Ok(AuthenticationInformation {
            authorization_url: url,
            state,
            pkce_verifier,
        })
    }

    fn get_auth_headers(&self) -> EsiResult<HeaderMap> {
        self.check_client_info()?;
        let mut map = HeaderMap::new();
        if self.client_secret.is_some() {
            let value = base64
                .encode(format!(
                    "{}:{}",
                    self.client_id.as_ref().unwrap(),
                    self.client_secret.as_ref().unwrap()
                ))
                .replace(['\n', ' '], "");
            map.insert(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!("Basic {value}"))?,
            );
        }
        map.insert(
            header::HOST,
            HeaderValue::from_static("login.eveonline.com"),
        );
        Ok(map)
    }

    /// Authenticate with ESI, exchanging a code from the authorize flow
    /// for an access token that is used to make authenticated calls to ESI.
    ///
    /// Note that this is one of the functions that requires the struct be
    /// mutable, as the struct mutates to include the resulting access token.
    ///
    /// If the "validate_jwt" feature is enabled (by default), then the access
    /// token's claims will be returned. If the feature is not enabled, then
    /// the returned value will be `None`.
    ///
    /// # Example (client secret)
    /// ```rust,no_run
    /// # async fn run() {
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .unwrap();
    /// let claims = esi.authenticate("abcdef...", None).await.unwrap();
    /// # }
    /// ```
    ///
    /// # Example (PKCE/Application authentication)
    /// ```rust,no_run
    /// # use rfesi::prelude::*;
    ///  async fn run() {
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .callback_url("your_callback_url")
    /// #     .enable_application_authentication(true)
    /// #     .build()
    /// #     .unwrap();
    /// # let auth_infos = esi.get_authorize_url().unwrap();
    /// # let claims = esi.authenticate("abcdef...", auth_infos.pkce_verifier).await.unwrap();
    /// # }
    /// ```
    pub async fn authenticate(
        &mut self,
        code: &str,
        pkce_verifier: Option<PkceVerifier>,
    ) -> EsiResult<Option<TokenClaims>> {
        debug!("Authenticating with code {}", code);
        let mut body = HashMap::from([("grant_type", "authorization_code"), ("code", code)]);
        if self.application_auth {
            let option = self.client_id.as_ref();
            body.insert("client_id", option.unwrap());
            body.insert("code_verifier", pkce_verifier.as_ref().unwrap());
        }

        let resp = self
            .client
            .post(TOKEN_URL)
            .headers(self.get_auth_headers()?)
            .form(&body)
            .send()
            .await?;
        if resp.status() != 200 {
            warn!(
                "Got status {} when making call to authenticate",
                resp.status()
            );
            return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
        }
        let data: AuthenticateResponse = resp.json().await?;
        #[allow(unused_variables)]
        let claim_data: Option<TokenClaims> = None;
        #[cfg(feature = "validate_jwt")]
        let claim_data = Some(
            crate::jwt_util::validate_jwt(
                &self.client,
                &data.access_token,
                self.client_id.as_ref().unwrap(),
            )
            .await?,
        );
        self.access_token = Some(data.access_token);
        // the response's "expires_in" field is seconds but need millis
        self.access_expiration = Some((data.expires_in as u128 * 1_000) + current_time_millis()?);
        self.refresh_token = data.refresh_token;
        Ok(claim_data)
    }

    /// Authenticate via a previously-fetched refresh token.
    ///
    /// The functionality of a refresh token allows re-authenticating this struct
    /// instance without prompting the user to log into EVE SSO again. When the user
    /// is authenticated in that manner, a refresh token is returned and available
    /// via the `refresh_token` struct field. Store this securely should you wish
    /// to later make authenticate calls for that user.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run() {
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .unwrap();
    /// esi.use_refresh_token("abcdef...").await.unwrap();
    /// # }
    /// ```
    pub async fn use_refresh_token(&mut self, refresh_token: &str) -> EsiResult<()> {
        self.refresh_access_token(Some(refresh_token)).await?;
        Ok(())
    }

    /// Authenticate via a refresh token given as input, or using the internal refresh_token if it's available.
    ///
    /// The functionality of a refresh token allows re-authenticating this struct
    /// instance without prompting the user to log into EVE SSO again. When the user
    /// is authenticated in that manner, a refresh token is returned and available
    /// via the `refresh_token` struct field. Store this securely should you wish
    /// to later make authenticate calls for that user.
    ///
    /// # Example with internal token
    /// ```rust,no_run
    /// # async fn run() {
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .refresh_token(Some("MyRefreshToken"))
    /// #     .build()
    /// #     .unwrap();
    /// esi.refresh_access_token(None).await.unwrap();
    /// # }
    /// ```
    /// # Example with input token
    /// ```rust,no_run
    /// # async fn run() {
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .build()
    /// #     .unwrap();
    /// esi.refresh_access_token(Some("MyRefreshToken")).await.unwrap();
    /// # }
    /// ```
    pub async fn refresh_access_token(&mut self, refresh_token: Option<&str>) -> EsiResult<()> {
        let token = if let Some(token) = refresh_token {
            token.to_string()
        } else if let Some(token) = self.refresh_token.clone() {
            token
        } else {
            return Err(EsiError::NoRefreshTokenAvailable);
        };

        debug!("Authenticating with refresh token");
        let mut body = HashMap::from([("grant_type", "refresh_token"), ("refresh_token", &token)]);
        if self.application_auth {
            let option = self.client_id.as_ref();
            body.insert("client_id", option.unwrap());
        }
        let resp = self
            .client
            .post(TOKEN_URL)
            .headers(self.get_auth_headers()?)
            .form(&body)
            .send()
            .await?;
        if resp.status() != 200 {
            warn!(
                "Got status {} when making call to authenticate via a refresh token",
                resp.status()
            );
            return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
        }
        let data: RefreshTokenAuthenticateResponse = resp.json().await?;
        self.access_token = Some(data.access_token);
        // the response's "expires_in" field is seconds, need millis
        self.access_expiration = Some((data.expires_in as u128 * 1_000) + current_time_millis()?);
        self.refresh_token = Some(data.refresh_token);
        Ok(())
    }

    /// Make a request to ESI.
    ///
    /// This is mainly used as the underlying function for this
    /// library when making calls to ESI; the other functions that
    /// you should primarily be using contain more functionality,
    /// including matching endpoint with deserialization struct,
    /// evaluating & replacing URL parameters, etc.
    ///
    /// In the event that there is not a wrapper function for the
    /// endpoint that you want to use, you can use this function
    /// to make an API call without waiting for the library to
    /// be updated.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run() {
    /// # use serde::Deserialize;
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .unwrap();
    /// #[derive(Deserialize)]
    /// struct ReturnedData {}
    /// let data: ReturnedData = esi.query("GET", RequestType::Public, "abc", None, None).await.unwrap();
    /// # }
    /// ```
    pub async fn query<T: DeserializeOwned>(
        &self,
        method: &str,
        request_type: RequestType,
        endpoint: &str,
        query: Option<&[(&str, &str)]>,
        body: Option<&str>,
    ) -> EsiResult<T> {
        debug!(
            "Making {:?} {} request to {} with query: {:?}",
            request_type, method, endpoint, query
        );
        if request_type == RequestType::Authenticated {
            if self.access_token.is_none() {
                return Err(EsiError::MissingAuthentication);
            }
            if self.access_expiration.unwrap() < current_time_millis()? {
                return Err(EsiError::AccessTokenExpired);
            }
        }
        let headers = {
            let mut map = HeaderMap::new();
            // The 'user-agent' and 'content-type' headers are set in the default headers
            // from the builder, so all that's required here is to set the authorization
            // header, if present.
            match &self.access_token {
                Some(at) => {
                    map.insert(
                        header::AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {at}"))?,
                    );
                }
                None => (),
            }
            map
        };
        let url = format!("{BASE_URL}{endpoint}");
        let mut req_builder = self
            .client
            .request(Method::from_str(method)?, &url)
            .headers(headers)
            .query(query.unwrap_or(&[]));
        req_builder = match body {
            Some(b) => req_builder.body(b.to_owned()),
            None => req_builder,
        };
        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;
        if !resp.status().is_success() {
            return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
        }
        let text = resp.text().await?;
        let data: T = serde_json::from_str(&text)?;
        Ok(data)
    }

    /// Resolve an `operationId` to a URL path utilizing the Swagger spec.
    ///
    /// If the spec has not yet been retrieved when calling this function,
    /// an API call will be made to ESI to fetch that data (thus the
    /// async signature of this function). If you don't need that help (by
    /// explicitly making a call to `update_spec` prior) then you can use
    /// the `get_endpoint_for_op_id` function, which is synchronous.
    ///
    /// Note that when making use of this function along with `query`, you
    /// are responsible for resolving any/all URL parameters that the endpoint
    /// may contain.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn run() {
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .unwrap();
    /// let endpoint = esi
    ///     .try_get_endpoint_for_op_id("get_alliances_alliance_id_contacts_labels")
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub async fn try_get_endpoint_for_op_id(&mut self, op_id: &str) -> EsiResult<String> {
        if self.spec.is_none() {
            debug!("Spec is `None`; must fetch before looking up op_id");
            self.update_spec().await?;
        }
        self.get_endpoint_for_op_id(op_id)
    }

    /// Resolve an `operationId` to a URL path utilizing the Swagger spec.
    ///
    /// If the spec has not yet been retrieved when calling this function,
    /// this function will return an error.
    ///
    /// Note that when making use of this function along with `query`, you
    /// are responsible for resolving any/all URL parameters that the endpoint
    /// may contain.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use rfesi::prelude::*;
    /// # let mut esi = EsiBuilder::new()
    /// #     .user_agent("some user agent")
    /// #     .client_id("your_client_id")
    /// #     .client_secret("your_client_secret")
    /// #     .callback_url("your_callback_url")
    /// #     .build()
    /// #     .unwrap();
    /// let endpoint = esi.get_endpoint_for_op_id("get_alliances_alliance_id_contacts_labels").unwrap();
    /// ```
    pub fn get_endpoint_for_op_id(&self, op_id: &str) -> EsiResult<String> {
        if self.spec.is_none() {
            return Err(EsiError::EmptySpec);
        }
        let data = self
            .spec
            .as_ref()
            .ok_or_else(|| EsiError::FailedSpecParse("Unwrapping JSON Value".to_owned()))?;
        let paths = data["paths"]
            .as_object()
            .ok_or_else(|| EsiError::FailedSpecParse("Getting paths".to_owned()))?;
        for (path_str, path_obj) in paths.iter() {
            let path = path_obj
                .as_object()
                .ok_or_else(|| EsiError::FailedSpecParse("Parsing a path".to_owned()))?;
            for method in path.values() {
                let operation_id = match method["operationId"].as_str() {
                    Some(o) => o,
                    None => continue,
                };
                if operation_id == op_id {
                    // the paths contain a leading slash, so strip it
                    return Ok(path_str.chars().skip(1).collect());
                }
            }
        }
        Err(EsiError::UnknownOperationID(op_id.to_owned()))
    }

    /// Retrieve this struct's OpenAPI specification.
    ///
    /// Use in tandem with [EsiBuilder::spec].
    pub fn get_spec(&self) -> Option<&Value> {
        self.spec.as_ref()
    }

    /// Call endpoints under the "alliance" group in ESI.
    pub fn group_alliance(&self) -> AllianceGroup {
        AllianceGroup { esi: self }
    }

    /// Call endpoints under the "Assets" group in ESI.
    pub fn group_assets(&self) -> AssetsGroup {
        AssetsGroup { esi: self }
    }

    /// Call endpoints under the "Bookmarks" group in ESI.
    pub fn group_bookmarks(&self) -> BookmarksGroup {
        BookmarksGroup { esi: self }
    }

    /// Call endpoints under the "Calendar" group in ESI.
    pub fn group_calendar(&self) -> CalendarGroup {
        CalendarGroup { esi: self }
    }

    /// Call endpoints under the "Character" group in ESI.
    pub fn group_character(&self) -> CharacterGroup {
        CharacterGroup { esi: self }
    }

    /// Call endpoints under the "Clones" group in ESI.
    pub fn group_clones(&self) -> ClonesGroup {
        ClonesGroup { esi: self }
    }

    /// Call endpoints under the "Contacts" group in ESI.
    pub fn group_contacts(&self) -> ContactsGroup {
        ContactsGroup { esi: self }
    }

    /// Call endpoints under the "Contracts" group in ESI.
    pub fn group_contracts(&self) -> ContractsGroup {
        ContractsGroup { esi: self }
    }

    /// Call endpoints under the "Corporation" group in ESI.
    pub fn group_corporation(&self) -> CorporationGroup {
        CorporationGroup { esi: self }
    }

    /// Call endpoints under the "Dogma" group in ESI.
    pub fn group_dogma(&self) -> DogmaGroup {
        DogmaGroup { esi: self }
    }

    /// Call endpoints under the "FactionWarfare" group in ESI.
    pub fn group_faction_warfare(&self) -> FactionWarfareGroup {
        FactionWarfareGroup { esi: self }
    }

    /// Call endpoints under the "Fittings" group in ESI.
    pub fn group_fittings(&self) -> FittingsGroup {
        FittingsGroup { esi: self }
    }

    /// Call endpoints under the "Fleets" group in ESI.
    pub fn group_fleets(&self) -> FleetsGroup {
        FleetsGroup { esi: self }
    }

    /// Call endpoints under the "Incursions" group in ESI.
    pub fn group_incursions(&self) -> IncursionsGroup {
        IncursionsGroup { esi: self }
    }

    /// Call endpoints under the "Industry" group in ESI.
    pub fn group_industry(&self) -> IndustryGroup {
        IndustryGroup { esi: self }
    }

    /// Call endpoints under the "Insurance" group in ESI.
    pub fn group_insurance(&self) -> InsuranceGroup {
        InsuranceGroup { esi: self }
    }

    /// Call endpoints under the "Killmails" group in ESI.
    pub fn group_killmails(&self) -> KillmailsGroup {
        KillmailsGroup { esi: self }
    }

    /// Call endpoints under the "Location" group in ESI.
    pub fn group_location(&self) -> LocationGroup {
        LocationGroup { esi: self }
    }

    /// Call endpoints under the "Loyalty" group in ESI.
    pub fn group_loyalty(&self) -> LoyaltyGroup {
        LoyaltyGroup { esi: self }
    }

    /// Call endpoints under the "Mail" group in ESI.
    pub fn group_mail(&self) -> MailGroup {
        MailGroup { esi: self }
    }

    /// Call endpoints under the "Market" group in ESI.
    pub fn group_market(&self) -> MarketGroup {
        MarketGroup { esi: self }
    }

    /// Call endpoints under the "Opportunities" group in ESI.
    pub fn group_opportunities(&self) -> OpportunitiesGroup {
        OpportunitiesGroup { esi: self }
    }

    /// Call endpoints under the "PlanetaryInteraction" group in ESI.
    pub fn group_planetary_interaction(&self) -> PlanetaryInteractionGroup {
        PlanetaryInteractionGroup { esi: self }
    }

    /// Call endpoints under the "Routes" group in ESI.
    pub fn group_routes(&self) -> RoutesGroup {
        RoutesGroup { esi: self }
    }

    /// Call endpoints under the "Search" group in ESI.
    pub fn group_search(&self) -> SearchGroup {
        SearchGroup { esi: self }
    }

    /// Call endpoints under the "Skills" group in ESI.
    pub fn group_skills(&self) -> SkillsGroup {
        SkillsGroup { esi: self }
    }

    /// Call endpoints under the "Sovereignty" group in ESI.
    pub fn group_sovereignty(&self) -> SovereigntyGroup {
        SovereigntyGroup { esi: self }
    }

    /// Call endpoints under the "Status" group in ESI.
    pub fn group_status(&self) -> StatusGroup {
        StatusGroup { esi: self }
    }

    /// Call endpoints under the "Universe" group in ESI.
    pub fn group_universe(&self) -> UniverseGroup {
        UniverseGroup { esi: self }
    }

    /// Call endpoints under the "UserInterface" group in ESI.
    pub fn group_user_interface(&self) -> UserInterfaceGroup {
        UserInterfaceGroup { esi: self }
    }

    /// Call endpoints under the "Wallet" group in ESI.
    pub fn group_wallet(&self) -> WalletGroup {
        WalletGroup { esi: self }
    }

    /// Call endpoints under the "Wars" group in ESI.
    pub fn group_wars(&self) -> WarsGroup {
        WarsGroup { esi: self }
    }
}

/// Get the current system timestamp since the epoch.
fn current_time_millis() -> Result<u128, EsiError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}

#[cfg(test)]
mod tests {
    use super::AuthenticateResponse;

    #[test]
    fn test_authenticateresponse_deserialize() {
        let source = r#"{
            "access_token": "abc",
            "expires_in": 1000,
            "refresh_token": "def"
          }"#;
        let data: AuthenticateResponse = serde_json::from_str(source).unwrap();

        assert_eq!(data.access_token, "abc");
        assert_eq!(data.expires_in, 1000);
        assert_eq!(data.refresh_token, Some("def".to_owned()));
    }

    #[test]
    fn test_authenticateresponse_deserialize_no_refresh_token() {
        let source = r#"{
            "access_token": "abc",
            "expires_in": 1000,
            "refresh_token": null
          }"#;
        let data: AuthenticateResponse = serde_json::from_str(source).unwrap();

        assert_eq!(data.access_token, "abc");
        assert_eq!(data.expires_in, 1000);
        assert_eq!(data.refresh_token, None);
    }
}

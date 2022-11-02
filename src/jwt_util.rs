use crate::prelude::*;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error};
use reqwest::Client;
use serde_json::Value;

const TOKEN_AUTH_INFO_URL: &str =
    "https://login.eveonline.com/.well-known/oauth-authorization-server";

/// Validation key information.
#[derive(Debug, Deserialize)]
struct KeyInfo {
    alg: String,
    e: String,
    kid: String,
    kty: String,
    n: String,
    #[serde(rename = "use")]
    use_: String,
}

/// JWT claims.
#[derive(Debug, Deserialize)]
pub(crate) struct Claims {
    pub(crate) aud: String,
    pub(crate) sub: String,
    pub(crate) iss: String,
    pub(crate) exp: usize,
    pub(crate) name: String,
    pub(crate) tenant: String,
    pub(crate) tier: String,
    pub(crate) region: String,
}

/// Get the URL that hosts the valid JWT signing keys.
async fn get_keys_url(client: &Client) -> EsiResult<String> {
    let resp = client.get(TOKEN_AUTH_INFO_URL).send().await?;
    if resp.status() != 200 {
        error!(
            "Got status {} when making call to authenticate",
            resp.status()
        );
        return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
    }
    let data: Value = resp.json().await?;
    let url = data["jwks_uri"]
        .as_str()
        .ok_or_else(|| EsiError::InvalidJWT(String::from("Could not get keys URL")))?;
    debug!("JWT signing keys URL: {}", url);
    Ok(url.to_owned())
}

/// Fetch keys of the RS256 algorithm type from SSO.
// This function could use some improved error handling.
async fn get_keys(client: &Client) -> EsiResult<Vec<KeyInfo>> {
    let keys_url = get_keys_url(client).await?;
    let resp = client.get(&keys_url).send().await?;
    let data: Value = resp.json().await?;
    for entry in data["keys"].as_array().unwrap() {
        if entry["alg"].as_str().unwrap() == "RS256" {}
    }
    let keys = data["keys"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry["alg"].as_str().unwrap() == "RS256")
        .map(|entry| serde_json::from_value::<KeyInfo>(entry.clone()).unwrap())
        .collect();
    Ok(keys)
}

// TODO handle the "validate_jwt" feature.

/// Decode and validate the SSO JWT, returning the contents.
pub(crate) async fn validate_jwt(client: &Client, token: &str) -> EsiResult<Claims> {
    // fetch signing key
    let validation_keys = get_keys(client).await.map_err(|_| {
        EsiError::InvalidJWT(String::from("Could not get validation key information"))
    })?;
    let validation = Validation::new(Algorithm::RS256);
    // validate and decode
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            validation_keys
                .first()
                .ok_or_else(|| EsiError::InvalidJWT(String::from("No validation keys")))?
                .n
                .as_bytes(),
        ),
        &validation,
    ) {
        Ok(c) => c,
        Err(e) => {
            // TODO this fails, citing key something or other
            error!("Error validating JWT: {}", e);
            return Err(EsiError::InvalidJWT(String::from("Validation failed")));
        }
    };

    /* Additional verifications from https://docs.esi.evetech.net/docs/sso/validating_eve_jwt.html */
    if token_data.claims.iss != "login.eveonline.com"
        && token_data.claims.iss != "https://login.eveonline.com"
    {
        return Err(EsiError::InvalidJWT(String::from(
            "JWT issuer is incorrect",
        )));
    }
    if token_data.claims.exp == 0 {
        return Err(EsiError::InvalidJWT(String::from(
            "JWT expiration field is 0",
        )));
    }
    if token_data.claims.aud != "EVE Online" {
        return Err(EsiError::InvalidJWT(String::from(
            "JWT audience field is incorrect",
        )));
    }

    Ok(token_data.claims)
}

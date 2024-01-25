use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use log::error;
use reqwest::Client;
use serde_json::Value;

use crate::prelude::*;

const TOKEN_AUTH_INFO_URL: &str =
    "https://login.eveonline.com/.well-known/oauth-authorization-server";

/// Get the URL that hosts the valid JWT signing keys.
async fn get_keys_url(client: &Client) -> EsiResult<String> {
    let resp = client.get(TOKEN_AUTH_INFO_URL).send().await?;
    if resp.status() != 200 {
        error!(
            "Got status {} when making call to get token info",
            resp.status()
        );
        return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
    }
    let data: Value = resp.json().await?;
    let url = data["jwks_uri"]
        .as_str()
        .ok_or_else(|| EsiError::InvalidJWT(String::from("Could not get keys URL")))?;
    Ok(url.to_owned())
}

/// Get the RS256 key to use.
async fn get_rs256_key(client: &Client) -> EsiResult<String> {
    let keys_url = get_keys_url(client).await?;
    let resp = client.get(&keys_url).send().await?;
    let data: Value = resp.json().await?;
    let key = data["keys"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entry| entry["alg"].as_str().unwrap() == "RS256")
        .map(|entry| serde_json::to_string(entry).unwrap())
        .next()
        .ok_or_else(|| EsiError::InvalidJWT(String::from("Could not find an RS256 key")))?;
    Ok(key)
}

/// Decode and validate the JWT token
fn validate(
    token: &str,
    client_id: &str,
    decoding_key: &DecodingKey,
) -> Result<TokenClaims, EsiError> {
    let mut validations = Validation::new(Algorithm::RS256);
    validations.required_spec_claims = vec![String::from("sub")].into_iter().collect();
    let aud = vec![client_id, "EVE Online"];
    validations.set_audience(&aud);

    let token: TokenData<Value> = decode(token, decoding_key, &validations)?;
    /* Additional verifications from https://docs.esi.evetech.net/docs/sso/validating_eve_jwt.html */
    if token.claims["iss"].as_str().unwrap() != "login.eveonline.com"
        && token.claims["iss"].as_str().unwrap() != "https://login.eveonline.com"
    {
        return Err(EsiError::InvalidJWT(String::from(
            "JWT issuer is incorrect",
        )));
    }

    let token_claims = serde_json::from_value(token.claims)?;
    Ok(token_claims)
}

/// Decode and validate the SSO JWT, returning the contents.
pub(crate) async fn validate_jwt(
    client: &Client,
    token: &str,
    client_id: &str,
) -> EsiResult<TokenClaims> {
    let validation_key_str = get_rs256_key(client).await?;
    let validation_key: Jwk = serde_json::from_str(&validation_key_str)?;
    let decoding_key = DecodingKey::from_jwk(&validation_key)?;

    validate(token, client_id, &decoding_key)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::{env, fs};

    use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
    use serde_json::Value;

    use crate::jwt_util::validate;
    use crate::prelude::TokenClaims;

    #[test]
    fn test_jwt_validity() {
        let header = Header::new(Algorithm::RS256);
        let (claim, client_id) = generate_valid_claims();
        let (private_key, public_key) = load_key();

        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
        let token = jsonwebtoken::encode(&header, &claim, &encoding_key).unwrap();

        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

        let decoded_claim = validate(&token, &client_id, &decoding_key).unwrap();

        assert_eq!(decoded_claim, claim);
    }

    #[test]
    fn test_jwt_validity_wrong_exp() {
        let header = Header::new(Algorithm::RS256);
        let (mut claim, client_id) = generate_valid_claims();
        claim.exp = (chrono::Utc::now() - chrono::Duration::minutes(5)).timestamp();
        let (private_key, public_key) = load_key();

        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
        let token = jsonwebtoken::encode(&header, &claim, &encoding_key).unwrap();

        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

        assert!(validate(&token, &client_id, &decoding_key).is_err())
    }

    #[test]
    fn test_jwt_validity_no_aud() {
        let header = Header::new(Algorithm::RS256);
        let (mut claim, client_id) = generate_valid_claims();
        claim.aud = vec![];
        let (private_key, public_key) = load_key();

        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
        let token = jsonwebtoken::encode(&header, &claim, &encoding_key).unwrap();

        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

        assert!(validate(&token, &client_id, &decoding_key).is_err())
    }

    #[test]
    fn test_jwt_validity_wrong_iss() {
        let header = Header::new(Algorithm::RS256);
        let (mut claim, client_id) = generate_valid_claims();
        claim.iss = "".to_string();
        let (private_key, public_key) = load_key();

        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
        let token = jsonwebtoken::encode(&header, &claim, &encoding_key).unwrap();

        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

        assert!(validate(&token, &client_id, &decoding_key).is_err())
    }

    fn generate_valid_claims() -> (TokenClaims, String) {
        let client_id = String::from("client_id");
        let issued = chrono::Utc::now();
        let expiry = issued + chrono::Duration::minutes(5);
        let claim = TokenClaims {
            aud: vec![client_id.clone(), "EVE Online".to_string()],
            azp: client_id.clone(),
            exp: expiry.timestamp(),
            iat: issued.timestamp(),
            iss: "https://login.eveonline.com".to_string(),
            jti: uuid::Uuid::new_v4().to_string(),
            kid: "JWT-Signature-Key".to_string(),
            name: "Xxxxxx Yyyyyyy".to_string(),
            owner: "8PmzCeTKb4VFUDrHLc/AeZXDSWM=".to_string(),
            region: "world".to_string(),
            scp: Some(Value::Array(vec![Value::String(
                "esi-skills.read_skills.v1".to_string(),
            )])),
            sub: "CHARACTER:EVE:123123".to_string(),
            tenant: "tranquility".to_string(),
            tier: "live".to_string(),
        };
        (claim, client_id)
    }

    fn load_key() -> (String, String) {
        let mut root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root_dir.push("resources/test/keys");

        let mut private_key = root_dir.clone();
        private_key.push("jwtRS256.key");
        let private_key = fs::read_to_string(private_key).unwrap();

        let mut public_key = root_dir.clone();
        public_key.push("jwtRS256.key.pub");
        let public_key = fs::read_to_string(public_key).unwrap();

        (private_key, public_key)
    }
}

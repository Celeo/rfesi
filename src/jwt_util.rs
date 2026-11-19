use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use log::error;
use reqwest::Client;
use serde_json::Value;

use crate::prelude::*;

/// Get the URL that hosts the valid JWT signing keys.
async fn get_keys_url(client: &Client, metadata_url: &str) -> EsiResult<String> {
    let resp = client.get(metadata_url).send().await?;
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

/// The set of signing keys published by the SSO.
struct Jwks;

impl Jwks {
    /// The first RS256 key of a JWKS document, as a JSON string.
    fn rs256_key(data: &Value) -> EsiResult<String> {
        let keys = data["keys"]
            .as_array()
            .ok_or_else(|| EsiError::InvalidJWT(String::from("The key set has no keys")))?;
        keys.iter()
            .find(|entry| entry["alg"].as_str() == Some("RS256"))
            .map(|entry| entry.to_string())
            .ok_or_else(|| EsiError::InvalidJWT(String::from("Could not find an RS256 key")))
    }
}

/// Get the RS256 key to use.
async fn get_rs256_key(client: &Client, metadata_url: &str) -> EsiResult<String> {
    let keys_url = get_keys_url(client, metadata_url).await?;
    let resp = client.get(&keys_url).send().await?;
    if resp.status() != 200 {
        error!(
            "Got status {} when making call to get the token keys",
            resp.status()
        );
        return Err(EsiError::InvalidStatusCode(resp.status().as_u16()));
    }
    let data: Value = resp.json().await?;
    Jwks::rs256_key(&data)
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
    // `set_audience` accepts a token that lists any of the audiences, and every
    // token lists "EVE Online": the client ID itself has to be there too.
    let for_this_client = match &token.claims["aud"] {
        Value::Array(audiences) => audiences.iter().any(|aud| aud.as_str() == Some(client_id)),
        Value::String(audience) => audience == client_id,
        _ => false,
    };
    if !for_this_client {
        return Err(EsiError::InvalidJWT(String::from(
            "JWT was not issued for this client",
        )));
    }
    let issuer = token.claims["iss"].as_str().unwrap_or_default();
    if issuer != "login.eveonline.com" && issuer != "https://login.eveonline.com" {
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
    metadata_url: &str,
    token: &str,
    client_id: &str,
) -> EsiResult<TokenClaims> {
    let validation_key_str = get_rs256_key(client, metadata_url).await?;
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

    use crate::jwt_util::{validate, Jwks};
    use crate::prelude::{EsiError, TokenClaims};

    #[test]
    fn the_key_set_yields_its_first_rs256_key() {
        let data = serde_json::json!({"keys": [
            {"alg": "ES256", "kid": "ec"},
            {"alg": "RS256", "kid": "first"},
            {"alg": "RS256", "kid": "second"}
        ]});

        let key: Value = serde_json::from_str(&Jwks::rs256_key(&data).unwrap()).unwrap();
        assert_eq!(key["kid"], "first");
    }

    #[test]
    fn a_malformed_key_set_is_an_error_not_a_panic() {
        for data in [
            serde_json::json!({}),
            serde_json::json!({"keys": "nope"}),
            serde_json::json!({"keys": []}),
            serde_json::json!({"keys": [{"kid": "no-alg"}]}),
            serde_json::json!({"keys": [{"alg": "ES256"}]}),
        ] {
            assert!(
                matches!(Jwks::rs256_key(&data), Err(EsiError::InvalidJWT(_))),
                "{data}"
            );
        }
    }

    #[test]
    fn a_token_without_an_issuer_is_rejected_not_a_panic() {
        let header = Header::new(Algorithm::RS256);
        let (private_key, public_key) = load_key();
        let (claim, client_id) = generate_valid_claims();
        let mut value = serde_json::to_value(&claim).unwrap();
        value.as_object_mut().unwrap().remove("iss");
        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
        let token = jsonwebtoken::encode(&header, &value, &encoding_key).unwrap();
        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

        assert!(matches!(
            validate(&token, &client_id, &decoding_key),
            Err(EsiError::InvalidJWT(_))
        ));
    }

    #[test]
    fn a_token_issued_for_another_client_is_rejected() {
        let header = Header::new(Algorithm::RS256);
        let (mut claim, client_id) = generate_valid_claims();
        claim.aud = vec!["another_client".to_string(), "EVE Online".to_string()];
        let (private_key, public_key) = load_key();
        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap();
        let token = jsonwebtoken::encode(&header, &claim, &encoding_key).unwrap();
        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap();

        assert!(matches!(
            validate(&token, &client_id, &decoding_key),
            Err(EsiError::InvalidJWT(_))
        ));
    }

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
        claim.exp = (jiff::Timestamp::now() - jiff::SignedDuration::from_mins(5)).as_second();
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
        let issued = jiff::Timestamp::now();
        let expiry = issued + jiff::SignedDuration::from_mins(5);
        let claim = TokenClaims {
            aud: vec![client_id.clone(), "EVE Online".to_string()],
            azp: client_id.clone(),
            exp: expiry.as_second(),
            iat: issued.as_second(),
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

//! Models relating to authenticating and authorizing.

use serde::Deserialize;

/// Response from SSO when exchanging a SSO code for tokens.
#[derive(Debug, Deserialize)]
pub(crate) struct AuthenticateResponse {
    pub(crate) access_token: String,
    pub(crate) token_type: String,
    pub(crate) expires_in: u64,
    pub(crate) refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WhoAmIResponse {
    #[serde(rename = "CharacterID")]
    character_id: u64,
    character_name: String,
    expires_on: String,
    scopes: String,
    token_type: String,
    character_owner_hash: String,
    intellectual_property: String,
}

#[cfg(test)]
mod tests {
    use super::AuthenticateResponse;

    #[test]
    fn test_authenticateresponse_deserialize() {
        let source = r#"{
            "access_token": "abc",
            "token_type": "Bearer",
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
            "token_type": "Bearer",
            "expires_in": 1000,
            "refresh_token": null
          }"#;
        let data: AuthenticateResponse = serde_json::from_str(source).unwrap();

        assert_eq!(data.access_token, "abc");
        assert_eq!(data.expires_in, 1000);
        assert_eq!(data.refresh_token, None);
    }
}

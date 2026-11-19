//! The OAuth 2.0 side of the client: the authorize URL for both flows, the PKCE
//! challenge, and the token exchange against a local stand-in for the SSO.

mod support;

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use esi_openapi::prelude::*;
use sha2::{Digest, Sha256};
use support::{Reply, Server};

struct Flow;

impl Flow {
    /// A client with a secret: the classic web-application flow.
    fn with_secret() -> EsiBuilder {
        EsiBuilder::new()
            .user_agent("oauth tests")
            .client_id("the-client")
            .client_secret("the-secret")
            .callback_url("http://localhost/callback")
            .scope("esi-skills.read_skills.v1 esi-wallet.read_character_wallet.v1")
    }

    /// A client without a secret: the PKCE flow.
    fn with_pkce() -> EsiBuilder {
        EsiBuilder::new()
            .user_agent("oauth tests")
            .client_id("the-client")
            .callback_url("http://localhost/callback")
            .enable_application_authentication(true)
    }

    /// RFC 7636 section 4.2: `BASE64URL(SHA256(ASCII(verifier)))`.
    fn challenge_of(verifier: &str) -> String {
        URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()))
    }

    /// The value of a parameter in the query of a URL.
    fn parameter(url: &str, name: &str) -> Option<String> {
        let query = url.split_once('?')?.1;
        query
            .split('&')
            .find_map(|pair| pair.strip_prefix(&format!("{name}=")))
            .map(str::to_owned)
    }

    fn token_reply() -> Reply {
        Reply::json(
            r#"{"access_token":"new-access","expires_in":1199,"refresh_token":"new-refresh","token_type":"Bearer"}"#,
        )
    }

    fn token_url(server: &Server) -> String {
        format!("{}v2/oauth/token", server.url)
    }
}

#[test]
fn the_challenge_of_the_rfc_7636_example_verifier_matches_the_rfc() {
    // Appendix B of RFC 7636. It anchors the helper the tests below rely on.
    assert_eq!(
        Flow::challenge_of("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"),
        "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
    );
}

#[test]
fn the_secret_flow_url_has_the_standard_parameters_and_no_challenge() {
    let esi = Flow::with_secret().build().unwrap();
    let info = esi.get_authorize_url().unwrap();
    let url = &info.authorization_url;

    assert_eq!(
        Flow::parameter(url, "response_type").as_deref(),
        Some("code")
    );
    assert_eq!(
        Flow::parameter(url, "client_id").as_deref(),
        Some("the-client")
    );
    assert_eq!(
        Flow::parameter(url, "redirect_uri").as_deref(),
        Some("http://localhost/callback")
    );
    assert_eq!(
        Flow::parameter(url, "scope").as_deref(),
        Some("esi-skills.read_skills.v1%20esi-wallet.read_character_wallet.v1")
    );
    assert_eq!(
        Flow::parameter(url, "state").as_deref(),
        Some(info.state.as_str())
    );
    assert!(!url.contains("code_challenge"), "{url}");
    assert_eq!(info.pkce_verifier, None);
}

#[cfg(feature = "random_state")]
#[test]
fn the_state_is_random_and_alphanumeric() {
    let esi = Flow::with_secret().build().unwrap();
    let first = esi.get_authorize_url().unwrap().state;
    let second = esi.get_authorize_url().unwrap().state;

    assert_eq!(first.len(), 10);
    assert!(first.chars().all(|c| c.is_ascii_alphanumeric()), "{first}");
    assert_ne!(first, second);
}

#[test]
fn the_pkce_url_carries_the_challenge_of_the_returned_verifier() {
    let esi = Flow::with_pkce().build().unwrap();
    let info = esi.get_authorize_url().unwrap();
    let verifier = info
        .pkce_verifier
        .expect("the PKCE flow returns a verifier");
    let url = &info.authorization_url;

    assert_eq!(
        Flow::parameter(url, "code_challenge_method").as_deref(),
        Some("S256")
    );
    assert_eq!(
        Flow::parameter(url, "code_challenge").as_deref(),
        Some(Flow::challenge_of(&verifier).as_str())
    );
}

#[test]
fn the_pkce_verifier_follows_the_rfc_7636_alphabet_and_length() {
    let esi = Flow::with_pkce().build().unwrap();
    let verifier = esi.get_authorize_url().unwrap().pkce_verifier.unwrap();

    // The RFC asks for 43 to 128 characters of [A-Za-z0-9-._~].
    assert!((43..=128).contains(&verifier.len()), "{verifier}");
    assert!(
        verifier
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-._~".contains(c)),
        "{verifier}"
    );
}

#[test]
fn every_authorization_gets_its_own_verifier() {
    let esi = Flow::with_pkce().build().unwrap();
    let first = esi.get_authorize_url().unwrap().pkce_verifier.unwrap();
    let second = esi.get_authorize_url().unwrap().pkce_verifier.unwrap();

    assert_ne!(first, second);
}

#[test]
fn the_authorize_url_needs_the_client_id_and_the_callback() {
    let no_id = EsiBuilder::new()
        .user_agent("oauth tests")
        .client_secret("the-secret")
        .callback_url("http://localhost/callback")
        .build()
        .unwrap();
    assert!(matches!(
        no_id.get_authorize_url(),
        Err(EsiError::EmptyClientValue(name)) if name == "client_id"
    ));

    let no_callback = EsiBuilder::new()
        .user_agent("oauth tests")
        .client_id("the-client")
        .client_secret("the-secret")
        .build()
        .unwrap();
    assert!(matches!(
        no_callback.get_authorize_url(),
        Err(EsiError::EmptyClientValue(name)) if name == "callback_url"
    ));
}

#[test]
fn a_secret_and_the_pkce_flow_together_are_rejected() {
    let esi = Flow::with_secret()
        .enable_application_authentication(true)
        .build()
        .unwrap();

    assert!(matches!(
        esi.get_authorize_url(),
        Err(EsiError::MissingAuthenticationFlowInformation)
    ));
}

#[test]
fn neither_a_secret_nor_the_pkce_flow_is_rejected() {
    let esi = EsiBuilder::new()
        .user_agent("oauth tests")
        .client_id("the-client")
        .callback_url("http://localhost/callback")
        .build()
        .unwrap();

    assert!(matches!(
        esi.get_authorize_url(),
        Err(EsiError::MissingAuthenticationFlowInformation)
    ));
}

#[tokio::test]
async fn refreshing_posts_the_form_with_basic_credentials_and_stores_the_answer() {
    let server = Server::start(Flow::token_reply()).await;
    let mut esi = Flow::with_secret()
        .token_url(&Flow::token_url(&server))
        .refresh_token(Some("old-refresh"))
        .build()
        .unwrap();
    let before = jiff::Timestamp::now().as_millisecond();
    esi.refresh_access_token(None).await.unwrap();
    let after = jiff::Timestamp::now().as_millisecond();

    let request = server.only();
    assert_eq!(request.method, "POST");
    assert_eq!(request.path(), "/v2/oauth/token");
    assert_eq!(
        request.form(),
        ["grant_type=refresh_token", "refresh_token=old-refresh"]
    );
    let expected = format!("Basic {}", STANDARD.encode("the-client:the-secret"));
    assert_eq!(request.header("authorization"), Some(expected.as_str()));

    assert_eq!(esi.access_token.as_deref(), Some("new-access"));
    assert_eq!(esi.refresh_token.as_deref(), Some("new-refresh"));
    let expiration = esi.access_expiration.unwrap();
    assert!(
        (before + 1_199_000..=after + 1_199_000).contains(&expiration),
        "expires_in is seconds: {expiration}"
    );
}

#[tokio::test]
async fn a_refresh_token_given_to_the_call_wins_over_the_stored_one() {
    let server = Server::start(Flow::token_reply()).await;
    let mut esi = Flow::with_secret()
        .token_url(&Flow::token_url(&server))
        .refresh_token(Some("stored"))
        .build()
        .unwrap();
    esi.refresh_access_token(Some("given")).await.unwrap();

    assert_eq!(
        server.only().form(),
        ["grant_type=refresh_token", "refresh_token=given"]
    );
}

#[tokio::test]
async fn the_pkce_refresh_sends_the_client_id_and_no_credentials() {
    let server = Server::start(Flow::token_reply()).await;
    let mut esi = Flow::with_pkce()
        .token_url(&Flow::token_url(&server))
        .refresh_token(Some("old-refresh"))
        .build()
        .unwrap();
    esi.refresh_access_token(None).await.unwrap();

    let request = server.only();
    assert_eq!(
        request.form(),
        [
            "client_id=the-client",
            "grant_type=refresh_token",
            "refresh_token=old-refresh"
        ]
    );
    assert_eq!(request.header("authorization"), None);
}

#[tokio::test]
async fn refreshing_without_any_refresh_token_does_not_call_the_sso() {
    let server = Server::start(Flow::token_reply()).await;
    let mut esi = Flow::with_secret()
        .token_url(&Flow::token_url(&server))
        .build()
        .unwrap();
    let result = esi.refresh_access_token(None).await;

    assert!(matches!(result, Err(EsiError::NoRefreshTokenAvailable)));
    assert!(server.requests().is_empty());
}

#[tokio::test]
async fn a_rejected_refresh_reports_the_status_and_keeps_the_old_tokens() {
    let server = Server::start(Reply::status(400)).await;
    let mut esi = Flow::with_secret()
        .token_url(&Flow::token_url(&server))
        .access_token(Some("still-here"))
        .refresh_token(Some("old-refresh"))
        .build()
        .unwrap();
    let result = esi.refresh_access_token(None).await;

    assert!(matches!(result, Err(EsiError::InvalidStatusCode(400))));
    assert_eq!(esi.access_token.as_deref(), Some("still-here"));
    assert_eq!(esi.refresh_token.as_deref(), Some("old-refresh"));
}

/// Without `validate_jwt` the exchange is testable offline: with it the client
/// downloads the SSO keys from a fixed address.
#[cfg(not(feature = "validate_jwt"))]
#[tokio::test]
async fn the_code_exchange_posts_the_code_and_stores_the_tokens() {
    let server = Server::start(Flow::token_reply()).await;
    let mut esi = Flow::with_secret()
        .token_url(&Flow::token_url(&server))
        .build()
        .unwrap();
    esi.authenticate("the-code", None).await.unwrap();

    let request = server.only();
    assert_eq!(request.method, "POST");
    assert_eq!(
        request.form(),
        ["code=the-code", "grant_type=authorization_code"]
    );
    assert_eq!(esi.access_token.as_deref(), Some("new-access"));
    assert_eq!(esi.refresh_token.as_deref(), Some("new-refresh"));
}

#[cfg(not(feature = "validate_jwt"))]
#[tokio::test]
async fn the_pkce_code_exchange_sends_the_verifier() {
    let server = Server::start(Flow::token_reply()).await;
    let mut esi = Flow::with_pkce()
        .token_url(&Flow::token_url(&server))
        .build()
        .unwrap();
    esi.authenticate("the-code", Some("the-verifier".to_owned()))
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(
        request.form(),
        [
            "client_id=the-client",
            "code=the-code",
            "code_verifier=the-verifier",
            "grant_type=authorization_code"
        ]
    );
    assert_eq!(request.header("authorization"), None);
}

/// The default feature: the token the SSO returns is validated against the
/// keys the metadata document points to.
#[cfg(feature = "validate_jwt")]
mod validated {
    use super::*;
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

    struct Sso;

    impl Sso {
        /// The claims of an EVE SSO token; `expires_in` is seconds from now.
        fn claims(audience: &str, issuer: &str, expires_in: i64) -> serde_json::Value {
            let now = jiff::Timestamp::now().as_second();
            serde_json::json!({
                "aud": [audience, "EVE Online"],
                "azp": audience,
                "exp": now + expires_in,
                "iat": now,
                "iss": issuer,
                "jti": "00000000-0000-4000-8000-000000000000",
                "kid": "JWT-Signature-Key",
                "name": "Test Pilot",
                "owner": "8PmzCeTKb4VFUDrHLc/AeZXDSWM=",
                "region": "world",
                "scp": ["esi-skills.read_skills.v1"],
                "sub": "CHARACTER:EVE:123123",
                "tenant": "tranquility",
                "tier": "live"
            })
        }

        fn sign(claims: &serde_json::Value) -> String {
            let key = include_str!("../resources/test/keys/jwtRS256.key");
            let mut header = Header::new(Algorithm::RS256);
            header.kid = Some("JWT-Signature-Key".to_owned());
            encode(
                &header,
                claims,
                &EncodingKey::from_rsa_pem(key.as_bytes()).unwrap(),
            )
            .unwrap()
        }

        /// A token endpoint that answers with `access_token`.
        fn token_reply(access_token: &str) -> Reply {
            Reply::json(&format!(
                r#"{{"access_token":"{access_token}","expires_in":1199,"refresh_token":"new-refresh","token_type":"Bearer"}}"#
            ))
        }

        /// A server that plays the SSO: token, metadata and keys.
        async fn server(token: Reply) -> Server {
            Server::routes(vec![
                ("/v2/oauth/token", token),
                (
                    "/.well-known",
                    Reply::json(r#"{"jwks_uri":"{base}oauth/jwks"}"#),
                ),
                (
                    "/oauth/jwks",
                    Reply::json(include_str!("../resources/test/keys/jwks.json")),
                ),
            ])
            .await
        }

        fn client(server: &Server) -> EsiBuilder {
            Flow::with_secret()
                .token_url(&Flow::token_url(server))
                .metadata_url(&format!(
                    "{}.well-known/oauth-authorization-server",
                    server.url
                ))
        }
    }

    #[tokio::test]
    async fn a_valid_token_is_stored_and_its_claims_returned() {
        let token = Sso::sign(&Sso::claims(
            "the-client",
            "https://login.eveonline.com",
            1200,
        ));
        let server = Sso::server(Sso::token_reply(&token)).await;
        let mut esi = Sso::client(&server).build().unwrap();
        let claims = esi.authenticate("the-code", None).await.unwrap().unwrap();

        assert_eq!(claims.sub, "CHARACTER:EVE:123123");
        assert_eq!(claims.name, "Test Pilot");
        assert_eq!(esi.access_token.as_deref(), Some(token.as_str()));
        assert_eq!(esi.refresh_token.as_deref(), Some("new-refresh"));
        let paths: Vec<_> = server
            .requests()
            .iter()
            .map(|r| r.path().to_owned())
            .collect();
        assert_eq!(
            paths,
            [
                "/v2/oauth/token",
                "/.well-known/oauth-authorization-server",
                "/oauth/jwks"
            ]
        );
    }

    #[tokio::test]
    async fn the_legacy_issuer_without_the_scheme_is_accepted() {
        let token = Sso::sign(&Sso::claims("the-client", "login.eveonline.com", 1200));
        let server = Sso::server(Sso::token_reply(&token)).await;
        let mut esi = Sso::client(&server).build().unwrap();

        assert!(esi.authenticate("the-code", None).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn a_token_for_another_client_is_rejected_and_not_stored() {
        let token = Sso::sign(&Sso::claims(
            "someone-else",
            "https://login.eveonline.com",
            1200,
        ));
        let server = Sso::server(Sso::token_reply(&token)).await;
        let mut esi = Sso::client(&server).build().unwrap();
        let result = esi.authenticate("the-code", None).await;

        assert!(matches!(result, Err(EsiError::InvalidJWT(_))));
        assert_eq!(esi.access_token, None);
        assert_eq!(esi.refresh_token, None);
    }

    #[tokio::test]
    async fn an_expired_token_is_rejected() {
        let token = Sso::sign(&Sso::claims(
            "the-client",
            "https://login.eveonline.com",
            -3600,
        ));
        let server = Sso::server(Sso::token_reply(&token)).await;
        let mut esi = Sso::client(&server).build().unwrap();
        let result = esi.authenticate("the-code", None).await;

        assert!(matches!(result, Err(EsiError::JwtValidationFailed(_))));
        assert_eq!(esi.access_token, None);
    }

    #[tokio::test]
    async fn a_token_from_another_issuer_is_rejected() {
        let token = Sso::sign(&Sso::claims("the-client", "https://evil.example", 1200));
        let server = Sso::server(Sso::token_reply(&token)).await;
        let mut esi = Sso::client(&server).build().unwrap();
        let result = esi.authenticate("the-code", None).await;

        assert!(matches!(result, Err(EsiError::InvalidJWT(_))));
        assert_eq!(esi.access_token, None);
    }

    #[tokio::test]
    async fn a_token_with_a_tampered_signature_is_rejected() {
        let token = Sso::sign(&Sso::claims(
            "the-client",
            "https://login.eveonline.com",
            1200,
        ));
        let (body, signature) = token.rsplit_once('.').unwrap();
        let flipped = if signature.starts_with('A') { 'B' } else { 'A' };
        let tampered = format!("{body}.{flipped}{}", &signature[1..]);
        let server = Sso::server(Sso::token_reply(&tampered)).await;
        let mut esi = Sso::client(&server).build().unwrap();
        let result = esi.authenticate("the-code", None).await;

        assert!(matches!(result, Err(EsiError::JwtValidationFailed(_))));
        assert_eq!(esi.access_token, None);
    }

    #[tokio::test]
    async fn a_metadata_document_without_keys_url_is_an_error() {
        let token = Sso::sign(&Sso::claims(
            "the-client",
            "https://login.eveonline.com",
            1200,
        ));
        let server = Server::routes(vec![
            ("/v2/oauth/token", Sso::token_reply(&token)),
            ("/.well-known", Reply::json("{}")),
        ])
        .await;
        let mut esi = Sso::client(&server).build().unwrap();
        let result = esi.authenticate("the-code", None).await;

        assert!(matches!(result, Err(EsiError::InvalidJWT(_))));
        assert_eq!(esi.access_token, None);
    }

    #[tokio::test]
    async fn a_failing_keys_endpoint_is_reported_with_its_status() {
        let token = Sso::sign(&Sso::claims(
            "the-client",
            "https://login.eveonline.com",
            1200,
        ));
        let server = Server::routes(vec![
            ("/v2/oauth/token", Sso::token_reply(&token)),
            (
                "/.well-known",
                Reply::json(r#"{"jwks_uri":"{base}oauth/jwks"}"#),
            ),
            ("/oauth/jwks", Reply::status(503)),
        ])
        .await;
        let mut esi = Sso::client(&server).build().unwrap();
        let result = esi.authenticate("the-code", None).await;

        assert!(matches!(result, Err(EsiError::InvalidStatusCode(503))));
        assert_eq!(esi.access_token, None);
    }

    #[tokio::test]
    async fn the_pkce_exchange_validates_the_token_too() {
        let token = Sso::sign(&Sso::claims(
            "the-client",
            "https://login.eveonline.com",
            1200,
        ));
        let server = Sso::server(Sso::token_reply(&token)).await;
        let mut esi = Flow::with_pkce()
            .token_url(&Flow::token_url(&server))
            .metadata_url(&format!("{}.well-known/x", server.url))
            .build()
            .unwrap();
        esi.authenticate("the-code", Some("the-verifier".to_owned()))
            .await
            .unwrap();

        let token_request = &server.requests()[0];
        assert_eq!(
            token_request.form(),
            [
                "client_id=the-client",
                "code=the-code",
                "code_verifier=the-verifier",
                "grant_type=authorization_code"
            ]
        );
        assert_eq!(esi.access_token.as_deref(), Some(token.as_str()));
    }
}

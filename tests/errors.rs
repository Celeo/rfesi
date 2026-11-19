//! What the client does when things go wrong: error statuses, the rate and
//! error limits that ESI announces in its headers, bodies that cannot be read,
//! and a server that is down or does not answer.
//!
//! Every test runs against a local server (`tests/support`), so it checks the
//! whole path from the HTTP answer to the `EsiError` the caller gets, and
//! whether a request reached the server at all.

mod support;

use esi_openapi::prelude::*;
use esi_openapi::spec::Spec;
use support::{Reply, Server};

struct Harness;

impl Harness {
    fn spec() -> Spec {
        serde_json::from_str(include_str!("../resources/test/openapi.json")).unwrap()
    }

    fn builder(server: &Server) -> EsiBuilder {
        EsiBuilder::new()
            .user_agent("error tests")
            .base_api_url(&server.url)
            .spec(Some(Self::spec()))
    }

    fn public(server: &Server) -> Esi {
        Self::builder(server).build().unwrap()
    }

    /// A client that is signed in for another hour.
    fn signed_in(server: &Server) -> Esi {
        let expiration = jiff::Timestamp::now().as_millisecond() + 3_600_000;
        Self::builder(server)
            .access_token(Some("the-token"))
            .access_expiration(Some(expiration))
            .build()
            .unwrap()
    }

    /// Any public call; the answer is not what the tests look at.
    async fn call(esi: &Esi) -> EsiResult<Vec<HistoryItem>> {
        esi.group_market().get_region_history(10000002, 34).await
    }

    /// An answer that carries the error limit headers.
    fn with_error_limit(reply: Reply, remain: &str, reset: &str) -> Reply {
        reply
            .with_header("X-ESI-Error-Limit-Remain", remain)
            .with_header("X-ESI-Error-Limit-Reset", reset)
    }
}

use esi_openapi::groups::HistoryItem;

#[tokio::test]
async fn every_error_status_is_reported_with_its_code_and_not_retried() {
    for status in [400, 401, 403, 404, 500, 502, 503, 504] {
        let server = Server::start(Reply::status(status)).await;
        let result = Harness::call(&Harness::public(&server)).await;

        assert!(
            matches!(result, Err(EsiError::InvalidStatusCode(code)) if code == status),
            "{status}: {result:?}"
        );
        assert_eq!(server.requests().len(), 1, "{status} must not be retried");
    }
}

#[tokio::test]
async fn a_rejected_token_is_reported_and_kept() {
    let server = Server::start(Reply::status(401)).await;
    let esi = Harness::signed_in(&server);
    let result = esi
        .group_contacts()
        .get_character_contacts(90000001, None)
        .await;

    assert!(matches!(result, Err(EsiError::InvalidStatusCode(401))));
    assert_eq!(esi.access_token.as_deref(), Some("the-token"));
}

#[tokio::test]
async fn a_429_without_headers_has_no_group_and_no_retry_after() {
    let server = Server::start(Reply::status(429)).await;
    let result = Harness::call(&Harness::public(&server)).await;

    assert!(matches!(
        result,
        Err(EsiError::RateLimited {
            group: None,
            retry_after_secs: None
        })
    ));
}

#[tokio::test]
async fn a_429_carries_the_group_and_the_retry_after_of_its_headers() {
    let reply = Reply::status(429)
        .with_header("X-Ratelimit-Group", "market")
        .with_header("Retry-After", "7");
    let server = Server::start(reply).await;
    let result = Harness::call(&Harness::public(&server)).await;

    match result {
        Err(EsiError::RateLimited {
            group,
            retry_after_secs,
        }) => {
            assert_eq!(group.as_deref(), Some("market"));
            assert_eq!(retry_after_secs, Some(7));
        }
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
async fn an_exhausted_error_limit_refuses_the_next_request_without_sending_it() {
    let reply = Harness::with_error_limit(Reply::status(500), "0", "60");
    let server = Server::start(reply).await;
    let esi = Harness::public(&server);

    let first = Harness::call(&esi).await;
    assert!(matches!(first, Err(EsiError::InvalidStatusCode(500))));
    match esi.is_error_limited().await.unwrap() {
        ErrorLimitStatus::Limited { for_millis } => {
            assert!((55_000..=60_000).contains(&for_millis), "{for_millis}")
        }
        ErrorLimitStatus::NotLimited => panic!("the limit is exhausted"),
    }
    let second = Harness::call(&esi).await;
    assert!(
        matches!(second, Err(EsiError::ErrorLimited(ms)) if (55_000..=60_000).contains(&ms)),
        "{second:?}"
    );
    assert_eq!(
        server.requests().len(),
        1,
        "the second call must not be sent"
    );
}

#[tokio::test]
async fn an_error_limit_with_errors_left_does_not_block() {
    let reply = Harness::with_error_limit(Reply::status(500), "99", "60");
    let server = Server::start(reply).await;
    let esi = Harness::public(&server);

    let _ = Harness::call(&esi).await;
    let second = Harness::call(&esi).await;

    assert!(matches!(second, Err(EsiError::InvalidStatusCode(500))));
    assert_eq!(server.requests().len(), 2);
    assert!(matches!(
        esi.is_error_limited().await.unwrap(),
        ErrorLimitStatus::NotLimited
    ));
}

#[tokio::test]
async fn the_error_limit_headers_of_a_successful_answer_count_too() {
    let reply = Harness::with_error_limit(Reply::json("[]"), "0", "60");
    let server = Server::start(reply).await;
    let esi = Harness::public(&server);

    Harness::call(&esi).await.unwrap();
    let second = Harness::call(&esi).await;

    assert!(matches!(second, Err(EsiError::ErrorLimited(_))));
    assert_eq!(server.requests().len(), 1);
}

#[tokio::test]
async fn an_error_limit_met_at_the_sso_also_blocks_the_api_calls() {
    let sso = Server::start(Harness::with_error_limit(Reply::status(400), "0", "60")).await;
    let api = Server::start(Reply::json("[]")).await;
    let mut esi = Harness::builder(&api)
        .client_id("the-client")
        .client_secret("the-secret")
        .callback_url("http://localhost/callback")
        .token_url(&format!("{}v2/oauth/token", sso.url))
        .refresh_token(Some("old-refresh"))
        .build()
        .unwrap();

    let refresh = esi.refresh_access_token(None).await;
    assert!(matches!(refresh, Err(EsiError::InvalidStatusCode(400))));
    let call = Harness::call(&esi).await;

    assert!(matches!(call, Err(EsiError::ErrorLimited(_))));
    assert!(api.requests().is_empty(), "the API must not be called");
}

#[tokio::test]
async fn unreadable_error_limit_headers_are_an_error() {
    let reply = Harness::with_error_limit(Reply::json("[]"), "many", "10");
    let server = Server::start(reply).await;
    let result = Harness::call(&Harness::public(&server)).await;

    assert!(
        matches!(result, Err(EsiError::HeaderParseError(_, _))),
        "{result:?}"
    );
}

#[tokio::test]
async fn a_body_that_is_not_json_is_a_parse_error() {
    let server = Server::start(Reply::json("{not json")).await;
    let result = Harness::call(&Harness::public(&server)).await;

    assert!(
        matches!(result, Err(EsiError::FailedJsonParse(_))),
        "{result:?}"
    );
}

#[tokio::test]
async fn a_body_of_the_wrong_shape_is_a_parse_error() {
    let server = Server::start(Reply::json(r#"{"unexpected": true}"#)).await;
    let result = Harness::call(&Harness::public(&server)).await;

    assert!(
        matches!(result, Err(EsiError::FailedJsonParse(_))),
        "{result:?}"
    );
}

#[tokio::test]
async fn a_server_that_is_down_is_a_connection_error() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::public(&server);
    // Nothing listens on a port that was just released.
    let free = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = free.local_addr().unwrap();
    drop(free);
    let esi_down = Harness::builder(&server)
        .base_api_url(&format!("http://{address}/"))
        .build()
        .unwrap();

    assert!(matches!(
        Harness::call(&esi_down).await,
        Err(EsiError::ReqwestError(_))
    ));
    Harness::call(&esi).await.unwrap();
}

#[tokio::test]
async fn a_server_that_does_not_answer_times_out() {
    let server = Server::start(Reply::never()).await;
    let esi = Harness::builder(&server)
        .http_timeout(Some(300))
        .build()
        .unwrap();
    let started = std::time::Instant::now();
    let result = Harness::call(&esi).await;

    match result {
        Err(EsiError::ReqwestError(error)) => assert!(error.is_timeout(), "{error}"),
        other => panic!("{other:?}"),
    }
    assert!(started.elapsed() < std::time::Duration::from_secs(5));
}

#[tokio::test]
async fn an_error_answer_is_not_cached() {
    let server = Server::sequence(vec![
        Reply::status(500),
        Reply::json(r#"[]"#).with_header("Cache-Control", "max-age=60"),
    ])
    .await;
    let esi = Harness::builder(&server)
        .enable_cache(true)
        .build()
        .unwrap();

    assert!(matches!(
        Harness::call(&esi).await,
        Err(EsiError::InvalidStatusCode(500))
    ));
    Harness::call(&esi).await.unwrap();
    assert_eq!(server.requests().len(), 2);
    assert_eq!(esi.cache_len().await, 1);
}

#[tokio::test]
async fn an_operation_missing_from_the_spec_never_reaches_the_server() {
    let server = Server::start(Reply::json("[]")).await;
    let empty: Spec = serde_json::from_str(r#"{"paths": {}}"#).unwrap();
    let esi = Harness::builder(&server).spec(Some(empty)).build().unwrap();
    let result = Harness::call(&esi).await;

    assert!(
        matches!(result, Err(EsiError::UnknownOperationID(ref id)) if id == "GetMarketsRegionIdHistory"),
        "{result:?}"
    );
    assert!(server.requests().is_empty());
}

#[tokio::test]
async fn a_failing_spec_download_is_an_error_and_the_client_still_has_no_spec() {
    let server = Server::start(Reply::status(500)).await;
    let mut esi = EsiBuilder::new()
        .user_agent("error tests")
        .base_api_url(&server.url)
        .spec_url(&format!("{}meta/openapi.json", server.url))
        .build()
        .unwrap();
    let result = esi.update_spec().await;

    assert!(
        matches!(result, Err(EsiError::InvalidStatusCode(500))),
        "{result:?}"
    );
    let call = Harness::call(&esi).await;
    assert!(matches!(call, Err(EsiError::EmptySpec)), "{call:?}");
}

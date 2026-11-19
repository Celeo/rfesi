//! What the client puts on the wire for each kind of endpoint macro.
//!
//! Every test points the client at a recording local server and checks the
//! method, the path, the query string, the body and the headers of the one
//! request it receives. The real OpenAPI spec resolves the operation IDs, so
//! a path that drifts from the spec also shows up here.

mod support;

use esi_openapi::groups::{FittingFlag, FittingItem, NewFitting};
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
            .user_agent("request tests")
            .base_api_url(&server.url)
            .spec(Some(Self::spec()))
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

    fn anonymous(server: &Server) -> Esi {
        Self::builder(server).build().unwrap()
    }
}

#[tokio::test]
async fn a_public_get_sends_its_path_and_required_query() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::anonymous(&server);
    esi.group_market()
        .get_region_history(10000002, 34)
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(request.method, "GET");
    assert_eq!(request.path(), "/markets/10000002/history");
    assert_eq!(request.query(), ["type_id=34"]);
    assert!(request.body.is_empty());
}

#[tokio::test]
async fn a_public_request_carries_no_authorization_even_with_a_token() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::signed_in(&server);
    esi.group_market()
        .get_region_history(10000002, 34)
        .await
        .unwrap();

    assert_eq!(server.only().header("authorization"), None);
}

#[tokio::test]
async fn every_request_carries_the_user_agent_and_the_compatibility_date() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::anonymous(&server);
    esi.group_market()
        .get_region_history(10000002, 34)
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(request.header("user-agent"), Some("request tests"));
    let date = request
        .header("x-compatibility-date")
        .expect("the compatibility date is missing");
    assert_eq!(date.len(), "2026-08-18".len());
    assert!(date.chars().nth(4) == Some('-'), "not a date: {date}");
}

#[tokio::test]
async fn an_authenticated_get_sends_the_bearer_token_and_omits_absent_optionals() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::signed_in(&server);
    esi.group_contacts()
        .get_character_contacts(90000001, None)
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(request.method, "GET");
    assert_eq!(request.path(), "/characters/90000001/contacts");
    assert_eq!(request.header("authorization"), Some("Bearer the-token"));
    assert!(request.query().is_empty(), "{:?}", request.query());
}

#[tokio::test]
async fn an_optional_query_parameter_is_sent_when_given() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::signed_in(&server);
    esi.group_contacts()
        .get_character_contacts(90000001, Some(3))
        .await
        .unwrap();

    assert_eq!(server.only().query(), ["page=3"]);
}

#[tokio::test]
async fn an_authenticated_request_without_a_token_never_reaches_the_server() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::anonymous(&server);
    let result = esi
        .group_contacts()
        .get_character_contacts(90000001, None)
        .await;

    assert!(matches!(result, Err(EsiError::MissingAuthentication)));
    assert!(server.requests().is_empty());
}

#[tokio::test]
async fn a_token_without_an_expiration_is_sent_and_does_not_panic() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::builder(&server)
        .access_token(Some("no-expiration"))
        .build()
        .unwrap();
    esi.group_contacts()
        .get_character_contacts(90000001, None)
        .await
        .unwrap();

    assert_eq!(
        server.only().header("authorization"),
        Some("Bearer no-expiration")
    );
}

#[tokio::test]
async fn an_expired_token_never_reaches_the_server() {
    let server = Server::start(Reply::json("[]")).await;
    let expired = jiff::Timestamp::now().as_millisecond() - 1_000;
    let esi = Harness::builder(&server)
        .access_token(Some("old"))
        .access_expiration(Some(expired))
        .build()
        .unwrap();
    let result = esi
        .group_contacts()
        .get_character_contacts(90000001, None)
        .await;

    assert!(matches!(result, Err(EsiError::AccessTokenExpired)));
    assert!(server.requests().is_empty());
}

#[tokio::test]
async fn a_delete_repeats_the_key_for_each_value_of_a_many_parameter() {
    let server = Server::start(Reply::status(204)).await;
    let esi = Harness::signed_in(&server);
    esi.group_contacts()
        .delete_character_contacts(90000001, &[11, 22, 33])
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(request.method, "DELETE");
    assert_eq!(request.path(), "/characters/90000001/contacts");
    assert_eq!(
        request.query(),
        ["contact_ids=11", "contact_ids=22", "contact_ids=33"]
    );
    assert_eq!(request.header("authorization"), Some("Bearer the-token"));
}

#[tokio::test]
async fn a_delete_with_two_path_parameters_fills_both() {
    let server = Server::start(Reply::status(204)).await;
    let esi = Harness::signed_in(&server);
    esi.group_fittings()
        .delete_character_fitting(90000001, 777)
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(request.method, "DELETE");
    assert_eq!(request.path(), "/characters/90000001/fittings/777");
}

#[tokio::test]
async fn a_post_sends_the_json_body_and_its_query_parameters() {
    let server = Server::start(Reply::json("[5,6]")).await;
    let esi = Harness::signed_in(&server);
    let created = esi
        .group_contacts()
        .add_character_contacts(90000001, 5.5, Some(&[1, 2]), Some(true), &[100, 200])
        .await
        .unwrap();

    assert_eq!(created, [5, 6]);
    let request = server.only();
    assert_eq!(request.method, "POST");
    assert_eq!(request.path(), "/characters/90000001/contacts");
    assert_eq!(
        request.query(),
        ["label_ids=1", "label_ids=2", "standing=5.5", "watched=true"]
    );
    assert_eq!(request.json(), serde_json::json!([100, 200]));
}

#[tokio::test]
async fn a_request_with_a_body_declares_it_as_json() {
    let server = Server::start(Reply::json("[5]")).await;
    let esi = Harness::signed_in(&server);
    esi.group_contacts()
        .add_character_contacts(90000001, 1.0, None, None, &[100])
        .await
        .unwrap();

    assert_eq!(
        server.only().header("content-type"),
        Some("application/json")
    );
}

#[tokio::test]
async fn a_post_leaves_out_the_optional_parameters_that_are_none() {
    let server = Server::start(Reply::json("[5]")).await;
    let esi = Harness::signed_in(&server);
    esi.group_contacts()
        .add_character_contacts(90000001, -10.0, None, None, &[100])
        .await
        .unwrap();

    assert_eq!(server.only().query(), ["standing=-10"]);
}

#[tokio::test]
async fn a_post_serializes_a_struct_body() {
    let server = Server::start(Reply::json(r#"{"fitting_id": 4242}"#)).await;
    let esi = Harness::signed_in(&server);
    let fitting = NewFitting {
        description: "A test fit".to_owned(),
        items: vec![FittingItem {
            flag: FittingFlag::HiSlot0,
            quantity: 1,
            type_id: 3001,
        }],
        name: "Test".to_owned(),
        ship_type_id: 587,
    };
    let id = esi
        .group_fittings()
        .create_character_fitting(90000001, &fitting)
        .await
        .unwrap();

    assert_eq!(id, 4242);
    let request = server.only();
    assert_eq!(request.method, "POST");
    assert_eq!(request.path(), "/characters/90000001/fittings");
    let body = request.json();
    assert_eq!(body["name"], "Test");
    assert_eq!(body["ship_type_id"], 587);
    assert_eq!(body["items"][0]["flag"], "HiSlot0");
    assert_eq!(body["items"][0]["type_id"], 3001);
}

#[tokio::test]
async fn a_put_sends_the_json_body_and_its_query_parameters() {
    let server = Server::start(Reply::status(204)).await;
    let esi = Harness::signed_in(&server);
    esi.group_contacts()
        .update_character_contacts(90000001, 2.0, None, Some(false), &[100, 200, 300])
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(request.method, "PUT");
    assert_eq!(request.path(), "/characters/90000001/contacts");
    assert_eq!(request.query(), ["standing=2", "watched=false"]);
    assert_eq!(request.json(), serde_json::json!([100, 200, 300]));
}

#[tokio::test]
async fn the_language_and_tenant_settings_become_headers() {
    let server = Server::start(Reply::json("[]")).await;
    let esi = Harness::builder(&server)
        .language(Language::De)
        .tenant("serenity")
        .build()
        .unwrap();
    esi.group_market()
        .get_region_history(10000002, 34)
        .await
        .unwrap();

    let request = server.only();
    assert_eq!(request.header("accept-language"), Some("de"));
    assert_eq!(request.header("x-tenant"), Some("serenity"));
}

#[tokio::test]
async fn a_server_error_is_reported_with_its_status() {
    let server = Server::start(Reply::status(404)).await;
    let esi = Harness::anonymous(&server);
    let result = esi.group_market().get_region_history(1, 34).await;

    assert!(result.is_err(), "a 404 must not parse as success");
}

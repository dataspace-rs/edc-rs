//! DCP scopes need the DCP feature in the connector runtime, which the test
//! stack does not ship, so these tests run against a wiremock management API
//! and check the requests the client sends. No docker stack needed.

#![allow(clippy::unwrap_used)]

use edc_connector_client::{
    types::{
        dcp_scope::{DcpScope, DcpScopeType, NewDcpScope},
        query::Query,
    },
    Auth, EdcConnectorApiVersion, EdcConnectorClient, Error, ManagementApiError,
    ManagementApiErrorDetailKind,
};
use reqwest::StatusCode;
use serde_json::json;
use wiremock::{
    matchers::{body_partial_json, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

const CONTEXT: &str = "https://w3id.org/edc/connector/management/v2";

/// A client configured with a participant context: DCP scopes are a global
/// resource, so the paths below must not carry `/participants/{ctx}`.
fn client(server: &MockServer) -> EdcConnectorClient {
    EdcConnectorClient::builder()
        .management_url(format!("{}/management", server.uri()))
        .with_auth(Auth::api_token("admin-token"))
        .participant_context("ctx-1")
        .build()
        .unwrap()
}

fn scope_json(id: &str, kind: &str) -> serde_json::Value {
    json!({
        "@context": [CONTEXT],
        "@type": "DcpScope",
        "@id": id,
        "type": kind,
        "value": "org.eclipse.edc.vc.type:MembershipCredential:read",
        "profile": "dsp2025_1",
        "prefixMapping": "dspace"
    })
}

#[tokio::test]
async fn should_create_a_scope() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/management/v5/dcpscopes"))
        .and(header("x-api-key", "admin-token"))
        .and(body_partial_json(json!({
            "@context": [CONTEXT],
            "@type": "DcpScope",
            "@id": "scope-1",
            "type": "POLICY",
            "value": "org.eclipse.edc.vc.type:MembershipCredential:read",
            "profile": "dsp2025_1"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "@context": [CONTEXT],
            "@type": "IdResponse",
            "@id": "scope-1",
            "createdAt": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let scope = NewDcpScope::builder()
        .id("scope-1")
        .kind(DcpScopeType::Policy)
        .value("org.eclipse.edc.vc.type:MembershipCredential:read")
        .profile("dsp2025_1")
        .build();

    let response = client(&server)
        .dcp_scopes(EdcConnectorApiVersion::V5)
        .create(&scope)
        .await
        .unwrap();

    assert_eq!("scope-1", response.id());
    assert_eq!(1, response.created_at());
}

#[tokio::test]
async fn should_update_a_scope() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/management/v5/dcpscopes/scope-1"))
        .and(body_partial_json(json!({
            "@context": [CONTEXT],
            "@type": "DcpScope",
            "@id": "scope-1",
            "type": "DEFAULT",
            "value": "org.eclipse.edc.vc.type:MembershipCredential:write",
            "prefixMapping": "dspace"
        })))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let scope = DcpScope::builder()
        .id("scope-1")
        .kind(DcpScopeType::Default)
        .value("org.eclipse.edc.vc.type:MembershipCredential:write")
        .prefix_mapping("dspace")
        .build();

    client(&server)
        .dcp_scopes(EdcConnectorApiVersion::V5)
        .update(&scope)
        .await
        .unwrap();
}

#[tokio::test]
async fn should_query_scopes() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/management/v5/dcpscopes/request"))
        .and(body_partial_json(json!({
            "@context": [CONTEXT],
            "@type": "QuerySpec",
            "filterExpression": [{
                "@type": "Criterion",
                "operandLeft": "profile",
                "operator": "=",
                "operandRight": "dsp2025_1"
            }]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            scope_json("scope-1", "DEFAULT"),
            scope_json("scope-2", "POLICY"),
        ])))
        .expect(1)
        .mount(&server)
        .await;

    let scopes = client(&server)
        .dcp_scopes(EdcConnectorApiVersion::V5)
        .query(Query::builder().filter("profile", "=", "dsp2025_1").build())
        .await
        .unwrap();

    assert_eq!(2, scopes.len());
    assert_eq!("scope-1", scopes[0].id());
    assert_eq!(&DcpScopeType::Default, scopes[0].kind());
    assert_eq!(&DcpScopeType::Policy, scopes[1].kind());
    assert_eq!(
        "org.eclipse.edc.vc.type:MembershipCredential:read",
        scopes[1].value()
    );
    assert_eq!(Some("dsp2025_1"), scopes[1].profile());
    assert_eq!(Some("dspace"), scopes[1].prefix_mapping());
}

#[tokio::test]
async fn should_delete_a_scope() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/management/v5/dcpscopes/scope-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .dcp_scopes(EdcConnectorApiVersion::V5)
        .delete("scope-1")
        .await
        .unwrap();
}

#[tokio::test]
async fn should_fail_to_delete_a_scope_when_not_existing() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/management/v5/dcpscopes/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!([{
            "message": "DcpScope missing not found",
            "type": "ObjectNotFound",
            "path": null,
            "invalidValue": null
        }])))
        .expect(1)
        .mount(&server)
        .await;

    let response = client(&server)
        .dcp_scopes(EdcConnectorApiVersion::V5)
        .delete("missing")
        .await;

    assert!(matches!(
        response,
        Err(Error::ManagementApi(ManagementApiError {
            status_code: StatusCode::NOT_FOUND,
            error_detail: ManagementApiErrorDetailKind::Parsed(..)
        }))
    ))
}

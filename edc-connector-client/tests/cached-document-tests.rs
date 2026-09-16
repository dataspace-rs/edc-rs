mod common;

// Cached documents are a global (admin scoped) V5 resource of the virtual
// connector: every test authenticates as the admin.
#[allow(clippy::unwrap_used)]
mod cached_documents {
    use edc_connector_client::{
        types::cached_document::{CachedDocument, DocumentType, NewCachedDocument, PullStrategy},
        EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;
    use serde_json::json;
    use uuid::Uuid;

    use crate::common::{provider_virtual_edc, setup_admin_client, ClientParams};

    fn new_document(id: &str) -> NewCachedDocument {
        NewCachedDocument::builder()
            .id(id)
            .url(format!("https://example.com/schema/{id}.json"))
            .document_type(DocumentType::JsonSchema)
            .pull_strategy(PullStrategy::Never)
            .content(json!({"type": "object"}))
            .build()
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_create_and_get_a_document(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        let response = client
            .cached_documents(version)
            .create(&new_document(&id))
            .await
            .unwrap();
        assert_eq!(&id, response.id());

        let document = client.cached_documents(version).get(&id).await.unwrap();

        assert_eq!(id, document.id());
        assert_eq!(
            format!("https://example.com/schema/{id}.json"),
            document.url()
        );
        assert_eq!(&DocumentType::JsonSchema, document.document_type());
        assert_eq!(&PullStrategy::Never, document.pull_strategy());
        assert_eq!(Some(&json!({"type": "object"})), document.content());

        client.cached_documents(version).delete(&id).await.unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_list_documents(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        client
            .cached_documents(version)
            .create(&new_document(&id))
            .await
            .unwrap();

        let documents = client.cached_documents(version).list().await.unwrap();

        assert!(documents.iter().any(|document| document.id() == id));

        client.cached_documents(version).delete(&id).await.unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_update_a_document(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        client
            .cached_documents(version)
            .create(&new_document(&id))
            .await
            .unwrap();

        let updated = CachedDocument::builder()
            .id(&id)
            .url(format!("https://example.com/schema/{id}-v2.json"))
            .document_type(DocumentType::JsonSchema)
            .pull_strategy(PullStrategy::IfNotPresent)
            .content(json!({"type": "array"}))
            .build();

        client
            .cached_documents(version)
            .update(&updated)
            .await
            .unwrap();

        let document = client.cached_documents(version).get(&id).await.unwrap();

        assert_eq!(
            format!("https://example.com/schema/{id}-v2.json"),
            document.url()
        );
        assert_eq!(&PullStrategy::IfNotPresent, document.pull_strategy());
        assert_eq!(Some(&json!({"type": "array"})), document.content());

        client.cached_documents(version).delete(&id).await.unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_delete_a_document(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        client
            .cached_documents(version)
            .create(&new_document(&id))
            .await
            .unwrap();

        client.cached_documents(version).delete(&id).await.unwrap();

        let response = client.cached_documents(version).get(&id).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_refresh_a_document(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        // Keycloak is reachable from the connector container and serves JSON.
        let document = NewCachedDocument::builder()
            .id(&id)
            .url("http://keycloak:8080/realms/edcv/.well-known/openid-configuration")
            .document_type(DocumentType::JsonSchema)
            .pull_strategy(PullStrategy::Always)
            .build();

        client
            .cached_documents(version)
            .create(&document)
            .await
            .unwrap();

        let refreshed = client.cached_documents(version).refresh(&id).await.unwrap();

        assert_eq!(id, refreshed.id());
        let content = refreshed.content().unwrap();
        assert!(content.get("issuer").is_some(), "content: {content}");

        client.cached_documents(version).delete(&id).await.unwrap();
    }
}

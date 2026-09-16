mod common;

// Schema validator registrations are a global (admin scoped) V5 resource of
// the virtual connector: every test authenticates as the admin.
//
// The connector (1.0.0-rc1) answers 500 when registrations are created
// concurrently, so the tests of this module run one at a time.
#[allow(clippy::unwrap_used)]
mod schema_validators {
    use std::sync::LazyLock;

    use tokio::sync::Mutex;

    static SERIAL: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    use edc_connector_client::{
        types::{
            cached_document::{DocumentType, NewCachedDocument, PullStrategy},
            schema_validator::{NewSchemaValidatorRegistration, SchemaValidatorRegistration},
        },
        EdcConnectorApiVersion, EdcConnectorClient, Error, ManagementApiError,
        ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;
    use serde_json::json;
    use uuid::Uuid;

    use crate::common::{provider_virtual_edc, setup_admin_client, ClientParams};

    fn schema_url(id: &str) -> String {
        format!("https://example.com/schema/{id}.json")
    }

    /// A registration only accepts schemas already cached as `JSON_SCHEMA`
    /// documents, so every test caches the schema first (keyed by the same id).
    async fn cache_schema(client: &EdcConnectorClient, version: EdcConnectorApiVersion, id: &str) {
        client
            .cached_documents(version)
            .create(
                &NewCachedDocument::builder()
                    .id(id)
                    .url(schema_url(id))
                    .document_type(DocumentType::JsonSchema)
                    .pull_strategy(PullStrategy::Never)
                    .content(json!({"type": "object"}))
                    .build(),
            )
            .await
            .unwrap();
    }

    async fn cleanup(client: &EdcConnectorClient, version: EdcConnectorApiVersion, id: &str) {
        let _ = client.schema_validators(version).delete(id).await;
        client.cached_documents(version).delete(id).await.unwrap();
    }

    fn new_registration(id: &str) -> NewSchemaValidatorRegistration {
        NewSchemaValidatorRegistration::builder()
            .id(id)
            .version("v5")
            .validated_type(format!("CustomType{id}"))
            .schema(schema_url(id))
            .profile("http-dsp-profile-2025-1")
            .build()
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_create_and_get_a_registration(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let _serial = SERIAL.lock().await;
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();
        cache_schema(&client, version, &id).await;

        let response = client
            .schema_validators(version)
            .create(&new_registration(&id))
            .await
            .unwrap();
        assert_eq!(&id, response.id());

        let registration = client.schema_validators(version).get(&id).await.unwrap();

        assert_eq!(id, registration.id());
        assert_eq!("v5", registration.version());
        assert_eq!(format!("CustomType{id}"), registration.validated_type());
        assert_eq!(schema_url(&id), registration.schema());
        assert_eq!(["http-dsp-profile-2025-1"], registration.profiles());

        cleanup(&client, version, &id).await;
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_list_registrations(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let _serial = SERIAL.lock().await;
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();
        cache_schema(&client, version, &id).await;

        client
            .schema_validators(version)
            .create(&new_registration(&id))
            .await
            .unwrap();

        let registrations = client.schema_validators(version).list().await.unwrap();

        assert!(registrations
            .iter()
            .any(|registration| registration.id() == id));

        cleanup(&client, version, &id).await;
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_update_a_registration(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let _serial = SERIAL.lock().await;
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();
        cache_schema(&client, version, &id).await;

        client
            .schema_validators(version)
            .create(&new_registration(&id))
            .await
            .unwrap();

        let updated = SchemaValidatorRegistration::builder()
            .id(&id)
            .version("v5")
            .validated_type(format!("CustomType{id}"))
            .schema(schema_url(&id))
            .build();

        client
            .schema_validators(version)
            .update(&updated)
            .await
            .unwrap();

        let registration = client.schema_validators(version).get(&id).await.unwrap();

        assert_eq!(schema_url(&id), registration.schema());
        assert!(registration.profiles().is_empty());

        cleanup(&client, version, &id).await;
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_delete_a_registration(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let _serial = SERIAL.lock().await;
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();
        cache_schema(&client, version, &id).await;

        client
            .schema_validators(version)
            .create(&new_registration(&id))
            .await
            .unwrap();

        client.schema_validators(version).delete(&id).await.unwrap();

        let response = client.schema_validators(version).get(&id).await;
        cleanup(&client, version, &id).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }
}

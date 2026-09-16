mod common;

// Dataspace profiles are a global V5 resource of the virtual connector;
// writing them needs the admin scope.
#[allow(clippy::unwrap_used)]
mod dataspace_profiles {
    use edc_connector_client::{
        types::{
            dataspace_profile::{DataspaceProfile, DataspaceProtocol, TrustedIssuer},
            query::Query,
        },
        EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::common::{provider_virtual_edc, setup_admin_client, ClientParams};

    fn new_profile(name: &str) -> DataspaceProfile {
        DataspaceProfile::builder()
            .name(name)
            .protocol(
                DataspaceProtocol::builder()
                    .version("2025-1")
                    .path(format!("/{name}"))
                    .binding("HTTPS")
                    .namespace("https://w3id.org/dspace/2025/1/")
                    .build(),
            )
            .json_ld_context_url("https://w3id.org/dspace/2025/1/context.jsonld")
            .trusted_issuer(
                TrustedIssuer::builder()
                    .id("did:web:trusted.issuer")
                    .supported_type("MembershipCredential")
                    .build(),
            )
            .build()
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_create_and_get_a_profile(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let name = format!("profile-{}", Uuid::new_v4());

        let created = client
            .dataspace_profiles(version)
            .create(&new_profile(&name))
            .await
            .unwrap();
        assert_eq!(name, created.name());

        let profile = client.dataspace_profiles(version).get(&name).await.unwrap();

        assert_eq!(name, profile.name());
        assert_eq!("2025-1", profile.protocol().version());
        assert_eq!(format!("/{name}"), profile.protocol().path());
        assert_eq!("HTTPS", profile.protocol().binding());
        assert_eq!(
            "https://w3id.org/dspace/2025/1/",
            profile.protocol().namespace()
        );
        assert_eq!(
            ["https://w3id.org/dspace/2025/1/context.jsonld"],
            profile.json_ld_contexts_url()
        );
        assert_eq!(1, profile.trusted_issuers().len());
        assert_eq!("did:web:trusted.issuer", profile.trusted_issuers()[0].id());
        assert_eq!(
            ["MembershipCredential"],
            profile.trusted_issuers()[0].supported_types()
        );

        client
            .dataspace_profiles(version)
            .delete(&name)
            .await
            .unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_update_a_profile(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let name = format!("profile-{}", Uuid::new_v4());

        client
            .dataspace_profiles(version)
            .create(&new_profile(&name))
            .await
            .unwrap();

        let updated = DataspaceProfile::builder()
            .name(&name)
            .protocol(
                DataspaceProtocol::builder()
                    .version("2025-1")
                    .path(format!("/{name}"))
                    .binding("HTTPS")
                    .namespace("https://w3id.org/dspace/2025/1/")
                    .build(),
            )
            .json_ld_context_url("https://w3id.org/dspace/2025/1/context.jsonld")
            .json_ld_context_url("https://w3id.org/edc/dspace/v0.0.1")
            .build();

        client
            .dataspace_profiles(version)
            .update(&updated)
            .await
            .unwrap();

        let profile = client.dataspace_profiles(version).get(&name).await.unwrap();

        assert_eq!(2, profile.json_ld_contexts_url().len());
        assert!(profile.trusted_issuers().is_empty());

        client
            .dataspace_profiles(version)
            .delete(&name)
            .await
            .unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_query_profiles(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let name = format!("profile-{}", Uuid::new_v4());

        client
            .dataspace_profiles(version)
            .create(&new_profile(&name))
            .await
            .unwrap();

        let profiles = client
            .dataspace_profiles(version)
            .query(Query::builder().filter("name", "=", name.as_str()).build())
            .await
            .unwrap();

        assert_eq!(1, profiles.len());
        assert_eq!(name, profiles[0].name());

        client
            .dataspace_profiles(version)
            .delete(&name)
            .await
            .unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_delete_a_profile(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let name = format!("profile-{}", Uuid::new_v4());

        client
            .dataspace_profiles(version)
            .create(&new_profile(&name))
            .await
            .unwrap();

        client
            .dataspace_profiles(version)
            .delete(&name)
            .await
            .unwrap();

        let response = client.dataspace_profiles(version).get(&name).await;

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
    async fn should_fail_to_get_a_profile_when_not_existing(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);

        let response = client
            .dataspace_profiles(version)
            .get(&Uuid::new_v4().to_string())
            .await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }
}

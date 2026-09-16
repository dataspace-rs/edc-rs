mod common;

#[allow(clippy::unwrap_used)]
mod dataplane {

    // Only V4 connectors expose a data plane list (`GET /dataplanes`); V5
    // replaced it with the `POST /dataplanes/request` query covered below.
    mod list {
        use crate::common::{
            provider, register_dataplane, setup_client, unregister_dataplane, ClientParams,
            DATAPLANE_ENDPOINT,
        };
        use edc_connector_client::EdcConnectorApiVersion;
        use rstest::rstest;

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_fetch_dataplanes(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            // Nothing registers a data plane with the test connectors on
            // startup, so the list is only non-empty once we put one there.
            let (id, transfer_type) = register_dataplane(&client, version).await;

            let response = client.data_planes(version).list().await.unwrap();

            assert!(!response.is_empty());

            let registered = response.iter().find(|dp| dp.id() == id).unwrap();
            assert_eq!(registered.url(), DATAPLANE_ENDPOINT);
            assert!(registered.allowed_transfer_types().contains(&transfer_type));

            unregister_dataplane(&client, version, &id).await;
        }
    }

    // `POST /dataplanes/request` is served by the V5 data plane selector API,
    // which the virtual-controlplane 1.0.0-rc1 image does not ship yet.
    mod query {
        use crate::common::{
            provider_virtual_edc, register_dataplane, setup_client, unregister_dataplane,
            ClientParams, DATAPLANE_ENDPOINT,
        };
        use edc_connector_client::{types::query::Query, EdcConnectorApiVersion};
        use rstest::rstest;

        #[rstest]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_query_dataplanes(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            let (id, transfer_type) = register_dataplane(&client, version).await;

            let response = client
                .data_planes(version)
                .query(Query::builder().filter("id", "=", id.as_str()).build())
                .await
                .unwrap();

            assert_eq!(1, response.len());
            assert_eq!(response[0].id(), id);
            assert_eq!(response[0].url(), DATAPLANE_ENDPOINT);
            assert!(response[0]
                .allowed_transfer_types()
                .contains(&transfer_type));
            assert!(response[0].destination_provision_types().is_empty());

            unregister_dataplane(&client, version, &id).await;
        }
    }

    mod register {
        use edc_connector_client::{
            types::dataplane::{DataPlaneAuthorization, DataPlaneRegistrationMessage},
            EdcConnectorApiVersion,
        };
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{
            new_dataplane_registration, new_transfer_type, provider, provider_virtual_edc,
            register_dataplane, setup_client, unregister_dataplane, ClientParams,
            DATAPLANE_ENDPOINT,
        };

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_register_a_dataplane(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            let (id, _) = register_dataplane(&client, version).await;

            unregister_dataplane(&client, version, &id).await;
        }

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_register_a_dataplane_with_an_authorization_profile(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();

            let registration = DataPlaneRegistrationMessage::builder()
                .dataplane_id(&id)
                .endpoint(DATAPLANE_ENDPOINT)
                .transfer_type(new_transfer_type())
                .authorization(
                    DataPlaneAuthorization::builder()
                        .kind("oauth2_client_credentials")
                        .property("tokenEndpoint", "http://keycloak:8080/token")
                        .property("clientId", "dataplane")
                        .property("clientSecret", "secret")
                        .build()
                        .unwrap(),
                )
                .build();

            client
                .data_planes(version)
                .register(&registration)
                .await
                .unwrap();

            unregister_dataplane(&client, version, &id).await;
        }

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_update_a_registered_dataplane(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            let (id, transfer_type) = register_dataplane(&client, version).await;

            let updated_transfer_type = new_transfer_type();
            client
                .data_planes(version)
                .register(&new_dataplane_registration(&id, &updated_transfer_type))
                .await
                .unwrap();

            let instances = client.data_planes(version).list().await.unwrap();
            let registered = instances.iter().find(|dp| dp.id() == id).unwrap();

            assert!(registered
                .allowed_transfer_types()
                .contains(&updated_transfer_type));
            assert!(!registered.allowed_transfer_types().contains(&transfer_type));

            unregister_dataplane(&client, version, &id).await;
        }
    }

    mod delete {
        use edc_connector_client::{
            EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{
            provider, provider_virtual_edc, register_dataplane, setup_client, ClientParams,
        };

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_delete_a_registered_dataplane(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            let (id, _) = register_dataplane(&client, version).await;

            let response = client.data_planes(version).delete(&id).await;

            assert!(response.is_ok());
        }

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_fail_to_delete_a_dataplane_when_not_existing(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            let response = client
                .data_planes(version)
                .delete(&Uuid::new_v4().to_string())
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
}

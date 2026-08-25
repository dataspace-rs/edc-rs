mod common;

mod contract_definition {
    mod create {
        use edc_connector_client::{
            types::contract_definition::NewContractDefinition, EdcConnectorApiVersion, Error,
            ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider, provider_virtual_edc, setup_client, ClientParams};

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_create_a_contract_definition(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            let id = Uuid::new_v4().to_string();

            let contract_definition = NewContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("contract_id")
                .build();

            let response = client
                .contract_definitions(version)
                .create(&contract_definition)
                .await
                .unwrap();

            assert_eq!(&id, response.id());
            assert!(response.created_at() > 0);
        }

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_failt_to_create_a_contract_definition_when_existing(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);

            let id = Uuid::new_v4().to_string();

            let contract_definition = NewContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("contract_id")
                .build();

            let response = client
                .contract_definitions(version)
                .create(&contract_definition)
                .await
                .unwrap();

            assert_eq!(&id, response.id());
            assert!(response.created_at() > 0);

            let response = client
                .contract_definitions(version)
                .create(&contract_definition)
                .await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::CONFLICT,
                    error_detail: ManagementApiErrorDetailKind::Parsed(..)
                }))
            ))
        }
    }

    mod delete {
        use edc_connector_client::{
            types::contract_definition::NewContractDefinition, EdcConnectorApiVersion, Error,
            ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider, provider_virtual_edc, setup_client, ClientParams};

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_delete_a_contract_definition(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();

            let contract_definition = NewContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("contract_id")
                .build();

            let definition = client
                .contract_definitions(version)
                .create(&contract_definition)
                .await
                .unwrap();

            let response = client
                .contract_definitions(version)
                .delete(definition.id())
                .await;

            assert!(response.is_ok());
        }

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_fail_to_delete_policy_definition_when_not_existing(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();

            let response = client.policies(version).delete(&id).await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::NOT_FOUND,
                    error_detail: ManagementApiErrorDetailKind::Parsed(..)
                }))
            ))
        }
    }

    mod get {
        use edc_connector_client::{
            types::contract_definition::NewContractDefinition, EdcConnectorApiVersion, Error,
            ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider, provider_virtual_edc, setup_client, ClientParams};

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_get_a_contract_definition(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();

            let contract_definition = NewContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("contract_id")
                .build();

            let created = client
                .contract_definitions(version)
                .create(&contract_definition)
                .await
                .unwrap();

            let definition = client
                .contract_definitions(version)
                .get(created.id())
                .await
                .unwrap();

            assert_eq!(definition.access_policy_id(), "access_id");
            assert_eq!(definition.contract_policy_id(), "contract_id");
        }

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_fail_to_get_a_policy_definition_when_not_existing(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();

            let response = client.policies(version).get(&id).await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::NOT_FOUND,
                    error_detail: ManagementApiErrorDetailKind::Parsed(..)
                }))
            ))
        }
    }

    mod update {
        use edc_connector_client::{
            types::contract_definition::{ContractDefinition, NewContractDefinition},
            EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider, provider_virtual_edc, setup_client, ClientParams};

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_update_policy_definition(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();
            let contract_definition = NewContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("contract_id")
                .build();

            client
                .contract_definitions(version)
                .create(&contract_definition)
                .await
                .unwrap();

            let updated_definition = ContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("updated_contract_id")
                .build();

            client
                .contract_definitions(version)
                .update(&updated_definition)
                .await
                .unwrap();

            let definition = client.contract_definitions(version).get(&id).await.unwrap();

            assert_eq!("updated_contract_id", definition.contract_policy_id());
        }

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_fail_to_update_an_contract_definition_when_not_existing(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();

            let updated_definition = ContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("updated_contract_id")
                .build();

            let response = client
                .contract_definitions(version)
                .update(&updated_definition)
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

    mod query {
        use crate::common::{provider, provider_virtual_edc, setup_client, ClientParams};
        use edc_connector_client::types::{
            contract_definition::NewContractDefinition, query::Query,
        };
        use edc_connector_client::EdcConnectorApiVersion;
        use rstest::rstest;
        use uuid::Uuid;

        #[rstest]
        #[case(provider(), EdcConnectorApiVersion::V3)]
        #[case(provider(), EdcConnectorApiVersion::V4)]
        #[case(provider_virtual_edc(), EdcConnectorApiVersion::V4)]
        #[tokio::test]
        async fn should_query_contract_definitions(
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let client = setup_client(provider, version);
            let id = Uuid::new_v4().to_string();
            let contract_definition = NewContractDefinition::builder()
                .id(&id)
                .access_policy_id("access_id")
                .contract_policy_id("contract_id")
                .build();

            client
                .contract_definitions(version)
                .create(&contract_definition)
                .await
                .unwrap();

            let definitions = client
                .contract_definitions(version)
                .query(Query::builder().filter("id", "=", id).build())
                .await
                .unwrap();

            assert_eq!(1, definitions.len());
        }
    }
}

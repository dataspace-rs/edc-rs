mod common;
mod create {
    use edc_connector_client::{
        types::contract_definition::NewContractDefinition, Error, ManagementApiError,
        ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use uuid::Uuid;

    use crate::common::setup_provider_client;

    #[tokio::test]
    async fn should_create_a_contract_definition() {
        let client = setup_provider_client();

        let id = Uuid::new_v4().to_string();

        let contract_definition = NewContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("contract_id")
            .build()
            .unwrap();

        let response = client
            .contract_definitions()
            .create(&contract_definition)
            .await
            .unwrap();

        assert_eq!(&id, response.id());
        assert!(response.created_at() > 0);
    }

    #[tokio::test]
    async fn should_failt_to_create_a_contract_definition_when_existing() {
        let client = setup_provider_client();

        let id = Uuid::new_v4().to_string();

        let contract_definition = NewContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("contract_id")
            .build()
            .unwrap();

        let response = client
            .contract_definitions()
            .create(&contract_definition)
            .await
            .unwrap();

        assert_eq!(&id, response.id());
        assert!(response.created_at() > 0);

        let response = client
            .contract_definitions()
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
        types::contract_definition::NewContractDefinition, Error, ManagementApiError,
        ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use uuid::Uuid;

    use crate::common::setup_provider_client;

    #[tokio::test]
    async fn should_delete_a_contract_definition() {
        let client = setup_provider_client();
        let id = Uuid::new_v4().to_string();

        let contract_definition = NewContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("contract_id")
            .build()
            .unwrap();

        let definition = client
            .contract_definitions()
            .create(&contract_definition)
            .await
            .unwrap();

        let response = client.contract_definitions().delete(definition.id()).await;

        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn should_fail_to_delete_policy_definition_when_not_existing() {
        let client = setup_provider_client();
        let id = Uuid::new_v4().to_string();

        let response = client.policies().delete(&id).await;

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
        types::contract_definition::NewContractDefinition, Error, ManagementApiError,
        ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use uuid::Uuid;

    use crate::common::setup_provider_client;

    #[tokio::test]
    async fn should_get_a_contract_definition() {
        let client = setup_provider_client();
        let id = Uuid::new_v4().to_string();

        let contract_definition = NewContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("contract_id")
            .build()
            .unwrap();

        let created = client
            .contract_definitions()
            .create(&contract_definition)
            .await
            .unwrap();

        let definition = client
            .contract_definitions()
            .get(created.id())
            .await
            .unwrap();

        assert_eq!(definition.access_policy_id(), "access_id");
        assert_eq!(definition.contract_policy_id(), "contract_id");
    }

    #[tokio::test]
    async fn should_fail_to_get_a_policy_definition_when_not_existing() {
        let client = setup_provider_client();
        let id = Uuid::new_v4().to_string();

        let response = client.policies().get(&id).await;

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
        Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use uuid::Uuid;

    use crate::common::setup_provider_client;

    #[tokio::test]
    async fn should_update_policy_definition() {
        let client = setup_provider_client();
        let id = Uuid::new_v4().to_string();
        let contract_definition = NewContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("contract_id")
            .build()
            .unwrap();

        client
            .contract_definitions()
            .create(&contract_definition)
            .await
            .unwrap();

        let updated_definition = ContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("updated_contract_id")
            .build()
            .unwrap();

        client
            .contract_definitions()
            .update(&updated_definition)
            .await
            .unwrap();

        let definition = client.contract_definitions().get(&id).await.unwrap();

        assert_eq!("updated_contract_id", definition.contract_policy_id());
    }

    #[tokio::test]
    async fn should_fail_to_update_an_contract_definition_when_not_existing() {
        let client = setup_provider_client();
        let id = Uuid::new_v4().to_string();

        let updated_definition = ContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("updated_contract_id")
            .build()
            .unwrap();

        let response = client
            .contract_definitions()
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
    use edc_connector_client::types::{contract_definition::NewContractDefinition, query::Query};
    use uuid::Uuid;

    use crate::common::setup_provider_client;

    #[tokio::test]
    async fn should_query_contract_definitions() {
        let client = setup_provider_client();
        let id = Uuid::new_v4().to_string();
        let contract_definition = NewContractDefinition::builder()
            .id(&id)
            .access_policy_id("access_id")
            .contract_policy_id("contract_id")
            .build()
            .unwrap();

        client
            .contract_definitions()
            .create(&contract_definition)
            .await
            .unwrap();

        let definitions = client
            .contract_definitions()
            .query(Query::builder().filter("id", "=", id).build())
            .await
            .unwrap();

        assert_eq!(1, definitions.len());
    }
}

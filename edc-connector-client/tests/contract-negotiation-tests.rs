mod common;

mod initiate {
    use edc_connector_client::{
        types::{
            catalog::DatasetRequest,
            contract_negotiation::ContractRequest,
            policy::{Policy, PolicyKind, Target},
        },
        Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;

    use crate::common::{
        seed, setup_consumer_client, setup_provider_client, PROVIDER_ID, PROVIDER_PROTOCOL,
    };

    #[tokio::test]
    async fn should_initiate_a_contract_negotiation() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (asset_id, _, _) = seed(&provider).await;

        let dataset_request = DatasetRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
            .id(&asset_id)
            .build();

        let dataset = consumer
            .catalogue()
            .dataset(&dataset_request)
            .await
            .unwrap();

        let offer_id = dataset.offers()[0].id().unwrap();

        let request = ContractRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
            .counter_party_id(PROVIDER_ID)
            .policy(
                Policy::builder()
                    .kind(PolicyKind::Offer)
                    .id(offer_id)
                    .assigner(PROVIDER_ID)
                    .target(Target::id(&asset_id))
                    .build(),
            )
            .build();

        let response = consumer
            .contract_negotiations()
            .initiate(&request)
            .await
            .unwrap();

        assert!(response.created_at() > 0);
    }

    #[tokio::test]
    async fn should_fail_to_initiate_a_contact_negotiation_with_wrong_policy() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (asset_id, _, _) = seed(&provider).await;

        let dataset_request = DatasetRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
            .id(&asset_id)
            .build();

        let dataset = consumer
            .catalogue()
            .dataset(&dataset_request)
            .await
            .unwrap();

        let offer_id = dataset.offers()[0].id().unwrap();

        let request = ContractRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
            .counter_party_id(PROVIDER_ID)
            .policy(
                Policy::builder()
                    .id(offer_id)
                    .assigner(PROVIDER_ID)
                    .target(Target::id(&asset_id))
                    .build(),
            )
            .build();

        let response = consumer.contract_negotiations().initiate(&request).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::BAD_REQUEST,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }
}

mod get {

    use edc_connector_client::types::contract_negotiation::{
        ContractNegotiationKind, ContractNegotiationState,
    };

    use crate::common::{seed_contract_negotiation, setup_consumer_client, setup_provider_client};

    #[tokio::test]
    async fn should_get_a_contract_negotiation() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (contract_negotiation_id, _) = seed_contract_negotiation(&consumer, &provider).await;

        let cn = consumer
            .contract_negotiations()
            .get(&contract_negotiation_id)
            .await
            .unwrap();

        assert_eq!(contract_negotiation_id, cn.id());
        assert_ne!(&ContractNegotiationState::Terminated, cn.state());
        assert_eq!(0, cn.callback_addresses().len());
        assert_eq!(Some("provider".to_string()), cn.counter_party_id().clone());
        assert_eq!(&ContractNegotiationKind::Consumer, cn.kind());
    }

    #[tokio::test]
    async fn should_get_a_state_of_contract_negotiation() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (contract_negotiation_id, _) = seed_contract_negotiation(&consumer, &provider).await;

        let state_response = consumer
            .contract_negotiations()
            .get_state(&contract_negotiation_id)
            .await;

        assert!(state_response.is_ok())
    }
}

mod query {
    use edc_connector_client::types::query::Query;

    use crate::common::{seed_contract_negotiation, setup_consumer_client, setup_provider_client};

    #[tokio::test]
    async fn should_query_contract_negotiations() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (contract_negotiation_id, _) = seed_contract_negotiation(&consumer, &provider).await;

        let negotiations = consumer
            .contract_negotiations()
            .query(
                Query::builder()
                    .filter("id", "=", contract_negotiation_id)
                    .build(),
            )
            .await
            .unwrap();

        assert_eq!(1, negotiations.len());
    }
}

mod terminate {
    use edc_connector_client::{
        types::contract_negotiation::ContractNegotiationState, Error, ManagementApiError,
    };

    use crate::common::{
        seed_contract_negotiation, setup_consumer_client, setup_provider_client,
        wait_for_negotiation_state,
    };

    #[tokio::test]
    async fn should_terminate_a_contract_negotiations() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (contract_negotiation_id, _) = seed_contract_negotiation(&consumer, &provider).await;

        wait_for_negotiation_state(
            &consumer,
            &contract_negotiation_id,
            ContractNegotiationState::Finalized,
        )
        .await;

        let result = consumer
            .contract_negotiations()
            .terminate(&contract_negotiation_id, "test")
            .await;

        assert!(matches!(
            result,
            Err(Error::ManagementApi(ManagementApiError { .. }))
        ));
    }
}

mod common;

mod contract_negotiations {

    mod initiate {
        use edc_connector_client::{
            types::{
                catalog::DatasetRequest,
                contract_negotiation::ContractRequest,
                policy::{Action, Permission, Policy, PolicyKind, Target},
            },
            Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_virtual_edc, seed, setup_client, ClientParams, PROVIDER_ID,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_initiate_a_contract_negotiation(
            #[case] consumer: ClientParams,
            #[case] provider: ClientParams,
        ) {
            let provider_addr = provider.protocol_address.clone();
            let provider_id = provider.protocol_id.clone();
            let protocol = consumer.protocol.clone();
            let provider = setup_client(provider);
            let consumer = setup_client(consumer);

            let (asset_id, _, _) = seed(&provider).await;

            let dataset_request = DatasetRequest::builder()
                .counter_party_address(provider_addr.clone())
                .counter_party_id(provider_id.clone())
                .protocol(protocol.clone())
                .id(&asset_id)
                .build();

            let dataset = consumer
                .catalogue()
                .dataset(&dataset_request)
                .await
                .unwrap();

            let offer_id = dataset.offers()[0].id().unwrap();

            let request = ContractRequest::builder()
                .counter_party_address(provider_addr)
                .counter_party_id(provider_id)
                .protocol(protocol)
                .policy(
                    Policy::builder()
                        .kind(PolicyKind::Offer)
                        .id(offer_id)
                        .assigner(PROVIDER_ID)
                        .target(Target::simple(&asset_id))
                        .permission(Permission::builder().action(Action::simple("use")).build())
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

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_fail_to_initiate_a_contact_negotiation_with_wrong_policy(
            #[case] consumer: ClientParams,
            #[case] provider: ClientParams,
        ) {
            let provider_addr = provider.protocol_address.clone();
            let provider_id = provider.protocol_id.clone();
            let protocol = consumer.protocol.clone();
            let provider = setup_client(provider);
            let consumer = setup_client(consumer);

            let (asset_id, _, _) = seed(&provider).await;

            let dataset_request = DatasetRequest::builder()
                .counter_party_address(provider_addr.clone())
                .counter_party_id(provider_id.clone())
                .protocol(protocol.clone())
                .id(&asset_id)
                .build();

            let dataset = consumer
                .catalogue()
                .dataset(&dataset_request)
                .await
                .unwrap();

            let offer_id = dataset.offers()[0].id().unwrap();

            let request = ContractRequest::builder()
                .counter_party_address(provider_addr)
                .counter_party_id(provider_id)
                .protocol(protocol)
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
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_virtual_edc, seed_contract_negotiation, setup_client, ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_get_a_contract_negotiation(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (contract_negotiation_id, _) =
                seed_contract_negotiation(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            let cn = consumer
                .contract_negotiations()
                .get(&contract_negotiation_id)
                .await
                .unwrap();

            assert_eq!(contract_negotiation_id, cn.id());
            assert_ne!(&ContractNegotiationState::Terminated, cn.state());
            assert_eq!(0, cn.callback_addresses().len());
            assert_eq!("provider", cn.counter_party_id());
            assert_eq!(&ContractNegotiationKind::Consumer, cn.kind());
        }

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_get_a_state_of_contract_negotiation(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (contract_negotiation_id, _) =
                seed_contract_negotiation(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            let state_response = consumer
                .contract_negotiations()
                .get_state(&contract_negotiation_id)
                .await;

            assert!(state_response.is_ok())
        }
    }

    mod query {
        use edc_connector_client::types::query::Query;
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_virtual_edc, seed_contract_negotiation, setup_client, ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_query_contract_negotiations(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (contract_negotiation_id, _) =
                seed_contract_negotiation(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

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
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_virtual_edc, seed_contract_negotiation, setup_client,
            wait_for_negotiation_state, ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_terminate_a_contract_negotiations(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (contract_negotiation_id, _) =
                seed_contract_negotiation(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

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
}

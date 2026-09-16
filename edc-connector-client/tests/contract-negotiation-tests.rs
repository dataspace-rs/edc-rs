mod common;

mod contract_negotiations {

    mod initiate {
        use edc_connector_client::{
            types::{
                catalog::DatasetRequest,
                contract_negotiation::ContractRequest,
                policy::{Action, Permission, Policy, PolicyKind, Target},
            },
            EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;

        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc, seed, setup_client,
            CatalogExtraFields, ClientParams, PROVIDER_ID,
        };

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_initiate_a_contract_negotiation(
            #[case] consumer: ClientParams,
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider_addr = provider.protocol_address.clone();
            let provider_id = provider.protocol_id.clone();
            let protocol = consumer.protocol.clone();
            let provider = setup_client(provider, version);
            let consumer = setup_client(consumer, version);

            let (asset_id, _, _) = seed(&provider, version).await;

            let dataset_request = DatasetRequest::builder()
                .counter_party_address(provider_addr.clone())
                .counter_party_id(provider_id.clone())
                .protocol(protocol.clone())
                .id(&asset_id)
                .build();

            let dataset = consumer
                .catalogue(version)
                .dataset::<CatalogExtraFields>(&dataset_request)
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
                .contract_negotiations(version)
                .initiate(&request)
                .await
                .unwrap();

            assert!(response.created_at() > 0);
        }

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_fail_to_initiate_a_contact_negotiation_with_wrong_policy(
            #[case] consumer: ClientParams,
            #[case] provider: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider_addr = provider.protocol_address.clone();
            let provider_id = provider.protocol_id.clone();
            let protocol = consumer.protocol.clone();
            let provider = setup_client(provider, version);
            let consumer = setup_client(consumer, version);

            let (asset_id, _, _) = seed(&provider, version).await;

            let dataset_request = DatasetRequest::builder()
                .counter_party_address(provider_addr.clone())
                .counter_party_id(provider_id.clone())
                .protocol(protocol.clone())
                .id(&asset_id)
                .build();

            let dataset = consumer
                .catalogue(version)
                .dataset::<CatalogExtraFields>(&dataset_request)
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

            let response = consumer
                .contract_negotiations(version)
                .initiate(&request)
                .await;

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
        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc,
            seed_contract_negotiation, setup_client, ClientParams,
        };
        use edc_connector_client::types::contract_negotiation::{
            ContractNegotiationKind, ContractNegotiationState,
        };
        use edc_connector_client::EdcConnectorApiVersion;
        use rstest::rstest;

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_get_a_contract_negotiation(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (contract_negotiation_id, _) = seed_contract_negotiation(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
            )
            .await;

            let cn = consumer
                .contract_negotiations(version)
                .get(&contract_negotiation_id)
                .await
                .unwrap();

            assert_eq!(contract_negotiation_id, cn.id());
            assert_ne!(&ContractNegotiationState::Terminated, cn.state());
            assert_eq!(0, cn.callback_addresses().len());
            assert_eq!(&Some("provider".to_string()), cn.counter_party_id());
            assert_eq!(&ContractNegotiationKind::Consumer, cn.kind());
        }

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_get_a_state_of_contract_negotiation(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (contract_negotiation_id, _) = seed_contract_negotiation(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
            )
            .await;

            let state_response = consumer
                .contract_negotiations(version)
                .get_state(&contract_negotiation_id)
                .await;

            assert!(state_response.is_ok())
        }
    }

    mod query {
        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc,
            seed_contract_negotiation, setup_client, ClientParams,
        };
        use edc_connector_client::types::query::Query;
        use edc_connector_client::EdcConnectorApiVersion;
        use rstest::rstest;

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_query_contract_negotiations(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (contract_negotiation_id, _) = seed_contract_negotiation(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
            )
            .await;

            let negotiations = consumer
                .contract_negotiations(version)
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
            types::contract_negotiation::ContractNegotiationState, EdcConnectorApiVersion, Error,
            ManagementApiError,
        };
        use rstest::rstest;

        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc,
            seed_contract_negotiation, setup_client, wait_for_negotiation_state, ClientParams,
        };

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_terminate_a_contract_negotiations(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (contract_negotiation_id, _) = seed_contract_negotiation(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
            )
            .await;

            wait_for_negotiation_state(
                &consumer,
                &contract_negotiation_id,
                ContractNegotiationState::Finalized,
                version,
            )
            .await;

            let result = consumer
                .contract_negotiations(version)
                .terminate(&contract_negotiation_id, "test")
                .await;

            assert!(matches!(
                result,
                Err(Error::ManagementApi(ManagementApiError { .. }))
            ));
        }
    }

    mod agreement {
        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc,
            seed_contract_agreement, setup_client, ClientParams,
        };
        use edc_connector_client::{
            EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_get_the_agreement_of_a_contract_negotiation(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (agreement_id, contract_negotiation_id, asset_id) = seed_contract_agreement(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
            )
            .await;

            let agreement = consumer
                .contract_negotiations(version)
                .get_agreement(&contract_negotiation_id)
                .await
                .unwrap();

            assert_eq!(agreement_id, agreement.id());
            assert_eq!(asset_id, agreement.asset_id());
        }

        #[rstest]
        #[case(consumer(), EdcConnectorApiVersion::V4)]
        #[case(consumer_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_fail_to_get_the_agreement_when_negotiation_not_existing(
            #[case] consumer_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let consumer = setup_client(consumer_cfg, version);

            let response = consumer
                .contract_negotiations(version)
                .get_agreement(&Uuid::new_v4().to_string())
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

    mod delete {
        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc,
            seed_contract_agreement, seed_contract_negotiation_with_offer, setup_client,
            wait_for_negotiation_state, ClientParams,
        };
        use edc_connector_client::{
            types::contract_negotiation::ContractNegotiationState, EdcConnectorApiVersion, Error,
            ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_delete_a_terminated_contract_negotiation(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            // An offer id the provider cannot resolve: the negotiation is
            // accepted by the consumer but ends up terminated.
            let (contract_negotiation_id, _) = seed_contract_negotiation_with_offer(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
                |offer_id| format!("{offer_id}-unknown"),
            )
            .await;

            wait_for_negotiation_state(
                &consumer,
                &contract_negotiation_id,
                ContractNegotiationState::Terminated,
                version,
            )
            .await;

            consumer
                .contract_negotiations(version)
                .delete(&contract_negotiation_id)
                .await
                .unwrap();

            let response = consumer
                .contract_negotiations(version)
                .get(&contract_negotiation_id)
                .await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::NOT_FOUND,
                    ..
                }))
            ))
        }

        #[rstest]
        #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
        #[case(
            consumer_virtual_edc(),
            provider_virtual_edc(),
            EdcConnectorApiVersion::V5
        )]
        #[tokio::test]
        async fn should_fail_to_delete_a_contract_negotiation_with_an_agreement(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (_, contract_negotiation_id, _) = seed_contract_agreement(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
            )
            .await;

            let response = consumer
                .contract_negotiations(version)
                .delete(&contract_negotiation_id)
                .await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::CONFLICT,
                    error_detail: ManagementApiErrorDetailKind::Parsed(..)
                }))
            ))
        }

        #[rstest]
        #[case(consumer(), EdcConnectorApiVersion::V4)]
        #[case(consumer_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_fail_to_delete_a_contract_negotiation_when_not_existing(
            #[case] consumer_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let consumer = setup_client(consumer_cfg, version);

            let response = consumer
                .contract_negotiations(version)
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

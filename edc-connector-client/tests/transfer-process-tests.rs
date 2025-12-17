mod common;

mod transfer_processes {

    mod initiate {
        use edc_connector_client::{
            types::{
                data_address::DataAddress,
                transfer_process::{TransferProcessState, TransferRequest},
            },
            Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_v4_2025, wait_for_transfer_state, ClientParams,
        };
        use crate::common::{seed_contract_agreement, setup_client};

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_v4_2025())]
        #[tokio::test]
        async fn should_initiate_a_transfer_process(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (agreement_id, _, _) =
                seed_contract_agreement(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            let request = TransferRequest::builder()
                .counter_party_address(provider_cfg.protocol_address)
                .protocol(consumer_cfg.protocol)
                .contract_id(&agreement_id)
                .transfer_type("HttpData-PULL")
                .destination(DataAddress::builder().kind("HttpProxy").build().unwrap())
                .build();

            let response = consumer
                .transfer_processes()
                .initiate(&request)
                .await
                .unwrap();

            assert!(response.created_at() > 0);

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Started).await;
        }

        #[rstest]
        #[case(consumer_v3())]
        #[case(consumer_v4())]
        #[case(consumer_virtual_edc())]
        #[tokio::test]
        async fn should_fail_to_initiate_a_transfer_process_with_wrong_contract(
            #[case] consumer_cfg: ClientParams,
        ) {
            let consumer = setup_client(consumer_cfg.clone());

            let request = TransferRequest::builder()
                .counter_party_address(consumer_cfg.protocol_address)
                .protocol(consumer_cfg.protocol)
                .contract_id(&Uuid::new_v4().to_string())
                .transfer_type("HttpData-PULL")
                .destination(DataAddress::builder().kind("HttpProxy").build().unwrap())
                .build();

            let response = consumer.transfer_processes().initiate(&request).await;

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

        use edc_connector_client::types::{
            callback_address::CallbackAddress,
            data_address::DataAddress,
            transfer_process::{TransferProcessKind, TransferProcessState, TransferRequest},
        };
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_v4_2025, wait_for_transfer_state, ClientParams,
        };
        use crate::common::{seed_contract_agreement, setup_client};

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_v4_2025())]
        #[tokio::test]
        async fn should_get_a_transfer_process(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (agreement_id, _, asset_id) =
                seed_contract_agreement(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            let cb = CallbackAddress::builder()
                .uri("http://localhost:80")
                .events(vec!["transfer.process".to_string()])
                .build();

            let request = TransferRequest::builder()
                .counter_party_address(provider_cfg.protocol_address)
                .protocol(consumer_cfg.protocol)
                .contract_id(&agreement_id)
                .transfer_type("HttpData-PULL")
                .callback_address(cb.clone())
                .destination(DataAddress::builder().kind("HttpProxy").build().unwrap())
                .build();

            let response = consumer
                .transfer_processes()
                .initiate(&request)
                .await
                .unwrap();

            assert!(response.created_at() > 0);

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Started).await;

            let tp = consumer
                .transfer_processes()
                .get(response.id())
                .await
                .unwrap();

            assert_eq!(response.id(), tp.id());
            assert_eq!("HttpData-PULL", tp.transfer_type());
            assert_eq!(asset_id, tp.asset_id());
            assert_eq!(agreement_id, tp.contract_id());
            assert_eq!(
                "HttpProxy",
                tp.data_destination()
                    .and_then(|destination| destination.property::<String>("type").unwrap())
                    .unwrap()
            );

            assert_eq!(&TransferProcessKind::Consumer, tp.kind());
            assert!(tp.state_timestamp() > 0);

            assert!(tp.callback_addresses().contains(&cb))
        }
    }

    mod query {
        use edc_connector_client::types::{
            data_address::DataAddress,
            query::Query,
            transfer_process::{TransferProcessState, TransferRequest},
        };
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_v4_2025, seed_contract_agreement, setup_client, wait_for_transfer_state,
            ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_v4_2025())]
        #[tokio::test]
        async fn should_query_transfer_processes(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (agreement_id, _, asset_id) =
                seed_contract_agreement(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            let request = TransferRequest::builder()
                .counter_party_address(provider_cfg.protocol_address)
                .protocol(consumer_cfg.protocol)
                .contract_id(&agreement_id)
                .transfer_type("HttpData-PULL")
                .destination(DataAddress::builder().kind("HttpProxy").build().unwrap())
                .build();

            let response = consumer
                .transfer_processes()
                .initiate(&request)
                .await
                .unwrap();

            assert!(response.created_at() > 0);

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Started).await;

            let processes = consumer
                .transfer_processes()
                .query(Query::builder().filter("assetId", "=", asset_id).build())
                .await
                .unwrap();

            assert_eq!(processes.len(), 1);
        }
    }

    mod terminate {

        use edc_connector_client::types::{
            data_address::DataAddress,
            transfer_process::{TransferProcessState, TransferRequest},
        };
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_v4_2025, seed_contract_agreement, setup_client, wait_for_transfer_state,
            ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_v4_2025())]
        #[tokio::test]
        async fn should_terminate_transfer_processes(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (agreement_id, _, _) =
                seed_contract_agreement(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            let request = TransferRequest::builder()
                .counter_party_address(provider_cfg.protocol_address)
                .protocol(consumer_cfg.protocol)
                .contract_id(&agreement_id)
                .transfer_type("HttpData-PULL")
                .destination(DataAddress::builder().kind("HttpProxy").build().unwrap())
                .build();

            let response = consumer
                .transfer_processes()
                .initiate(&request)
                .await
                .unwrap();

            assert!(response.created_at() > 0);

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Started).await;

            consumer
                .transfer_processes()
                .terminate(response.id(), "reason")
                .await
                .unwrap();

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Terminated)
                .await;
        }
    }

    mod suspend {

        use edc_connector_client::types::{
            data_address::DataAddress,
            transfer_process::{TransferProcessState, TransferRequest},
        };
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, provider_v3, provider_v4, seed_contract_agreement,
            setup_client, wait_for_transfer_state, ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[tokio::test]
        async fn should_suspend_and_resume_transfer_processes(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (agreement_id, _, _) =
                seed_contract_agreement(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            let request = TransferRequest::builder()
                .counter_party_address(provider_cfg.protocol_address)
                .protocol(consumer_cfg.protocol)
                .contract_id(&agreement_id)
                .transfer_type("HttpData-PULL")
                .destination(DataAddress::builder().kind("HttpProxy").build().unwrap())
                .build();

            let response = consumer
                .transfer_processes()
                .initiate(&request)
                .await
                .unwrap();

            assert!(response.created_at() > 0);

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Started).await;

            consumer
                .transfer_processes()
                .suspend(response.id(), "reason")
                .await
                .unwrap();

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Suspended)
                .await;

            consumer
                .transfer_processes()
                .resume(response.id())
                .await
                .unwrap();

            wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Started).await;
        }
    }
}

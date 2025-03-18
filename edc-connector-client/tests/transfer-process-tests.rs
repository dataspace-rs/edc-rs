mod common;

mod initiate {
    use edc_connector_client::{
        types::{
            data_address::DataAddress,
            transfer_process::{TransferProcessState, TransferRequest},
        },
        Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use uuid::Uuid;

    use crate::common::seed_contract_agreement;
    use crate::common::{
        setup_consumer_client, setup_provider_client, wait_for_transfer_state, PROVIDER_PROTOCOL,
    };

    #[tokio::test]
    async fn should_initiate_a_transfer_process() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (agreement_id, _, _) = seed_contract_agreement(&consumer, &provider).await;

        let request = TransferRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
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

    #[tokio::test]
    async fn should_fail_to_initiate_a_transfer_process_with_wrong_contract() {
        let consumer = setup_consumer_client();

        let request = TransferRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
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

    use crate::common::seed_contract_agreement;
    use crate::common::{
        setup_consumer_client, setup_provider_client, wait_for_transfer_state, PROVIDER_PROTOCOL,
    };

    #[tokio::test]
    async fn should_get_a_transfer_process() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (agreement_id, _, asset_id) = seed_contract_agreement(&consumer, &provider).await;

        let cb = CallbackAddress::builder()
            .uri("http://localhost:80")
            .events(vec!["transfer.process".to_string()])
            .build();

        let request = TransferRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
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

    use crate::common::{
        seed_contract_agreement, setup_consumer_client, setup_provider_client,
        wait_for_transfer_state, PROVIDER_PROTOCOL,
    };

    #[tokio::test]
    async fn should_query_transfer_processes() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (agreement_id, _, asset_id) = seed_contract_agreement(&consumer, &provider).await;

        let request = TransferRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
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

    use crate::common::{
        seed_contract_agreement, setup_consumer_client, setup_provider_client,
        wait_for_transfer_state, PROVIDER_PROTOCOL,
    };

    #[tokio::test]
    async fn should_terminate_transfer_processes() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (agreement_id, _, _) = seed_contract_agreement(&consumer, &provider).await;

        let request = TransferRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
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

        wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Terminated).await;
    }
}

mod suspend {

    use edc_connector_client::types::{
        data_address::DataAddress,
        transfer_process::{TransferProcessState, TransferRequest},
    };

    use crate::common::{
        seed_contract_agreement, setup_consumer_client, setup_provider_client,
        wait_for_transfer_state, PROVIDER_PROTOCOL,
    };

    #[tokio::test]
    async fn should_suspend_and_resume_transfer_processes() {
        let provider = setup_provider_client();
        let consumer = setup_consumer_client();

        let (agreement_id, _, _) = seed_contract_agreement(&consumer, &provider).await;

        let request = TransferRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
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

        wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Suspended).await;

        consumer
            .transfer_processes()
            .resume(response.id())
            .await
            .unwrap();

        wait_for_transfer_state(&consumer, response.id(), TransferProcessState::Started).await;
    }
}

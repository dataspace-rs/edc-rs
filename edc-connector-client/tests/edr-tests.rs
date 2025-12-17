mod common;

mod get {

    use edc_connector_client::types::transfer_process::TransferProcessState;
    use rstest::rstest;

    use crate::common::{
        consumer_v3, consumer_v4, provider_v3, provider_v4, setup_client, wait_for_transfer_state,
        ClientParams,
    };
    use crate::common::{seed_transfer_process, wait_for};

    #[rstest]
    #[case(consumer_v3(), provider_v3())]
    #[case(consumer_v4(), provider_v4())]
    #[tokio::test]
    async fn should_receive_an_edr_in_cache(
        #[case] consumer_cfg: ClientParams,
        #[case] provider_cfg: ClientParams,
    ) {
        let provider = setup_client(provider_cfg.clone());
        let consumer = setup_client(consumer_cfg.clone());

        let (transfer_process_id, agreement_id, _, asset_id) =
            seed_transfer_process(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id,
            TransferProcessState::Started,
        )
        .await;

        let edr = wait_for(|| async { consumer.edrs().get_entry(&transfer_process_id).await })
            .await
            .unwrap();

        assert_eq!(agreement_id, edr.agreement_id());
        assert_eq!(asset_id, edr.asset_id());
    }
}

mod query {

    use edc_connector_client::types::query::Query;
    use edc_connector_client::types::transfer_process::TransferProcessState;
    use rstest::rstest;

    use crate::common::{
        consumer_v3, consumer_v4, provider_v3, provider_v4, setup_client, wait_for_transfer_state,
        ClientParams,
    };
    use crate::common::{seed_transfer_process, wait_for};

    #[rstest]
    #[case(consumer_v3(), provider_v3())]
    #[case(consumer_v4(), provider_v4())]
    #[tokio::test]
    async fn should_query_the_edr_cache(
        #[case] consumer_cfg: ClientParams,
        #[case] provider_cfg: ClientParams,
    ) {
        let provider = setup_client(provider_cfg.clone());
        let consumer = setup_client(consumer_cfg.clone());

        let (transfer_process_id, _, _, asset_id) =
            seed_transfer_process(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id,
            TransferProcessState::Started,
        )
        .await;

        let _ = wait_for(|| async { consumer.edrs().get_entry(&transfer_process_id).await })
            .await
            .unwrap();

        let edrs = wait_for(|| async {
            consumer
                .edrs()
                .query(Query::builder().filter("assetId", "=", &asset_id).build())
                .await
        })
        .await
        .unwrap();

        assert_eq!(1, edrs.len());
    }
}

mod delete {

    use edc_connector_client::types::transfer_process::TransferProcessState;
    use edc_connector_client::{Error, ManagementApiError, ManagementApiErrorDetailKind};
    use reqwest::StatusCode;
    use rstest::rstest;

    use crate::common::{
        consumer_v3, consumer_v4, provider_v3, provider_v4, setup_client, wait_for_transfer_state,
        ClientParams,
    };
    use crate::common::{seed_transfer_process, wait_for};

    #[rstest]
    #[case(consumer_v3(), provider_v3())]
    #[case(consumer_v4(), provider_v4())]
    #[tokio::test]
    async fn should_delete_a_cached_edr(
        #[case] consumer_cfg: ClientParams,
        #[case] provider_cfg: ClientParams,
    ) {
        let provider = setup_client(provider_cfg.clone());
        let consumer = setup_client(consumer_cfg.clone());

        let (transfer_process_id, _, _, _) =
            seed_transfer_process(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id,
            TransferProcessState::Started,
        )
        .await;

        wait_for(|| async { consumer.edrs().get_entry(&transfer_process_id).await })
            .await
            .unwrap();

        consumer.edrs().delete(&transfer_process_id).await.unwrap();

        let response = consumer.edrs().get_entry(&transfer_process_id).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Raw(..)
            }))
        ))
    }
}

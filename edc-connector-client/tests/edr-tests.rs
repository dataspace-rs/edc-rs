mod common;

mod get {
    use crate::common::{consumer, provider, setup_client, wait_for_transfer_state, ClientParams};
    use crate::common::{seed_transfer_process, wait_for};
    use edc_connector_client::types::transfer_process::TransferProcessState;
    use edc_connector_client::EdcConnectorApiVersion;
    use rstest::rstest;

    #[rstest]
    #[case(consumer(), provider(), EdcConnectorApiVersion::V3)]
    #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
    #[tokio::test]
    #[ignore]
    async fn should_receive_an_edr_in_cache(
        #[case] consumer_cfg: ClientParams,
        #[case] provider_cfg: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let provider = setup_client(provider_cfg.clone(), version);
        let consumer = setup_client(consumer_cfg.clone(), version);

        let (transfer_process_id, agreement_id, _, asset_id) =
            seed_transfer_process(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id,
            TransferProcessState::Started,
        )
        .await;

        let edr =
            wait_for(|| async { consumer.edrs(version).get_entry(&transfer_process_id).await })
                .await
                .unwrap();

        assert_eq!(agreement_id, edr.agreement_id());
        assert_eq!(asset_id, edr.asset_id());
    }
}

mod query {
    use crate::common::{consumer, provider, setup_client, wait_for_transfer_state, ClientParams};
    use crate::common::{seed_transfer_process, wait_for};
    use edc_connector_client::types::query::Query;
    use edc_connector_client::types::transfer_process::TransferProcessState;
    use edc_connector_client::EdcConnectorApiVersion;
    use rstest::rstest;

    #[rstest]
    #[case(consumer(), provider(), EdcConnectorApiVersion::V3)]
    #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
    #[tokio::test]
    #[ignore]
    async fn should_query_the_edr_cache(
        #[case] consumer_cfg: ClientParams,
        #[case] provider_cfg: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let provider = setup_client(provider_cfg.clone(), version);
        let consumer = setup_client(consumer_cfg.clone(), version);

        let (transfer_process_id, _, _, asset_id) =
            seed_transfer_process(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id,
            TransferProcessState::Started,
        )
        .await;

        let _ = wait_for(|| async { consumer.edrs(version).get_entry(&transfer_process_id).await })
            .await
            .unwrap();

        let edrs = wait_for(|| async {
            consumer
                .edrs(version)
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
    use edc_connector_client::{
        EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;

    use crate::common::{consumer, provider, setup_client, wait_for_transfer_state, ClientParams};
    use crate::common::{seed_transfer_process, wait_for};

    #[rstest]
    #[case(consumer(), provider(), EdcConnectorApiVersion::V3)]
    #[case(consumer(), provider(), EdcConnectorApiVersion::V4)]
    #[tokio::test]
    #[ignore]
    async fn should_delete_a_cached_edr(
        #[case] consumer_cfg: ClientParams,
        #[case] provider_cfg: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let provider = setup_client(provider_cfg.clone(), version);
        let consumer = setup_client(consumer_cfg.clone(), version);

        let (transfer_process_id, _, _, _) =
            seed_transfer_process(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

        wait_for_transfer_state(
            &consumer,
            &transfer_process_id,
            TransferProcessState::Started,
        )
        .await;

        wait_for(|| async { consumer.edrs(version).get_entry(&transfer_process_id).await })
            .await
            .unwrap();

        consumer
            .edrs(version)
            .delete(&transfer_process_id)
            .await
            .unwrap();

        let response = consumer.edrs(version).get_entry(&transfer_process_id).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Raw(..)
            }))
        ))
    }
}

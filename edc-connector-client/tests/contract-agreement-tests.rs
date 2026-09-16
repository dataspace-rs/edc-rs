mod common;

mod contract_agreements {

    mod get {
        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc,
            seed_contract_negotiation, setup_client, wait_for_negotiation_state, ClientParams,
        };
        use edc_connector_client::types::contract_negotiation::ContractNegotiationState;
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
        async fn should_get_a_contract_agreement(
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

            let agreement_id = consumer
                .contract_negotiations(version)
                .get(&contract_negotiation_id)
                .await
                .map(|cn| cn.contract_agreement_id().cloned())
                .unwrap()
                .unwrap();

            let contract_agreement = consumer
                .contract_agreements(version)
                .get(&agreement_id)
                .await
                .unwrap();

            assert_eq!(agreement_id, contract_agreement.id());
        }
    }

    mod query {
        use crate::common::{
            consumer, consumer_virtual_edc, provider, provider_virtual_edc,
            seed_contract_negotiation, setup_client, wait_for_negotiation_state, ClientParams,
        };
        use edc_connector_client::types::{
            contract_negotiation::ContractNegotiationState, query::Query,
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
        async fn should_query_contract_agreements(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (contract_negotiation_id, asset_id) = seed_contract_negotiation(
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

            let agreements = consumer
                .contract_agreements(version)
                .query(Query::builder().filter("assetId", "=", asset_id).build())
                .await
                .unwrap();

            assert_eq!(1, agreements.len());
        }
    }

    mod negotiation {
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
        async fn should_get_the_negotiation_of_a_contract_agreement(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let provider = setup_client(provider_cfg.clone(), version);
            let consumer = setup_client(consumer_cfg.clone(), version);

            let (agreement_id, contract_negotiation_id, _) = seed_contract_agreement(
                &consumer,
                &consumer_cfg,
                &provider,
                &provider_cfg,
                version,
            )
            .await;

            let negotiation = consumer
                .contract_agreements(version)
                .get_negotiation(&agreement_id)
                .await
                .unwrap();

            assert_eq!(contract_negotiation_id, negotiation.id());
            assert_eq!(Some(&agreement_id), negotiation.contract_agreement_id());
        }

        #[rstest]
        #[case(consumer(), EdcConnectorApiVersion::V4)]
        #[case(consumer_virtual_edc(), EdcConnectorApiVersion::V5)]
        #[tokio::test]
        async fn should_fail_to_get_the_negotiation_when_agreement_not_existing(
            #[case] consumer_cfg: ClientParams,
            #[case] version: EdcConnectorApiVersion,
        ) {
            let consumer = setup_client(consumer_cfg, version);

            let response = consumer
                .contract_agreements(version)
                .get_negotiation(&Uuid::new_v4().to_string())
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

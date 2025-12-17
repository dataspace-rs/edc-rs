mod common;

mod contract_agreements {

    mod get {

        use edc_connector_client::types::contract_negotiation::ContractNegotiationState;
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
        async fn should_get_a_contract_agreement(
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

            let agreement_id = consumer
                .contract_negotiations()
                .get(&contract_negotiation_id)
                .await
                .map(|cn| cn.contract_agreement_id().cloned())
                .unwrap()
                .unwrap();

            let contract_agreement = consumer
                .contract_agreements()
                .get(&agreement_id)
                .await
                .unwrap();

            assert_eq!(agreement_id, contract_agreement.id());
        }
    }

    mod query {
        use edc_connector_client::types::{
            contract_negotiation::ContractNegotiationState, query::Query,
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
        async fn should_query_contract_agreements(
            #[case] consumer_cfg: ClientParams,
            #[case] provider_cfg: ClientParams,
        ) {
            let provider = setup_client(provider_cfg.clone());
            let consumer = setup_client(consumer_cfg.clone());

            let (contract_negotiation_id, asset_id) =
                seed_contract_negotiation(&consumer, &consumer_cfg, &provider, &provider_cfg).await;

            wait_for_negotiation_state(
                &consumer,
                &contract_negotiation_id,
                ContractNegotiationState::Finalized,
            )
            .await;

            let agreements = consumer
                .contract_agreements()
                .query(Query::builder().filter("assetId", "=", asset_id).build())
                .await
                .unwrap();

            assert_eq!(1, agreements.len());
        }
    }
}

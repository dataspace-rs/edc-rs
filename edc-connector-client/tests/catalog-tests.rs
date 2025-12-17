mod common;

mod catalog {

    mod catalog {
        use edc_connector_client::{
            types::{catalog::CatalogRequest, query::Query},
            EDC_NAMESPACE,
        };
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_virtual_edc, seed, setup_client, ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_get_the_catalog(
            #[case] consumer: ClientParams,
            #[case] provider: ClientParams,
        ) {
            use crate::common::PROVIDER_ID;

            let provider_protocol_addr = provider.protocol_address.clone();
            let protocol = consumer.protocol.clone();
            let consumer = setup_client(consumer);
            let provider = setup_client(provider);

            let (asset_id, _, _) = seed(&provider).await;

            let request = CatalogRequest::builder()
                .counter_party_address(provider_protocol_addr)
                .counter_party_id(PROVIDER_ID)
                .protocol(protocol)
                .query_spec(
                    Query::builder()
                        .filter(&format!("{}id", EDC_NAMESPACE), "=", asset_id.to_string())
                        .build(),
                )
                .build();

            let response = consumer.catalogue().request(&request).await.unwrap();

            let dataset = response.datasets().iter().find(|ds| ds.id() == asset_id);

            assert!(dataset.is_some());
        }
    }

    mod dataset {
        use edc_connector_client::types::catalog::DatasetRequest;
        use rstest::rstest;

        use crate::common::{
            consumer_v3, consumer_v4, consumer_virtual_edc, provider_v3, provider_v4,
            provider_virtual_edc, seed, setup_client, ClientParams,
        };

        #[rstest]
        #[case(consumer_v3(), provider_v3())]
        #[case(consumer_v4(), provider_v4())]
        #[case(consumer_virtual_edc(), provider_virtual_edc())]
        #[tokio::test]
        async fn should_get_the_dataset(
            #[case] consumer: ClientParams,
            #[case] provider: ClientParams,
        ) {
            let provider_protocol_addr = provider.protocol_address.clone();
            let provider_protocol_id = provider.protocol_id.clone();
            let protocol = consumer.protocol.clone();
            let consumer = setup_client(consumer);
            let provider = setup_client(provider);

            let (asset_id, _, _) = seed(&provider).await;

            let request = DatasetRequest::builder()
                .counter_party_address(provider_protocol_addr)
                .counter_party_id(provider_protocol_id)
                .protocol(protocol)
                .id(&asset_id)
                .build();

            let dataset = consumer.catalogue().dataset(&request).await.unwrap();

            assert_eq!(asset_id, dataset.id());
        }
    }
}

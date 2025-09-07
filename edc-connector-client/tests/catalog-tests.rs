mod common;

mod catalog {
    use edc_connector_client::{
        types::{catalog::CatalogRequest, query::Query},
        EDC_NAMESPACE,
    };

    use crate::common::{seed, setup_consumer_client, setup_provider_client, PROVIDER_PROTOCOL};

    #[tokio::test]
    async fn should_get_the_catalog() {
        let consumer = setup_consumer_client();
        let provider = setup_provider_client();

        let (asset_id, _, _) = seed(&provider).await;

        let request = CatalogRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
            .query_spec(
                Query::builder()
                    .filter(&format!("{}id", EDC_NAMESPACE), "=", asset_id.to_string())
                    .build(),
            )
            .build();

        let response = consumer.catalogue().request(&request).await.unwrap();

        let dataset = response
            .datasets()
            .into_iter()
            .find(|ds| ds.id() == asset_id);

        assert!(dataset.is_some());
    }
}

mod dataset {
    use edc_connector_client::types::catalog::DatasetRequest;

    use crate::common::{seed, setup_consumer_client, setup_provider_client, PROVIDER_PROTOCOL};

    #[tokio::test]
    async fn should_get_the_dataset() {
        let consumer = setup_consumer_client();
        let provider = setup_provider_client();

        let (asset_id, _, _) = seed(&provider).await;

        let request = DatasetRequest::builder()
            .counter_party_address(PROVIDER_PROTOCOL)
            .id(&asset_id)
            .build();

        let dataset = consumer.catalogue().dataset(&request).await.unwrap();

        assert_eq!(asset_id, dataset.id());
    }
}

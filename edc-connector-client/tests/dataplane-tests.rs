mod common;

mod dataplane {
    use crate::common::{provider, setup_client, ClientParams};
    use edc_connector_client::EdcConnectorApiVersion;
    use rstest::rstest;

    #[rstest]
    #[case(provider(), EdcConnectorApiVersion::V3)]
    #[case(provider(), EdcConnectorApiVersion::V4)]
    #[tokio::test]
    async fn should_fetch_dataplanes(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_client(provider, version);

        let response = client.data_planes(version).list().await.unwrap();
        assert!(!response.is_empty());
    }
}

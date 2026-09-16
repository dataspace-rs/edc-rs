mod common;

// Discovery is a participant scoped V5 resource: the consumer resolves the
// provider's well-known versions document and matches it against its profiles.
#[allow(clippy::unwrap_used)]
mod discovery {
    use edc_connector_client::{types::discovery::DiscoveryRequest, EdcConnectorApiVersion};
    use rstest::rstest;

    use crate::common::{consumer_virtual_edc, provider_virtual_edc, setup_client, ClientParams};

    #[rstest]
    #[case(
        consumer_virtual_edc(),
        provider_virtual_edc(),
        EdcConnectorApiVersion::V5
    )]
    #[tokio::test]
    async fn should_discover_a_counter_party(
        #[case] consumer_cfg: ClientParams,
        #[case] provider_cfg: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let _provider = setup_client(provider_cfg.clone(), version);
        let consumer = setup_client(consumer_cfg.clone(), version);

        // The base protocol URL of the provider, without the profile segment.
        let provider_address = provider_cfg
            .protocol_address
            .trim_end_matches("/http-dsp-profile-2025-1")
            .to_string();

        let matches = consumer
            .discovery(version)
            .discover(
                &DiscoveryRequest::builder()
                    .counter_party_address(provider_address)
                    .build(),
            )
            .await
            .unwrap();

        assert!(!matches.is_empty());
        let dsp_2025 = matches
            .iter()
            .find(|m| m.profile() == "http-dsp-profile-2025-1")
            .unwrap();
        assert_eq!("2025-1", dsp_2025.version());
        assert_eq!("HTTPS", dsp_2025.binding());
        assert_eq!("/http-dsp-profile-2025-1", dsp_2025.counter_party().path());
        assert!(dsp_2025.counter_party().data_service_endpoint().is_some());
    }
}

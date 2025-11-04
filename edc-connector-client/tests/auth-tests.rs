mod common;

mod auth {
    use edc_connector_client::types::{asset::NewAsset, data_address::DataAddress};
    use edc_connector_client::{Error, ManagementApiError, ManagementApiErrorDetailKind};
    use reqwest::StatusCode;
    use uuid::Uuid;

    use crate::common::setup_provider_client_with_auth;

    #[tokio::test]
    async fn should_failt_to_create_an_asset() {
        let client = setup_provider_client_with_auth(edc_connector_client::Auth::NoAuth);

        let id = Uuid::new_v4().to_string();

        let asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let response = client.assets().create(&asset).await;

        assert!(matches!(response, Err(Error::ManagementApi(ManagementApiError { status_code: StatusCode::UNAUTHORIZED, error_detail: ManagementApiErrorDetailKind::Parsed(_)}))));
    }
}

use common::{provider_v3, provider_v4, setup_client, ClientParams};
use rstest::rstest;

mod common;

#[rstest]
#[case(provider_v3())]
#[case(provider_v4())]
#[tokio::test]
async fn should_fetch_dataplanes(#[case] provider: ClientParams) {
    let client = setup_client(provider);

    let response = client.data_planes().list().await.unwrap();
    assert!(!response.is_empty());
}

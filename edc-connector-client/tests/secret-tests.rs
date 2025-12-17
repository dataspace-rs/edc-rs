mod common;

mod secrets {

    mod create {
        use edc_connector_client::{
            types::secret::NewSecret, Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_create_a_secret(#[case] provider: ClientParams) {
            let client = setup_client(provider);

            let id = Uuid::new_v4().to_string();

            let secret = NewSecret::builder().id(&id).value("bar").build();

            let response = client.secrets().create(&secret).await.unwrap();

            assert_eq!(&id, response.id());
            assert!(response.created_at() > 0);
        }

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_failt_to_create_a_secret_when_existing(#[case] provider: ClientParams) {
            let client = setup_client(provider);

            let id = Uuid::new_v4().to_string();

            let secret = NewSecret::builder().id(&id).value("bar").build();

            let response = client.secrets().create(&secret).await.unwrap();

            assert_eq!(&id, response.id());
            assert!(response.created_at() > 0);

            let response = client.secrets().create(&secret).await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::CONFLICT,
                    error_detail: ManagementApiErrorDetailKind::Parsed(..)
                }))
            ))
        }
    }

    mod delete {
        use edc_connector_client::{
            types::secret::NewSecret, Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_delete_an_secret(#[case] provider: ClientParams) {
            let client = setup_client(provider);
            let id = Uuid::new_v4().to_string();

            let secret = NewSecret::builder().id(&id).value("bar").build();

            let asset = client.secrets().create(&secret).await.unwrap();

            let response = client.secrets().delete(asset.id()).await;

            assert!(response.is_ok());
        }

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_fail_to_delete_a_secret_when_not_existing(#[case] provider: ClientParams) {
            let client = setup_client(provider);
            let id = Uuid::new_v4().to_string();

            let response = client.secrets().delete(&id).await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::NOT_FOUND,
                    error_detail: ManagementApiErrorDetailKind::Parsed(..)
                }))
            ))
        }
    }

    mod get {
        use edc_connector_client::{
            types::secret::NewSecret, Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_get_a_secret(#[case] provider: ClientParams) {
            let client = setup_client(provider);
            let id = Uuid::new_v4().to_string();
            let secret = NewSecret::builder().id(&id).value("bar").build();

            let secret = client.secrets().create(&secret).await.unwrap();

            let secret = client.secrets().get(secret.id()).await.unwrap();

            assert_eq!("bar", secret.value())
        }

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_fail_to_get_a_secret_when_not_existing(#[case] provider: ClientParams) {
            let client = setup_client(provider);
            let id = Uuid::new_v4().to_string();

            let response = client.secrets().get(&id).await;

            assert!(matches!(
                response,
                Err(Error::ManagementApi(ManagementApiError {
                    status_code: StatusCode::NOT_FOUND,
                    error_detail: ManagementApiErrorDetailKind::Parsed(..)
                }))
            ))
        }
    }

    mod update {
        use edc_connector_client::{
            types::secret::{NewSecret, Secret},
            Error, ManagementApiError, ManagementApiErrorDetailKind,
        };
        use reqwest::StatusCode;
        use rstest::rstest;
        use uuid::Uuid;

        use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_update_a_secret(#[case] provider: ClientParams) {
            let client = setup_client(provider);
            let id = Uuid::new_v4().to_string();
            let secret = NewSecret::builder().id(&id).value("bar").build();

            client.secrets().create(&secret).await.unwrap();

            let updated_secret = Secret::builder().id(&id).value("bar2").build();

            client.secrets().update(&updated_secret).await.unwrap();

            let secret = client.secrets().get(&id).await.unwrap();

            assert_eq!("bar2", secret.value())
        }

        #[rstest]
        #[case(provider_v3())]
        #[case(provider_v4())]
        #[tokio::test]
        async fn should_fail_to_update_a_secret_when_not_existing(#[case] provider: ClientParams) {
            let client = setup_client(provider);
            let id = Uuid::new_v4().to_string();

            let updated_secret = Secret::builder().id(&id).value("bar2").build();

            let response = client.secrets().update(&updated_secret).await;

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

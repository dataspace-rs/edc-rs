mod common;

mod create {
    use edc_connector_client::{
        types::{asset::NewAsset, data_address::DataAddress},
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
    async fn should_create_an_asset(#[case] provider: ClientParams) {
        let client = setup_client(provider);

        let id = Uuid::new_v4().to_string();

        let asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let response = client.assets().create(&asset).await.unwrap();

        assert_eq!(&id, response.id());
        assert!(response.created_at() > 0);
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_failt_to_create_an_asset_when_existing(#[case] provider: ClientParams) {
        let client = setup_client(provider);

        let id = Uuid::new_v4().to_string();

        let asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let response = client.assets().create(&asset).await.unwrap();

        assert_eq!(&id, response.id());
        assert!(response.created_at() > 0);

        let response = client.assets().create(&asset).await;

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
        types::{asset::NewAsset, data_address::DataAddress},
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
    async fn should_delete_an_asset(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let new_asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let asset = client.assets().create(&new_asset).await.unwrap();

        let response = client.assets().delete(asset.id()).await;

        assert!(response.is_ok());
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_fail_to_delete_an_asset_when_not_existing(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();

        let response = client.assets().delete(&id).await;

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
        types::{asset::NewAsset, data_address::DataAddress},
        ConversionError, Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_get_an_asset(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let new_asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let asset = client.assets().create(&new_asset).await.unwrap();

        let asset = client.assets().get(asset.id()).await.unwrap();

        assert_eq!("bar", asset.property::<String>("foo").unwrap().unwrap())
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_get_an_asset_with_array_property(#[case] provider: ClientParams) {
        let client = setup_client(provider);

        let id = Uuid::new_v4().to_string();

        let asset = NewAsset::builder()
            .id(&id)
            .property("foo", vec!["bar"])
            .property("foo_arr", vec!["bar", "baz"])
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let asset = client.assets().create(&asset).await.unwrap();

        let asset = client.assets().get(asset.id()).await.unwrap();

        assert_eq!(
            vec!["bar"],
            asset.property::<Vec<String>>("foo").unwrap().unwrap()
        );
        assert_eq!("bar", asset.property::<String>("foo").unwrap().unwrap());

        assert_eq!(
            vec!["bar", "baz"],
            asset.property::<Vec<String>>("foo_arr").unwrap().unwrap()
        );

        assert!(matches!(
            asset.property::<String>("foo_arr"),
            Err(ConversionError { .. })
        ));
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_fail_to_get_an_asset_when_not_existing(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();

        let response = client.assets().get(&id).await;

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
        types::{
            asset::{Asset, NewAsset},
            data_address::DataAddress,
        },
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
    async fn should_update_an_asset(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let new_asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        client.assets().create(&new_asset).await.unwrap();

        let updated_asset = Asset::builder()
            .id(&id)
            .property("foo", "bar2")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        client.assets().update(&updated_asset).await.unwrap();

        let asset = client.assets().get(&id).await.unwrap();

        assert_eq!("bar2", asset.property::<String>("foo").unwrap().unwrap())
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_fail_to_update_an_asset_when_not_existing(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();

        let updated_asset = Asset::builder()
            .id(&id)
            .property("foo", "bar2")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let response = client.assets().update(&updated_asset).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }
}

mod query {
    use edc_connector_client::{
        types::{
            asset::NewAsset,
            data_address::DataAddress,
            query::{Query, SortOrder},
        },
        EDC_NAMESPACE,
    };
    use rstest::rstest;
    use uuid::Uuid;

    use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_query_an_asset(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let new_asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        client.assets().create(&new_asset).await.unwrap();

        let assets = client
            .assets()
            .query(
                Query::builder()
                    .filter(&format!("{}{}", EDC_NAMESPACE, "id"), "=", &id)
                    .filter(&format!("{}{}", EDC_NAMESPACE, "foo"), "=", "bar")
                    .build(),
            )
            .await
            .unwrap();

        assert_eq!(1, assets.len());

        assert_eq!(
            "bar",
            assets
                .first()
                .unwrap()
                .property::<String>("foo")
                .unwrap()
                .unwrap()
        )
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_query_an_asset_with_sort(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let id_1 = Uuid::new_v4().to_string();
        let group = Uuid::new_v4().to_string();
        let new_asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .property("group", &group)
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let new_asset_2 = NewAsset::builder()
            .id(&id_1)
            .property("foo", "baz")
            .property("group", &group)
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        client.assets().create(&new_asset).await.unwrap();
        client.assets().create(&new_asset_2).await.unwrap();

        let query = Query::builder()
            .filter(&format!("{}{}", EDC_NAMESPACE, "group"), "=", &group)
            .sort(&format!("{}{}", EDC_NAMESPACE, "foo"), SortOrder::Desc)
            .build();

        let assets = client.assets().query(query).await.unwrap();

        assert_eq!(2, assets.len());

        assert_eq!(
            Ok(Some("baz".to_string())),
            assets.first().unwrap().property::<String>("foo")
        )
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_query_an_asset_with_limit(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let id_1 = Uuid::new_v4().to_string();
        let group = Uuid::new_v4().to_string();
        let new_asset = NewAsset::builder()
            .id(&id)
            .property("foo", "bar")
            .property("group", &group)
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        let new_asset_2 = NewAsset::builder()
            .id(&id_1)
            .property("foo", "baz")
            .property("group", &group)
            .data_address(DataAddress::builder().kind("type").build().unwrap())
            .build();

        client.assets().create(&new_asset).await.unwrap();
        client.assets().create(&new_asset_2).await.unwrap();

        let query = Query::builder()
            .filter(&format!("{}{}", EDC_NAMESPACE, "group"), "=", &group)
            .sort(&format!("{}{}", EDC_NAMESPACE, "foo"), SortOrder::Asc)
            .limit(1)
            .build();

        let assets = client.assets().query(query).await.unwrap();

        assert_eq!(1, assets.len());

        assert_eq!(
            Ok(Some("bar".to_string())),
            assets.first().unwrap().property::<String>("foo")
        );

        let query = Query::builder()
            .filter(&format!("{}{}", EDC_NAMESPACE, "group"), "=", &group)
            .sort(&format!("{}{}", EDC_NAMESPACE, "foo"), SortOrder::Asc)
            .offset(1)
            .limit(1)
            .build();

        let assets = client.assets().query(query).await.unwrap();

        assert_eq!(1, assets.len());

        assert_eq!(
            Ok(Some("baz".to_string())),
            assets.first().unwrap().property::<String>("foo")
        )
    }
}

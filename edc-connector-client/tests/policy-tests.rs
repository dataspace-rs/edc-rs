mod common;
mod create {
    use edc_connector_client::{
        types::policy::{NewPolicyDefinition, Policy},
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
    async fn should_create_a_policy_definition(#[case] provider: ClientParams) {
        let client = setup_client(provider);

        let id = Uuid::new_v4().to_string();

        let policy_definition = NewPolicyDefinition::builder()
            .id(&id)
            .policy(Policy::builder().build())
            .build();

        let response = client.policies().create(&policy_definition).await.unwrap();

        assert_eq!(&id, response.id());
        assert!(response.created_at() > 0);
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_failt_to_create_an_policy_definition_when_existing(
        #[case] provider: ClientParams,
    ) {
        let client = setup_client(provider);

        let id = Uuid::new_v4().to_string();
        let policy_definition = NewPolicyDefinition::builder()
            .id(&id)
            .policy(Policy::builder().build())
            .build();

        let response = client.policies().create(&policy_definition).await.unwrap();

        assert_eq!(&id, response.id());
        assert!(response.created_at() > 0);

        let response = client.policies().create(&policy_definition).await;

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
        types::policy::{NewPolicyDefinition, Policy},
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
    async fn should_delete_a_policy_definition(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let policy_definition = NewPolicyDefinition::builder()
            .id(&id)
            .policy(Policy::builder().build())
            .build();

        let definition = client.policies().create(&policy_definition).await.unwrap();

        let response = client.policies().delete(definition.id()).await;

        assert!(response.is_ok());
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_fail_to_delete_policy_definition_when_not_existing(
        #[case] provider: ClientParams,
    ) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();

        let response = client.policies().delete(&id).await;

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
        types::policy::{
            Action, AtomicConstraint, Constraint, LeftOperand, NewPolicyDefinition, Operator,
            Permission, Policy, PolicyKind,
        },
        EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_get_a_policy_definition(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();

        let policy = Policy::builder()
            .permission(
                Permission::builder()
                    .action(Action::simple("use"))
                    .constraint(Constraint::Atomic(AtomicConstraint::new(
                        "foo", "eq", "bar",
                    )))
                    .build(),
            )
            .build();

        let policy_definition = NewPolicyDefinition::builder()
            .id(&id)
            .policy(policy.clone())
            .build();

        let created = client.policies().create(&policy_definition).await.unwrap();

        let definition = client.policies().get(created.id()).await.unwrap();

        assert_eq!(definition.policy().kind(), &PolicyKind::Set);
        assert_eq!(definition.policy().permissions().len(), 1);

        let permission = &definition.policy().permissions()[0];

        assert_eq!(permission.constraints().len(), 1);

        let constraint = &permission.constraints()[0];

        match client.api_version() {
            EdcConnectorApiVersion::V3 => {
                assert_eq!(permission.action().id(), "odrl:use");

                assert_eq!(
                    constraint,
                    &Constraint::Atomic(AtomicConstraint::new_with_operator(
                        "edc:foo",
                        Operator::id("odrl:eq"),
                        "bar"
                    ))
                );
            }
            EdcConnectorApiVersion::V4 => {
                assert_eq!(permission.action().id(), "use");

                assert_eq!(
                    constraint,
                    &Constraint::Atomic(AtomicConstraint::new_with_operator(
                        LeftOperand::simple("foo"),
                        Operator::simple("eq"),
                        "bar"
                    ))
                );
            }
        }
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_fail_to_get_a_policy_definition_when_not_existing(
        #[case] provider: ClientParams,
    ) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();

        let response = client.policies().get(&id).await;

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
        types::policy::{Action, NewPolicyDefinition, Permission, Policy, PolicyDefinition},
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
    async fn should_update_policy_definition(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let new_policy = NewPolicyDefinition::builder()
            .id(&id)
            .policy(Policy::builder().build())
            .build();

        client.policies().create(&new_policy).await.unwrap();

        let updated_policy = PolicyDefinition::builder()
            .id(&id)
            .policy(
                Policy::builder()
                    .permission(Permission::builder().action(Action::simple("use")).build())
                    .build(),
            )
            .build();

        client.policies().update(&updated_policy).await.unwrap();

        let definition = client.policies().get(&id).await.unwrap();

        assert_eq!(1, definition.policy().permissions().len());
    }

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_fail_to_update_an_policy_definition_when_not_existing(
        #[case] provider: ClientParams,
    ) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();

        let updated_policy = PolicyDefinition::builder()
            .id(&id)
            .policy(
                Policy::builder()
                    .permission(Permission::builder().action(Action::simple("use")).build())
                    .build(),
            )
            .build();

        let response = client.policies().update(&updated_policy).await;

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
    use edc_connector_client::types::{
        policy::{NewPolicyDefinition, Policy},
        query::Query,
    };
    use rstest::rstest;
    use uuid::Uuid;

    use crate::common::{provider_v3, provider_v4, setup_client, ClientParams};

    #[rstest]
    #[case(provider_v3())]
    #[case(provider_v4())]
    #[tokio::test]
    async fn should_query_policy_definitions(#[case] provider: ClientParams) {
        let client = setup_client(provider);
        let id = Uuid::new_v4().to_string();
        let new_policy = NewPolicyDefinition::builder()
            .id(&id)
            .policy(Policy::builder().build())
            .build();

        client.policies().create(&new_policy).await.unwrap();

        let definitions = client
            .policies()
            .query(Query::builder().filter("id", "=", id).build())
            .await
            .unwrap();

        assert_eq!(1, definitions.len());
    }
}

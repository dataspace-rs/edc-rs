mod common;

// CEL expressions are a global (admin scoped) V5 resource of the virtual
// connector: every test authenticates as the admin.
#[allow(clippy::unwrap_used)]
mod celexpressions {
    use edc_connector_client::{
        types::{
            common_expression_language::{
                CelExpressionTestRequest, CommonExpressionLanguage, NewCommonExpressionLanguage,
            },
            query::Query,
        },
        EdcConnectorApiVersion, Error, ManagementApiError, ManagementApiErrorDetailKind,
    };
    use reqwest::StatusCode;
    use rstest::rstest;
    use uuid::Uuid;

    use crate::common::{provider_virtual_edc, setup_admin_client, ClientParams};

    fn new_expression(id: &str) -> NewCommonExpressionLanguage {
        NewCommonExpressionLanguage::builder()
            .id(id)
            .left_operand(format!("https://w3id.org/edc/v0.0.1/ns/{id}"))
            .expression("ctx.agent.id == 'agent-1'".to_string())
            .description("test expression".to_string())
            .scopes(vec!["catalog".to_string()])
            .actions(vec!["use".to_string()])
            .build()
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_create_and_get_an_expression(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        let response = client
            .common_expression_language(version)
            .create(&new_expression(&id))
            .await
            .unwrap();
        assert_eq!(&id, response.id());

        let expression = client
            .common_expression_language(version)
            .get(&id)
            .await
            .unwrap();

        assert_eq!(id, expression.id());
        assert_eq!("ctx.agent.id == 'agent-1'", expression.expression());
        assert_eq!(&vec!["catalog".to_string()], expression.scopes());
        assert_eq!(&vec!["use".to_string()], expression.actions());
        assert_eq!(
            &Some("test expression".to_string()),
            expression.description()
        );

        client
            .common_expression_language(version)
            .delete(&id)
            .await
            .unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_update_an_expression(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        client
            .common_expression_language(version)
            .create(&new_expression(&id))
            .await
            .unwrap();

        let updated = CommonExpressionLanguage::builder()
            .id(&id)
            .left_operand(format!("https://w3id.org/edc/v0.0.1/ns/{id}"))
            .expression("ctx.agent.id == 'agent-2'".to_string())
            .description("updated".to_string())
            .scopes(vec![
                "catalog".to_string(),
                "contract.negotiation".to_string(),
            ])
            .build();

        client
            .common_expression_language(version)
            .update(&updated)
            .await
            .unwrap();

        let expression = client
            .common_expression_language(version)
            .get(&id)
            .await
            .unwrap();

        assert_eq!("ctx.agent.id == 'agent-2'", expression.expression());
        assert_eq!(2, expression.scopes().len());

        client
            .common_expression_language(version)
            .delete(&id)
            .await
            .unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_query_expressions(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        client
            .common_expression_language(version)
            .create(&new_expression(&id))
            .await
            .unwrap();

        let expressions = client
            .common_expression_language(version)
            .query(Query::builder().filter("id", "=", id.as_str()).build())
            .await
            .unwrap();

        assert_eq!(1, expressions.len());
        assert_eq!(id, expressions[0].id());

        client
            .common_expression_language(version)
            .delete(&id)
            .await
            .unwrap();
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_delete_an_expression(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);
        let id = Uuid::new_v4().to_string();

        client
            .common_expression_language(version)
            .create(&new_expression(&id))
            .await
            .unwrap();

        client
            .common_expression_language(version)
            .delete(&id)
            .await
            .unwrap();

        let response = client.common_expression_language(version).get(&id).await;

        assert!(matches!(
            response,
            Err(Error::ManagementApi(ManagementApiError {
                status_code: StatusCode::NOT_FOUND,
                error_detail: ManagementApiErrorDetailKind::Parsed(..)
            }))
        ))
    }

    #[rstest]
    #[case(provider_virtual_edc(), EdcConnectorApiVersion::V5)]
    #[tokio::test]
    async fn should_test_an_expression(
        #[case] provider: ClientParams,
        #[case] version: EdcConnectorApiVersion,
    ) {
        let client = setup_admin_client(provider, version);

        let request = CelExpressionTestRequest::builder()
            .left_operand("leftOperand")
            .expression("ctx.agent.id == 'agent-1'")
            .operator("EQ")
            .right_operand("rightOperand")
            .param("agent", serde_json::json!({"id": "agent-1"}))
            .build();

        let response = client
            .common_expression_language(version)
            .test(&request)
            .await
            .unwrap();

        assert_eq!(Some(true), response.evaluation_result());
        assert_eq!(None, response.error());

        let request = CelExpressionTestRequest::builder()
            .left_operand("leftOperand")
            .expression("ctx.agent.id == 'agent-1'")
            .operator("EQ")
            .right_operand("rightOperand")
            .param("agent", serde_json::json!({"id": "someone-else"}))
            .build();

        let response = client
            .common_expression_language(version)
            .test(&request)
            .await
            .unwrap();

        assert_eq!(Some(false), response.evaluation_result());
    }
}

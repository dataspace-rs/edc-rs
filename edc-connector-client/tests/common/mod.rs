#![allow(dead_code)]

use std::{future::Future, time::Duration};

use bon::Builder;
use edc_connector_client::{
    types::{
        asset::NewAsset,
        catalog::DatasetRequest,
        contract_definition::NewContractDefinition,
        contract_negotiation::{ContractNegotiationState, ContractRequest},
        data_address::DataAddress,
        policy::{Action, NewPolicyDefinition, Permission, Policy, PolicyKind, Target},
        query::Criterion,
        transfer_process::{TransferProcessState, TransferRequest},
    },
    Auth, EdcConnectorApiVersion, EdcConnectorClient, EDC_NAMESPACE,
};
use tokio::time::sleep;
use uuid::Uuid;

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const PROVIDER_ID: &str = "provider";

#[derive(Builder)]
pub struct ClientParams {
    pub management_url: String,
    #[builder(default = EdcConnectorApiVersion::V3)]
    pub version: EdcConnectorApiVersion,
}

pub fn provider_v3() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:29193/management".to_string())
        .version(EdcConnectorApiVersion::V3)
        .build()
}

pub fn consumer_v3() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:19193/management".to_string())
        .version(EdcConnectorApiVersion::V3)
        .build()
}

pub fn provider_v4() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:29193/management".to_string())
        .version(EdcConnectorApiVersion::V4)
        .build()
}

pub fn consumer_v4() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:19193/management".to_string())
        .version(EdcConnectorApiVersion::V4)
        .build()
}

pub fn setup_provider_client() -> EdcConnectorClient {
    setup_provider_client_with_auth(Auth::ApiToken("123456".to_string()))
}

pub fn setup_provider_client_with_auth(auth: Auth) -> EdcConnectorClient {
    EdcConnectorClient::builder()
        .management_url("http://localhost:29193/management")
        .with_auth(auth)
        .build()
        .unwrap()
}

pub fn setup_consumer_client() -> EdcConnectorClient {
    EdcConnectorClient::builder()
        .management_url("http://localhost:19193/management")
        .with_auth(Auth::api_token("123456"))
        .build()
        .unwrap()
}

pub fn setup_client(params: ClientParams) -> EdcConnectorClient {
    EdcConnectorClient::builder()
        .management_url(&params.management_url)
        .with_auth(Auth::api_token("123456"))
        .version(params.version)
        .build()
        .unwrap()
}

pub fn setup_consumer_client_v4() -> EdcConnectorClient {
    EdcConnectorClient::builder()
        .management_url("http://localhost:19193/management")
        .with_auth(Auth::api_token("123456"))
        .build()
        .unwrap()
}

pub async fn seed(client: &EdcConnectorClient) -> (String, String, String) {
    let asset = NewAsset::builder()
        .id(Uuid::new_v4().to_string().as_str())
        .data_address(
            DataAddress::builder()
                .kind("HttpData")
                .property("baseUrl", "https://jsonplaceholder.typicode.com/users")
                .build()
                .unwrap(),
        )
        .build();

    let asset_response = client.assets().create(&asset).await.unwrap();

    let policy_definition = NewPolicyDefinition::builder()
        .id(Uuid::new_v4().to_string().as_str())
        .policy(
            Policy::builder()
                .permission(Permission::builder().action(Action::simple("use")).build())
                .build(),
        )
        .build();

    let policy_response = client.policies().create(&policy_definition).await.unwrap();

    let contract_definition = NewContractDefinition::builder()
        .id(Uuid::new_v4().to_string().as_str())
        .asset_selector(Criterion::new(
            &format!("{}id", EDC_NAMESPACE),
            "=",
            asset_response.id(),
        ))
        .access_policy_id(policy_response.id())
        .contract_policy_id(policy_response.id())
        .build();

    let definition_response = client
        .contract_definitions()
        .create(&contract_definition)
        .await
        .unwrap();

    (
        asset_response.id().to_string(),
        policy_response.id().to_string(),
        definition_response.id().to_string(),
    )
}

pub async fn seed_contract_negotiation(
    consumer: &EdcConnectorClient,
    provider: &EdcConnectorClient,
) -> (String, String) {
    let (asset_id, _, _) = seed(provider).await;

    let dataset_request = DatasetRequest::builder()
        .counter_party_address(PROVIDER_PROTOCOL)
        .counter_party_id(PROVIDER_ID)
        .id(&asset_id)
        .build();

    let dataset = consumer
        .catalogue()
        .dataset(&dataset_request)
        .await
        .unwrap();

    let offer_id = dataset.offers()[0].id().unwrap();

    let request = ContractRequest::builder()
        .counter_party_address(PROVIDER_PROTOCOL)
        .counter_party_id(PROVIDER_ID)
        .policy(
            Policy::builder()
                .id(offer_id)
                .kind(PolicyKind::Offer)
                .assigner(PROVIDER_ID)
                .target(Target::simple(&asset_id))
                .permission(Permission::builder().action(Action::simple("use")).build())
                .build(),
        )
        .build();

    let response = consumer
        .contract_negotiations()
        .initiate(&request)
        .await
        .unwrap();

    (response.id().to_string(), asset_id)
}

pub async fn seed_contract_agreement(
    consumer: &EdcConnectorClient,
    provider: &EdcConnectorClient,
) -> (String, String, String) {
    let (contract_negotiation_id, asset_id) = seed_contract_negotiation(consumer, provider).await;

    wait_for_negotiation_state(
        consumer,
        &contract_negotiation_id,
        ContractNegotiationState::Finalized,
    )
    .await;

    let agreement_id = consumer
        .contract_negotiations()
        .get(&contract_negotiation_id)
        .await
        .map(|cn| cn.contract_agreement_id().cloned())
        .unwrap()
        .unwrap();

    let contract_agreement = consumer
        .contract_agreements()
        .get(&agreement_id)
        .await
        .unwrap();

    (
        contract_agreement.id().to_string(),
        contract_negotiation_id,
        asset_id,
    )
}

pub async fn seed_transfer_process(
    consumer: &EdcConnectorClient,
    provider: &EdcConnectorClient,
) -> (String, String, String, String) {
    let (contract_negotiation_id, asset_id) = seed_contract_negotiation(consumer, provider).await;

    wait_for_negotiation_state(
        consumer,
        &contract_negotiation_id,
        ContractNegotiationState::Finalized,
    )
    .await;

    let agreement_id = consumer
        .contract_negotiations()
        .get(&contract_negotiation_id)
        .await
        .map(|cn| cn.contract_agreement_id().cloned())
        .unwrap()
        .unwrap();

    let contract_agreement = consumer
        .contract_agreements()
        .get(&agreement_id)
        .await
        .unwrap();

    let request = TransferRequest::builder()
        .counter_party_address(PROVIDER_PROTOCOL)
        .contract_id(&agreement_id)
        .transfer_type("HttpData-PULL")
        .destination(DataAddress::builder().kind("HttpProxy").build().unwrap())
        .build();

    let response = consumer
        .transfer_processes()
        .initiate(&request)
        .await
        .unwrap();

    (
        response.id().to_string(),
        contract_agreement.id().to_string(),
        contract_negotiation_id,
        asset_id,
    )
}

pub async fn wait_for_negotiation_state(
    client: &EdcConnectorClient,
    id: &str,
    state: ContractNegotiationState,
) {
    wait_for(|| {
        let i_state = state.clone();
        async {
            client
                .contract_negotiations()
                .get_state(id)
                .await
                .map_err(|err| err.to_string())
                .and_then(|s| {
                    if s == state {
                        Ok(i_state)
                    } else {
                        Err("State mismatch".to_string())
                    }
                })
        }
    })
    .await
    .unwrap();
}

pub async fn wait_for_transfer_state(
    client: &EdcConnectorClient,
    id: &str,
    state: TransferProcessState,
) {
    wait_for(|| {
        let i_state = state.clone();
        async {
            client
                .transfer_processes()
                .get_state(id)
                .await
                .map_err(|err| err.to_string())
                .and_then(|s| {
                    if s == state {
                        Ok(i_state)
                    } else {
                        Err("State mismatch".to_string())
                    }
                })
        }
    })
    .await
    .unwrap();
}

pub async fn wait_for<F, Fut, R, E>(f: F) -> Result<R, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<R, E>>,
{
    let timeout = tokio::time::timeout(Duration::from_secs(30), async move {
        loop {
            match f().await {
                Ok(r) => break Ok(r),
                Err(_) => {
                    sleep(Duration::from_millis(200)).await;
                }
            }
        }
    });

    timeout.await.unwrap()
}

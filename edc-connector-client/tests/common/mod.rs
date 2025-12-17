#![allow(dead_code)]

use std::{collections::HashMap, future::Future, thread, time::Duration};

use bon::Builder;
use edc_connector_client::{
    types::{
        asset::NewAsset,
        catalog::DatasetRequest,
        contract_definition::NewContractDefinition,
        contract_negotiation::{ContractNegotiationState, ContractRequest},
        data_address::DataAddress,
        participants::{NewParticipantContext, ParticipantContextConfig},
        policy::{Action, NewPolicyDefinition, Permission, Policy, PolicyKind, Target},
        query::Criterion,
        transfer_process::{TransferProcessState, TransferRequest},
        Protocol,
    },
    Auth, EdcConnectorApiVersion, EdcConnectorClient, OAuth2Config, EDC_NAMESPACE,
};
use tokio::time::sleep;
use uuid::Uuid;

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const PROVIDER_ID: &str = "provider";

pub const CONSUMER_PROTOCOL: &str = "http://provider-connector:9194/protocol";
pub const CONSUMER_ID: &str = "consumer";

#[derive(Builder, Clone)]
pub struct ClientParams {
    pub management_url: String,
    #[builder(default = EdcConnectorApiVersion::V3)]
    pub version: EdcConnectorApiVersion,
    #[builder(into)]
    pub participant_context: Option<String>,
    pub auth: Auth,
    #[builder(into)]
    pub protocol_address: String,
    #[builder(into)]
    pub protocol_id: String,
    #[builder(default)]
    pub protocol: Protocol,
}

pub fn provider_v3() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:29193/management".to_string())
        .version(EdcConnectorApiVersion::V3)
        .auth(Auth::ApiToken("123456".to_string()))
        .protocol_address(PROVIDER_PROTOCOL)
        .protocol_id(PROVIDER_ID)
        .build()
}

pub fn consumer_v3() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:19193/management".to_string())
        .version(EdcConnectorApiVersion::V3)
        .auth(Auth::ApiToken("123456".to_string()))
        .protocol_address(CONSUMER_PROTOCOL)
        .protocol_id(CONSUMER_ID)
        .build()
}

pub fn provider_v4() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:29193/management".to_string())
        .version(EdcConnectorApiVersion::V4)
        .auth(Auth::ApiToken("123456".to_string()))
        .protocol_address(PROVIDER_PROTOCOL)
        .protocol_id(PROVIDER_ID)
        .build()
}

pub fn provider_v4_2025() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:29193/management".to_string())
        .version(EdcConnectorApiVersion::V4)
        .auth(Auth::ApiToken("123456".to_string()))
        .protocol_address("http://provider-connector:9194/protocol/2025-1")
        .protocol_id(PROVIDER_ID)
        .build()
}

pub fn consumer_v4() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:19193/management".to_string())
        .version(EdcConnectorApiVersion::V4)
        .auth(Auth::ApiToken("123456".to_string()))
        .protocol_address(CONSUMER_PROTOCOL)
        .protocol_id(CONSUMER_ID)
        .build()
}

pub fn consumer_virtual_edc() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:39193/api/mgmt".to_string())
        .version(EdcConnectorApiVersion::V4)
        .participant_context("consumer")
        .auth(
            Auth::oauth(
                OAuth2Config::builder()
                    .client_id("consumer")
                    .client_secret("consumer-secret")
                    .token_url("http://localhost:8080/realms/edcv/protocol/openid-connect/token")
                    .build(),
            )
            .unwrap(),
        )
        .protocol_address("http://virtual-connector:8282/api/protocol/consumer/2025-1")
        .protocol_id(CONSUMER_ID)
        .protocol(Protocol::new("dataspace-protocol-http:2025-1"))
        .build()
}

pub fn provider_virtual_edc() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:39193/api/mgmt".to_string())
        .version(EdcConnectorApiVersion::V4)
        .participant_context("provider")
        .auth(
            Auth::oauth(
                OAuth2Config::builder()
                    .client_id("provider")
                    .client_secret("provider-secret")
                    .token_url("http://localhost:8080/realms/edcv/protocol/openid-connect/token")
                    .build(),
            )
            .unwrap(),
        )
        .protocol_address("http://virtual-connector:8282/api/protocol/provider/2025-1")
        .protocol_id(PROVIDER_ID)
        .protocol(Protocol::new("dataspace-protocol-http:2025-1"))
        .build()
}

pub fn setup_provider_client_with_auth(auth: Auth) -> EdcConnectorClient {
    EdcConnectorClient::builder()
        .management_url("http://localhost:29193/management")
        .with_auth(auth)
        .build()
        .unwrap()
}

pub fn setup_client(params: ClientParams) -> EdcConnectorClient {
    if let Some(participant_context) = params.participant_context.clone() {
        let auth = OAuth2Config::builder()
            .client_id("provisioner")
            .client_secret("provisioner-secret")
            .token_url("http://localhost:8080/realms/edcv/protocol/openid-connect/token")
            .build();

        let client = EdcConnectorClient::builder()
            .management_url(&params.management_url)
            .with_auth(Auth::oauth(auth).unwrap())
            .version(EdcConnectorApiVersion::V4)
            .participant_context(participant_context.clone())
            .build()
            .unwrap();

        thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let _ = client
                        .participants()
                        .create(
                            &NewParticipantContext::builder()
                                .id(&participant_context)
                                .identity(&participant_context)
                                .build(),
                        )
                        .await;

                    let mut entries = HashMap::new();

                    entries.insert(
                        "edc.participant.id".to_string(),
                        participant_context.clone(),
                    );

                    let _ = client
                        .participant_configs()
                        .save(
                            &participant_context,
                            &ParticipantContextConfig::builder().entries(entries).build(),
                        )
                        .await
                        .unwrap();
                });
        });
    }

    EdcConnectorClient::builder()
        .management_url(&params.management_url)
        .with_auth(Auth::api_token("123456"))
        .version(params.version)
        .with_auth(params.auth)
        .maybe_participant_context(params.participant_context)
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
    consumer_cfg: &ClientParams,
    provider: &EdcConnectorClient,
    provider_cfg: &ClientParams,
) -> (String, String) {
    let (asset_id, _, _) = seed(provider).await;

    let dataset_request = DatasetRequest::builder()
        .counter_party_address(&provider_cfg.protocol_address)
        .counter_party_id(&provider_cfg.protocol_id)
        .protocol(consumer_cfg.protocol.clone())
        .id(&asset_id)
        .build();

    let dataset = consumer
        .catalogue()
        .dataset(&dataset_request)
        .await
        .unwrap();

    let offer_id = dataset.offers()[0].id().unwrap();

    let request = ContractRequest::builder()
        .counter_party_address(&provider_cfg.protocol_address)
        .counter_party_id(&provider_cfg.protocol_id)
        .protocol(consumer_cfg.protocol.clone())
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
    consumer_cfg: &ClientParams,
    provider: &EdcConnectorClient,
    provider_cfg: &ClientParams,
) -> (String, String, String) {
    let (contract_negotiation_id, asset_id) =
        seed_contract_negotiation(consumer, consumer_cfg, provider, provider_cfg).await;

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
    consumer_cfg: &ClientParams,
    provider: &EdcConnectorClient,
    provider_cfg: &ClientParams,
) -> (String, String, String, String) {
    let (contract_negotiation_id, asset_id) =
        seed_contract_negotiation(consumer, consumer_cfg, provider, provider_cfg).await;

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

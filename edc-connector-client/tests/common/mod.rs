#![allow(dead_code)]

use std::{collections::HashMap, future::Future, thread, time::Duration};

use bon::Builder;
use edc_connector_client::types::dataspace_profile::{DataspaceProfile, DataspaceProtocol};
use edc_connector_client::types::ExtraTokenFields;
use edc_connector_client::{
    types::{
        asset::NewAsset,
        catalog::DatasetRequest,
        contract_definition::NewContractDefinition,
        contract_negotiation::{ContractNegotiationState, ContractRequest},
        data_address::DataAddress,
        dataplane::DataPlaneRegistrationMessage,
        participants::{NewParticipantContext, ParticipantContextConfig},
        policy::{Action, NewPolicyDefinition, Permission, Policy, PolicyKind, Target},
        query::Criterion,
        transfer_process::{TransferProcessState, TransferRequest},
        Protocol,
    },
    Auth, EdcConnectorApiVersion, EdcConnectorClient, OAuth2Config, EDC_NAMESPACE,
};
use serde::Deserialize;
use tokio::time::sleep;
use uuid::Uuid;

pub const PROVIDER_PROTOCOL: &str = "http://provider-connector:9194/protocol/2025-1";
pub const PROVIDER_ID: &str = "provider";

pub const CONSUMER_PROTOCOL: &str = "http://provider-connector:9194/protocol/2025-1";
pub const CONSUMER_ID: &str = "consumer";

#[derive(Deserialize, Debug, Clone)]
pub struct CatalogExtraFields {}

impl ExtraTokenFields for CatalogExtraFields {}

#[derive(Builder, Clone)]
pub struct ClientParams {
    pub management_url: String,
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

pub fn provider() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:29193/management".to_string())
        .auth(Auth::ApiToken("123456".to_string()))
        .protocol_address(PROVIDER_PROTOCOL)
        .protocol_id(PROVIDER_ID)
        .build()
}

pub fn consumer() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:19193/management".to_string())
        .auth(Auth::ApiToken("123456".to_string()))
        .protocol_address(CONSUMER_PROTOCOL)
        .protocol_id(CONSUMER_ID)
        .build()
}

#[allow(clippy::unwrap_used)]
pub fn consumer_virtual_edc() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:39193/api/management".to_string())
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
        .protocol_address(
            "http://virtual-connector:8282/api/protocol/consumer/http-dsp-profile-2025-1",
        )
        .protocol_id(CONSUMER_ID)
        .protocol(Protocol::new("http-dsp-profile-2025-1"))
        .build()
}

#[allow(clippy::unwrap_used)]
pub fn provider_virtual_edc() -> ClientParams {
    ClientParams::builder()
        .management_url("http://localhost:39193/api/management".to_string())
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
        .protocol_address(
            "http://virtual-connector:8282/api/protocol/provider/http-dsp-profile-2025-1",
        )
        .protocol_id(PROVIDER_ID)
        .protocol(Protocol::new("http-dsp-profile-2025-1"))
        .build()
}

#[allow(clippy::unwrap_used)]
pub fn setup_provider_client_with_auth(auth: Auth) -> EdcConnectorClient {
    EdcConnectorClient::builder()
        .management_url("http://localhost:29193/management")
        .with_auth(auth)
        .build()
        .unwrap()
}

/// Client credentials of the virtual connector's admin, whose token carries
/// the `management-api:admin` scope required by the global v5 resources.
#[allow(clippy::unwrap_used)]
pub fn admin_auth() -> Auth {
    Auth::oauth(
        OAuth2Config::builder()
            .client_id("admin")
            .client_secret("edc-v-admin-secret")
            .token_url("http://localhost:8080/realms/edcv/protocol/openid-connect/token")
            .scopes(vec!["management-api:admin".to_string()])
            .build(),
    )
    .unwrap()
}

/// Same provisioning as [`setup_client`], but the returned client
/// authenticates as the admin so it can call the global (admin scoped) APIs.
#[allow(clippy::unwrap_used)]
pub fn setup_admin_client(
    params: ClientParams,
    version: EdcConnectorApiVersion,
) -> EdcConnectorClient {
    let _ = setup_client(params.clone(), version);

    EdcConnectorClient::builder()
        .management_url(&params.management_url)
        .with_auth(admin_auth())
        .maybe_participant_context(params.participant_context)
        .build()
        .unwrap()
}

#[allow(clippy::unwrap_used)]
pub fn setup_client(params: ClientParams, version: EdcConnectorApiVersion) -> EdcConnectorClient {
    if let Some(participant_context) = params.participant_context.clone() {
        let client = EdcConnectorClient::builder()
            .management_url(&params.management_url)
            .with_auth(admin_auth())
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
                        .participants(version)
                        .create(
                            &NewParticipantContext::builder()
                                .id(&participant_context)
                                .identity(&participant_context)
                                .build(),
                        )
                        .await;

                    register_dataplane_with_transfer_type(
                        &client,
                        version,
                        &params.protocol_id,
                        "HttpData-PULL",
                    )
                    .await;

                    let mut entries = HashMap::new();

                    entries.insert(
                        "edc.participant.id".to_string(),
                        participant_context.clone(),
                    );

                    client
                        .participant_configs(version)
                        .save(
                            &participant_context,
                            &ParticipantContextConfig::builder().entries(entries).build(),
                        )
                        .await
                        .unwrap();

                    let _ = client
                        .dataspace_profiles(version)
                        .create(
                            &DataspaceProfile::builder()
                                .name("http-dsp-profile-2025-1")
                                .protocol(
                                    DataspaceProtocol::builder()
                                        .binding("HTTPS")
                                        .version("2025-1")
                                        .namespace("https://w3id.org/dspace/2025/1/")
                                        .path("/http-dsp-profile-2025-1")
                                        .build(),
                                )
                                .json_ld_context_url(
                                    "https://w3id.org/dspace/2025/1/context.jsonld",
                                )
                                .build(),
                        )
                        .await;

                    client
                        .participants(version)
                        .associate_profiles(&participant_context, &["http-dsp-profile-2025-1"])
                        .await
                        .unwrap();
                });
        })
        .join()
        .unwrap();
    } else {
        let client = EdcConnectorClient::builder()
            .management_url(&params.management_url)
            .with_auth(params.auth.clone())
            .build()
            .unwrap();
        thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    register_dataplane_with_transfer_type(
                        &client,
                        version,
                        &params.protocol_id,
                        "HttpData-PULL",
                    )
                    .await;
                });
        })
        .join()
        .unwrap();
    }

    EdcConnectorClient::builder()
        .management_url(&params.management_url)
        .with_auth(params.auth)
        .maybe_participant_context(params.participant_context)
        .build()
        .unwrap()
}

/// Address of the `signaling-mock` data plane as seen *from the connectors*,
/// i.e. its compose service alias. The image sets `MOCK_PORT=8080`, which takes
/// precedence over `port` in `testing/conf/dataplane.config/mock.toml`. No host
/// port is published, so tests can only hand this address over, not call it.
pub const DATAPLANE_ENDPOINT: &str = "http://dataplane:8080/api/v1/dataflows";

/// A transfer type unique to the calling test, so that a data plane registered
/// for it can never be selected for the real transfers driven by the other
/// tests against the same connector.
pub fn new_transfer_type() -> String {
    format!("MockData-{}-PULL", Uuid::new_v4())
}

pub fn new_dataplane_registration(id: &str, transfer_type: &str) -> DataPlaneRegistrationMessage {
    DataPlaneRegistrationMessage::builder()
        .dataplane_id(id)
        .endpoint(DATAPLANE_ENDPOINT)
        .transfer_type(transfer_type)
        .build()
}

/// Registers the mock data plane under a fresh id and transfer type, returning
/// both so the test can assert on them and unregister afterwards.
#[allow(clippy::unwrap_used)]
pub async fn register_dataplane(
    client: &EdcConnectorClient,
    version: EdcConnectorApiVersion,
) -> (String, String) {
    let transfer_type = new_transfer_type();
    let id = Uuid::new_v4().to_string();

    register_dataplane_with_transfer_type(client, version, &id, &transfer_type).await;
    (id, transfer_type)
}

/// Registers the mock data plane under the given id and transfer type.
///
/// Every test's setup registers the same id concurrently, and the connectors
/// occasionally reject one of the racing upserts, so the call is retried.
#[allow(clippy::unwrap_used)]
pub async fn register_dataplane_with_transfer_type(
    client: &EdcConnectorClient,
    version: EdcConnectorApiVersion,
    id: &str,
    transfer_type: &str,
) {
    let registration = new_dataplane_registration(id, transfer_type);
    let mut attempts = 0;
    loop {
        attempts += 1;
        match client.data_planes(version).register(&registration).await {
            Ok(()) => break,
            Err(err) if attempts < 5 => {
                eprintln!("data plane registration attempt {attempts} failed: {err}");
                sleep(Duration::from_millis(200)).await;
            }
            Err(err) => panic!("data plane registration failed: {err}"),
        }
    }
}

#[allow(clippy::unwrap_used)]
pub async fn unregister_dataplane(
    client: &EdcConnectorClient,
    version: EdcConnectorApiVersion,
    id: &str,
) {
    client.data_planes(version).delete(id).await.unwrap();
}

#[allow(clippy::unwrap_used)]
pub async fn seed(
    client: &EdcConnectorClient,
    version: EdcConnectorApiVersion,
) -> (String, String, String) {
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

    let asset_response = client.assets(version).create(&asset).await.unwrap();

    let policy_definition = NewPolicyDefinition::builder()
        .id(Uuid::new_v4().to_string().as_str())
        .policy(
            Policy::builder()
                .permission(Permission::builder().action(Action::simple("use")).build())
                .build(),
        )
        .build();

    let policy_response = client
        .policies(version)
        .create(&policy_definition)
        .await
        .unwrap();

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
        .contract_definitions(version)
        .create(&contract_definition)
        .await
        .unwrap();

    (
        asset_response.id().to_string(),
        policy_response.id().to_string(),
        definition_response.id().to_string(),
    )
}

#[allow(clippy::unwrap_used)]
pub async fn seed_contract_negotiation(
    consumer: &EdcConnectorClient,
    consumer_cfg: &ClientParams,
    provider: &EdcConnectorClient,
    provider_cfg: &ClientParams,
    version: EdcConnectorApiVersion,
) -> (String, String) {
    seed_contract_negotiation_with_offer(
        consumer,
        consumer_cfg,
        provider,
        provider_cfg,
        version,
        |offer_id| offer_id.to_string(),
    )
    .await
}

/// Like [`seed_contract_negotiation`], but lets the caller rewrite the offer
/// id taken from the catalog before the negotiation is initiated.
#[allow(clippy::unwrap_used)]
pub async fn seed_contract_negotiation_with_offer(
    consumer: &EdcConnectorClient,
    consumer_cfg: &ClientParams,
    provider: &EdcConnectorClient,
    provider_cfg: &ClientParams,
    version: EdcConnectorApiVersion,
    offer_id: impl FnOnce(&str) -> String,
) -> (String, String) {
    let (asset_id, _, _) = seed(provider, version).await;

    let dataset_request = DatasetRequest::builder()
        .counter_party_address(&provider_cfg.protocol_address)
        .counter_party_id(&provider_cfg.protocol_id)
        .protocol(consumer_cfg.protocol.clone())
        .id(&asset_id)
        .build();

    let dataset = consumer
        .catalogue(version)
        .dataset::<CatalogExtraFields>(&dataset_request)
        .await
        .unwrap();

    let offer_id = offer_id(dataset.offers()[0].id().unwrap());

    let request = ContractRequest::builder()
        .counter_party_address(&provider_cfg.protocol_address)
        .counter_party_id(&provider_cfg.protocol_id)
        .protocol(consumer_cfg.protocol.clone())
        .policy(
            Policy::builder()
                .id(&offer_id)
                .kind(PolicyKind::Offer)
                .assigner(PROVIDER_ID)
                .target(Target::simple(&asset_id))
                .permission(Permission::builder().action(Action::simple("use")).build())
                .build(),
        )
        .build();

    let response = consumer
        .contract_negotiations(version)
        .initiate(&request)
        .await
        .unwrap();

    (response.id().to_string(), asset_id)
}

#[allow(clippy::unwrap_used)]
pub async fn seed_contract_agreement(
    consumer: &EdcConnectorClient,
    consumer_cfg: &ClientParams,
    provider: &EdcConnectorClient,
    provider_cfg: &ClientParams,
    version: EdcConnectorApiVersion,
) -> (String, String, String) {
    let (contract_negotiation_id, asset_id) =
        seed_contract_negotiation(consumer, consumer_cfg, provider, provider_cfg, version).await;

    wait_for_negotiation_state(
        consumer,
        &contract_negotiation_id,
        ContractNegotiationState::Finalized,
        version,
    )
    .await;

    let agreement_id = consumer
        .contract_negotiations(version)
        .get(&contract_negotiation_id)
        .await
        .map(|cn| cn.contract_agreement_id().cloned())
        .unwrap()
        .unwrap();

    let contract_agreement = consumer
        .contract_agreements(version)
        .get(&agreement_id)
        .await
        .unwrap();

    (
        contract_agreement.id().to_string(),
        contract_negotiation_id,
        asset_id,
    )
}

#[allow(clippy::unwrap_used)]
pub async fn seed_transfer_process(
    consumer: &EdcConnectorClient,
    consumer_cfg: &ClientParams,
    provider: &EdcConnectorClient,
    provider_cfg: &ClientParams,
    version: EdcConnectorApiVersion,
) -> (String, String, String, String) {
    let (contract_negotiation_id, asset_id) =
        seed_contract_negotiation(consumer, consumer_cfg, provider, provider_cfg, version).await;

    wait_for_negotiation_state(
        consumer,
        &contract_negotiation_id,
        ContractNegotiationState::Finalized,
        version,
    )
    .await;

    let agreement_id = consumer
        .contract_negotiations(version)
        .get(&contract_negotiation_id)
        .await
        .map(|cn| cn.contract_agreement_id().cloned())
        .unwrap()
        .unwrap();

    let contract_agreement = consumer
        .contract_agreements(version)
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
        .transfer_processes(version)
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

#[allow(clippy::unwrap_used)]
pub async fn wait_for_negotiation_state(
    client: &EdcConnectorClient,
    id: &str,
    state: ContractNegotiationState,
    version: EdcConnectorApiVersion,
) {
    wait_for(|| {
        let i_state = state.clone();
        async {
            client
                .contract_negotiations(version)
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

#[allow(clippy::unwrap_used)]
pub async fn wait_for_transfer_state(
    client: &EdcConnectorClient,
    id: &str,
    state: TransferProcessState,
    version: EdcConnectorApiVersion,
) {
    wait_for(|| {
        let i_state = state.clone();
        async {
            client
                .transfer_processes(version)
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

#[allow(clippy::unwrap_used)]
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

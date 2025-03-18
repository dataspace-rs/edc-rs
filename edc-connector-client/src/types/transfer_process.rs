use crate::ConversionError;
use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use super::{
    callback_address::CallbackAddress,
    data_address::DataAddress,
    properties::{FromValue, Properties},
    Protocol,
};

#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    #[builder(field)]
    callback_addresses: Vec<CallbackAddress>,
    #[builder(default)]
    #[builder(into)]
    protocol: Protocol,
    #[builder(into)]
    counter_party_address: String,
    #[builder(into)]
    contract_id: String,
    #[builder(into)]
    transfer_type: String,
    destination: Option<DataAddress>,
}

impl<S: transfer_request_builder::State> TransferRequestBuilder<S> {
    pub fn callback_address(mut self, callback_address: CallbackAddress) -> Self {
        self.callback_addresses.push(callback_address);
        self
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferProcess {
    #[serde(rename = "@id")]
    id: String,
    #[serde(default)]
    private_properties: Properties,
    state: TransferProcessState,
    state_timestamp: i64,
    asset_id: String,
    contract_id: String,
    correlation_id: Option<String>,
    data_destination: Option<DataAddress>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    callback_addresses: Vec<CallbackAddress>,
    transfer_type: String,
    #[serde(rename = "type")]
    kind: TransferProcessKind,
}

impl TransferProcess {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> &TransferProcessState {
        &self.state
    }

    pub fn private_property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.private_properties.get(property)
    }

    pub fn private_properties(&self) -> &Properties {
        &self.private_properties
    }

    pub fn kind(&self) -> &TransferProcessKind {
        &self.kind
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn contract_id(&self) -> &str {
        &self.contract_id
    }

    pub fn correlation_id(&self) -> Option<&String> {
        self.correlation_id.as_ref()
    }

    pub fn data_destination(&self) -> Option<&DataAddress> {
        self.data_destination.as_ref()
    }

    pub fn transfer_type(&self) -> &str {
        &self.transfer_type
    }

    pub fn state_timestamp(&self) -> i64 {
        self.state_timestamp
    }

    pub fn callback_addresses(&self) -> &[CallbackAddress] {
        &self.callback_addresses
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferProcessState {
    Initial,
    Provisioning,
    ProvisioningRequested,
    Provisioned,
    Requesting,
    Requested,
    Starting,
    Started,
    Suspending,
    Suspended,
    Resuming,
    Resumed,
    Completing,
    Completed,
    Terminating,
    Terminated,
    Deprovisioning,
    DeprovisioningRequested,
    Deprovisioned,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferProcessKind {
    Consumer,
    Provider,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferState {
    state: TransferProcessState,
}

impl TransferState {
    pub fn state(&self) -> &TransferProcessState {
        &self.state
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateTransfer {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    pub(crate) reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuspendTransfer {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    pub(crate) reason: String,
}

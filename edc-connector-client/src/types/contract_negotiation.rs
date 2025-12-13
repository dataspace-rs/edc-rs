use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::ConversionError;

use super::{
    callback_address::CallbackAddress,
    policy::Policy,
    properties::{FromValue, Properties},
    Protocol,
};

#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ContractRequest {
    #[builder(field)]
    callback_addresses: Vec<CallbackAddress>,
    #[builder(default)]
    protocol: Protocol,
    #[builder(into)]
    counter_party_id: String,
    #[builder(into)]
    counter_party_address: String,
    policy: Policy,
    #[builder(default = "ContractRequest".to_string())]
    #[serde(rename = "@type")]
    ty: String,
}

impl<S: contract_request_builder::State> ContractRequestBuilder<S> {
    pub fn callback_address(mut self, callback_address: CallbackAddress) -> Self {
        self.callback_addresses.push(callback_address);
        self
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractNegotiation {
    #[serde(rename = "@id")]
    id: String,
    #[serde(default)]
    private_properties: Properties,
    state: ContractNegotiationState,
    contract_agreement_id: Option<String>,
    counter_party_id: String,
    counter_party_address: String,
    protocol: String,
    created_at: i64,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    callback_addresses: Vec<CallbackAddress>,
    #[serde(rename = "type")]
    kind: ContractNegotiationKind,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractNegotiationKind {
    Consumer,
    Provider,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractNegotiationState {
    Initial,
    Requesting,
    Requested,
    Offering,
    Offered,
    Accepting,
    Accepted,
    Agreeing,
    Agreed,
    Verifying,
    Verified,
    Finalizing,
    Finalized,
    Terminating,
    Terminated,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NegotiationState {
    state: ContractNegotiationState,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateNegotiation {
    #[serde(rename = "@id")]
    pub(crate) id: String,
    pub(crate) reason: String,
}

impl NegotiationState {
    pub fn state(&self) -> &ContractNegotiationState {
        &self.state
    }
}

impl ContractNegotiation {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> &ContractNegotiationState {
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

    pub fn contract_agreement_id(&self) -> Option<&String> {
        self.contract_agreement_id.as_ref()
    }

    pub fn counter_party_id(&self) -> &str {
        &self.counter_party_id
    }

    pub fn counter_party_address(&self) -> &str {
        &self.counter_party_address
    }

    pub fn kind(&self) -> &ContractNegotiationKind {
        &self.kind
    }

    pub fn created_at(&self) -> i64 {
        self.created_at
    }

    pub fn callback_addresses(&self) -> &[CallbackAddress] {
        &self.callback_addresses
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }
}

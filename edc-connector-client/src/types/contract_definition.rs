use crate::ConversionError;
use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use super::{
    properties::{FromValue, Properties, ToValue},
    query::Criterion,
};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ContractDefinition {
    #[builder(field)]
    #[serde(default)]
    private_properties: Properties,
    #[builder(field)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(default)]
    assets_selector: Vec<Criterion>,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(into)]
    access_policy_id: String,
    #[builder(into)]
    contract_policy_id: String,
}

impl ContractDefinition {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn access_policy_id(&self) -> &str {
        &self.access_policy_id
    }

    pub fn contract_policy_id(&self) -> &str {
        &self.contract_policy_id
    }

    pub fn assets_selector(&self) -> &[Criterion] {
        &self.assets_selector
    }

    pub fn private_property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.private_properties.get(property)
    }
}

impl<S: contract_definition_builder::State> ContractDefinitionBuilder<S> {
    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewContractDefinition {
    #[builder(field)]
    #[serde(default)]
    private_properties: Properties,
    #[builder(field)]
    #[serde(default)]
    assets_selector: Vec<Criterion>,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: Option<String>,
    #[builder(into)]
    access_policy_id: String,
    #[builder(into)]
    contract_policy_id: String,
}

impl<S: new_contract_definition_builder::State> NewContractDefinitionBuilder<S> {
    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }

    pub fn asset_selector(mut self, selector: Criterion) -> Self {
        self.assets_selector.push(selector);
        self
    }
}

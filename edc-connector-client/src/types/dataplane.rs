use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use super::properties::Properties;

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataPlaneInstance {
    #[serde(rename = "@id")]
    id: String,
    url: String,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    allowed_source_types: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    allowed_dest_types: Vec<String>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    allowed_transfer_types: Vec<String>,
    state: DataPlaneInstanceState,
    #[serde(default)]
    properties: Properties,
}

impl DataPlaneInstance {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn allowed_source_types(&self) -> &Vec<String> {
        &self.allowed_source_types
    }

    pub fn allowed_dest_types(&self) -> &Vec<String> {
        &self.allowed_dest_types
    }

    pub fn allowed_transfer_types(&self) -> &Vec<String> {
        &self.allowed_transfer_types
    }

    pub fn state(&self) -> &DataPlaneInstanceState {
        &self.state
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataPlaneInstanceState {
    Available,
    Registered,
    Unavailable,
    Unregistered,
    #[serde(untagged)]
    Other(String),
}

use crate::types::catalog::common_properties::CommonProperties;
use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[serde_as]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Service {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "type", alias = "@type")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    types: Vec<String>,
    #[serde(flatten)]
    common_properties: CommonProperties,
    #[serde(rename = "endpointURL", alias = "dcat:endpointURL", default)]
    endpoint_url: Option<String>,
    #[serde(
        rename = "endpointDescription",
        alias = "dcat:endpointDescription",
        default
    )]
    endpoint_description: Option<String>,
}

impl Service {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn types(&self) -> &[String] {
        &self.types
    }
    pub fn common_properties(&self) -> &CommonProperties {
        &self.common_properties
    }
    pub fn endpoint_url(&self) -> Option<&str> {
        self.endpoint_url.as_deref()
    }
    pub fn endpoint_description(&self) -> Option<&str> {
        self.endpoint_description.as_deref()
    }
}

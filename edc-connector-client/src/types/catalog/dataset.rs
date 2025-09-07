use crate::types::catalog::common_properties::CommonProperties;
use crate::types::policy::Policy;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Dataset {
    id: String,
    types: Vec<String>,
    dcterms_types: Vec<String>,
    policies: Vec<Policy>,
    common_properties: CommonProperties,
}

impl Dataset {
    pub fn new(
        id: String,
        types: Vec<String>,
        dcterms_types: Vec<String>,
        policies: Vec<Policy>,
        common_properties: CommonProperties,
    ) -> Self {
        Self {
            id,
            types,
            dcterms_types,
            policies,
            common_properties,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn types(&self) -> &Vec<String> {
        &self.types
    }

    pub fn dcterms_types(&self) -> &Vec<String> {
        &self.dcterms_types
    }

    pub fn policies(&self) -> &Vec<Policy> {
        &self.policies
    }

    pub fn common_properties(&self) -> &CommonProperties {
        &self.common_properties
    }
}

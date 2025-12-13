use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewSecret {
    #[builder(into)]
    #[serde(rename = "@id")]
    id: Option<String>,
    #[builder(into)]
    value: String,
    #[builder(default = "Secret".to_string())]
    #[serde(rename = "@type")]
    ty: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(into)]
    value: String,
    #[builder(default = "Secret".to_string())]
    #[serde(rename = "@type")]
    ty: String,
}

impl Secret {
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

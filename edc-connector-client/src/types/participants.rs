use std::collections::HashMap;

use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewParticipantContext {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    id: Option<String>,
    #[builder(into)]
    identity: String,
    #[builder(default = "ParticipantContext".to_string())]
    #[serde(rename = "@type")]
    ty: String,
}

#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantContextConfig {
    #[builder(default = "ParticipantContextConfig".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    entries: HashMap<String, String>,
    #[builder(default)]
    private_entries: HashMap<String, String>,
}

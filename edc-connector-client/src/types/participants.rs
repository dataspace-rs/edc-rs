//! Participant contexts (EDC-V tenants), their configuration and the
//! dataspace profiles they are associated with.

use std::collections::HashMap;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::ConversionError;

use super::properties::{FromValue, Properties, PropertyValue, ToValue};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParticipantContextState {
    #[default]
    Created,
    Activated,
    Deactivated,
    #[serde(untagged)]
    Other(String),
}

/// A participant context to create.
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewParticipantContext {
    #[builder(field)]
    #[serde(skip_serializing_if = "Properties::is_empty")]
    properties: Properties,
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

impl<S: new_participant_context_builder::State> NewParticipantContextBuilder<S> {
    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }
}

impl NewParticipantContext {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }
}

/// A stored participant context (`@type: ParticipantContext`).
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantContext {
    #[builder(field)]
    #[serde(default)]
    properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(default = "ParticipantContext".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[builder(into)]
    identity: String,
    #[builder(default)]
    #[serde(default)]
    state: ParticipantContextState,
}

impl<S: participant_context_builder::State> ParticipantContextBuilder<S> {
    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }
}

impl ParticipantContext {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn state(&self) -> &ParticipantContextState {
        &self.state
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.properties.get(property)
    }

    pub fn raw_property(&self, property: &str) -> Option<&PropertyValue> {
        self.properties.get_raw(property)
    }
}

/// The full configuration of a participant context. Saving it replaces every
/// entry; use [`ParticipantContextConfigPatch`] to change single keys.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantContextConfig {
    #[builder(default = "ParticipantContextConfig".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[serde(default)]
    entries: HashMap<String, String>,
    #[builder(default)]
    #[serde(default)]
    private_entries: HashMap<String, String>,
}

impl ParticipantContextConfig {
    pub fn entries(&self) -> &HashMap<String, String> {
        &self.entries
    }

    pub fn private_entries(&self) -> &HashMap<String, String> {
        &self.private_entries
    }
}

/// A partial update of a participant context configuration: given keys are
/// set, keys mapped to `None` are removed (sent as JSON `null`), everything
/// else is left untouched.
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantContextConfigPatch {
    #[builder(field)]
    entries: HashMap<String, Option<String>>,
    #[builder(field)]
    private_entries: HashMap<String, Option<String>>,
    #[builder(default = "ParticipantContextConfig".to_string())]
    #[serde(rename = "@type")]
    ty: String,
}

impl<S: participant_context_config_patch_builder::State> ParticipantContextConfigPatchBuilder<S> {
    pub fn entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.entries.insert(key.into(), Some(value.into()));
        self
    }

    pub fn remove_entry(mut self, key: impl Into<String>) -> Self {
        self.entries.insert(key.into(), None);
        self
    }

    pub fn private_entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.private_entries.insert(key.into(), Some(value.into()));
        self
    }

    pub fn remove_private_entry(mut self, key: impl Into<String>) -> Self {
        self.private_entries.insert(key.into(), None);
        self
    }
}

impl ParticipantContextConfigPatch {
    pub fn entries(&self) -> &HashMap<String, Option<String>> {
        &self.entries
    }

    pub fn private_entries(&self) -> &HashMap<String, Option<String>> {
        &self.private_entries
    }
}

/// Body of `PUT /participants/{id}/profiles`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AssociateDataspaceProfile {
    #[serde(rename = "@type")]
    ty: String,
    profiles: Vec<String>,
}

impl AssociateDataspaceProfile {
    pub(crate) fn new(profiles: Vec<String>) -> Self {
        Self {
            ty: "AssociateDataspaceProfile".to_string(),
            profiles,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::{
        AssociateDataspaceProfile, NewParticipantContext, ParticipantContext,
        ParticipantContextConfigPatch, ParticipantContextState,
    };

    #[test]
    fn should_omit_empty_properties_on_creation() {
        let ctx = NewParticipantContext::builder()
            .id("p1")
            .identity("p1")
            .build();

        assert_eq!(
            serde_json::to_value(&ctx).unwrap(),
            json!({"@id": "p1", "@type": "ParticipantContext", "identity": "p1"})
        );
    }

    #[test]
    fn should_serialize_properties_on_creation() {
        let ctx = NewParticipantContext::builder()
            .identity("p1")
            .property("region", "eu")
            .build();

        assert_eq!(
            serde_json::to_value(&ctx).unwrap(),
            json!({"@type": "ParticipantContext", "identity": "p1", "properties": {"region": "eu"}})
        );
    }

    #[test]
    fn should_deserialize_a_participant_context() {
        let ctx: ParticipantContext = serde_json::from_value(json!({
            "@id": "p1", "@type": "ParticipantContext", "identity": "p1",
            "properties": {"region": "eu"}, "state": "ACTIVATED"
        }))
        .unwrap();
        assert_eq!(ctx.state(), &ParticipantContextState::Activated);
        assert_eq!(ctx.property::<String>("region").unwrap().unwrap(), "eu");

        let ctx: ParticipantContext = serde_json::from_value(json!({
            "@id": "p1", "@type": "ParticipantContext", "identity": "p1", "state": "FROZEN"
        }))
        .unwrap();
        assert_eq!(
            ctx.state(),
            &ParticipantContextState::Other("FROZEN".to_string())
        );
    }

    #[test]
    fn should_serialize_removed_keys_as_null() {
        let patch = ParticipantContextConfigPatch::builder()
            .entry("a", "1")
            .remove_entry("b")
            .remove_private_entry("secret")
            .build();

        assert_eq!(
            serde_json::to_value(&patch).unwrap(),
            json!({
                "@type": "ParticipantContextConfig",
                "entries": {"a": "1", "b": null},
                "privateEntries": {"secret": null}
            })
        );
    }

    #[test]
    fn should_serialize_profile_association() {
        let body = AssociateDataspaceProfile::new(vec!["p1".to_string()]);
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({"@type": "AssociateDataspaceProfile", "profiles": ["p1"]})
        );
    }
}

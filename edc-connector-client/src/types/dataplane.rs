use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::{error::BuilderError, ConversionError};

use super::properties::{FromValue, Properties, ToValue};

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataPlaneInstance {
    #[serde(rename = "@id")]
    id: String,
    url: String,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    allowed_source_types: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    allowed_dest_types: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    allowed_transfer_types: Vec<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    destination_provision_types: Vec<String>,
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

    pub fn destination_provision_types(&self) -> &Vec<String> {
        &self.destination_provision_types
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

/// A request to register (or update) a data plane on a connector.
///
/// Registration is an upsert: registering again with the same
/// [`dataplane_id`](DataPlaneRegistrationMessage::dataplane_id) replaces the
/// stored instance, so transfer types, labels and the authorization profile
/// have to be repeated on every call or they are cleared.
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DataPlaneRegistrationMessage {
    #[builder(field)]
    transfer_types: Vec<String>,
    #[builder(field)]
    labels: Vec<String>,
    #[builder(into)]
    dataplane_id: String,
    #[builder(into)]
    endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization: Option<DataPlaneAuthorization>,
}

impl<S: data_plane_registration_message_builder::State> DataPlaneRegistrationMessageBuilder<S> {
    pub fn transfer_type(mut self, transfer_type: impl Into<String>) -> Self {
        self.transfer_types.push(transfer_type.into());
        self
    }

    pub fn transfer_types<I, T>(mut self, transfer_types: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.transfer_types
            .extend(transfer_types.into_iter().map(Into::into));
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    pub fn labels<I, T>(mut self, labels: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.labels.extend(labels.into_iter().map(Into::into));
        self
    }
}

impl DataPlaneRegistrationMessage {
    pub fn dataplane_id(&self) -> &str {
        &self.dataplane_id
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn transfer_types(&self) -> &[String] {
        &self.transfer_types
    }

    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    pub fn authorization(&self) -> Option<&DataPlaneAuthorization> {
        self.authorization.as_ref()
    }
}

/// The authorization profile of a data plane.
///
/// A free form set of properties of which `type` is mandatory: it selects the
/// authorization implementation on the connector side (e.g.
/// `oauth2_client_credentials`), while the whole set of properties is handed
/// over to it as its configuration.
#[derive(Debug, Serialize, Clone)]
#[serde(transparent)]
pub struct DataPlaneAuthorization(Properties);

impl DataPlaneAuthorization {
    pub fn builder() -> DataPlaneAuthorizationBuilder {
        DataPlaneAuthorizationBuilder::default()
    }

    pub fn kind(&self) -> Option<&str> {
        self.0.get_raw("type").and_then(|value| value.0.as_str())
    }

    pub fn property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.0.get(property)
    }
}

#[derive(Default)]
pub struct DataPlaneAuthorizationBuilder(Properties);

impl DataPlaneAuthorizationBuilder {
    pub fn kind(mut self, kind: &str) -> Self {
        self.0.set("type", kind);

        self
    }

    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.0.set(property, value);
        self
    }

    pub fn build(self) -> Result<DataPlaneAuthorization, BuilderError> {
        if self.0.contains("type") {
            Ok(DataPlaneAuthorization(self.0))
        } else {
            Err(BuilderError::missing_property("type"))
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::{DataPlaneAuthorization, DataPlaneRegistrationMessage};

    #[test]
    fn should_serialize_a_registration_message() {
        let registration = DataPlaneRegistrationMessage::builder()
            .dataplane_id("dp-id")
            .endpoint("http://dataplane/endpoint")
            .transfer_type("HttpData-PUSH")
            .label("label")
            .build();

        assert_eq!(
            serde_json::to_value(&registration).unwrap(),
            json!({
                "dataplaneId": "dp-id",
                "endpoint": "http://dataplane/endpoint",
                "transferTypes": ["HttpData-PUSH"],
                "labels": ["label"]
            })
        );
    }

    #[test]
    fn should_serialize_a_registration_message_with_an_authorization_profile() {
        let registration = DataPlaneRegistrationMessage::builder()
            .dataplane_id("dp-id")
            .endpoint("http://dataplane/endpoint")
            .transfer_types(["HttpData-PUSH", "HttpData-PULL"])
            .authorization(
                DataPlaneAuthorization::builder()
                    .kind("oauth2")
                    .property("tokenUrl", "http://token-url")
                    .build()
                    .unwrap(),
            )
            .build();

        assert_eq!(
            serde_json::to_value(&registration).unwrap(),
            json!({
                "dataplaneId": "dp-id",
                "endpoint": "http://dataplane/endpoint",
                "transferTypes": ["HttpData-PUSH", "HttpData-PULL"],
                "labels": [],
                "authorization": {
                    "type": "oauth2",
                    "tokenUrl": "http://token-url"
                }
            })
        );
    }

    #[test]
    fn should_fail_to_build_an_authorization_without_a_kind() {
        assert!(DataPlaneAuthorization::builder()
            .property("tokenUrl", "http://token-url")
            .build()
            .is_err());
    }
}

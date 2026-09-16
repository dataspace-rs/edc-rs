//! Counter party discovery: matches the DSP versions advertised by a counter
//! party against the local dataspace profiles.

use bon::Builder;
use serde::{Deserialize, Serialize};

/// Request body of `POST /discover/request`. At least one of
/// `counter_party_id` (a DID, resolved to find the well-known endpoint) or
/// `counter_party_address` (the `/.well-known/dspace-version` URL or its host)
/// must be set; the address takes precedence when both are given.
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryRequest {
    #[builder(default = "DiscoveryRequest".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    counter_party_id: Option<String>,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    counter_party_address: Option<String>,
}

impl DiscoveryRequest {
    pub fn counter_party_id(&self) -> Option<&str> {
        self.counter_party_id.as_deref()
    }

    pub fn counter_party_address(&self) -> Option<&str> {
        self.counter_party_address.as_deref()
    }
}

/// A protocol version shared by a local dataspace profile and the counter party.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryResponse {
    /// The local dataspace profile that matches the counter party version.
    #[serde(default)]
    profile: String,
    /// The DSP protocol version shared by both sides.
    #[serde(default)]
    version: String,
    /// The protocol binding of the match.
    #[serde(default)]
    binding: String,
    counter_party: DiscoveredCounterParty,
}

impl DiscoveryResponse {
    pub fn profile(&self) -> &str {
        &self.profile
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn binding(&self) -> &str {
        &self.binding
    }

    pub fn counter_party(&self) -> &DiscoveredCounterParty {
        &self.counter_party
    }
}

/// Where the matched version is exposed on the counter party.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveredCounterParty {
    /// Path under the counter party base URL serving the matched DSP version.
    #[serde(default)]
    path: String,
    #[serde(default)]
    data_service_endpoint: Option<String>,
}

impl DiscoveredCounterParty {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn data_service_endpoint(&self) -> Option<&str> {
        self.data_service_endpoint.as_deref()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::{DiscoveryRequest, DiscoveryResponse};

    #[test]
    fn should_omit_unset_operands() {
        let request = DiscoveryRequest::builder()
            .counter_party_address("http://counter-party/.well-known/dspace-version")
            .build();

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            json!({
                "@type": "DiscoveryRequest",
                "counterPartyAddress": "http://counter-party/.well-known/dspace-version"
            })
        );
    }

    #[test]
    fn should_deserialize_a_response() {
        let response: DiscoveryResponse = serde_json::from_value(json!({
            "@type": "DiscoveryResponse",
            "profile": "dsp2025_1",
            "version": "2025-1",
            "binding": "HTTPS",
            "counterParty": {"path": "/2025-1", "dataServiceEndpoint": "http://cp/dsp"}
        }))
        .unwrap();

        assert_eq!(response.profile(), "dsp2025_1");
        assert_eq!(response.counter_party().path(), "/2025-1");
        assert_eq!(
            response.counter_party().data_service_endpoint(),
            Some("http://cp/dsp")
        );
    }
}

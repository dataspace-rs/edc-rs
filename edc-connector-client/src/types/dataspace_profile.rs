//! Dataspace profiles: the protocol version/binding a participant speaks and
//! the issuers it trusts.

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

/// A dataspace profile (`@type: DataspaceProfile`), identified by its `name`.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DataspaceProfile {
    #[builder(field)]
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    json_ld_contexts_url: Vec<String>,
    #[builder(field)]
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    trusted_issuers: Vec<TrustedIssuer>,
    #[builder(default = "DataspaceProfile".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[builder(into)]
    name: String,
    protocol: DataspaceProtocol,
}

impl<S: dataspace_profile_builder::State> DataspaceProfileBuilder<S> {
    pub fn json_ld_context_url(mut self, url: impl Into<String>) -> Self {
        self.json_ld_contexts_url.push(url.into());
        self
    }

    pub fn trusted_issuer(mut self, issuer: TrustedIssuer) -> Self {
        self.trusted_issuers.push(issuer);
        self
    }
}

impl DataspaceProfile {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn protocol(&self) -> &DataspaceProtocol {
        &self.protocol
    }

    pub fn json_ld_contexts_url(&self) -> &[String] {
        &self.json_ld_contexts_url
    }

    pub fn trusted_issuers(&self) -> &[TrustedIssuer] {
        &self.trusted_issuers
    }
}

/// The protocol section of a [`DataspaceProfile`].
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DataspaceProtocol {
    /// DSP version, e.g. `2025-1`.
    #[builder(into)]
    #[serde(default)]
    version: String,
    /// Path under which the protocol endpoints are exposed.
    #[builder(into)]
    #[serde(default)]
    path: String,
    /// Protocol binding, e.g. `HTTPS`.
    #[builder(into)]
    #[serde(default)]
    binding: String,
    /// JSON-LD namespace of the protocol, e.g. `https://w3id.org/dspace/2025/1/`.
    #[builder(into)]
    #[serde(default)]
    namespace: String,
}

impl DataspaceProtocol {
    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn binding(&self) -> &str {
        &self.binding
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }
}

/// An issuer whose credentials of the given types are trusted in a profile.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TrustedIssuer {
    #[builder(field)]
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    supported_types: Vec<String>,
    /// The issuer DID.
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(default = "TrustedIssuer".to_string())]
    #[serde(rename = "@type", default = "default_trusted_issuer_type")]
    ty: String,
}

fn default_trusted_issuer_type() -> String {
    "TrustedIssuer".to_string()
}

impl<S: trusted_issuer_builder::State> TrustedIssuerBuilder<S> {
    pub fn supported_type(mut self, credential_type: impl Into<String>) -> Self {
        self.supported_types.push(credential_type.into());
        self
    }
}

impl TrustedIssuer {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn supported_types(&self) -> &[String] {
        &self.supported_types
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use serde_json::json;

    use super::{DataspaceProfile, DataspaceProtocol, TrustedIssuer};

    #[test]
    fn should_serialize_a_profile() {
        let profile = DataspaceProfile::builder()
            .name("dsp2025_1")
            .protocol(
                DataspaceProtocol::builder()
                    .version("2025-1")
                    .path("/2025-1")
                    .binding("HTTPS")
                    .namespace("https://w3id.org/dspace/2025/1/")
                    .build(),
            )
            .json_ld_context_url("https://w3id.org/dspace/2025/1/context.jsonld")
            .trusted_issuer(
                TrustedIssuer::builder()
                    .id("did:web:issuer")
                    .supported_type("MembershipCredential")
                    .build(),
            )
            .build();

        assert_eq!(
            serde_json::to_value(&profile).unwrap(),
            json!({
                "@type": "DataspaceProfile",
                "name": "dsp2025_1",
                "protocol": {
                    "version": "2025-1",
                    "path": "/2025-1",
                    "binding": "HTTPS",
                    "namespace": "https://w3id.org/dspace/2025/1/"
                },
                "jsonLdContextsUrl": ["https://w3id.org/dspace/2025/1/context.jsonld"],
                "trustedIssuers": [{
                    "@id": "did:web:issuer",
                    "@type": "TrustedIssuer",
                    "supportedTypes": ["MembershipCredential"]
                }]
            })
        );
    }

    #[test]
    fn should_deserialize_single_valued_lists() {
        let profile: DataspaceProfile = serde_json::from_value(json!({
            "@type": "DataspaceProfile",
            "name": "p",
            "protocol": {"version": "2025-1", "path": "/p", "binding": "HTTPS", "namespace": "ns"},
            "jsonLdContextsUrl": "https://ctx",
            "trustedIssuers": {"@id": "did:web:issuer", "supportedTypes": "Cred"}
        }))
        .unwrap();

        assert_eq!(profile.json_ld_contexts_url(), ["https://ctx"]);
        assert_eq!(profile.trusted_issuers()[0].supported_types(), ["Cred"]);
    }
}

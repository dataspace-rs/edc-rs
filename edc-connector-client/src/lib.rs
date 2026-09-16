//! Experimental client for EDC (Eclipse Dataspace Connector)
//!
//! You can use edc-connector-client this lines in your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! edc-connector-client = "<version>"
//! ```
//!
//! Here it is an usage example:
//!
//!
//! ```rust,no_run
//!
//! use edc_connector_client::{EdcConnectorClient, EdcConnectorApiVersion, Auth};
//!
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!     let client = EdcConnectorClient::builder()
//!         .management_url("http://myedc")
//!         .with_auth(Auth::api_token("password"))
//!         .build()?;
//!
//!     let asset = client.assets(EdcConnectorApiVersion::V4).get("1").await?;
//!     println!("Got {:?}", asset);
//!
//!     Ok(())
//! }
//! ```
//!
//! Inside a cluster that authenticates workloads through an RFC 8693 token exchange broker
//! (see the [JAD token exchange spec](https://github.com/eclipse-dataspace-hub/jad/blob/main/docs/token-exchange.md)),
//! the projected ServiceAccount token can be exchanged for a scoped EDC token on every request:
//!
//! ```rust,no_run
//! use edc_connector_client::{Auth, EdcConnectorClient, SubjectToken, TokenExchangeConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let auth = Auth::token_exchange(
//!     TokenExchangeConfig::builder()
//!         .token_exchange_url("http://jwtlet.edc-v.svc.cluster.local:8080/token")
//!         .subject_token(SubjectToken::file("/var/run/secrets/jwtlet/token"))
//!         .resource("<participant-context-id>")
//!         .scopes(vec!["management-api:assets:read".to_string()])
//!         .build(),
//! )?;
//!
//! let client = EdcConnectorClient::builder()
//!     .management_url("http://myedc")
//!     .with_auth(auth)
//!     .participant_context("<participant-context-id>")
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! With the Management API v5 (EDC-V) the participant scoped resources are
//! routed under `/v5/participants/{participant_context}/...`, while the global
//! ones ignore the participant context and need the `management-api:admin`
//! scope: [`EdcConnectorClient::participants`],
//! [`EdcConnectorClient::participant_configs`],
//! [`EdcConnectorClient::dataspace_profiles`],
//! [`EdcConnectorClient::common_expression_language`],
//! [`EdcConnectorClient::dcp_scopes`], [`EdcConnectorClient::cached_documents`]
//! and [`EdcConnectorClient::schema_validators`].

pub mod api;
mod auth;
mod client;
mod error;

pub mod types;
pub use auth::{Auth, OAuth2Config, SubjectToken, TokenExchangeConfig, TokenExchangeError};
pub use client::{EdcConnectorApiVersion, EdcConnectorClient};
pub use error::{
    BuilderError, ConversionError, Error, ManagementApiError, ManagementApiErrorDetail,
    ManagementApiErrorDetailKind,
};

pub const EDC_NAMESPACE: &str = "https://w3id.org/edc/v0.0.1/ns/";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http:2025-1";

pub type EdcResult<T> = Result<T, Error>;

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
//! use edc_connector_client::{EdcConnectorClient, Auth};
//!
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!    let client = EdcConnectorClient::builder()
//!        .management_url("http://myedc")
//!        .with_auth(Auth::api_token("password"))
//!        .build()?;
//!
//!    let asset = client.assets().get("1").await?;
//!    println!("Got {:?}", asset);
//!
//!    Ok(())
//!}
//!

pub mod api;
mod auth;
mod client;
mod error;

pub mod types;
pub use auth::Auth;
pub use client::{EdcConnectorApiVersion, EdcConnectorClient};
pub use error::{
    BuilderError, ConversionError, Error, ManagementApiError, ManagementApiErrorDetail,
    ManagementApiErrorDetailKind,
};

pub const EDC_NAMESPACE: &str = "https://w3id.org/edc/v0.0.1/ns/";
pub const DATASPACE_PROTOCOL: &str = "dataspace-protocol-http";

pub type EdcResult<T> = Result<T, Error>;

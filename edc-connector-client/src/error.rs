use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ManagementApi(ManagementApiError),

    #[error(transparent)]
    Auth(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
#[error("Connector management api returned {status_code}")]
pub struct ManagementApiError {
    pub status_code: StatusCode,
    pub error_detail: ManagementApiErrorDetailKind,
}

#[derive(Debug, Deserialize)]
pub struct ManagementApiErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug)]
pub enum ManagementApiErrorDetailKind {
    Raw(String),
    Parsed(Vec<ManagementApiErrorDetail>),
}

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("Missing mandatory property {0}")]
    MissingProperty(String),
}

impl BuilderError {
    pub fn missing_property(property: &str) -> BuilderError {
        BuilderError::MissingProperty(property.to_string())
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
#[error("Failed to convert")]
pub struct ConversionError {}

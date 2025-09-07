mod common_properties;
mod creator;
mod dataset;
mod dataset_or_catalog;
mod dataset_or_service;
mod service;
mod thumbnail;

use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use super::{query::Query, Protocol};
pub use creator::Creator;
pub use dataset::Dataset;
pub use dataset_or_catalog::DatasetOrCatalog;
pub use dataset_or_service::DatasetOrService;
pub use service::Service;
pub use thumbnail::Thumbnail;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Catalog {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "dataset", alias = "dcat:dataset")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    datasets_or_catalogs: Vec<DatasetOrCatalog>,

    #[serde(rename = "participantId", alias = "dspace:participantId")]
    participant_id: Option<String>,
}

impl Catalog {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn datasets_or_catalogs(&self) -> &[DatasetOrCatalog] {
        &self.datasets_or_catalogs
    }

    pub fn participant_id(&self) -> &Option<String> {
        &self.participant_id
    }

    pub fn datasets(&self) -> Vec<Dataset> {
        self.datasets_or_catalogs
            .iter()
            .flat_map(|dataset_or_catalog| dataset_or_catalog.flatten_datasets())
            .collect()
    }

    pub fn services(&self) -> Vec<Service> {
        self.datasets_or_catalogs
            .iter()
            .flat_map(|dataset_or_catalog| dataset_or_catalog.flatten_services())
            .collect()
    }

    pub fn datasets_and_services(&self) -> Vec<DatasetOrService> {
        self.datasets_or_catalogs
            .iter()
            .flat_map(|dataset_or_catalog| dataset_or_catalog.flatten_datasets_and_services())
            .collect()
    }
}

#[derive(Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CatalogRequest {
    #[builder(into)]
    counter_party_address: String,
    #[builder(into)]
    #[builder(default)]
    protocol: Protocol,
    #[builder(into)]
    counter_party_id: Option<String>,
    query_spec: Query,
}

#[derive(Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DatasetRequest {
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(into)]
    counter_party_address: String,
    #[builder(into)]
    #[builder(default)]
    protocol: Protocol,
}

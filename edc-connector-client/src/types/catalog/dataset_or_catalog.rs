use crate::types::catalog::common_properties::CommonProperties;
use crate::types::catalog::dataset_or_service::DatasetOrService;
use crate::types::catalog::{Dataset, Service};
use crate::types::policy::Policy;
use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[serde_as]
#[derive(Clone, Deserialize, Debug)]
pub struct DatasetOrCatalog {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "type", alias = "@type")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    types: Vec<String>,

    #[serde(rename = "hasPolicy", alias = "odrl:hasPolicy", default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    offers: Vec<Policy>,

    #[serde(rename = "dataset", alias = "dcat:dataset", default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    datasets_or_catalogs: Vec<DatasetOrCatalog>,

    #[serde(rename = "service", alias = "dcat:service", default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    services: Vec<Service>,
    
    #[serde(rename = "dcterms_type", alias = "dct:type", default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    dcterms_types: Vec<String>,

    #[serde(flatten)]
    common_properties: CommonProperties,
}

impl DatasetOrCatalog {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn types(&self) -> &Vec<String> {
        &self.types
    }
    
    pub fn dcterms_types(&self) -> &Vec<String> {
        &self.dcterms_types
    }

    pub fn is_catalog(&self) -> bool {
        self.types.contains(&"dcat:Catalog".to_string())
    }

    pub fn is_dataset(&self) -> bool {
        self.types.contains(&"dcat:Dataset".to_string())
    }

    pub fn common_properties(&self) -> &CommonProperties {
        &self.common_properties
    }

    pub fn offers(&self) -> &[Policy] {
        &self.offers
    }

    pub fn datasets_or_catalogs(&self) -> &[DatasetOrCatalog] {
        &self.datasets_or_catalogs
    }

    pub fn flatten_datasets(&self) -> Vec<Dataset> {
        if self.is_dataset() {
            vec![Dataset::new(
                self.id.clone(),
                self.types.clone(),
                self.dcterms_types.clone(),
                self.offers.clone(),
                self.common_properties().clone(),
            )]
        } else {
            self.datasets_or_catalogs
                .iter()
                .flat_map(|dataset_or_catalog| dataset_or_catalog.flatten_datasets())
                .collect()
        }
    }

    pub fn flatten_services(&self) -> Vec<Service> {
        if self.is_catalog() {
            self.services.clone()
        } else {
            self.datasets_or_catalogs
                .iter()
                .flat_map(|dataset_or_catalog| dataset_or_catalog.flatten_services())
                .filter(|service| service.common_properties().title.is_some())
                .collect()
        }
    }

    pub fn flatten_datasets_and_services(&self) -> Vec<DatasetOrService> {
        if self.is_dataset() {
            vec![DatasetOrService::Dataset(Dataset::new(
                self.id.clone(),
                self.types.clone(),
                self.dcterms_types.clone(),
                self.offers.clone(),
                self.common_properties.clone(),
            ))]
        } else {
            self.services
                .iter()
                .filter(|service| service.common_properties().title.is_some())
                .map(|service| DatasetOrService::Service(service.clone()))
                .chain(
                    self.datasets_or_catalogs
                        .iter()
                        .flat_map(|dataset_or_catalog| {
                            dataset_or_catalog.flatten_datasets_and_services()
                        }),
                )
                .collect::<Vec<DatasetOrService>>()
        }
    }
}

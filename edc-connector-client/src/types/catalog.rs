use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use super::{policy::Policy, query::Query, Protocol};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Catalog {
    #[serde(rename = "dataset", alias = "dcat:dataset")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    datasets: Vec<Dataset>,
}

impl Catalog {
    pub fn datasets(&self) -> &[Dataset] {
        &self.datasets
    }
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Dataset {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "hasPolicy", alias = "odrl:hasPolicy")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    offers: Vec<Policy>,
}

impl Dataset {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn offers(&self) -> &[Policy] {
        &self.offers
    }
}

#[derive(Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CatalogRequest {
    #[builder(default = "CatalogRequest".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[builder(into)]
    counter_party_address: String,
    #[builder(into)]
    #[builder(default)]
    protocol: Protocol,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    counter_party_id: Option<String>,
    query_spec: Query,
}

#[derive(Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DatasetRequest {
    #[builder(default = "DatasetRequest".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(into)]
    counter_party_address: String,
    #[builder(into)]
    #[builder(default)]
    protocol: Protocol,
    #[builder(into)]
    #[serde(skip_serializing_if = "Option::is_none")]
    counter_party_id: Option<String>,
}

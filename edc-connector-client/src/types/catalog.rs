use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use super::{policy::Policy, query::Query, ExtraTokenFields, Protocol};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Catalog<EF: ExtraTokenFields> {
    #[serde(rename = "dataset", alias = "dcat:dataset")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    datasets: Vec<Dataset<EF>>,
}

impl<EF: ExtraTokenFields> Catalog<EF> {
    pub fn datasets(&self) -> &[Dataset<EF>] {
        &self.datasets
    }
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Dataset<EF: ExtraTokenFields> {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "hasPolicy", alias = "odrl:hasPolicy")]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    offers: Vec<Policy>,
    #[serde(flatten, bound = "EF: ExtraTokenFields")]
    pub extra: EF,
}

impl<EF: ExtraTokenFields> Dataset<EF> {
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

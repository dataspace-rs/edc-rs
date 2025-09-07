use crate::types::catalog::{Creator, Thumbnail};
use serde::Deserialize;
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[serde_as]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct CommonProperties {
    #[serde(alias = "dct:title")]
    pub title: Option<String>,
    #[serde(alias = "rdfs:comment")]
    pub comment: Option<String>,
    #[serde(rename = "keyword", alias = "dcat:keyword", default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub keywords: Vec<String>,
    #[serde(alias = "http://xmlns.com/foaf/0.1/thumbnail", default)]
    pub thumbnail: Option<Thumbnail>,
    #[serde(alias = "dct:creator", default)]
    pub creator: Option<Creator>,
    #[serde(alias = "dcat:version")]
    pub version: Option<semver::Version>,
}

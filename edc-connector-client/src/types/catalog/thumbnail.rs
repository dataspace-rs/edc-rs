use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Thumbnail {
    #[serde(alias = "rdf:resource")]
    resource: String,
}

impl Thumbnail {
    pub fn resource(&self) -> &str {
        &self.resource
    }
}

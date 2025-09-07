use super::Thumbnail;
use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Creator {
    #[serde(alias = "http://xmlns.com/foaf/0.1/name", default)]
    name: Option<String>,
    #[serde(alias = "http://xmlns.com/foaf/0.1/thumbnail", default)]
    thumbnail: Option<Thumbnail>,
}

impl Creator {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }
}

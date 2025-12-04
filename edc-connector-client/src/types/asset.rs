use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::error::ConversionError;

use super::{
    data_address::DataAddress,
    properties::{FromValue, Properties, PropertyValue, ToValue},
};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[builder(field)]
    properties: Properties,
    #[builder(field)]
    #[serde(default = "Default::default")]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(default = "Asset".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    data_address: DataAddress,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewAsset {
    #[builder(field)]
    properties: Properties,
    #[builder(field)]
    #[serde(default = "Default::default")]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: Option<String>,
    #[builder(default = "Asset".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    data_address: DataAddress,
}

impl NewAsset {}

impl Asset {
    pub fn property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.properties.get(property)
    }

    pub fn raw_property(&self, property: &str) -> Option<&PropertyValue>
where {
        self.properties.get_raw(property)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn private_properties(&self) -> &Properties {
        &self.private_properties
    }

    pub fn data_address(&self) -> &DataAddress {
        &self.data_address
    }
}

impl<S: asset_builder::State> AssetBuilder<S> {
    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }

    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }
}

impl<S: new_asset_builder::State> NewAssetBuilder<S> {
    pub fn property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.properties.set(property, value);
        self
    }

    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }
}

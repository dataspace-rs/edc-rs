use crate::types::properties::{FromValue, Properties, PropertyValue, ToValue};
use crate::ConversionError;
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CommonExpressionLanguage {
    #[builder(field)]
    properties: Properties,
    #[builder(field)]
    #[serde(default = "Default::default")]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    id: String,
    #[builder(default = "CelExpression".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    left_operand: String,
    description: Option<String>,
    scopes: Vec<String>,
    expression: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewCommonExpressionLanguage {
    #[builder(field)]
    properties: Properties,
    #[builder(field)]
    #[serde(default = "Default::default")]
    private_properties: Properties,
    #[builder(into)]
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[builder(default = "CelExpression".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    left_operand: String,
    description: Option<String>,
    scopes: Vec<String>,
    expression: String,
}

impl CommonExpressionLanguage {
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

    pub fn left_operand(&self) -> &str {
        &self.left_operand
    }
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }
    pub fn expression(&self) -> &str {
        &self.expression
    }
}

impl<S: common_expression_language_builder::State> CommonExpressionLanguageBuilder<S> {
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

impl<S: new_common_expression_language_builder::State> NewCommonExpressionLanguageBuilder<S> {
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

use crate::types::properties::{FromValue, Properties, PropertyValue, ToValue};
use crate::ConversionError;
use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

/// A stored CEL expression (`@type: CelExpression`).
///
/// `properties` and `private_properties` are client-side extras kept for
/// compatibility; the connector model only carries the other fields.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CommonExpressionLanguage {
    #[builder(field)]
    #[serde(default)]
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
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    scopes: Vec<String>,
    /// Policy actions the expression is bound to (e.g. `use`).
    #[builder(default)]
    #[serde(default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    actions: Vec<String>,
    expression: String,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct NewCommonExpressionLanguage {
    #[builder(field)]
    #[serde(default)]
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
    /// Policy actions the expression is bound to (e.g. `use`).
    #[builder(default)]
    #[serde(default)]
    actions: Vec<String>,
    expression: String,
}

/// Request body of `POST /celexpressions/test`: evaluates `expression` with
/// the operands of an atomic constraint and the free-form `params` exposed to
/// the expression as `ctx`.
#[derive(Debug, Serialize, Clone, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CelExpressionTestRequest {
    #[builder(field)]
    params: Properties,
    #[builder(default = "CelExpressionTestRequest".to_string())]
    #[serde(rename = "@type")]
    ty: String,
    #[builder(into)]
    left_operand: String,
    #[builder(into)]
    expression: String,
    #[builder(into)]
    operator: String,
    #[builder(with = |value: impl ToValue| PropertyValue(value.into_value()))]
    right_operand: PropertyValue,
}

impl<S: cel_expression_test_request_builder::State> CelExpressionTestRequestBuilder<S> {
    pub fn param<T>(mut self, name: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.params.set(name, value);
        self
    }
}

impl CelExpressionTestRequest {
    pub fn left_operand(&self) -> &str {
        &self.left_operand
    }

    pub fn expression(&self) -> &str {
        &self.expression
    }

    pub fn operator(&self) -> &str {
        &self.operator
    }

    pub fn right_operand(&self) -> &PropertyValue {
        &self.right_operand
    }

    pub fn params(&self) -> &Properties {
        &self.params
    }
}

/// Outcome of a CEL test evaluation: either a boolean result or the error
/// raised while compiling/evaluating the expression.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CelExpressionTestResponse {
    #[serde(default)]
    evaluation_result: Option<bool>,
    #[serde(default)]
    error: Option<String>,
}

impl CelExpressionTestResponse {
    pub fn evaluation_result(&self) -> Option<bool> {
        self.evaluation_result
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
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
    pub fn actions(&self) -> &Vec<String> {
        &self.actions
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

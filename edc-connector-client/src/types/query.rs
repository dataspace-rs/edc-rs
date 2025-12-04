use serde::{Deserialize, Serialize};

use super::properties::{PropertyValue, ToValue};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    #[serde(rename = "@type")]
    ty: String,
    offset: u32,
    limit: u32,
    #[serde(flatten)]
    sort: Option<Sort>,
    filter_expression: Vec<Criterion>,
}

impl Query {
    pub fn builder() -> QueryBuilder {
        QueryBuilder(Query::default())
    }

    pub fn to_builder(&self) -> QueryBuilder {
        QueryBuilder(self.clone())
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn sort(&self) -> Option<&Sort> {
        self.sort.as_ref()
    }

    pub fn filter_expression(&self) -> &[Criterion] {
        &self.filter_expression
    }
}

pub struct QueryBuilder(Query);

impl QueryBuilder {
    pub fn filter<T>(mut self, left: &str, operator: &str, right: T) -> Self
    where
        T: ToValue,
    {
        self.0.filter_expression.push(Criterion {
            ty: "Criterion".to_string(),
            operand_left: left.to_string(),
            operator: operator.to_string(),
            operand_right: PropertyValue(right.into_value()),
        });
        self
    }

    pub fn sort(mut self, field: &str, order: SortOrder) -> Self {
        self.0.sort = Some(Sort::new(field.to_owned(), order));
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.0.limit = limit;
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.0.offset = offset;
        self
    }

    pub fn build(self) -> Query {
        self.0
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Sort {
    #[serde(rename = "sortField")]
    field: String,
    #[serde(rename = "sortOrder")]
    order: SortOrder,
}

impl Sort {
    pub fn new(field: String, order: SortOrder) -> Self {
        Self { field, order }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn order(&self) -> &SortOrder {
        &self.order
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Criterion {
    #[serde(rename = "@type")]
    ty: String,
    operand_left: String,
    operator: String,
    operand_right: PropertyValue,
}

impl Criterion {
    pub fn new<T: ToValue>(operand_left: &str, operator: &str, operand_right: T) -> Self {
        Self {
            ty: "Criterion".to_string(),
            operand_left: operand_left.to_string(),
            operator: operator.to_string(),
            operand_right: PropertyValue(operand_right.into_value()),
        }
    }

    pub fn operand_left(&self) -> &str {
        &self.operand_left
    }

    pub fn operator(&self) -> &str {
        &self.operator
    }

    pub fn operand_right(&self) -> &PropertyValue {
        &self.operand_right
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            ty: "QuerySpec".to_string(),
            offset: 0,
            limit: 50,
            sort: None,
            filter_expression: Default::default(),
        }
    }
}

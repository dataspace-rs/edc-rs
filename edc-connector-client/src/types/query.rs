use serde::{Deserialize, Serialize};

use super::properties::{PropertyValue, ToValue};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    offset: i32,
    limit: i32,
    #[serde(flatten)]
    sort: Option<Sort>,
    filter_expression: Vec<Criterion>,
}

impl Query {
    pub fn builder() -> QueryBuilder {
        QueryBuilder(Query::default())
    }
}

pub struct QueryBuilder(Query);

impl QueryBuilder {
    pub fn filter<T>(mut self, left: &str, operator: &str, right: T) -> Self
    where
        T: ToValue,
    {
        self.0.filter_expression.push(Criterion {
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

    pub fn limit(mut self, limit: i32) -> Self {
        self.0.limit = limit;
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.0.offset = offset;
        self
    }

    pub fn build(self) -> Query {
        self.0
    }
}

#[derive(Serialize)]
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
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Criterion {
    operand_left: String,
    operator: String,
    operand_right: PropertyValue,
}

impl Criterion {
    pub fn new<T: ToValue>(operand_left: &str, operator: &str, operand_right: T) -> Self {
        Self {
            operand_left: operand_left.to_string(),
            operator: operator.to_string(),
            operand_right: PropertyValue(operand_right.into_value()),
        }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 50,
            sort: None,
            filter_expression: Default::default(),
        }
    }
}

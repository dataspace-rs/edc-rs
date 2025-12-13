use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::EDC_NAMESPACE;

const ODRL_CONTEXT: &str = "http://www.w3.org/ns/odrl.jsonld";
const EDC_V4_CONTEXT: &str = "https://w3id.org/edc/connector/management/v2";

static DEFAULT_CONTEXT_JSON: LazyLock<Value> = LazyLock::new(|| json!({ "@vocab": EDC_NAMESPACE }));
static ODRL_CONTEXT_JSON: LazyLock<Value> =
    LazyLock::new(|| json!([ ODRL_CONTEXT,{ "@vocab": EDC_NAMESPACE }]));

static EDC_V4_CONTEXT_JSON: LazyLock<Value> = LazyLock::new(|| json!([EDC_V4_CONTEXT]));

#[derive(Deserialize, Debug)]
pub struct WithContext<T> {
    #[allow(dead_code)]
    #[serde(rename = "@context")]
    context: Value,
    #[serde(flatten)]
    pub(crate) inner: T,
}

#[derive(Serialize, Debug)]
pub struct WithContextRef<'a, T> {
    #[serde(rename = "@context")]
    context: Value,
    #[serde(flatten)]
    inner: &'a T,
}

impl<'a, T> WithContextRef<'a, T> {
    pub fn new(context: Value, inner: &'a T) -> WithContextRef<'a, T> {
        WithContextRef { context, inner }
    }

    pub fn default_context(inner: &'a T) -> WithContextRef<'a, T> {
        WithContextRef::new(DEFAULT_CONTEXT_JSON.clone(), inner)
    }

    pub fn edc_v4_context(inner: &'a T) -> WithContextRef<'a, T> {
        WithContextRef::new(EDC_V4_CONTEXT_JSON.clone(), inner)
    }

    pub fn odrl_context(inner: &'a T) -> WithContextRef<'a, T> {
        WithContextRef::new(ODRL_CONTEXT_JSON.clone(), inner)
    }
}

impl<T> WithContext<T> {
    pub fn new(context: Value, inner: T) -> WithContext<T> {
        WithContext { context, inner }
    }
}

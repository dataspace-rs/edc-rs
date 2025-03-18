use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Builder)]
pub struct CallbackAddress {
    #[builder(field)]
    #[serde(flatten)]
    auth: Option<CallbackAddressAuth>,
    #[builder(default)]
    transactional: bool,
    #[builder(into)]
    uri: String,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[builder(into)]
    events: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CallbackAddressAuth {
    auth_key: String,
    auth_code_id: String,
}

impl<S: callback_address_builder::State> CallbackAddressBuilder<S> {
    pub fn auth(mut self, auth_key: &str, auth_code_id: &str) -> Self {
        self.auth = Some(CallbackAddressAuth {
            auth_key: auth_key.to_string(),
            auth_code_id: auth_code_id.to_string(),
        });
        self
    }
}

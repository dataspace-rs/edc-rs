use serde::{Deserialize, Serialize};

use crate::DATASPACE_PROTOCOL;

pub mod asset;
pub mod callback_address;
pub mod catalog;
pub mod context;
pub mod contract_agreement;
pub mod contract_definition;
pub mod contract_negotiation;
pub mod data_address;
pub mod dataplane;
pub mod edr;
pub mod policy;
pub mod properties;
pub mod query;
pub mod response;
pub mod secret;
pub mod transfer_process;

#[derive(Deserialize, Serialize, Debug)]
pub struct Protocol(String);

impl Protocol {
    pub fn new(protocol: &str) -> Protocol {
        Protocol(protocol.to_string())
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self(DATASPACE_PROTOCOL.to_string())
    }
}

impl From<&str> for Protocol {
    fn from(protocol: &str) -> Self {
        Protocol(protocol.to_string())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EdcConnectorApiVersion {
    V3,
    V4Alpha,
    V4,
    V5Beta,
}

impl EdcConnectorApiVersion {
    pub fn as_str(&self) -> &str {
        match self {
            Self::V3 => "v3",
            Self::V4Alpha => "v4alpha",
            Self::V4 => "v4",
            Self::V5Beta => "v5beta",
        }
    }
}

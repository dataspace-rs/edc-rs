use crate::EdcResult;
use oauth::OAuth2;
pub use oauth::OAuth2Config;

mod oauth;

#[derive(Clone)]
pub enum Auth {
    NoAuth,
    ApiToken(String),
    OAuth2(OAuth2),
}

impl Auth {
    pub fn api_token(token: impl Into<String>) -> Auth {
        Auth::ApiToken(token.into())
    }

    pub fn oauth(cfg: OAuth2Config) -> EdcResult<Auth> {
        Ok(Auth::OAuth2(OAuth2::init(cfg)?))
    }
}

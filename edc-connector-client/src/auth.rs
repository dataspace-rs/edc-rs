#[derive(Clone)]
pub enum Auth {
    NoAuth,
    ApiToken(String),
}

impl Auth {
    pub fn api_token(token: impl Into<String>) -> Auth {
        Auth::ApiToken(token.into())
    }
}

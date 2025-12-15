use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use bon::Builder;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    AccessToken, AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, RefreshToken, Scope,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use reqwest::Client;
use tokio::sync::Mutex;

use crate::{EdcResult, Error};

#[derive(Clone)]
pub struct OAuth2(Arc<OAuth2Internal>);

type OAuthErrorResponse = oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>;
pub type OAuthTokenResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
type OAuthTokenIntrospection =
    oauth2::StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>;
type OAuthRevocableToken = oauth2::StandardRevocableToken;
type OAuthRevocationError = oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>;
type OAuthClient = oauth2::Client<
    OAuthErrorResponse,
    OAuthTokenResponse,
    OAuthTokenIntrospection,
    OAuthRevocableToken,
    OAuthRevocationError,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

pub struct OAuth2Internal {
    oauth_client: OAuthClient,
    session: Mutex<Option<OAuthTokenSession>>,
    http_client: Client,
    scopes: Vec<String>,
}

pub struct OAuthTokenSession {
    access_token: AccessToken,
    refresh_token: Option<RefreshToken>,
    expires_at: std::time::Instant,
}

impl OAuthTokenSession {
    pub fn new(
        access_token: AccessToken,
        refresh_token: Option<RefreshToken>,
        expires_at: std::time::Instant,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            expires_at,
        }
    }

    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    pub fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at - (Duration::from_secs(30))
    }
}

#[derive(Builder)]
pub struct OAuth2Config {
    #[builder(into)]
    client_id: String,
    #[builder(into)]
    client_secret: String,
    #[builder(into)]
    token_url: String,
    #[builder(default = vec!["management-api:read".to_string(), "management-api:write".to_string()])]
    scopes: Vec<String>,
}

impl OAuth2 {
    pub fn init(cfg: OAuth2Config) -> EdcResult<OAuth2> {
        let client = BasicClient::new(ClientId::new(cfg.client_id))
            .set_client_secret(ClientSecret::new(cfg.client_secret))
            .set_auth_uri(AuthUrl::new("http://authorize".to_string()).unwrap())
            .set_token_uri(TokenUrl::new(cfg.token_url).unwrap());

        Ok(OAuth2(Arc::new(OAuth2Internal {
            oauth_client: client,
            session: Mutex::default(),
            http_client: Client::new(),
            scopes: cfg.scopes,
        })))
    }

    pub async fn token(&self) -> EdcResult<String> {
        self.0.token().await
    }
}

impl OAuth2Internal {
    pub async fn token(&self) -> EdcResult<String> {
        let mut session = self.session.lock().await;

        match session.as_ref() {
            Some(t) if !t.is_expired() => {
                Ok(t.access_token().secret().to_string())
            }
            Some(t) => {
                let new_session = self.refresh_session(t).await?;
                let access_token = new_session.access_token().secret().to_string();
                *session = Some(new_session);
                Ok(access_token)
            }
            _ => {
                let new_session = self.new_session().await?;
                let access_token = new_session.access_token().secret().to_string();
                *session = Some(new_session);
                Ok(access_token)
            }
        }
    }

    async fn new_session(&self) -> EdcResult<OAuthTokenSession> {
        let scopes = self
            .scopes
            .iter()
            .cloned()
            .map(Scope::new)
            .collect::<Vec<_>>();
        let token_result = self
            .oauth_client
            .exchange_client_credentials()
            .add_scopes(scopes)
            .request_async(&self.http_client)
            .await
            .map_err(|e| Error::Auth(Box::new(e)))?;

        let expires_at = Instant::now()
            + token_result
                .expires_in()
                .unwrap_or(Duration::from_secs(3600));

        Ok(OAuthTokenSession::new(
            token_result.access_token().clone(),
            token_result.refresh_token().cloned(),
            expires_at,
        ))
    }

    async fn refresh_session(&self, session: &OAuthTokenSession) -> EdcResult<OAuthTokenSession> {
        if let Some(refresh) = session.refresh_token() {
            let token_result = self
                .oauth_client
                .exchange_refresh_token(refresh)
                .request_async(&self.http_client)
                .await
                .map_err(|e| Error::Auth(Box::new(e)))?;

            let expires_at = Instant::now()
                + token_result
                    .expires_in()
                    .unwrap_or(Duration::from_secs(3600));

            let refresh_token = token_result
                .refresh_token()
                .cloned()
                .or_else(|| Some(refresh.clone()));

            Ok(OAuthTokenSession::new(
                token_result.access_token().clone(),
                refresh_token,
                expires_at,
            ))
        } else {
            self.new_session().await
        }
    }
}

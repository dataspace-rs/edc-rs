use std::{future::Future, sync::Arc};

use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    api::{
        AssetApi, CatalogApi, ContractAgreementApi, ContractDefinitionApi, ContractNegotiationApi,
        DataPlaneApi, EdrApi, PolicyApi, SecretsApi, TransferProcessApi,
    },
    error::{
        BuilderError, ManagementApiError, ManagementApiErrorDetail, ManagementApiErrorDetailKind,
    },
    EdcResult, Error,
};

#[derive(Clone)]
pub struct EdcConnectorClient(Arc<EdcConnectorClientInternal>);

pub(crate) struct EdcConnectorClientInternal {
    client: Client,
    pub(crate) management_url: String,
    pub(crate) auth: Auth,
}

impl EdcConnectorClientInternal {
    pub(crate) fn new(client: Client, management_url: String, auth: Auth) -> Self {
        Self {
            client,
            management_url,
            auth,
        }
    }

    pub(crate) async fn get<R: DeserializeOwned>(&self, path: impl AsRef<str>) -> EdcResult<R> {
        let response = self
            .client
            .get(path.as_ref())
            .authenticated(&self.auth)
            .send()
            .await?;

        self.handle_response(response, as_json).await
    }

    pub(crate) async fn put(&self, path: impl AsRef<str>, body: &impl Serialize) -> EdcResult<()> {
        let response = self
            .client
            .put(path.as_ref())
            .json(body)
            .authenticated(&self.auth)
            .send()
            .await?;

        self.handle_response(response, empty).await
    }

    pub(crate) async fn del(&self, path: impl AsRef<str>) -> EdcResult<()> {
        let response = self
            .client
            .delete(path.as_ref())
            .authenticated(&self.auth)
            .send()
            .await?;

        self.handle_response(response, empty).await
    }

    pub(crate) async fn post<I: Serialize, R: DeserializeOwned>(
        &self,
        path: impl AsRef<str>,
        body: &I,
    ) -> EdcResult<R> {
        self.internal_post(path, body, as_json).await
    }

    pub(crate) async fn post_no_response<I: Serialize>(
        &self,
        path: impl AsRef<str>,
        body: &I,
    ) -> EdcResult<()> {
        self.internal_post(path, body, empty).await
    }

    async fn internal_post<I, F, Fut, R>(
        &self,
        path: impl AsRef<str>,
        body: &I,
        handler: F,
    ) -> EdcResult<R>
    where
        I: Serialize,
        F: Fn(Response) -> Fut,
        Fut: Future<Output = EdcResult<R>>,
    {
        let response = self
            .client
            .post(path.as_ref())
            .json(body)
            .authenticated(&self.auth)
            .send()
            .await?;

        self.handle_response(response, handler).await
    }

    async fn handle_response<F, Fut, R>(&self, response: Response, handler: F) -> EdcResult<R>
    where
        F: Fn(Response) -> Fut,
        Fut: Future<Output = EdcResult<R>>,
    {
        if response.status().is_success() {
            handler(response).await
        } else {
            let status = response.status();
            let text = response.text().await?;

            let err = match serde_json::from_str::<Vec<ManagementApiErrorDetail>>(&text) {
                Ok(parsed) => ManagementApiErrorDetailKind::Parsed(parsed),
                Err(_) => ManagementApiErrorDetailKind::Raw(text),
            };

            Err(Error::ManagementApi(ManagementApiError {
                status_code: status,
                error_detail: err,
            }))
        }
    }
}

async fn as_json<R: DeserializeOwned>(response: Response) -> EdcResult<R> {
    response.json().await.map(Ok)?
}

async fn empty(_response: Response) -> EdcResult<()> {
    Ok(())
}

impl EdcConnectorClient {
    pub(crate) fn new(client: Client, management_url: String, auth: Auth) -> Self {
        Self(Arc::new(EdcConnectorClientInternal::new(
            client,
            management_url,
            auth,
        )))
    }

    pub fn builder() -> EdcClientConnectorBuilder {
        EdcClientConnectorBuilder::default()
    }

    pub fn assets(&self) -> AssetApi<'_> {
        AssetApi::new(&self.0)
    }

    pub fn policies(&self) -> PolicyApi<'_> {
        PolicyApi::new(&self.0)
    }

    pub fn contract_definitions(&self) -> ContractDefinitionApi<'_> {
        ContractDefinitionApi::new(&self.0)
    }

    pub fn catalogue(&self) -> CatalogApi<'_> {
        CatalogApi::new(&self.0)
    }

    pub fn contract_negotiations(&self) -> ContractNegotiationApi<'_> {
        ContractNegotiationApi::new(&self.0)
    }

    pub fn contract_agreements(&self) -> ContractAgreementApi<'_> {
        ContractAgreementApi::new(&self.0)
    }

    pub fn transfer_processes(&self) -> TransferProcessApi<'_> {
        TransferProcessApi::new(&self.0)
    }

    pub fn data_planes(&self) -> DataPlaneApi<'_> {
        DataPlaneApi::new(&self.0)
    }

    pub fn edrs(&self) -> EdrApi<'_> {
        EdrApi::new(&self.0)
    }

    pub fn secrets(&self) -> SecretsApi<'_> {
        SecretsApi::new(&self.0)
    }
}

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

pub struct EdcClientConnectorBuilder {
    management_url: Option<String>,
    auth: Auth,
}

impl EdcClientConnectorBuilder {
    pub fn management_url(mut self, url: impl Into<String>) -> Self {
        self.management_url = Some(url.into());
        self
    }

    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn build(self) -> Result<EdcConnectorClient, BuilderError> {
        let url = self
            .management_url
            .ok_or_else(|| BuilderError::missing_property("management_url"))?;
        let client = Client::new();
        Ok(EdcConnectorClient::new(client, url, self.auth))
    }
}

impl Default for EdcClientConnectorBuilder {
    fn default() -> Self {
        Self {
            management_url: Default::default(),
            auth: Auth::NoAuth,
        }
    }
}

trait BuilderExt {
    fn authenticated(self, auth: &Auth) -> Self;
}

impl BuilderExt for RequestBuilder {
    fn authenticated(self, auth: &Auth) -> Self {
        match auth {
            Auth::NoAuth => self,
            Auth::ApiToken(token) => self.header("X-Api-Key", token),
        }
    }
}

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
    types::context::WithContextRef,
    Auth, EdcResult, Error,
};

#[derive(Clone)]
pub struct EdcConnectorClient(Arc<EdcConnectorClientInternal>);

#[derive(Clone)]
pub enum EdcConnectorApiVersion {
    V3,
    V4,
}

impl EdcConnectorApiVersion {
    pub fn as_str(&self) -> &str {
        match self {
            EdcConnectorApiVersion::V3 => "v3",
            EdcConnectorApiVersion::V4 => "v4beta",
        }
    }
}

pub(crate) struct EdcConnectorClientInternal {
    client: Client,
    pub(crate) management_url: String,
    pub(crate) auth: Auth,
    pub(crate) version: EdcConnectorApiVersion,
}

impl EdcConnectorClientInternal {
    pub(crate) fn new(
        client: Client,
        management_url: String,
        auth: Auth,
        version: EdcConnectorApiVersion,
    ) -> Self {
        Self {
            client,
            management_url,
            auth,
            version,
        }
    }

    pub(crate) async fn get<R: DeserializeOwned>(&self, path: impl AsRef<str>) -> EdcResult<R> {
        let response = self
            .client
            .get(path.as_ref())
            .authenticated(&self.auth)
            .await?
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
            .await?
            .send()
            .await?;

        self.handle_response(response, empty).await
    }

    pub(crate) async fn del(&self, path: impl AsRef<str>) -> EdcResult<()> {
        let response = self
            .client
            .delete(path.as_ref())
            .authenticated(&self.auth)
            .await?
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
            .await?
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

    pub(crate) fn path_for(&self, paths: &[&str]) -> String {
        [self.management_url.as_str(), self.version.as_str()]
            .into_iter()
            .chain(paths.iter().copied())
            .collect::<Vec<_>>()
            .join("/")
    }

    pub(crate) fn context_for<'a, T>(&'a self, body: &'a T) -> WithContextRef<'a, T> {
        self.context_for_with_opts(body, false)
    }

    pub(crate) fn context_for_with_opts<'a, T>(
        &'a self,
        body: &'a T,
        include_odrl: bool,
    ) -> WithContextRef<'a, T> {
        match self.version {
            EdcConnectorApiVersion::V3 => {
                if include_odrl {
                    WithContextRef::odrl_context(body)
                } else {
                    WithContextRef::default_context(body)
                }
            }
            EdcConnectorApiVersion::V4 => WithContextRef::edc_v4_context(body),
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
    pub(crate) fn new(
        client: Client,
        management_url: String,
        auth: Auth,
        version: EdcConnectorApiVersion,
    ) -> Self {
        Self(Arc::new(EdcConnectorClientInternal::new(
            client,
            management_url,
            auth,
            version,
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

    pub fn api_version(&self) -> EdcConnectorApiVersion {
        self.0.version.clone()
    }
}

pub struct EdcClientConnectorBuilder {
    management_url: Option<String>,
    auth: Auth,
    version: EdcConnectorApiVersion,
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

    pub fn version(mut self, version: EdcConnectorApiVersion) -> Self {
        self.version = version;
        self
    }

    pub fn build(self) -> Result<EdcConnectorClient, BuilderError> {
        let url = self
            .management_url
            .ok_or_else(|| BuilderError::missing_property("management_url"))?;
        let client = Client::new();

        Ok(EdcConnectorClient::new(
            client,
            url,
            self.auth,
            self.version,
        ))
    }
}

impl Default for EdcClientConnectorBuilder {
    fn default() -> Self {
        Self {
            management_url: Default::default(),
            auth: Auth::NoAuth,
            version: EdcConnectorApiVersion::V3,
        }
    }
}

trait BuilderExt: Sized {
    async fn authenticated(self, auth: &Auth) -> EdcResult<Self>;
}

impl BuilderExt for RequestBuilder {
    async fn authenticated(self, auth: &Auth) -> EdcResult<Self> {
        match auth {
            Auth::NoAuth => Ok(self),
            Auth::ApiToken(token) => Ok(self.header("X-Api-Key", token)),
        }
    }
}

mod edc_connector_api_version;

pub use edc_connector_api_version::EdcConnectorApiVersion;
use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, sync::Arc};

use crate::{
    api::{
        AssetApi, CatalogApi, CommonExpressionLanguageApi, ContractAgreementApi,
        ContractDefinitionApi, ContractNegotiationApi, DataPlaneApi, EdrApi, ParticipantContextApi,
        ParticipantContextConfigApi, PolicyApi, SecretsApi, TransferProcessApi,
    },
    error::{
        BuilderError, ManagementApiError, ManagementApiErrorDetail, ManagementApiErrorDetailKind,
    },
    types::context::WithContextRef,
    Auth, EdcResult, Error,
};

#[derive(Clone)]
pub struct EdcConnectorClient(Arc<EdcConnectorClientInternal>);

#[allow(unused)]
pub enum ApiTarget {
    Participant,
    Admin,
}

pub(crate) struct EdcConnectorClientInternal {
    client: Client,
    pub(crate) management_url: String,
    pub(crate) auth: Auth,
    pub(crate) participant_context: Option<String>,
}

impl EdcConnectorClientInternal {
    pub(crate) fn new(
        client: Client,
        management_url: String,
        auth: Auth,
        participant_context: Option<String>,
    ) -> Self {
        Self {
            client,
            management_url,
            auth,
            participant_context,
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

    pub(crate) async fn put_no_response<I: Serialize>(
        &self,
        path: impl AsRef<str>,
        body: &I,
    ) -> EdcResult<()> {
        self.internal_put(path, body, empty).await
    }

    pub(crate) async fn post_no_response<I: Serialize>(
        &self,
        path: impl AsRef<str>,
        body: &I,
    ) -> EdcResult<()> {
        self.internal_post(path, body, empty).await
    }

    async fn internal_put<I, F, Fut, R>(
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
            .put(path.as_ref())
            .json(body)
            .authenticated(&self.auth)
            .await?
            .send()
            .await?;

        self.handle_response(response, handler).await
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

    pub(crate) fn path_for(&self, version: EdcConnectorApiVersion, paths: &[&str]) -> String {
        self.path_for_target(ApiTarget::Participant, version, paths)
    }

    pub(crate) fn path_for_target(
        &self,
        target: ApiTarget,
        mut version: EdcConnectorApiVersion,
        paths: &[&str],
    ) -> String {
        let base: &[&str] = if let Some(participant_context) = &self.participant_context {
            version = EdcConnectorApiVersion::V4Alpha;

            match target {
                ApiTarget::Participant => &[
                    self.management_url.as_str(),
                    version.as_str(),
                    "participants",
                    participant_context.as_str(),
                ],
                ApiTarget::Admin => &[self.management_url.as_str(), version.as_str()],
            }
        } else {
            &[self.management_url.as_str(), version.as_str()]
        };

        base.iter()
            .chain(paths.iter())
            .copied()
            .collect::<Vec<_>>()
            .join("/")
    }

    pub(crate) fn context_for<'a, T>(
        &'a self,
        version: EdcConnectorApiVersion,
        body: &'a T,
    ) -> WithContextRef<'a, T> {
        self.context_for_with_opts(version, body, false)
    }

    pub(crate) fn context_for_with_opts<'a, T>(
        &'a self,
        version: EdcConnectorApiVersion,
        body: &'a T,
        include_odrl: bool,
    ) -> WithContextRef<'a, T> {
        match version {
            EdcConnectorApiVersion::V3 => {
                if include_odrl {
                    WithContextRef::odrl_context(body)
                } else {
                    WithContextRef::default_context(body)
                }
            }
            EdcConnectorApiVersion::V4Alpha => WithContextRef::edc_v4_context(body),
            EdcConnectorApiVersion::V4 => WithContextRef::edc_v4_context(body),
            EdcConnectorApiVersion::V5Beta => WithContextRef::edc_v4_context(body),
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
        participant_context: Option<String>,
    ) -> Self {
        Self(Arc::new(EdcConnectorClientInternal::new(
            client,
            management_url,
            auth,
            participant_context,
        )))
    }

    pub fn builder() -> EdcClientConnectorBuilder {
        EdcClientConnectorBuilder::default()
    }

    pub fn assets(&self, version: EdcConnectorApiVersion) -> AssetApi<'_> {
        AssetApi::new(&self.0, version)
    }

    pub fn policies(&self, version: EdcConnectorApiVersion) -> PolicyApi<'_> {
        PolicyApi::new(&self.0, version)
    }

    pub fn contract_definitions(
        &self,
        version: EdcConnectorApiVersion,
    ) -> ContractDefinitionApi<'_> {
        ContractDefinitionApi::new(&self.0, version)
    }

    pub fn catalogue(&self, version: EdcConnectorApiVersion) -> CatalogApi<'_> {
        CatalogApi::new(&self.0, version)
    }

    pub fn contract_negotiations(
        &self,
        version: EdcConnectorApiVersion,
    ) -> ContractNegotiationApi<'_> {
        ContractNegotiationApi::new(&self.0, version)
    }

    pub fn contract_agreements(&self, version: EdcConnectorApiVersion) -> ContractAgreementApi<'_> {
        ContractAgreementApi::new(&self.0, version)
    }

    pub fn transfer_processes(&self, version: EdcConnectorApiVersion) -> TransferProcessApi<'_> {
        TransferProcessApi::new(&self.0, version)
    }

    pub fn data_planes(&self, version: EdcConnectorApiVersion) -> DataPlaneApi<'_> {
        DataPlaneApi::new(&self.0, version)
    }

    pub fn edrs(&self, version: EdcConnectorApiVersion) -> EdrApi<'_> {
        EdrApi::new(&self.0, version)
    }

    pub fn secrets(&self, version: EdcConnectorApiVersion) -> SecretsApi<'_> {
        SecretsApi::new(&self.0, version)
    }

    pub fn participants(&self, version: EdcConnectorApiVersion) -> ParticipantContextApi<'_> {
        ParticipantContextApi::new(&self.0, version)
    }

    pub fn participant_configs(
        &self,
        version: EdcConnectorApiVersion,
    ) -> ParticipantContextConfigApi<'_> {
        ParticipantContextConfigApi::new(&self.0, version)
    }

    pub fn common_expression_language(
        &self,
        version: EdcConnectorApiVersion,
    ) -> CommonExpressionLanguageApi<'_> {
        CommonExpressionLanguageApi::new(&self.0, version)
    }
}

pub struct EdcClientConnectorBuilder {
    management_url: Option<String>,
    auth: Auth,
    participant_context: Option<String>,
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

    pub fn participant_context(mut self, participant_context: impl Into<String>) -> Self {
        self.participant_context = Some(participant_context.into());
        self
    }

    pub fn maybe_participant_context(
        mut self,
        participant_context: Option<impl Into<String>>,
    ) -> Self {
        self.participant_context = participant_context.map(|s| s.into());
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
            self.participant_context,
        ))
    }
}

impl Default for EdcClientConnectorBuilder {
    fn default() -> Self {
        Self {
            management_url: Default::default(),
            auth: Auth::NoAuth,
            participant_context: None,
        }
    }
}

trait BuilderExt: Sized {
    fn authenticated(self, auth: &Auth) -> impl Future<Output = EdcResult<Self>>;
}

impl BuilderExt for RequestBuilder {
    async fn authenticated(self, auth: &Auth) -> EdcResult<Self> {
        match auth {
            Auth::NoAuth => Ok(self),
            Auth::ApiToken(token) => Ok(self.header("X-Api-Key", token)),
            Auth::OAuth2(client) => {
                Ok(self.header("Authorization", format!("Bearer {}", client.token().await?)))
            }
            Auth::BearerToken(token) => {
                Ok(self.header("Authorization", format!("Bearer {}", token)))
            }
        }
    }
}

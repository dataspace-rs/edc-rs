use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        response::IdResponse,
        secret::{NewSecret, Secret},
    },
    EdcResult,
};

pub struct SecretsApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> SecretsApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> SecretsApi<'a> {
        SecretsApi(client)
    }

    pub async fn create(&self, secret: &NewSecret) -> EdcResult<IdResponse<String>> {
        let url = self.0.path_for(&["secrets"]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(url, &self.0.context_for(secret))
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<Secret> {
        let url = self.0.path_for(&["secrets", id]);
        self.0
            .get::<WithContext<Secret>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, secret: &Secret) -> EdcResult<()> {
        let url = self.0.path_for(&["secrets"]);
        self.0.put(url, &self.0.context_for(secret)).await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["secrets", id]);
        self.0.del(url).await
    }
}

use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        response::IdResponse,
        secret::{NewSecret, Secret},
    },
    EdcConnectorApiVersion, EdcResult,
};

const SECRETS_PATH: &str = "secrets";

pub struct SecretsApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> SecretsApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> SecretsApi<'a> {
        SecretsApi { client, version }
    }

    pub async fn create(&self, secret: &NewSecret) -> EdcResult<IdResponse<String>> {
        let url = self.client.path_for(self.version, &[SECRETS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, secret),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<Secret> {
        let url = self.client.path_for(self.version, &[SECRETS_PATH, id]);
        self.client
            .get::<WithContext<Secret>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, secret: &Secret) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[SECRETS_PATH]);
        self.client
            .put(url, &self.client.context_for(self.version, secret))
            .await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[SECRETS_PATH, id]);
        self.client.del(url).await
    }
}

use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        dataplane::{DataPlaneInstance, DataPlaneRegistrationMessage},
        query::Query,
    },
    EdcConnectorApiVersion, EdcResult,
};

const DATAPLANES_PATH: &str = "dataplanes";

pub struct DataPlaneApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> DataPlaneApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> DataPlaneApi<'a> {
        DataPlaneApi { client, version }
    }

    /// Registers a data plane, or updates it when one with the same id is
    /// already registered.
    pub async fn register(&self, registration: &DataPlaneRegistrationMessage) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[DATAPLANES_PATH]);
        self.client.put(url, registration).await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[DATAPLANES_PATH, id]);
        self.client.del(url).await
    }

    /// Lists all registered data planes (`GET /dataplanes`, V4). V5 replaced
    /// this endpoint with [`query`](Self::query).
    pub async fn list(&self) -> EdcResult<Vec<DataPlaneInstance>> {
        let url = self.client.path_for(self.version, &[DATAPLANES_PATH]);
        self.client
            .get::<Vec<WithContext<DataPlaneInstance>>>(url)
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    /// Queries the data planes registered for the participant
    /// (`POST /dataplanes/request`, V5).
    pub async fn query(&self, query: Query) -> EdcResult<Vec<DataPlaneInstance>> {
        let url = self
            .client
            .path_for(self.version, &[DATAPLANES_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<DataPlaneInstance>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}

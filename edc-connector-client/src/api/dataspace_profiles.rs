use crate::{
    client::{ApiTarget, EdcConnectorClientInternal},
    types::{context::WithContext, dataspace_profile::DataspaceProfile, query::Query},
    EdcConnectorApiVersion, EdcResult,
};

const DATASPACE_PROFILES_PATH: &str = "dataspaceprofiles";

/// Dataspace profiles (`/dataspaceprofiles`), a global resource identified by
/// name. Reads need `management-api:profiles:read`, writes need admin scope.
pub struct DataspaceProfileApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> DataspaceProfileApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> DataspaceProfileApi<'a> {
        DataspaceProfileApi { client, version }
    }

    /// Creates a profile and returns it as stored by the connector.
    pub async fn create(&self, profile: &DataspaceProfile) -> EdcResult<DataspaceProfile> {
        let url = self.path(&[DATASPACE_PROFILES_PATH]);
        self.client
            .post::<_, WithContext<DataspaceProfile>>(
                url,
                &self.client.context_for(self.version, profile),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, name: &str) -> EdcResult<DataspaceProfile> {
        let url = self.path(&[DATASPACE_PROFILES_PATH, name]);
        self.client
            .get::<WithContext<DataspaceProfile>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, profile: &DataspaceProfile) -> EdcResult<()> {
        let url = self.path(&[DATASPACE_PROFILES_PATH]);
        self.client
            .put(url, &self.client.context_for(self.version, profile))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<DataspaceProfile>> {
        let url = self.path(&[DATASPACE_PROFILES_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<DataspaceProfile>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, name: &str) -> EdcResult<()> {
        let url = self.path(&[DATASPACE_PROFILES_PATH, name]);
        self.client.del(url).await
    }

    fn path(&self, paths: &[&str]) -> String {
        self.client
            .path_for_target(ApiTarget::Admin, self.version, paths)
    }
}

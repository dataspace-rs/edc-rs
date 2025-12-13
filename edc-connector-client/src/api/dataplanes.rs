use crate::{
    client::EdcConnectorClientInternal,
    types::{context::WithContext, dataplane::DataPlaneInstance},
    EdcResult,
};

pub struct DataPlaneApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> DataPlaneApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> DataPlaneApi<'a> {
        DataPlaneApi(client)
    }

    pub async fn list(&self) -> EdcResult<Vec<DataPlaneInstance>> {
        let url = self.0.path_for(&["dataplanes"]);
        self.0
            .get::<Vec<WithContext<DataPlaneInstance>>>(url)
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}

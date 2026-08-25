use crate::{
    client::EdcConnectorClientInternal,
    types::{
        asset::{Asset, NewAsset},
        context::WithContext,
        query::Query,
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const ASSETS_PATH: &str = "assets";

pub struct AssetApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> AssetApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> AssetApi<'a> {
        AssetApi { client, version }
    }

    pub async fn create(&self, asset: &NewAsset) -> EdcResult<IdResponse<String>> {
        let url = self.client.path_for(self.version, &[ASSETS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, asset),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<Asset> {
        let url = self.client.path_for(self.version, &[ASSETS_PATH, id]);
        self.client
            .get::<WithContext<Asset>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, asset: &Asset) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[ASSETS_PATH]);
        self.client
            .put(url, &self.client.context_for(self.version, asset))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<Asset>> {
        let url = self
            .client
            .path_for(self.version, &[ASSETS_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<Asset>>>(url, &self.client.context_for(self.version, &query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[ASSETS_PATH, id]);
        self.client.del(url).await
    }
}

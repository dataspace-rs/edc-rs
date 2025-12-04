use crate::{
    client::EdcConnectorClientInternal,
    types::{
        asset::{Asset, NewAsset},
        context::WithContext,
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct AssetApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> AssetApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> AssetApi<'a> {
        AssetApi(client)
    }

    pub async fn create(&self, asset: &NewAsset) -> EdcResult<IdResponse<String>> {
        let url = self.0.path_for(&["assets"]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(url, &self.0.context_for(asset))
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<Asset> {
        let url = self.0.path_for(&["assets", id]);
        self.0
            .get::<WithContext<Asset>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, asset: &Asset) -> EdcResult<()> {
        let url = self.0.path_for(&["assets"]);
        self.0.put(url, &self.0.context_for(asset)).await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<Asset>> {
        let url = self.0.path_for(&["assets", "request"]);
        self.0
            .post::<_, Vec<WithContext<Asset>>>(url, &self.0.context_for(&query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["assets", id]);
        self.0.del(url).await
    }
}

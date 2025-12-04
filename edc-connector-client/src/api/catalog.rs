use crate::{
    client::EdcConnectorClientInternal,
    types::{
        catalog::{Catalog, CatalogRequest, Dataset, DatasetRequest},
        context::WithContext,
    },
    EdcResult,
};

pub struct CatalogApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> CatalogApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> CatalogApi<'a> {
        CatalogApi(client)
    }

    pub async fn request(&self, request: &CatalogRequest) -> EdcResult<Catalog> {
        let url = self.0.path_for(&["catalog", "request"]);

        self.0
            .post::<_, WithContext<Catalog>>(url, &self.0.context_for(request))
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn dataset(&self, request: &DatasetRequest) -> EdcResult<Dataset> {
        let url = self.0.path_for(&["catalog", "dataset", "request"]);
        self.0
            .post::<_, WithContext<Dataset>>(url, &self.0.context_for(request))
            .await
            .map(|ctx| ctx.inner)
    }
}

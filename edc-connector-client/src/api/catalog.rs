use crate::types::ExtraTokenFields;
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

    pub async fn request<EF: ExtraTokenFields>(
        &self,
        request: &CatalogRequest,
    ) -> EdcResult<Catalog<EF>> {
        let url = self.0.path_for(&["catalog", "request"]);

        self.0
            .post::<_, WithContext<Catalog<EF>>>(url, &self.0.context_for(request))
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn dataset<EF: ExtraTokenFields>(
        &self,
        request: &DatasetRequest,
    ) -> EdcResult<Dataset<EF>> {
        let url = self.0.path_for(&["catalog", "dataset", "request"]);
        self.0
            .post::<_, WithContext<Dataset<EF>>>(url, &self.0.context_for(request))
            .await
            .map(|ctx| ctx.inner)
    }
}

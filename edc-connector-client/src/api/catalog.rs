use crate::{
    client::EdcConnectorClientInternal,
    types::{
        catalog::{Catalog, CatalogRequest, Dataset, DatasetRequest},
        context::WithContext,
        ExtraTokenFields,
    },
    EdcConnectorApiVersion, EdcResult,
};

const CATALOG_PATH: &str = "catalog";

pub struct CatalogApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> CatalogApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> CatalogApi<'a> {
        CatalogApi { client, version }
    }

    pub async fn request<EF: ExtraTokenFields>(
        &self,
        request: &CatalogRequest,
    ) -> EdcResult<Catalog<EF>> {
        let url = self
            .client
            .path_for(self.version, &[CATALOG_PATH, "request"]);

        self.client
            .post::<_, WithContext<Catalog<EF>>>(
                url,
                &self.client.context_for(self.version, request),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn dataset<EF: ExtraTokenFields>(
        &self,
        request: &DatasetRequest,
    ) -> EdcResult<Dataset<EF>> {
        let url = self
            .client
            .path_for(self.version, &[CATALOG_PATH, "dataset", "request"]);
        self.client
            .post::<_, WithContext<Dataset<EF>>>(
                url,
                &self.client.context_for(self.version, request),
            )
            .await
            .map(|ctx| ctx.inner)
    }
}

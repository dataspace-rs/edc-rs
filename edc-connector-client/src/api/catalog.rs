use crate::{
    client::EdcConnectorClientInternal,
    types::{
        catalog::{Catalog, CatalogRequest, DatasetOrCatalog, DatasetRequest},
        context::{WithContext, WithContextRef},
    },
    EdcResult,
};

pub struct CatalogApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> CatalogApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> CatalogApi<'a> {
        CatalogApi(client)
    }

    pub async fn request(&self, request: &CatalogRequest) -> EdcResult<Catalog> {
        let url = self.get_endpoint(&["request"]);
        self.0
            .post::<_, WithContext<Catalog>>(url, &WithContextRef::default_context(request))
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn dataset(&self, request: &DatasetRequest) -> EdcResult<DatasetOrCatalog> {
        let url = self.get_endpoint(&["dataset", "request"]);
        self.0
            .post::<_, WithContext<DatasetOrCatalog>>(
                url,
                &WithContextRef::default_context(request),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    fn get_endpoint(&self, paths: &[&str]) -> String {
        [self.0.management_url.as_str(), "v3", "catalog"]
            .into_iter()
            .chain(paths.iter().copied())
            .collect::<Vec<_>>()
            .join("/")
    }
}

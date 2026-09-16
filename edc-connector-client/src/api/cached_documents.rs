use crate::{
    client::{ApiTarget, EdcConnectorClientInternal},
    types::{
        cached_document::{CachedDocument, NewCachedDocument},
        context::WithContext,
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const CACHED_DOCUMENTS_PATH: &str = "cacheddocuments";

/// Cached documents (`/cacheddocuments`), a global (admin) resource.
pub struct CachedDocumentApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> CachedDocumentApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> CachedDocumentApi<'a> {
        CachedDocumentApi { client, version }
    }

    /// Lists every cached document.
    pub async fn list(&self) -> EdcResult<Vec<CachedDocument>> {
        let url = self.path(&[CACHED_DOCUMENTS_PATH]);
        self.client
            .get::<Vec<WithContext<CachedDocument>>>(url)
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn get(&self, id: &str) -> EdcResult<CachedDocument> {
        let url = self.path(&[CACHED_DOCUMENTS_PATH, id]);
        self.client
            .get::<WithContext<CachedDocument>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn create(&self, document: &NewCachedDocument) -> EdcResult<IdResponse<String>> {
        let url = self.path(&[CACHED_DOCUMENTS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, document),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, document: &CachedDocument) -> EdcResult<()> {
        let url = self.path(&[CACHED_DOCUMENTS_PATH, document.id()]);
        self.client
            .put(url, &self.client.context_for(self.version, document))
            .await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.path(&[CACHED_DOCUMENTS_PATH, id]);
        self.client.del(url).await
    }

    /// Re-fetches the document from its URL and returns the refreshed entry.
    pub async fn refresh(&self, id: &str) -> EdcResult<CachedDocument> {
        let url = self.path(&[CACHED_DOCUMENTS_PATH, id, "refresh"]);
        self.client
            .post_empty::<WithContext<CachedDocument>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    fn path(&self, paths: &[&str]) -> String {
        self.client
            .path_for_target(ApiTarget::Admin, self.version, paths)
    }
}

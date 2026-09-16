use crate::{
    client::{ApiTarget, EdcConnectorClientInternal},
    types::{
        context::WithContext,
        dcp_scope::{DcpScope, NewDcpScope},
        query::Query,
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const DCP_SCOPES_PATH: &str = "dcpscopes";

/// DCP scopes (`/dcpscopes`), a global (admin) resource.
pub struct DcpScopeApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> DcpScopeApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> DcpScopeApi<'a> {
        DcpScopeApi { client, version }
    }

    pub async fn create(&self, scope: &NewDcpScope) -> EdcResult<IdResponse<String>> {
        let url = self.path(&[DCP_SCOPES_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, scope),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, scope: &DcpScope) -> EdcResult<()> {
        let url = self.path(&[DCP_SCOPES_PATH, scope.id()]);
        self.client
            .put(url, &self.client.context_for(self.version, scope))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<DcpScope>> {
        let url = self.path(&[DCP_SCOPES_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<DcpScope>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.path(&[DCP_SCOPES_PATH, id]);
        self.client.del(url).await
    }

    fn path(&self, paths: &[&str]) -> String {
        self.client
            .path_for_target(ApiTarget::Admin, self.version, paths)
    }
}

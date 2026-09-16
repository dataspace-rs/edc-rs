use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        discovery::{DiscoveryRequest, DiscoveryResponse},
    },
    EdcConnectorApiVersion, EdcResult,
};

const DISCOVER_PATH: &str = "discover";

/// Counter party discovery (`POST /discover/request`), participant scoped.
pub struct DiscoveryApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> DiscoveryApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> DiscoveryApi<'a> {
        DiscoveryApi { client, version }
    }

    /// Resolves the counter party's well-known versions document and returns
    /// every DSP version it shares with one of the participant's profiles.
    pub async fn discover(&self, request: &DiscoveryRequest) -> EdcResult<Vec<DiscoveryResponse>> {
        let url = self
            .client
            .path_for(self.version, &[DISCOVER_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<DiscoveryResponse>>>(
                url,
                &self.client.context_for(self.version, request),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}

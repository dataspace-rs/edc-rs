use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        policy::{NewPolicyDefinition, PolicyDefinition},
        query::Query,
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const POLICY_DEFINITIONS_PATH: &str = "policydefinitions";

pub struct PolicyApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> PolicyApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> PolicyApi<'a> {
        PolicyApi { client, version }
    }

    pub async fn create(
        &self,
        policy_definition: &NewPolicyDefinition,
    ) -> EdcResult<IdResponse<String>> {
        let url = self
            .client
            .path_for(self.version, &[POLICY_DEFINITIONS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self
                    .client
                    .context_for_with_opts(self.version, policy_definition, true),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<PolicyDefinition> {
        let url = self
            .client
            .path_for(self.version, &[POLICY_DEFINITIONS_PATH, id]);
        self.client
            .get::<WithContext<PolicyDefinition>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, policy_definition: &PolicyDefinition) -> EdcResult<()> {
        let url = self.client.path_for(
            self.version,
            &[POLICY_DEFINITIONS_PATH, policy_definition.id()],
        );
        self.client
            .put(
                url,
                &self
                    .client
                    .context_for_with_opts(self.version, policy_definition, true),
            )
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<PolicyDefinition>> {
        let url = self
            .client
            .path_for(self.version, &[POLICY_DEFINITIONS_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<PolicyDefinition>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[POLICY_DEFINITIONS_PATH, id]);
        self.client.del(url).await
    }
}

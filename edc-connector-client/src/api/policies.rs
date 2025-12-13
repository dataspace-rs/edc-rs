use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        policy::{NewPolicyDefinition, PolicyDefinition},
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct PolicyApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> PolicyApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> PolicyApi<'a> {
        PolicyApi(client)
    }

    pub async fn create(
        &self,
        policy_definition: &NewPolicyDefinition,
    ) -> EdcResult<IdResponse<String>> {
        let url = self.0.path_for(&["policydefinitions"]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.0.context_for_with_opts(policy_definition, true),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<PolicyDefinition> {
        let url = self.0.path_for(&["policydefinitions", id]);
        self.0
            .get::<WithContext<PolicyDefinition>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, policy_definition: &PolicyDefinition) -> EdcResult<()> {
        let url = self
            .0
            .path_for(&["policydefinitions", policy_definition.id()]);
        self.0
            .put(url, &self.0.context_for_with_opts(policy_definition, true))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<PolicyDefinition>> {
        let url = self.0.path_for(&["policydefinitions", "request"]);
        self.0
            .post::<_, Vec<WithContext<PolicyDefinition>>>(url, &self.0.context_for(&query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["policydefinitions", id]);
        self.0.del(url).await
    }
}

use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        policy::{NewPolicyDefinition, PolicyDefinition},
        policy_evaluation::{
            PolicyEvaluationPlan, PolicyEvaluationPlanRequest, PolicyValidationResult,
        },
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

    /// Validates a stored policy definition against the registered policy
    /// functions and scopes.
    pub async fn validate(&self, id: &str) -> EdcResult<PolicyValidationResult> {
        let url = self
            .client
            .path_for(self.version, &[POLICY_DEFINITIONS_PATH, id, "validate"]);
        self.client
            .post_empty::<WithContext<PolicyValidationResult>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    /// Builds the plan the policy engine would follow to evaluate a stored
    /// policy definition in the given scope (e.g. `catalog`).
    pub async fn evaluation_plan(
        &self,
        id: &str,
        policy_scope: &str,
    ) -> EdcResult<PolicyEvaluationPlan> {
        let url = self.client.path_for(
            self.version,
            &[POLICY_DEFINITIONS_PATH, id, "evaluationplan"],
        );
        let request = PolicyEvaluationPlanRequest::builder()
            .policy_scope(policy_scope)
            .build();
        self.client
            .post::<_, WithContext<PolicyEvaluationPlan>>(
                url,
                &self.client.context_for(self.version, &request),
            )
            .await
            .map(|ctx| ctx.inner)
    }
}

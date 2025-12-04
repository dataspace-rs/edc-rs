use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        contract_definition::{ContractDefinition, NewContractDefinition},
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct ContractDefinitionApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ContractDefinitionApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ContractDefinitionApi<'a> {
        ContractDefinitionApi(client)
    }

    pub async fn create(
        &self,
        contract_definition: &NewContractDefinition,
    ) -> EdcResult<IdResponse<String>> {
        let url = self.0.path_for(&["contractdefinitions"]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.0.context_for(&contract_definition),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractDefinition> {
        let url = self.0.path_for(&["contractdefinitions", id]);
        self.0
            .get::<WithContext<ContractDefinition>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, contract_definition: &ContractDefinition) -> EdcResult<()> {
        let url = self.0.path_for(&["contractdefinitions"]);
        self.0
            .put(url, &self.0.context_for(&contract_definition))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractDefinition>> {
        let url = self.0.path_for(&["contractdefinitions", "request"]);
        self.0
            .post::<_, Vec<WithContext<ContractDefinition>>>(url, &self.0.context_for(&query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["contractdefinitions", id]);
        self.0.del(url).await
    }
}

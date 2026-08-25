use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        contract_definition::{ContractDefinition, NewContractDefinition},
        query::Query,
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const CONTRACT_DEFINITIONS_PATH: &str = "contractdefinitions";

pub struct ContractDefinitionApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> ContractDefinitionApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> ContractDefinitionApi<'a> {
        ContractDefinitionApi { client, version }
    }

    pub async fn create(
        &self,
        contract_definition: &NewContractDefinition,
    ) -> EdcResult<IdResponse<String>> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_DEFINITIONS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, &contract_definition),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractDefinition> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_DEFINITIONS_PATH, id]);
        self.client
            .get::<WithContext<ContractDefinition>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, contract_definition: &ContractDefinition) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_DEFINITIONS_PATH]);
        self.client
            .put(
                url,
                &self.client.context_for(self.version, &contract_definition),
            )
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractDefinition>> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_DEFINITIONS_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<ContractDefinition>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_DEFINITIONS_PATH, id]);
        self.client.del(url).await
    }
}

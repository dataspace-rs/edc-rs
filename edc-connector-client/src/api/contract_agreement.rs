use crate::{
    client::EdcConnectorClientInternal,
    types::{context::WithContext, contract_agreement::ContractAgreement, query::Query},
    EdcConnectorApiVersion, EdcResult,
};

const CONTRACT_AGREEMENTS_PATH: &str = "contractagreements";

pub struct ContractAgreementApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> ContractAgreementApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> ContractAgreementApi<'a> {
        ContractAgreementApi { client, version }
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractAgreement> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_AGREEMENTS_PATH, id]);
        self.client
            .get::<WithContext<ContractAgreement>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractAgreement>> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_AGREEMENTS_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<ContractAgreement>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}

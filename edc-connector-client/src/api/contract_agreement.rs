use crate::{
    client::EdcConnectorClientInternal,
    types::{context::WithContext, contract_agreement::ContractAgreement, query::Query},
    EdcResult,
};

pub struct ContractAgreementApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ContractAgreementApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ContractAgreementApi<'a> {
        ContractAgreementApi(client)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractAgreement> {
        let url = self.0.path_for(&["contractagreements", id]);
        self.0
            .get::<WithContext<ContractAgreement>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractAgreement>> {
        let url = self.0.path_for(&["contractagreements", "request"]);
        self.0
            .post::<_, Vec<WithContext<ContractAgreement>>>(url, &self.0.context_for(&query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}

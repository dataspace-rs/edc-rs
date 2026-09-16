use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        contract_agreement::ContractAgreement,
        contract_negotiation::{
            ContractNegotiation, ContractNegotiationState, ContractRequest, NegotiationState,
            TerminateNegotiation,
        },
        query::Query,
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const CONTRACT_NEGOTIATIONS_PATH: &str = "contractnegotiations";

pub struct ContractNegotiationApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> ContractNegotiationApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> ContractNegotiationApi<'a> {
        ContractNegotiationApi { client, version }
    }

    pub async fn initiate(
        &self,
        contract_request: &ContractRequest,
    ) -> EdcResult<IdResponse<String>> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_NEGOTIATIONS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self
                    .client
                    .context_for_with_opts(self.version, contract_request, true),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractNegotiation> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_NEGOTIATIONS_PATH, id]);
        self.client
            .get::<WithContext<ContractNegotiation>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get_state(&self, id: &str) -> EdcResult<ContractNegotiationState> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_NEGOTIATIONS_PATH, id, "state"]);
        self.client
            .get::<WithContext<NegotiationState>>(url)
            .await
            .map(|ctx| ctx.inner.state().clone())
    }

    /// Fetches the contract agreement reached by a negotiation.
    pub async fn get_agreement(&self, id: &str) -> EdcResult<ContractAgreement> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_NEGOTIATIONS_PATH, id, "agreement"]);
        self.client
            .get::<WithContext<ContractAgreement>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    /// Deletes a contract negotiation. The connector only allows this for
    /// negotiations that are in a terminal state.
    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_NEGOTIATIONS_PATH, id]);
        self.client.del(url).await
    }

    pub async fn terminate(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_NEGOTIATIONS_PATH, id, "terminate"]);
        let request = TerminateNegotiation::new(id, reason);
        self.client
            .post_no_response(url, &self.client.context_for(self.version, &request))
            .await
            .map(|_| ())
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractNegotiation>> {
        let url = self
            .client
            .path_for(self.version, &[CONTRACT_NEGOTIATIONS_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<ContractNegotiation>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}

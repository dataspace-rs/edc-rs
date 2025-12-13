use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        contract_negotiation::{
            ContractNegotiation, ContractNegotiationState, ContractRequest, NegotiationState,
            TerminateNegotiation,
        },
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct ContractNegotiationApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ContractNegotiationApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ContractNegotiationApi<'a> {
        ContractNegotiationApi(client)
    }

    pub async fn initiate(
        &self,
        contract_request: &ContractRequest,
    ) -> EdcResult<IdResponse<String>> {
        let url = self.0.path_for(&["contractnegotiations"]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.0.context_for_with_opts(contract_request, true),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ContractNegotiation> {
        let url = self.0.path_for(&["contractnegotiations", id]);
        self.0
            .get::<WithContext<ContractNegotiation>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get_state(&self, id: &str) -> EdcResult<ContractNegotiationState> {
        let url = self.0.path_for(&["contractnegotiations", id]);
        self.0
            .get::<WithContext<NegotiationState>>(url)
            .await
            .map(|ctx| ctx.inner.state().clone())
    }

    pub async fn terminate(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["contractnegotiations", id, "terminate"]);
        let request = TerminateNegotiation {
            id: id.to_string(),
            reason: reason.to_string(),
        };
        self.0
            .post_no_response(url, &self.0.context_for(&request))
            .await
            .map(|_| ())
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<ContractNegotiation>> {
        let url = self.0.path_for(&["contractnegotiations", "request"]);
        self.0
            .post::<_, Vec<WithContext<ContractNegotiation>>>(url, &self.0.context_for(&query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }
}

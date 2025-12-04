use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext,
        query::Query,
        response::IdResponse,
        transfer_process::{
            SuspendTransfer, TerminateTransfer, TransferProcess, TransferProcessState,
            TransferRequest, TransferState,
        },
    },
    EdcResult,
};

pub struct TransferProcessApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> TransferProcessApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> TransferProcessApi<'a> {
        TransferProcessApi(client)
    }

    pub async fn initiate(
        &self,
        transfer_request: &TransferRequest,
    ) -> EdcResult<IdResponse<String>> {
        let url = self.0.path_for(&["transferprocesses"]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(url, &self.0.context_for(&transfer_request))
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<TransferProcess> {
        let url = self.0.path_for(&["transferprocesses", id]);
        self.0
            .get::<WithContext<TransferProcess>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get_state(&self, id: &str) -> EdcResult<TransferProcessState> {
        let url = self.0.path_for(&["transferprocesses", id]);
        self.0
            .get::<WithContext<TransferState>>(url)
            .await
            .map(|ctx| ctx.inner.state().clone())
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<TransferProcess>> {
        let url = self.0.path_for(&["transferprocesses", "request"]);

        self.0
            .post::<_, Vec<WithContext<TransferProcess>>>(url, &self.0.context_for(&query))
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn terminate(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["transferprocesses", id, "terminate"]);

        let request = TerminateTransfer::builder()
            .id(id.to_string())
            .reason(reason.to_string())
            .build();
        self.0
            .post_no_response(url, &self.0.context_for(&request))
            .await
            .map(|_| ())
    }

    pub async fn suspend(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["transferprocesses", id, "suspend"]);

        let request = SuspendTransfer::builder()
            .id(id.to_string())
            .reason(reason.to_string())
            .build();

        self.0
            .post_no_response(url, &self.0.context_for(&request))
            .await
            .map(|_| ())
    }

    pub async fn resume(&self, id: &str) -> EdcResult<()> {
        let url = self.0.path_for(&["transferprocesses", id, "resume"]);
        self.0
            .post_no_response(url, &Option::<()>::None)
            .await
            .map(|_| ())
    }
}

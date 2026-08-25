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
    EdcConnectorApiVersion, EdcResult,
};

const TRANSFER_PROCESSES_PATH: &str = "transferprocesses";

pub struct TransferProcessApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> TransferProcessApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> TransferProcessApi<'a> {
        TransferProcessApi { client, version }
    }

    pub async fn initiate(
        &self,
        transfer_request: &TransferRequest,
    ) -> EdcResult<IdResponse<String>> {
        let url = self
            .client
            .path_for(self.version, &[TRANSFER_PROCESSES_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, &transfer_request),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<TransferProcess> {
        let url = self
            .client
            .path_for(self.version, &[TRANSFER_PROCESSES_PATH, id]);
        self.client
            .get::<WithContext<TransferProcess>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get_state(&self, id: &str) -> EdcResult<TransferProcessState> {
        let url = self
            .client
            .path_for(self.version, &[TRANSFER_PROCESSES_PATH, id]);
        self.client
            .get::<WithContext<TransferState>>(url)
            .await
            .map(|ctx| ctx.inner.state().clone())
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<TransferProcess>> {
        let url = self
            .client
            .path_for(self.version, &[TRANSFER_PROCESSES_PATH, "request"]);

        self.client
            .post::<_, Vec<WithContext<TransferProcess>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn terminate(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[TRANSFER_PROCESSES_PATH, id, "terminate"]);

        let request = TerminateTransfer::builder()
            .id(id.to_string())
            .reason(reason.to_string())
            .build();

        self.client
            .post_no_response(url, &self.client.context_for(self.version, &request))
            .await
            .map(|_| ())
    }

    pub async fn suspend(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[TRANSFER_PROCESSES_PATH, id, "suspend"]);

        let request = SuspendTransfer::builder()
            .id(id.to_string())
            .reason(reason.to_string())
            .build();

        self.client
            .post_no_response(url, &self.client.context_for(self.version, &request))
            .await
            .map(|_| ())
    }

    pub async fn resume(&self, id: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[TRANSFER_PROCESSES_PATH, id, "resume"]);
        self.client
            .post_no_response(url, &Option::<()>::None)
            .await
            .map(|_| ())
    }
}

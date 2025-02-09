use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
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
        let url = self.get_endpoint(&[]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &WithContextRef::default_context(transfer_request),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<TransferProcess> {
        let url = self.get_endpoint(&[id]);
        self.0
            .get::<WithContext<TransferProcess>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get_state(&self, id: &str) -> EdcResult<TransferProcessState> {
        let url = self.get_endpoint(&[id]);
        self.0
            .get::<WithContext<TransferState>>(url)
            .await
            .map(|ctx| ctx.inner.state().clone())
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<TransferProcess>> {
        let url = self.get_endpoint(&["request"]);
        self.0
            .post::<_, Vec<WithContext<TransferProcess>>>(
                url,
                &WithContextRef::default_context(&query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn terminate(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self.get_endpoint(&[id, "terminate"]);

        let request = TerminateTransfer {
            id: id.to_string(),
            reason: reason.to_string(),
        };
        self.0
            .post_no_response(url, &WithContextRef::default_context(&request))
            .await
            .map(|_| ())
    }

    pub async fn suspend(&self, id: &str, reason: &str) -> EdcResult<()> {
        let url = self.get_endpoint(&[id, "suspend"]);

        let request = SuspendTransfer {
            id: id.to_string(),
            reason: reason.to_string(),
        };
        self.0
            .post_no_response(url, &WithContextRef::default_context(&request))
            .await
            .map(|_| ())
    }

    pub async fn resume(&self, id: &str) -> EdcResult<()> {
        let url = self.get_endpoint(&[id, "resume"]);
        self.0
            .post_no_response(url, &Option::<()>::None)
            .await
            .map(|_| ())
    }

    fn get_endpoint(&self, paths: &[&str]) -> String {
        [self.0.management_url.as_str(), "v3", "transferprocesses"]
            .into_iter()
            .chain(paths.iter().copied())
            .collect::<Vec<_>>()
            .join("/")
    }
}

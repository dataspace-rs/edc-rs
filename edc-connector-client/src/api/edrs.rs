use reqwest::StatusCode;

use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
        data_address::DataAddress,
        edr::EndpointDataReferenceEntry,
        query::Query,
    },
    EdcResult,
};

pub struct EdrApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> EdrApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> EdrApi<'a> {
        EdrApi(client)
    }

    pub async fn get_entry(&self, id: &str) -> EdcResult<EndpointDataReferenceEntry> {
        let query = Query::builder()
            .filter("transferProcessId", "=", id)
            .build();

        self.query(query).await.and_then(|edrs| {
            edrs.into_iter().next().ok_or_else(|| {
                crate::Error::ManagementApi(crate::ManagementApiError {
                    status_code: StatusCode::NOT_FOUND,
                    error_detail: crate::ManagementApiErrorDetailKind::Raw(format!(
                        "EDR entry with id {} not found",
                        id
                    )),
                })
            })
        })
    }

    pub async fn get_data_address(&self, id: &str) -> EdcResult<DataAddress> {
        let url = self.get_endpoint(&[id, "dataaddress"]);
        self.0
            .get::<WithContext<DataAddress>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<EndpointDataReferenceEntry>> {
        let url = self.get_endpoint(&["request"]);
        self.0
            .post::<_, Vec<WithContext<EndpointDataReferenceEntry>>>(
                url,
                &WithContextRef::default_context(&query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.get_endpoint(&[id]);
        self.0.del(url).await
    }

    fn get_endpoint(&self, paths: &[&str]) -> String {
        [self.0.management_url.as_str(), "v3", "edrs"]
            .into_iter()
            .chain(paths.iter().copied())
            .collect::<Vec<_>>()
            .join("/")
    }
}

use reqwest::StatusCode;

use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::WithContext, data_address::DataAddress, edr::EndpointDataReferenceEntry,
        query::Query,
    },
    EdcConnectorApiVersion, EdcResult,
};

const EDRS_PATH: &str = "edrs";

pub struct EdrApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> EdrApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> EdrApi<'a> {
        EdrApi { client, version }
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
        let url = self
            .client
            .path_for(self.version, &[EDRS_PATH, id, "dataaddress"]);
        self.client
            .get::<WithContext<DataAddress>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<EndpointDataReferenceEntry>> {
        let url = self.client.path_for(self.version, &[EDRS_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<EndpointDataReferenceEntry>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[EDRS_PATH, id]);
        self.client.del(url).await
    }
}

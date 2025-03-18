use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
        response::IdResponse,
        secret::{NewSecret, Secret},
    },
    EdcResult,
};

pub struct SecretsApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> SecretsApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> SecretsApi<'a> {
        SecretsApi(client)
    }

    pub async fn create(&self, asset: &NewSecret) -> EdcResult<IdResponse<String>> {
        let url = self.get_endpoint(&[]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &WithContextRef::default_context(asset),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<Secret> {
        let url = self.get_endpoint(&[id]);
        self.0
            .get::<WithContext<Secret>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, asset: &Secret) -> EdcResult<()> {
        let url = self.get_endpoint(&[]);
        self.0
            .put(url, &WithContextRef::default_context(asset))
            .await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.get_endpoint(&[id]);
        self.0.del(url).await
    }

    fn get_endpoint(&self, paths: &[&str]) -> String {
        [self.0.management_url.as_str(), "v3", "secrets"]
            .into_iter()
            .chain(paths.iter().copied())
            .collect::<Vec<_>>()
            .join("/")
    }
}

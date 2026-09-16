use crate::{
    client::{ApiTarget, EdcConnectorClientInternal},
    types::{
        context::WithContext,
        response::IdResponse,
        schema_validator::{NewSchemaValidatorRegistration, SchemaValidatorRegistration},
    },
    EdcConnectorApiVersion, EdcResult,
};

const SCHEMA_VALIDATORS_PATH: &str = "schemavalidators";

/// Schema validator registrations (`/schemavalidators`), a global (admin) resource.
pub struct SchemaValidatorApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> SchemaValidatorApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> SchemaValidatorApi<'a> {
        SchemaValidatorApi { client, version }
    }

    /// Lists every registration.
    pub async fn list(&self) -> EdcResult<Vec<SchemaValidatorRegistration>> {
        let url = self.path(&[SCHEMA_VALIDATORS_PATH]);
        self.client
            .get::<Vec<WithContext<SchemaValidatorRegistration>>>(url)
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn get(&self, id: &str) -> EdcResult<SchemaValidatorRegistration> {
        let url = self.path(&[SCHEMA_VALIDATORS_PATH, id]);
        self.client
            .get::<WithContext<SchemaValidatorRegistration>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn create(
        &self,
        registration: &NewSchemaValidatorRegistration,
    ) -> EdcResult<IdResponse<String>> {
        let url = self.path(&[SCHEMA_VALIDATORS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, registration),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, registration: &SchemaValidatorRegistration) -> EdcResult<()> {
        let url = self.path(&[SCHEMA_VALIDATORS_PATH, registration.id()]);
        self.client
            .put(url, &self.client.context_for(self.version, registration))
            .await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.path(&[SCHEMA_VALIDATORS_PATH, id]);
        self.client.del(url).await
    }

    fn path(&self, paths: &[&str]) -> String {
        self.client
            .path_for_target(ApiTarget::Admin, self.version, paths)
    }
}

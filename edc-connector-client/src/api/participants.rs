use crate::{
    client::{ApiTarget, EdcConnectorClientInternal},
    types::{
        context::WithContext,
        dataspace_profile::DataspaceProfile,
        participants::{
            AssociateDataspaceProfile, NewParticipantContext, ParticipantContext,
            ParticipantContextConfig, ParticipantContextConfigPatch,
        },
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const PARTICIPANTS_PATH: &str = "participants";

/// Participant contexts (`/participants`), a global (admin) resource: the
/// paths are never nested under the client's own participant context, the
/// context to act on is always passed explicitly.
pub struct ParticipantContextApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> ParticipantContextApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> ParticipantContextApi<'a> {
        ParticipantContextApi { client, version }
    }

    pub async fn create(&self, ctx: &NewParticipantContext) -> EdcResult<IdResponse<String>> {
        let url = self.path(&[PARTICIPANTS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, ctx),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<ParticipantContext> {
        let url = self.path(&[PARTICIPANTS_PATH, id]);
        self.client
            .get::<WithContext<ParticipantContext>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    /// Lists participant contexts page by page (`GET /participants?offset&limit`).
    pub async fn list(&self, offset: u32, limit: u32) -> EdcResult<Vec<ParticipantContext>> {
        let url = self.path(&[PARTICIPANTS_PATH]);
        self.client
            .get_with_query::<Vec<WithContext<ParticipantContext>>>(
                url,
                &[("offset", offset.to_string()), ("limit", limit.to_string())],
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn update(&self, ctx: &ParticipantContext) -> EdcResult<()> {
        let url = self.path(&[PARTICIPANTS_PATH, ctx.id()]);
        self.client
            .put(url, &self.client.context_for(self.version, ctx))
            .await
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.path(&[PARTICIPANTS_PATH, id]);
        self.client.del(url).await
    }

    /// The dataspace profiles a participant context is associated with.
    pub async fn profiles(&self, participant_context_id: &str) -> EdcResult<Vec<DataspaceProfile>> {
        let url = self.path(&[PARTICIPANTS_PATH, participant_context_id, "profiles"]);
        self.client
            .get::<Vec<WithContext<DataspaceProfile>>>(url)
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    /// Replaces the dataspace profiles a participant context is associated with.
    pub async fn associate_profiles(
        &self,
        participant_context_id: &str,
        profiles: &[&str],
    ) -> EdcResult<()> {
        let url = self.path(&[PARTICIPANTS_PATH, participant_context_id, "profiles"]);
        let body = AssociateDataspaceProfile::new(profiles.iter().map(|p| p.to_string()).collect());
        self.client
            .put_no_response(url, &self.client.context_for(self.version, &body))
            .await
    }

    fn path(&self, paths: &[&str]) -> String {
        self.client
            .path_for_target(ApiTarget::Admin, self.version, paths)
    }
}

/// Participant context configuration (`/participants/{id}/config`), admin scope.
pub struct ParticipantContextConfigApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> ParticipantContextConfigApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> ParticipantContextConfigApi<'a> {
        ParticipantContextConfigApi { client, version }
    }

    pub async fn get(&self, participant_context_id: &str) -> EdcResult<ParticipantContextConfig> {
        let url = self.path(participant_context_id);
        self.client
            .get::<WithContext<ParticipantContextConfig>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    /// Replaces the whole configuration.
    pub async fn save(
        &self,
        participant_context_id: &str,
        cfg: &ParticipantContextConfig,
    ) -> EdcResult<()> {
        let url = self.path(participant_context_id);
        self.client
            .put_no_response(url, &self.client.context_for(self.version, cfg))
            .await
    }

    /// Merges the given entries into the configuration; keys mapped to `None`
    /// are removed.
    pub async fn patch(
        &self,
        participant_context_id: &str,
        patch: &ParticipantContextConfigPatch,
    ) -> EdcResult<()> {
        let url = self.path(participant_context_id);
        self.client
            .patch_no_response(url, &self.client.context_for(self.version, patch))
            .await
    }

    fn path(&self, participant_context_id: &str) -> String {
        self.client.path_for_target(
            ApiTarget::Admin,
            self.version,
            &[PARTICIPANTS_PATH, participant_context_id, "config"],
        )
    }
}

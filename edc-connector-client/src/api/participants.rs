use crate::{
    client::{ApiTarget, EdcConnectorClientInternal},
    types::{
        context::WithContext,
        participants::{NewParticipantContext, ParticipantContextConfig},
        response::IdResponse,
    },
    EdcConnectorApiVersion, EdcResult,
};

const PARTICIPANTS_PATH: &str = "participants";

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
        let url = self
            .client
            .path_for_target(ApiTarget::Admin, self.version, &[PARTICIPANTS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self.client.context_for(self.version, ctx),
            )
            .await
            .map(|ctx| ctx.inner)
    }
}

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

    pub async fn save(
        &self,
        participant_context_id: &str,
        cfg: &ParticipantContextConfig,
    ) -> EdcResult<()> {
        let url = self.client.path_for_target(
            ApiTarget::Admin,
            self.version,
            &[PARTICIPANTS_PATH, participant_context_id, "config"],
        );
        self.client
            .put_no_response(url, &self.client.context_for(self.version, cfg))
            .await
    }
}

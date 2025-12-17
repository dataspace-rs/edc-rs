use crate::{
    client::{ApiTarget, EdcConnectorClientInternal},
    types::{
        context::WithContext,
        participants::{NewParticipantContext, ParticipantContextConfig},
        response::IdResponse,
    },
    EdcResult,
};

pub struct ParticipantContextApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ParticipantContextApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ParticipantContextApi<'a> {
        ParticipantContextApi(client)
    }

    pub async fn create(&self, ctx: &NewParticipantContext) -> EdcResult<IdResponse<String>> {
        let url = self.0.path_for_target(ApiTarget::Admin, &["participants"]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(url, &self.0.context_for(ctx))
            .await
            .map(|ctx| ctx.inner)
    }
}

pub struct ParticipantContextConfigApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> ParticipantContextConfigApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> ParticipantContextConfigApi<'a> {
        ParticipantContextConfigApi(client)
    }

    pub async fn save(
        &self,
        participant_context_id: &str,
        cfg: &ParticipantContextConfig,
    ) -> EdcResult<()> {
        let url = self.0.path_for_target(
            ApiTarget::Admin,
            &["participants", participant_context_id, "config"],
        );
        self.0.put_no_response(url, &self.0.context_for(cfg)).await
    }
}

use crate::client::EdcConnectorClientInternal;
use crate::types::common_expression_language::{
    CommonExpressionLanguage, NewCommonExpressionLanguage,
};
use crate::types::context::WithContext;
use crate::types::query::Query;
use crate::types::response::IdResponse;
use crate::{EdcConnectorApiVersion, EdcResult};

const CEL_EXPRESSIONS_PATH: &str = "celexpressions";

pub struct CommonExpressionLanguageApi<'a> {
    client: &'a EdcConnectorClientInternal,
    version: EdcConnectorApiVersion,
}

impl<'a> CommonExpressionLanguageApi<'a> {
    pub(crate) fn new(
        client: &'a EdcConnectorClientInternal,
        version: EdcConnectorApiVersion,
    ) -> CommonExpressionLanguageApi<'a> {
        CommonExpressionLanguageApi { client, version }
    }

    pub async fn create(
        &self,
        common_expression_language: &NewCommonExpressionLanguage,
    ) -> EdcResult<IdResponse<String>> {
        let url = self.client.path_for(self.version, &[CEL_EXPRESSIONS_PATH]);
        self.client
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &self
                    .client
                    .context_for(self.version, common_expression_language),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<CommonExpressionLanguage> {
        let url = self
            .client
            .path_for(self.version, &[CEL_EXPRESSIONS_PATH, id]);
        self.client
            .get::<WithContext<CommonExpressionLanguage>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(
        &self,
        common_expression_language: &CommonExpressionLanguage,
    ) -> EdcResult<()> {
        let url = self.client.path_for(self.version, &[CEL_EXPRESSIONS_PATH]);
        self.client
            .put(
                url,
                &self
                    .client
                    .context_for(self.version, common_expression_language),
            )
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<CommonExpressionLanguage>> {
        let url = self
            .client
            .path_for(self.version, &[CEL_EXPRESSIONS_PATH, "request"]);
        self.client
            .post::<_, Vec<WithContext<CommonExpressionLanguage>>>(
                url,
                &self.client.context_for(self.version, &query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self
            .client
            .path_for(self.version, &[CEL_EXPRESSIONS_PATH, id]);
        self.client.del(url).await
    }
}

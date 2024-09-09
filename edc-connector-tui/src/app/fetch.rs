use edc_connector_client::types::query::Query;

use crate::{
    components::{
        assets::AssetEntry, contract_definitions::ContractDefinitionEntry,
        policies::PolicyDefinitionEntry,
    },
    types::connector::Connector,
};

use super::App;

impl App {
    pub async fn fetch_assets(connector: Connector) -> anyhow::Result<Vec<AssetEntry>> {
        Ok(connector
            .client()
            .assets()
            .query(Query::default())
            .await?
            .into_iter()
            .map(AssetEntry::new)
            .collect())
    }

    pub async fn fetch_contract_definitions(
        connector: Connector,
    ) -> anyhow::Result<Vec<ContractDefinitionEntry>> {
        Ok(connector
            .client()
            .contract_definitions()
            .query(Query::default())
            .await?
            .into_iter()
            .map(ContractDefinitionEntry::new)
            .collect())
    }

    pub async fn fetch_policies(
        connector: Connector,
    ) -> anyhow::Result<Vec<PolicyDefinitionEntry>> {
        Ok(connector
            .client()
            .policies()
            .query(Query::default())
            .await?
            .into_iter()
            .map(PolicyDefinitionEntry::new)
            .collect())
    }
}
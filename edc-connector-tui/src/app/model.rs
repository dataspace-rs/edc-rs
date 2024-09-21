#[derive(Debug, Default)]
pub enum AppFocus {
    #[default]
    ConnectorList,
    LaunchBar,
    Assets,
    Policies,
    ContractDefinitions,
    ContractNegotiations,
    TransferProcesses,
}

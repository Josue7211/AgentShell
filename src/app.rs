use crate::{
    clients::{AgentSecretsClient, ClientError, OpenClawClient},
    config::AgentShellConfig,
    contracts::{
        ContractError, OpenClawSessionPlan, OpenClawSessionRequest, SecretApprovalPlan,
        SecretApprovalRequest,
    },
    state::AppState,
    workflow::WorkflowPlanner,
};

#[derive(Debug, Clone)]
pub struct AgentShellApp {
    pub state: AppState,
    planner: WorkflowPlanner,
}

impl AgentShellApp {
    pub fn new(config: AgentShellConfig) -> Self {
        let planner = WorkflowPlanner::new(config.clone());
        Self {
            state: AppState::new(config),
            planner,
        }
    }

    pub fn config(&self) -> &AgentShellConfig {
        &self.state.config
    }

    pub fn plan_openclaw_session(
        &self,
        request: OpenClawSessionRequest,
    ) -> Result<OpenClawSessionPlan, ContractError> {
        self.planner.plan_openclaw_session(request)
    }

    pub fn plan_secret_approval(
        &self,
        request: SecretApprovalRequest,
    ) -> Result<SecretApprovalPlan, ContractError> {
        self.planner.plan_secret_approval(request)
    }

    pub async fn dispatch_openclaw_session(
        &self,
        request: OpenClawSessionRequest,
    ) -> Result<serde_json::Value, ClientError> {
        let plan = self.plan_openclaw_session(request)?;
        OpenClawClient::new(plan.openclaw_url.clone())
            .launch_session(plan)
            .await
    }

    pub async fn dispatch_secret_approval(
        &self,
        request: SecretApprovalRequest,
    ) -> Result<serde_json::Value, ClientError> {
        let plan = self.plan_secret_approval(request)?;
        AgentSecretsClient::new(plan.agentsecrets_url.clone())
            .request_approval(plan)
            .await
    }
}

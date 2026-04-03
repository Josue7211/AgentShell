use crate::{
    config::AgentShellConfig,
    contracts::{
        OpenClawSessionPlan, OpenClawSessionRequest, SecretApprovalPlan, SecretApprovalRequest,
    },
};

#[derive(Debug, Clone)]
pub struct WorkflowPlanner {
    config: AgentShellConfig,
}

impl WorkflowPlanner {
    pub fn new(config: AgentShellConfig) -> Self {
        Self { config }
    }

    pub fn plan_openclaw_session(
        &self,
        request: OpenClawSessionRequest,
    ) -> Result<OpenClawSessionPlan, crate::contracts::ContractError> {
        request.validate()?;

        Ok(OpenClawSessionPlan {
            openclaw_url: self.config.openclaw.url.clone(),
            request,
        })
    }

    pub fn plan_secret_approval(
        &self,
        request: SecretApprovalRequest,
    ) -> Result<SecretApprovalPlan, crate::contracts::ContractError> {
        request.validate()?;

        Ok(SecretApprovalPlan {
            agentsecrets_url: self.config.secrets.url.clone(),
            request,
        })
    }
}

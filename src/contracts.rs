use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenClawSessionRequest {
    pub project: String,
    pub objective: String,
    pub profile: String,
    pub workspace: Option<String>,
    pub command: Option<String>,
    pub needs_secrets: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenClawSessionPlan {
    pub openclaw_url: String,
    pub request: OpenClawSessionRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecretApprovalRequest {
    pub session_id: String,
    pub secret_ref: String,
    pub action: String,
    pub target: String,
    pub reason: String,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecretApprovalPlan {
    pub agentsecrets_url: String,
    pub request: SecretApprovalRequest,
}

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("{field} must not be empty")]
    EmptyField { field: &'static str },
}

impl OpenClawSessionRequest {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.project.trim().is_empty() {
            return Err(ContractError::EmptyField { field: "project" });
        }
        if self.objective.trim().is_empty() {
            return Err(ContractError::EmptyField { field: "objective" });
        }
        if self.profile.trim().is_empty() {
            return Err(ContractError::EmptyField { field: "profile" });
        }
        Ok(())
    }
}

impl SecretApprovalRequest {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.session_id.trim().is_empty() {
            return Err(ContractError::EmptyField {
                field: "session_id",
            });
        }
        if self.secret_ref.trim().is_empty() {
            return Err(ContractError::EmptyField {
                field: "secret_ref",
            });
        }
        if self.action.trim().is_empty() {
            return Err(ContractError::EmptyField { field: "action" });
        }
        if self.target.trim().is_empty() {
            return Err(ContractError::EmptyField { field: "target" });
        }
        if self.reason.trim().is_empty() {
            return Err(ContractError::EmptyField { field: "reason" });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_session_request() {
        let request = OpenClawSessionRequest {
            project: "mission-control".into(),
            objective: "Do useful work".into(),
            profile: "default".into(),
            workspace: None,
            command: None,
            needs_secrets: false,
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn validates_approval_request() {
        let request = SecretApprovalRequest {
            session_id: "sess_123".into(),
            secret_ref: "bw://login/github".into(),
            action: "read".into(),
            target: "github".into(),
            reason: "CI deploy".into(),
            ttl_seconds: 300,
        };
        assert!(request.validate().is_ok());
    }
}

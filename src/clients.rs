use reqwest::StatusCode;

use crate::contracts::{OpenClawSessionPlan, SecretApprovalPlan, SecretApprovalRequest};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    Contract(#[from] crate::contracts::ContractError),
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("unexpected status {status}: {body}")]
    UnexpectedStatus { status: StatusCode, body: String },
}

#[derive(Debug, Clone)]
pub struct OpenClawClient {
    http: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct AgentSecretsClient {
    http: reqwest::Client,
    base_url: String,
}

impl OpenClawClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn launch_session(
        &self,
        plan: OpenClawSessionPlan,
    ) -> Result<serde_json::Value, ClientError> {
        let url = format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            plan.openclaw_launch_path
        );
        let response = self.http.post(url).json(&plan).send().await?;
        Self::decode_json_response(response).await
    }

    async fn decode_json_response(
        response: reqwest::Response,
    ) -> Result<serde_json::Value, ClientError> {
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(ClientError::UnexpectedStatus { status, body });
        }

        if body.trim().is_empty() {
            return Ok(serde_json::json!({ "status": "ok" }));
        }

        match serde_json::from_str(&body) {
            Ok(value) => Ok(value),
            Err(_) => Ok(serde_json::json!({ "raw": body })),
        }
    }
}

impl AgentSecretsClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }

    pub async fn request_approval(
        &self,
        plan: SecretApprovalPlan,
    ) -> Result<serde_json::Value, ClientError> {
        let url = format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            plan.agentsecrets_approval_path
        );
        let response = self.http.post(url).json(&plan).send().await?;
        OpenClawClient::decode_json_response(response).await
    }

    pub async fn request_secret_approval(
        &self,
        request: SecretApprovalRequest,
    ) -> Result<serde_json::Value, ClientError> {
        let plan = SecretApprovalPlan {
            agentsecrets_url: self.base_url.clone(),
            agentsecrets_approval_path: "/v1/approvals".to_string(),
            request,
        };
        self.request_approval(plan).await
    }
}

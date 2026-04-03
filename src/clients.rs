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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AgentShellConfig, OpenClawConfig, SecretBrokerConfig};
    use crate::contracts::{OpenClawSessionRequest, SecretApprovalRequest};
    use axum::{routing::post, Json, Router};
    use serde_json::json;
    use tokio::net::TcpListener;

    async fn spawn_mock_server(router: Router) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        format!("http://{}", addr)
    }

    #[tokio::test]
    async fn dispatches_openclaw_session_request() {
        let router = Router::new().route(
            "/api/openclaw/launch",
            post(|Json(payload): Json<OpenClawSessionPlan>| async move {
                Json(json!({
                    "status": "launched",
                    "project": payload.request.project,
                    "objective": payload.request.objective,
                    "path": payload.openclaw_launch_path,
                }))
            }),
        );
        let base_url = spawn_mock_server(router).await;
        let client = OpenClawClient::new(base_url.clone());

        let response = client
            .launch_session(OpenClawSessionPlan {
                openclaw_url: base_url,
                openclaw_launch_path: "/api/openclaw/launch".to_string(),
                request: OpenClawSessionRequest {
                    project: "mission-control".into(),
                    objective: "Do useful work".into(),
                    profile: "default".into(),
                    workspace: None,
                    command: None,
                    needs_secrets: false,
                },
            })
            .await
            .unwrap();

        assert_eq!(response["status"], "launched");
        assert_eq!(response["project"], "mission-control");
        assert_eq!(response["path"], "/api/openclaw/launch");
    }

    #[tokio::test]
    async fn dispatches_secret_approval_request() {
        let router = Router::new().route(
            "/api/agentsecrets/approve",
            post(|Json(payload): Json<SecretApprovalPlan>| async move {
                Json(json!({
                    "status": "approved",
                    "secret_ref": payload.request.secret_ref,
                    "path": payload.agentsecrets_approval_path,
                }))
            }),
        );
        let base_url = spawn_mock_server(router).await;
        let client = AgentSecretsClient::new(base_url.clone());

        let response = client
            .request_approval(SecretApprovalPlan {
                agentsecrets_url: base_url,
                agentsecrets_approval_path: "/api/agentsecrets/approve".to_string(),
                request: SecretApprovalRequest {
                    session_id: "sess_123".into(),
                    secret_ref: "bw://login/github".into(),
                    action: "read".into(),
                    target: "github".into(),
                    reason: "CI deploy".into(),
                    ttl_seconds: 300,
                },
            })
            .await
            .unwrap();

        assert_eq!(response["status"], "approved");
        assert_eq!(response["secret_ref"], "bw://login/github");
        assert_eq!(response["path"], "/api/agentsecrets/approve");
    }

    #[tokio::test]
    async fn app_dispatches_via_configured_endpoints() {
        let openclaw_router = Router::new().route(
            "/api/openclaw/launch",
            post(|Json(payload): Json<OpenClawSessionPlan>| async move {
                Json(json!({
                    "kind": "openclaw",
                    "project": payload.request.project,
                }))
            }),
        );
        let secrets_router = Router::new().route(
            "/api/agentsecrets/approve",
            post(|Json(payload): Json<SecretApprovalPlan>| async move {
                Json(json!({
                    "kind": "agentsecrets",
                    "secret_ref": payload.request.secret_ref,
                }))
            }),
        );

        let openclaw_url = spawn_mock_server(openclaw_router).await;
        let secrets_url = spawn_mock_server(secrets_router).await;

        let config = AgentShellConfig {
            bind_addr: "127.0.0.1:0".into(),
            openclaw: OpenClawConfig {
                url: openclaw_url.clone(),
                launch_path: "/api/openclaw/launch".into(),
            },
            secrets: SecretBrokerConfig {
                url: secrets_url.clone(),
                approval_path: "/api/agentsecrets/approve".into(),
            },
            profile: "default".into(),
        };

        let app = crate::AgentShellApp::new(config);

        let launch = app
            .dispatch_openclaw_session(OpenClawSessionRequest {
                project: "mission-control".into(),
                objective: "Do useful work".into(),
                profile: "default".into(),
                workspace: None,
                command: None,
                needs_secrets: false,
            })
            .await
            .unwrap();
        assert_eq!(launch["kind"], "openclaw");

        let approval = app
            .dispatch_secret_approval(SecretApprovalRequest {
                session_id: "sess_123".into(),
                secret_ref: "bw://login/github".into(),
                action: "read".into(),
                target: "github".into(),
                reason: "CI deploy".into(),
                ttl_seconds: 300,
            })
            .await
            .unwrap();
        assert_eq!(approval["kind"], "agentsecrets");
    }
}

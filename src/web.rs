use axum::{routing::get, routing::post, Json, Router};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::{
    contracts::{OpenClawSessionRequest, SecretApprovalRequest},
    state::AppState,
    workflow::WorkflowPlanner,
};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
    profile: String,
    openclaw_url: String,
    secret_broker_url: String,
}

#[derive(Debug, Serialize)]
pub struct PlanResponse<T> {
    status: &'static str,
    plan: T,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/sessions/plan", post(plan_session))
        .route("/v1/approvals/plan", post(plan_secret_approval))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn healthz(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        profile: state.config.profile,
        openclaw_url: state.config.openclaw.url,
        secret_broker_url: state.config.secrets.url,
    })
}

async fn plan_session(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(request): Json<OpenClawSessionRequest>,
) -> Result<
    Json<PlanResponse<crate::contracts::OpenClawSessionPlan>>,
    (axum::http::StatusCode, String),
> {
    let planner = WorkflowPlanner::new(state.config.clone());
    let plan = planner
        .plan_openclaw_session(request)
        .map_err(|err| (axum::http::StatusCode::BAD_REQUEST, err.to_string()))?;

    Ok(Json(PlanResponse { status: "ok", plan }))
}

async fn plan_secret_approval(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(request): Json<SecretApprovalRequest>,
) -> Result<
    Json<PlanResponse<crate::contracts::SecretApprovalPlan>>,
    (axum::http::StatusCode, String),
> {
    let planner = WorkflowPlanner::new(state.config.clone());
    let plan = planner
        .plan_secret_approval(request)
        .map_err(|err| (axum::http::StatusCode::BAD_REQUEST, err.to_string()))?;

    Ok(Json(PlanResponse { status: "ok", plan }))
}

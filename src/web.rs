use axum::{routing::get, Json, Router};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
    profile: String,
    openclaw_url: String,
    secret_broker_url: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
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

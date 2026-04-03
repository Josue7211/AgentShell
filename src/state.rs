use crate::config::AgentShellConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AgentShellConfig,
}

impl AppState {
    pub fn new(config: AgentShellConfig) -> Self {
        Self { config }
    }
}

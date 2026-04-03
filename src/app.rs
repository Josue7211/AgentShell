use crate::{config::AgentShellConfig, state::AppState};

#[derive(Debug, Clone)]
pub struct AgentShellApp {
    pub state: AppState,
}

impl AgentShellApp {
    pub fn new(config: AgentShellConfig) -> Self {
        Self {
            state: AppState::new(config),
        }
    }

    pub fn config(&self) -> &AgentShellConfig {
        &self.state.config
    }
}

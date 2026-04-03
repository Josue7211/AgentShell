use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentShellConfig {
    pub bind_addr: String,
    pub openclaw: OpenClawConfig,
    pub secrets: SecretBrokerConfig,
    pub profile: String,
}

impl AgentShellConfig {
    pub fn from_env() -> Self {
        Self {
            bind_addr: std::env::var("AGENTSHELL_BIND_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:8077".to_string()),
            openclaw: OpenClawConfig::from_env(),
            secrets: SecretBrokerConfig::from_env(),
            profile: std::env::var("AGENTSHELL_PROFILE").unwrap_or_else(|_| "default".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawConfig {
    pub url: String,
}

impl OpenClawConfig {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("OPENCLAW_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretBrokerConfig {
    pub url: String,
}

impl SecretBrokerConfig {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("AGENTSECRETS_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
        }
    }
}

use std::net::SocketAddr;

use agent_shell::{
    config::AgentShellConfig,
    contracts::{OpenClawSessionRequest, SecretApprovalRequest},
    state::AppState,
    web,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "agent-shell",
    about = "AgentShell: a thin Rust host layer for OpenClaw"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print the effective configuration.
    Config,
    /// Print an OpenClaw session launch plan.
    PlanSession {
        #[arg(long)]
        project: String,
        #[arg(long)]
        objective: String,
        #[arg(long)]
        command: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        needs_secrets: bool,
    },
    /// Print an AgentSecrets approval plan.
    PlanApproval {
        #[arg(long)]
        session_id: String,
        #[arg(long)]
        secret_ref: String,
        #[arg(long)]
        action: String,
        #[arg(long)]
        target: String,
        #[arg(long)]
        reason: String,
        #[arg(long, default_value_t = 300)]
        ttl_seconds: u64,
    },
    /// Run the local host service.
    Serve,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = AgentShellConfig::from_env();

    match cli.command {
        Commands::Config => {
            println!("{:#?}", config);
        }
        Commands::PlanSession {
            project,
            objective,
            command,
            workspace,
            needs_secrets,
        } => {
            let app = agent_shell::AgentShellApp::new(config);
            let plan = app.plan_openclaw_session(OpenClawSessionRequest {
                project,
                objective,
                profile: app.config().profile.clone(),
                workspace,
                command,
                needs_secrets,
            })?;
            println!("{}", serde_json::to_string_pretty(&plan)?);
        }
        Commands::PlanApproval {
            session_id,
            secret_ref,
            action,
            target,
            reason,
            ttl_seconds,
        } => {
            let app = agent_shell::AgentShellApp::new(config);
            let plan = app.plan_secret_approval(SecretApprovalRequest {
                session_id,
                secret_ref,
                action,
                target,
                reason,
                ttl_seconds,
            })?;
            println!("{}", serde_json::to_string_pretty(&plan)?);
        }
        Commands::Serve => {
            let addr: SocketAddr = config.bind_addr.parse()?;
            let state = AppState::new(config);
            let listener = tokio::net::TcpListener::bind(addr).await?;
            let app = web::router(state);

            println!("AgentShell listening on http://{}", addr);
            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}

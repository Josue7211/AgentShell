use std::net::SocketAddr;

use agent_shell::{config::AgentShellConfig, state::AppState, web};
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

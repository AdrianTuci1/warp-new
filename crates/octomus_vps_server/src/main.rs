use std::sync::Arc;

use clap::Parser;

use octomus_vps_server::{AppState, run_server};

#[derive(Parser, Debug)]
#[command(name = "octomus-vps-server")]
struct Args {
    #[arg(long, default_value_t = 9000)]
    port: u16,
    #[arg(long, env = "OCTOMUS_VPS_PAIRING_CODE")]
    pairing_code: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let state = Arc::new(AppState::new(args.pairing_code)?);
    // Try to load existing config/secrets on startup so we can survive restarts.
    if let Ok(Some(config)) = octomus_vps_server::load_config_from_disk(&state) {
        *state.config.lock().unwrap() = Some(config);
    }
    if let Ok(secrets) = octomus_vps_server::load_secrets_from_disk(&state) {
        *state.secrets.lock().unwrap() = secrets;
    }
    // Check if a CLI is already present from a previous run.
    let cli_path = state.cli_path();
    if cli_path.exists() {
        *state.cli_path.lock().unwrap() = Some(cli_path);
    }
    run_server(state, args.port).await
}

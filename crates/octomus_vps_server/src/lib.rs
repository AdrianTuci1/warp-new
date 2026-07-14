pub mod api;
mod config;
mod crypto;
mod download;
mod env;
mod pairing;
mod server;
mod state;
mod tasks;

pub use api::{
    AgentRunRequest, AgentRunResponse, AgentScheduleRequest, CliDownloadRequest,
    ConfigUploadRequest, PairingRequest, PairingResponse, SecretUploadRequest, ServerStatus,
};
pub use config::VpsConfig;
pub use crypto::{decrypt, derive_pairing_key, encrypt};
pub use pairing::{
    load_encrypted_config as load_config_from_disk,
    load_encrypted_secrets as load_secrets_from_disk,
};
pub use server::run_server;
pub use state::{AppState, RunningTask, SecretStore};

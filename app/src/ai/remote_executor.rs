// Octomus — generic remote executor for "Run Remote".
// Supports SSH, Modal, and custom HTTP endpoints configured via settings.

use std::path::PathBuf;
use std::sync::Arc;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};

/// The backend type configured in settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RemoteBackendType {
    /// Execute via SSH on a user-provided VPS.
    Ssh,
    /// Execute via Modal API (modal.com).
    Modal,
    /// Custom HTTP endpoint (user runs their own orchestrator).
    Custom,
}

/// Configuration for a remote backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteBackendConfig {
    pub backend_type: RemoteBackendType,
    /// SSH: "user@host:22", Modal: "modal", Custom: "https://..."
    pub host: String,
    /// SSH private key path, Modal token, or Custom API key
    pub credential: String,
    /// Optional: docker image, env vars, timeout
    pub image: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub extra_env: Vec<(String, String)>,
}

/// A single line of output from the remote execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemoteOutput {
    Stdout(String),
    Stderr(String),
    Exit(i32),
    Error(String),
}

/// Prep step: what to copy and run remotely.
#[derive(Debug, Clone)]
pub struct RemoteTask {
    pub files: Vec<PathBuf>,
    pub command: String,
    pub env: Vec<(String, String)>,
    pub remote_work_dir: String,
}

/// The trait that every backend implements.
#[async_trait::async_trait]
pub trait RemoteExecutor: Send + Sync {
    async fn test_connection(&self, config: &RemoteBackendConfig) -> Result<(), String>;
    async fn execute(
        &self,
        config: &RemoteBackendConfig,
        task: RemoteTask,
    ) -> Result<Box<dyn Stream<Item = RemoteOutput> + Send + Unpin>, String>;
}

/// Factory: build the right executor for a given backend type.
pub fn create_executor(backend_type: &RemoteBackendType) -> Box<dyn RemoteExecutor> {
    match backend_type {
        RemoteBackendType::Ssh => Box::new(ssh::SshExecutor::new()),
        RemoteBackendType::Modal => Box::new(modal::ModalExecutor::new()),
        RemoteBackendType::Custom => Box::new(custom::CustomExecutor::new()),
    }
}

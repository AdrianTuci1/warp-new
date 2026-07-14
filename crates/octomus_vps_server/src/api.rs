use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub paired: bool,
    pub version: String,
    pub cli_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliDownloadRequest {
    pub url: String,
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingRequest {
    pub challenge_response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingResponse {
    pub paired: bool,
    pub server_challenge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUploadRequest {
    pub encrypted_payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretUploadRequest {
    pub encrypted_payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRunRequest {
    pub prompt: String,
    /// Optional working directory on the VPS.
    pub cwd: Option<String>,
    /// Optional harness override. Defaults to oz.
    pub harness: Option<String>,
    /// Optional model override.
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRunResponse {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentScheduleRequest {
    pub cron_expression: String,
    pub run: AgentRunRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentLogResponse {
    pub task_id: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentListResponse {
    pub tasks: Vec<AgentTaskSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskSummary {
    pub task_id: String,
    pub prompt: String,
    pub status: String,
    pub started_at: String,
    pub pid: Option<u32>,
}

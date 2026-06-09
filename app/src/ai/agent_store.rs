// Octomus — local agent state store for multi-agent orchestration.
// Replaces cloud-dependent SSE event streaming, artifact reporting,
// and snapshot upload with a local JSONL-based directory.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

const STORE_BASE: &str = ".octomus/agent-store";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentStoreEvent {
    Lifecycle {
        agent: String,
        stage: String,
        timestamp: i64,
    },
    Message {
        from_agent: String,
        to_agent: String,
        message_type: String,
        payload: Value,
        timestamp: i64,
    },
    Artifact {
        agent: String,
        artifact_type: String,
        data: Value,
        timestamp: i64,
    },
    Error {
        agent: String,
        error: String,
        timestamp: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub status: String, // "running" | "succeeded" | "failed"
    pub summary: Option<String>,
    pub exit_code: Option<i32>,
}

pub struct AgentStore {
    store_root: PathBuf,
    conversation_id: String,
}

impl AgentStore {
    pub fn new(conversation_id: impl Into<String>) -> Self {
        let conversation_id = conversation_id.into();
        let store_root = Self::dirs_next()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(STORE_BASE)
            .join(&conversation_id);
        fs::create_dir_all(&store_root).ok();
        Self {
            store_root,
            conversation_id,
        }
    }

    fn dirs_next() -> Option<PathBuf> {
        std::env::var("OCTOMUS_AGENT_STORE_DIR")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::home_dir)
    }

    pub fn events_path(&self) -> PathBuf {
        self.store_root.join("events.jsonl")
    }

    pub fn agent_dir(&self, agent_name: &str) -> PathBuf {
        self.store_root.join(agent_name)
    }

    pub fn write_event(&self, event: AgentStoreEvent) -> std::io::Result<()> {
        let path = self.events_path();
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        let line = serde_json::to_string(&event)?;
        writeln!(f, "{}", line)?;
        Ok(())
    }

    pub fn poll_events(&self, since_line: usize) -> Vec<AgentStoreEvent> {
        let path = self.events_path();
        if !path.exists() {
            return vec![];
        }
        let f = match fs::File::open(&path) {
            Ok(f) => f,
            Err(_) => return vec![],
        };
        let reader = BufReader::new(f);
        reader
            .lines()
            .skip(since_line)
            .filter_map(|l| l.ok())
            .filter_map(|l| serde_json::from_str(&l).ok())
            .collect()
    }

    pub fn update_status(&self, agent_name: &str, status: AgentStatus) -> std::io::Result<()> {
        let dir = self.agent_dir(agent_name);
        fs::create_dir_all(&dir)?;
        let path = dir.join("status.json");
        let json = serde_json::to_string_pretty(&status)?;
        fs::write(&path, json)?;
        Ok(())
    }

    pub fn read_status(&self, agent_name: &str) -> Option<AgentStatus> {
        let path = self.agent_dir(agent_name).join("status.json");
        let json = fs::read_to_string(&path).ok()?;
        serde_json::from_str(&json).ok()
    }

    pub fn store_artifact(&self, agent_name: &str, name: &str, data: &str) -> std::io::Result<()> {
        let dir = self.agent_dir(agent_name).join("artifacts");
        fs::create_dir_all(&dir)?;
        fs::write(dir.join(name), data)?;
        Ok(())
    }

    pub fn snapshot_dir(&self, agent_name: &str) -> PathBuf {
        self.agent_dir(agent_name).join("snapshot")
    }

    /// Clean up stores older than 7 days.
    pub fn cleanup_old_stores() {
        let base = Self::dirs_next()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(STORE_BASE);
        if !base.exists() {
            return;
        }
        let cutoff =
            std::time::SystemTime::now().checked_sub(std::time::Duration::from_secs(7 * 86400));
        let Some(cutoff) = cutoff else { return };
        if let Ok(entries) = fs::read_dir(&base) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        if modified < cutoff {
                            let _ = fs::remove_dir_all(entry.path());
                        }
                    }
                }
            }
        }
    }

    /// List all agent names for this conversation.
    pub fn list_agents(&self) -> Vec<String> {
        let mut agents = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.store_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name != "artifacts" {
                            agents.push(name.to_string());
                        }
                    }
                }
            }
        }
        agents
    }

    /// Total line count in events.jsonl — used to know "since" for polling.
    pub fn event_count(&self) -> usize {
        let path = self.events_path();
        if !path.exists() {
            return 0;
        }
        fs::read_to_string(&path)
            .map(|s| s.lines().count())
            .unwrap_or(0)
    }
}

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Child;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::VpsConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretStore {
    pub secrets: HashMap<String, String>,
}

impl Default for SecretStore {
    fn default() -> Self {
        Self {
            secrets: HashMap::new(),
        }
    }
}

impl SecretStore {
    pub fn is_empty(&self) -> bool {
        self.secrets.is_empty()
    }
}

#[derive(Debug)]
pub struct RunningTask {
    pub task_id: String,
    pub prompt: String,
    pub started_at: String,
    pub child: Arc<Mutex<Child>>,
    pub log_dir: PathBuf,
}

pub struct AppState {
    pub pairing_key: [u8; 32],
    pub config: Mutex<Option<VpsConfig>>,
    pub secrets: Mutex<SecretStore>,
    pub cli_path: Mutex<Option<PathBuf>>,
    pub data_dir: PathBuf,
    pub tasks: Mutex<HashMap<String, RunningTask>>,
}

impl AppState {
    pub fn new(pairing_code: String) -> Result<Self> {
        let data_dir = Self::data_dir()?;
        fs::create_dir_all(&data_dir).context("failed to create data directory")?;
        fs::create_dir_all(data_dir.join("tasks")).ok();
        Ok(Self {
            pairing_key: crate::crypto::derive_pairing_key(&pairing_code),
            config: Mutex::new(None),
            secrets: Mutex::new(SecretStore::default()),
            cli_path: Mutex::new(None),
            data_dir,
            tasks: Mutex::new(HashMap::new()),
        })
    }

    pub fn data_dir() -> Result<PathBuf> {
        directories::BaseDirs::new()
            .map(|b| b.data_dir().join("octomus-vps"))
            .context("could not determine base directories")
    }

    pub fn new_task_id() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn task_log_dir(&self, task_id: &str) -> PathBuf {
        self.data_dir.join("tasks").join(task_id)
    }

    pub fn config_path(&self) -> PathBuf {
        self.data_dir.join("config.json.enc")
    }

    pub fn secrets_path(&self) -> PathBuf {
        self.data_dir.join("secrets.json.enc")
    }

    pub fn cli_path(&self) -> PathBuf {
        self.data_dir.join("octomus")
    }
}

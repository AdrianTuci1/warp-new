use serde::{Deserialize, Serialize};
use warpui_core::{Entity, ModelContext, SingletonEntity};
use warpui_extras::secure_storage::{self, AppContextExt};

const SECURE_STORAGE_KEY: &str = "CloudCredentials";

/// Emitted when cloud credentials are updated in-memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudCredentialsEvent {
    CredentialsUpdated,
}

/// Platform type for cloud credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudPlatform {
    Modal,
    Vps,
}

impl Default for CloudPlatform {
    fn default() -> Self {
        CloudPlatform::Modal
    }
}

impl CloudPlatform {
    pub fn label(&self) -> &'static str {
        match self {
            CloudPlatform::Modal => "Modal",
            CloudPlatform::Vps => "VPS",
        }
    }
}

/// A single cloud credential entry.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CloudCredentialEntry {
    pub id: String,
    pub platform: CloudPlatform,
    /// Display name / label for this entry
    pub name: Option<String>,
    /// Modal API key or VPS host
    pub host_or_key: Option<String>,
    /// VPS username (only used for VPS platform)
    pub vps_username: Option<String>,
    /// VPS SSH private key (only used for VPS platform)
    pub vps_ssh_key: Option<String>,
}

/// User-provided credentials for cloud platforms.
/// Used to launch Cloud Agents or Subagents on VPS or Modal.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CloudCredentials {
    pub entries: Vec<CloudCredentialEntry>,
}

impl CloudCredentials {
    pub fn entries(&self) -> &[CloudCredentialEntry] {
        &self.entries
    }

    pub fn modal_entries(&self) -> impl Iterator<Item = &CloudCredentialEntry> {
        self.entries
            .iter()
            .filter(|e| e.platform == CloudPlatform::Modal)
    }

    pub fn vps_entries(&self) -> impl Iterator<Item = &CloudCredentialEntry> {
        self.entries
            .iter()
            .filter(|e| e.platform == CloudPlatform::Vps)
    }
}

/// Manages cloud credentials in secure storage.
pub struct CloudCredentialsManager {
    credentials: CloudCredentials,
    secure_storage_write_version: u64,
}

impl CloudCredentialsManager {
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        let credentials = Self::load_from_secure_storage(ctx);
        Self {
            credentials,
            secure_storage_write_version: 0,
        }
    }

    pub fn credentials(&self) -> &CloudCredentials {
        &self.credentials
    }

    pub fn add_entry(&mut self, entry: CloudCredentialEntry, ctx: &mut ModelContext<Self>) {
        self.credentials.entries.push(entry);
        ctx.emit(CloudCredentialsEvent::CredentialsUpdated);
        self.write_to_secure_storage(ctx);
    }

    pub fn remove_entry(&mut self, id: &str, ctx: &mut ModelContext<Self>) {
        self.credentials.entries.retain(|e| e.id != id);
        ctx.emit(CloudCredentialsEvent::CredentialsUpdated);
        self.write_to_secure_storage(ctx);
    }

    pub fn update_entry(
        &mut self,
        id: &str,
        f: impl FnOnce(&mut CloudCredentialEntry),
        ctx: &mut ModelContext<Self>,
    ) {
        if let Some(entry) = self.credentials.entries.iter_mut().find(|e| e.id == id) {
            f(entry);
            ctx.emit(CloudCredentialsEvent::CredentialsUpdated);
            self.write_to_secure_storage(ctx);
        }
    }

    pub fn set_entries(
        &mut self,
        entries: Vec<CloudCredentialEntry>,
        ctx: &mut ModelContext<Self>,
    ) {
        self.credentials.entries = entries;
        ctx.emit(CloudCredentialsEvent::CredentialsUpdated);
        self.write_to_secure_storage(ctx);
    }

    fn load_from_secure_storage(ctx: &mut ModelContext<Self>) -> CloudCredentials {
        let key_json = match ctx.secure_storage().read_value(SECURE_STORAGE_KEY) {
            Ok(json) => json,
            Err(e) => {
                if !matches!(e, secure_storage::Error::NotFound) {
                    log::error!("Failed to read cloud credentials from secure storage: {e:#}");
                }
                return CloudCredentials::default();
            }
        };

        match serde_json::from_str(&key_json) {
            Ok(creds) => creds,
            Err(e) => {
                log::error!("Failed to deserialize cloud credentials: {e:#}");
                CloudCredentials::default()
            }
        }
    }

    fn write_to_secure_storage(&mut self, ctx: &mut ModelContext<Self>) {
        let json = match serde_json::to_string(&self.credentials) {
            Ok(json) => json,
            Err(e) => {
                log::error!("Failed to serialize cloud credentials: {e:#}");
                return;
            }
        };
        self.secure_storage_write_version += 1;
        let write_version = self.secure_storage_write_version;

        ctx.spawn(async move { json }, move |me, json, ctx| {
            if write_version != me.secure_storage_write_version {
                return;
            }
            if let Err(e) = ctx.secure_storage().write_value(SECURE_STORAGE_KEY, &json) {
                log::error!("Failed to write cloud credentials to secure storage: {e:#}");
            }
        });
    }
}

impl Entity for CloudCredentialsManager {
    type Event = CloudCredentialsEvent;
}

impl SingletonEntity for CloudCredentialsManager {}

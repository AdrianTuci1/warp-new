use serde::{Deserialize, Serialize};
use warpui_core::{Entity, ModelContext, SingletonEntity};
use warpui_extras::secure_storage::{self, AppContextExt};

const SECURE_STORAGE_KEY: &str = "CloudCredentials";

/// Emitted when cloud credentials are updated in-memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudCredentialsEvent {
    CredentialsUpdated,
}

/// User-provided credentials for cloud platforms.
/// Used to launch Cloud Agents or Subagents on VPS or Modal.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CloudCredentials {
    /// Modal API key for launching subagents on Modal platform
    pub modal_api_key: Option<String>,
    /// VPS IP address or hostname
    pub vps_host: Option<String>,
    /// VPS SSH private key or password
    pub vps_ssh_key: Option<String>,
    /// VPS username for SSH connection
    pub vps_username: Option<String>,
}

impl CloudCredentials {
    pub fn has_modal_key(&self) -> bool {
        self.modal_api_key.as_ref().is_some_and(|k| !k.trim().is_empty())
    }

    pub fn has_vps_credentials(&self) -> bool {
        self.vps_host.as_ref().is_some_and(|h| !h.trim().is_empty())
            && self.vps_ssh_key.as_ref().is_some_and(|k| !k.trim().is_empty())
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

    pub fn set_modal_api_key(&mut self, key: Option<String>, ctx: &mut ModelContext<Self>) {
        self.credentials.modal_api_key = key;
        ctx.emit(CloudCredentialsEvent::CredentialsUpdated);
        self.write_to_secure_storage(ctx);
    }

    pub fn set_vps_host(&mut self, host: Option<String>, ctx: &mut ModelContext<Self>) {
        self.credentials.vps_host = host;
        ctx.emit(CloudCredentialsEvent::CredentialsUpdated);
        self.write_to_secure_storage(ctx);
    }

    pub fn set_vps_ssh_key(&mut self, key: Option<String>, ctx: &mut ModelContext<Self>) {
        self.credentials.vps_ssh_key = key;
        ctx.emit(CloudCredentialsEvent::CredentialsUpdated);
        self.write_to_secure_storage(ctx);
    }

    pub fn set_vps_username(&mut self, username: Option<String>, ctx: &mut ModelContext<Self>) {
        self.credentials.vps_username = username;
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

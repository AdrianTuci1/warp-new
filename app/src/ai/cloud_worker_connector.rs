use std::collections::HashMap;

use ::ai::cloud_credentials::{
    CloudCredentialEntry, CloudCredentialsEvent, CloudCredentialsManager, CloudPlatform,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use warpui::{Entity, ModelContext, SingletonEntity};
use warpui_extras::secure_storage::{self, AppContextExt};

use crate::ai::agent::conversation::AIConversationId;
use crate::ai::ambient_agents::{AmbientAgentTask, AmbientAgentTaskId, AmbientAgentTaskState};
use crate::server::server_api::ai::SpawnAgentRequest;

const SECURE_STORAGE_KEY: &str = "CloudWorkerExecutionStore";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudWorkerConnectorEvent {
    ManagedHostsChanged,
    ExecutionRecordsChanged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedCloudWorkerHost {
    pub credential_entry_id: String,
    pub worker_host: String,
    pub platform: CloudPlatform,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vps_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vps_username: Option<String>,
}

impl ManagedCloudWorkerHost {
    fn from_entry(entry: &CloudCredentialEntry) -> Option<Self> {
        let worker_host = entry.worker_host_slug()?;
        Some(Self {
            credential_entry_id: entry.id.clone(),
            worker_host,
            platform: entry.platform,
            label: entry.display_label(),
            vps_host: (entry.platform == CloudPlatform::Vps)
                .then(|| {
                    entry
                        .host_or_key
                        .as_deref()
                        .map(str::trim)
                        .filter(|host| !host.is_empty())
                })
                .flatten()
                .map(ToOwned::to_owned),
            vps_username: (entry.platform == CloudPlatform::Vps)
                .then(|| {
                    entry
                        .vps_username
                        .as_deref()
                        .map(str::trim)
                        .filter(|username| !username.is_empty())
                })
                .flatten()
                .map(ToOwned::to_owned),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloudWorkerPersistencePaths {
    pub state_root: String,
    pub manifest_path: String,
    pub transcript_path: String,
    pub stdout_path: String,
    pub stderr_path: String,
    pub events_path: String,
    pub task_snapshot_path: String,
    pub artifacts_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloudWorkerRestoreCursor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_event_sequence: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_link: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_state: Option<AmbientAgentTaskState>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VpsBootstrapPlan {
    pub remote_state_root: String,
    pub launch_script_path: String,
    pub detached_launch_command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalBootstrapPlan {
    pub sandbox_label: String,
    pub persistent_filesystem_mount: String,
    pub entrypoint_command: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "platform", rename_all = "snake_case")]
pub enum CloudWorkerBootstrapPlan {
    Vps(VpsBootstrapPlan),
    Modal(ModalBootstrapPlan),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloudWorkerExecutionRecord {
    pub run_id: String,
    pub task_id: String,
    pub worker_host: String,
    pub credential_entry_id: String,
    pub platform: CloudPlatform,
    pub host_label: String,
    pub persistence: CloudWorkerPersistencePaths,
    pub bootstrap: CloudWorkerBootstrapPlan,
    pub restore: CloudWorkerRestoreCursor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_conversation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harness_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_from_task_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct CloudWorkerExecutionStore {
    records_by_run_id: HashMap<String, CloudWorkerExecutionRecord>,
}

pub struct CloudWorkerConnectorModel {
    managed_hosts: Vec<ManagedCloudWorkerHost>,
    store: CloudWorkerExecutionStore,
    secure_storage_write_version: u64,
}

impl CloudWorkerConnectorModel {
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        let credentials_handle = CloudCredentialsManager::handle(ctx);
        ctx.subscribe_to_model(&credentials_handle, |me, event, ctx| {
            if matches!(event, CloudCredentialsEvent::CredentialsUpdated) {
                me.refresh_managed_hosts(ctx);
            }
        });

        let mut me = Self {
            managed_hosts: Vec::new(),
            store: Self::load_store_from_secure_storage(ctx),
            secure_storage_write_version: 0,
        };
        me.refresh_managed_hosts(ctx);
        me
    }

    pub fn managed_worker_hosts(&self) -> &[ManagedCloudWorkerHost] {
        &self.managed_hosts
    }

    pub fn managed_worker_host_slugs(&self) -> Vec<String> {
        self.managed_hosts
            .iter()
            .map(|host| host.worker_host.clone())
            .collect()
    }

    pub fn execution_record_for_run_id(&self, run_id: &str) -> Option<&CloudWorkerExecutionRecord> {
        self.store.records_by_run_id.get(run_id)
    }

    pub fn record_spawn_for_request(
        &mut self,
        request: &SpawnAgentRequest,
        task_id: AmbientAgentTaskId,
        run_id: &str,
        local_conversation_id: Option<AIConversationId>,
        ctx: &mut ModelContext<Self>,
    ) {
        let Some(config) = request.config.as_ref() else {
            return;
        };
        let Some(worker_host) = config.worker_host.as_deref() else {
            return;
        };
        let Some(host) = self.host_for_worker_host(worker_host).cloned() else {
            return;
        };

        let now = Utc::now();
        let task_id_string = task_id.to_string();
        let record = CloudWorkerExecutionRecord {
            run_id: run_id.to_string(),
            task_id: task_id_string.clone(),
            worker_host: host.worker_host.clone(),
            credential_entry_id: host.credential_entry_id.clone(),
            platform: host.platform,
            host_label: host.label.clone(),
            persistence: build_persistence_paths(&host, run_id),
            bootstrap: build_bootstrap_plan(&host, &task_id_string, run_id),
            restore: CloudWorkerRestoreCursor {
                conversation_id: request.conversation_id.clone(),
                last_event_sequence: None,
                session_id: None,
                session_link: None,
                children: Vec::new(),
                parent_run_id: request.parent_run_id.clone(),
                task_state: Some(AmbientAgentTaskState::Queued),
                updated_at: now,
            },
            local_conversation_id: local_conversation_id.map(|id| id.to_string()),
            environment_id: config.environment_id.clone(),
            harness_type: config
                .harness
                .as_ref()
                .map(|harness| harness.harness_type.config_name().to_string()),
            updated_from_task_at: None,
            created_at: now,
        };

        self.store
            .records_by_run_id
            .insert(run_id.to_string(), record);
        ctx.emit(CloudWorkerConnectorEvent::ExecutionRecordsChanged);
        self.write_store_to_secure_storage(ctx);
    }

    pub fn attach_local_conversation_id(
        &mut self,
        run_id: &str,
        conversation_id: AIConversationId,
        ctx: &mut ModelContext<Self>,
    ) {
        let Some(record) = self.record_mut_by_run_or_task(run_id) else {
            return;
        };
        let local_conversation_id = conversation_id.to_string();
        if record.local_conversation_id.as_deref() == Some(local_conversation_id.as_str()) {
            return;
        }
        record.local_conversation_id = Some(local_conversation_id);
        ctx.emit(CloudWorkerConnectorEvent::ExecutionRecordsChanged);
        self.write_store_to_secure_storage(ctx);
    }

    pub fn record_session_started(
        &mut self,
        run_id: &str,
        session_id: Option<String>,
        session_link: Option<String>,
        ctx: &mut ModelContext<Self>,
    ) {
        let Some(record) = self.record_mut_by_run_or_task(run_id) else {
            return;
        };
        record.restore.session_id = session_id;
        record.restore.session_link = session_link;
        record.restore.task_state = Some(AmbientAgentTaskState::InProgress);
        record.restore.updated_at = Utc::now();
        ctx.emit(CloudWorkerConnectorEvent::ExecutionRecordsChanged);
        self.write_store_to_secure_storage(ctx);
    }

    pub fn restore_record_from_task(
        &mut self,
        task: &AmbientAgentTask,
        ctx: &mut ModelContext<Self>,
    ) {
        let Some(config) = task.agent_config_snapshot.as_ref() else {
            return;
        };
        let Some(worker_host) = config.worker_host.as_deref() else {
            return;
        };
        let Some(host) = self.host_for_worker_host(worker_host).cloned() else {
            return;
        };

        let run_id = task.run_id().to_string();
        let now = Utc::now();
        let task_id = task.task_id.to_string();
        let session_id = task
            .active_run_execution()
            .session_id
            .map(ToOwned::to_owned);
        let session_link = task
            .active_run_execution()
            .session_link
            .map(ToOwned::to_owned);

        let record = self
            .store
            .records_by_run_id
            .entry(run_id.clone())
            .or_insert_with(|| CloudWorkerExecutionRecord {
                run_id: run_id.clone(),
                task_id: task_id.clone(),
                worker_host: host.worker_host.clone(),
                credential_entry_id: host.credential_entry_id.clone(),
                platform: host.platform,
                host_label: host.label.clone(),
                persistence: build_persistence_paths(&host, &run_id),
                bootstrap: build_bootstrap_plan(&host, &task_id, &run_id),
                restore: CloudWorkerRestoreCursor {
                    conversation_id: task.conversation_id().map(ToOwned::to_owned),
                    last_event_sequence: task.last_event_sequence,
                    session_id: session_id.clone(),
                    session_link: session_link.clone(),
                    children: task.children.clone(),
                    parent_run_id: task.parent_run_id.clone(),
                    task_state: Some(task.state.clone()),
                    updated_at: now,
                },
                local_conversation_id: None,
                environment_id: config.environment_id.clone(),
                harness_type: config
                    .harness
                    .as_ref()
                    .map(|harness| harness.harness_type.config_name().to_string()),
                updated_from_task_at: Some(now),
                created_at: now,
            });

        record.task_id = task_id;
        let worker_host = host.worker_host.clone();
        record.worker_host = worker_host;
        let credential_entry_id = host.credential_entry_id.clone();
        record.credential_entry_id = credential_entry_id;
        record.platform = host.platform;
        let host_label = host.label.clone();
        record.host_label = host_label;
        record.environment_id = config.environment_id.clone();
        record.harness_type = config
            .harness
            .as_ref()
            .map(|harness| harness.harness_type.config_name().to_string());
        record.restore.conversation_id = task.conversation_id().map(ToOwned::to_owned);
        record.restore.last_event_sequence = task.last_event_sequence;
        record.restore.session_id = session_id;
        record.restore.session_link = session_link;
        record.restore.children = task.children.clone();
        record.restore.parent_run_id = task.parent_run_id.clone();
        record.restore.task_state = Some(task.state.clone());
        record.restore.updated_at = now;
        record.updated_from_task_at = Some(now);
        record.persistence = build_persistence_paths(&host, &run_id);
        record.bootstrap = build_bootstrap_plan(&host, &record.task_id, &run_id);

        ctx.emit(CloudWorkerConnectorEvent::ExecutionRecordsChanged);
        self.write_store_to_secure_storage(ctx);
    }

    fn refresh_managed_hosts(&mut self, ctx: &mut ModelContext<Self>) {
        let mut next_hosts = CloudCredentialsManager::as_ref(ctx)
            .credentials()
            .entries()
            .iter()
            .filter_map(ManagedCloudWorkerHost::from_entry)
            .collect::<Vec<_>>();
        next_hosts.sort_by(|left, right| left.worker_host.cmp(&right.worker_host));
        next_hosts.dedup_by(|left, right| left.worker_host == right.worker_host);

        if next_hosts == self.managed_hosts {
            return;
        }

        self.managed_hosts = next_hosts;
        self.store.records_by_run_id.retain(|_, record| {
            self.managed_hosts
                .iter()
                .any(|host| host.worker_host == record.worker_host)
        });
        ctx.emit(CloudWorkerConnectorEvent::ManagedHostsChanged);
        ctx.emit(CloudWorkerConnectorEvent::ExecutionRecordsChanged);
        self.write_store_to_secure_storage(ctx);
    }

    fn host_for_worker_host(&self, worker_host: &str) -> Option<&ManagedCloudWorkerHost> {
        self.managed_hosts
            .iter()
            .find(|host| host.worker_host == worker_host)
    }

    fn record_mut_by_run_or_task(
        &mut self,
        run_or_task_id: &str,
    ) -> Option<&mut CloudWorkerExecutionRecord> {
        if self.store.records_by_run_id.contains_key(run_or_task_id) {
            return self.store.records_by_run_id.get_mut(run_or_task_id);
        }
        self.store
            .records_by_run_id
            .values_mut()
            .find(|record| record.task_id == run_or_task_id)
    }

    fn load_store_from_secure_storage(ctx: &mut ModelContext<Self>) -> CloudWorkerExecutionStore {
        let key_json = match ctx.secure_storage().read_value(SECURE_STORAGE_KEY) {
            Ok(json) => json,
            Err(err) => {
                if !matches!(err, secure_storage::Error::NotFound) {
                    log::error!(
                        "Failed to read cloud worker execution store from secure storage: {err:#}"
                    );
                }
                return CloudWorkerExecutionStore::default();
            }
        };

        match serde_json::from_str(&key_json) {
            Ok(store) => store,
            Err(err) => {
                log::error!("Failed to deserialize cloud worker execution store: {err:#}");
                CloudWorkerExecutionStore::default()
            }
        }
    }

    fn write_store_to_secure_storage(&mut self, ctx: &mut ModelContext<Self>) {
        let json = match serde_json::to_string(&self.store) {
            Ok(json) => json,
            Err(err) => {
                log::error!("Failed to serialize cloud worker execution store: {err:#}");
                return;
            }
        };
        self.secure_storage_write_version += 1;
        let write_version = self.secure_storage_write_version;

        ctx.spawn(async move { json }, move |me, json, ctx| {
            if write_version != me.secure_storage_write_version {
                return;
            }
            if let Err(err) = ctx.secure_storage().write_value(SECURE_STORAGE_KEY, &json) {
                log::error!(
                    "Failed to write cloud worker execution store to secure storage: {err:#}"
                );
            }
        });
    }
}

fn build_persistence_paths(
    host: &ManagedCloudWorkerHost,
    run_id: &str,
) -> CloudWorkerPersistencePaths {
    let state_root = persistent_state_root(host, run_id);
    CloudWorkerPersistencePaths {
        manifest_path: format!("{state_root}/manifest.json"),
        transcript_path: format!("{state_root}/transcript.jsonl"),
        stdout_path: format!("{state_root}/stdout.log"),
        stderr_path: format!("{state_root}/stderr.log"),
        events_path: format!("{state_root}/events.jsonl"),
        task_snapshot_path: format!("{state_root}/task.json"),
        artifacts_dir: format!("{state_root}/artifacts"),
        state_root,
    }
}

fn build_bootstrap_plan(
    host: &ManagedCloudWorkerHost,
    task_id: &str,
    run_id: &str,
) -> CloudWorkerBootstrapPlan {
    let state_root = persistent_state_root(host, run_id);
    match host.platform {
        CloudPlatform::Vps => CloudWorkerBootstrapPlan::Vps(VpsBootstrapPlan {
            remote_state_root: state_root.clone(),
            launch_script_path: format!("{state_root}/launch-worker.sh"),
            detached_launch_command: format!(
                "mkdir -p '{state_root}' && nohup warp agent run --task-id {task_id} \
                 > '{state_root}/stdout.log' 2> '{state_root}/stderr.log' < /dev/null &"
            ),
            host: host.vps_host.clone(),
            username: host.vps_username.clone(),
        }),
        CloudPlatform::Modal => CloudWorkerBootstrapPlan::Modal(ModalBootstrapPlan {
            sandbox_label: format!("warp-agent-{run_id}"),
            persistent_filesystem_mount: "/vol/warp".to_string(),
            entrypoint_command: vec![
                "warp".to_string(),
                "agent".to_string(),
                "run".to_string(),
                "--task-id".to_string(),
                task_id.to_string(),
            ],
        }),
    }
}

fn persistent_state_root(host: &ManagedCloudWorkerHost, run_id: &str) -> String {
    match host.platform {
        CloudPlatform::Vps => format!("$HOME/.warp/cloud-agent/runs/{run_id}"),
        CloudPlatform::Modal => format!("/vol/warp/runs/{run_id}"),
    }
}

impl Entity for CloudWorkerConnectorModel {
    type Event = CloudWorkerConnectorEvent;
}

impl SingletonEntity for CloudWorkerConnectorModel {}

#[cfg(test)]
mod tests {
    use super::*;

    fn managed_host(platform: CloudPlatform, worker_host: &str) -> ManagedCloudWorkerHost {
        ManagedCloudWorkerHost {
            credential_entry_id: "cred-1".to_string(),
            worker_host: worker_host.to_string(),
            platform,
            label: "Test Host".to_string(),
            vps_host: Some("example.com".to_string()),
            vps_username: Some("warp".to_string()),
        }
    }

    #[test]
    fn builds_vps_paths_under_home_directory() {
        let paths =
            build_persistence_paths(&managed_host(CloudPlatform::Vps, "cloud-vps-1"), "run-1");
        assert_eq!(paths.state_root, "$HOME/.warp/cloud-agent/runs/run-1");
        assert_eq!(
            paths.manifest_path,
            "$HOME/.warp/cloud-agent/runs/run-1/manifest.json"
        );
        assert_eq!(
            paths.artifacts_dir,
            "$HOME/.warp/cloud-agent/runs/run-1/artifacts"
        );
    }

    #[test]
    fn builds_modal_paths_under_persistent_volume() {
        let paths = build_persistence_paths(
            &managed_host(CloudPlatform::Modal, "cloud-modal-1"),
            "run-1",
        );
        assert_eq!(paths.state_root, "/vol/warp/runs/run-1");
        assert_eq!(paths.stdout_path, "/vol/warp/runs/run-1/stdout.log");
    }

    #[test]
    fn builds_vps_bootstrap_plan_with_detached_command() {
        let plan = build_bootstrap_plan(
            &managed_host(CloudPlatform::Vps, "cloud-vps-1"),
            "task-1",
            "run-1",
        );
        let CloudWorkerBootstrapPlan::Vps(plan) = plan else {
            panic!("expected vps bootstrap plan");
        };
        assert!(
            plan.detached_launch_command
                .contains("warp agent run --task-id task-1")
        );
        assert!(plan.detached_launch_command.contains("stdout.log"));
        assert_eq!(plan.host.as_deref(), Some("example.com"));
        assert_eq!(plan.username.as_deref(), Some("warp"));
    }

    #[test]
    fn builds_modal_bootstrap_plan_without_secrets() {
        let plan = build_bootstrap_plan(
            &managed_host(CloudPlatform::Modal, "cloud-modal-1"),
            "task-1",
            "run-1",
        );
        let CloudWorkerBootstrapPlan::Modal(plan) = plan else {
            panic!("expected modal bootstrap plan");
        };
        assert_eq!(plan.persistent_filesystem_mount, "/vol/warp");
        assert_eq!(
            plan.entrypoint_command,
            vec!["warp", "agent", "run", "--task-id", "task-1"]
        );
    }
}

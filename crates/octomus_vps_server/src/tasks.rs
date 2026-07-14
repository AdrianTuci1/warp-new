use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

use crate::api::AgentTaskSummary;
use crate::state::AppState;

pub fn list_tasks(state: &AppState) -> Vec<AgentTaskSummary> {
    let tasks = state.tasks.lock().unwrap();
    let mut s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );
    s.refresh_processes(ProcessesToUpdate::All, true);
    let mut summaries = Vec::new();
    for (task_id, task) in tasks.iter() {
        let pid = task.child.lock().unwrap().id();
        let status = if pid > 0
            && s.process((pid as usize).into()).is_some()
            && task
                .child
                .lock()
                .unwrap()
                .try_wait()
                .unwrap_or(None)
                .is_none()
        {
            "running"
        } else {
            "completed"
        };
        summaries.push(AgentTaskSummary {
            task_id: task_id.clone(),
            prompt: task.prompt.clone(),
            status: status.to_string(),
            started_at: task.started_at.clone(),
            pid: Some(pid),
        });
    }
    summaries
}

pub fn task_status(state: &AppState, task_id: &str) -> Option<AgentTaskSummary> {
    list_tasks(state).into_iter().find(|t| t.task_id == task_id)
}

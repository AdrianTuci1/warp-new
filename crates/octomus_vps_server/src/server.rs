use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    api::*,
    crypto::*,
    download, env, pairing,
    state::{AppState, RunningTask},
    tasks,
};

pub async fn run_server(state: Arc<AppState>, port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { "octomus-vps-server" }))
        .route("/status", get(status_handler))
        .route("/pairing", post(pairing_handler))
        .route("/cli/download", post(cli_download_handler))
        .route("/config", post(config_upload_handler))
        .route("/secrets", post(secrets_upload_handler))
        .route("/agent/run", post(agent_run_handler))
        .route("/agent/schedule", post(agent_schedule_handler))
        .route("/agents", get(list_agents_handler))
        .route("/agent/:task_id/logs", get(agent_logs_handler))
        .route("/agent/:task_id", get(agent_status_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind to port {port}: {e}"))?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn status_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let cli_path = state
        .cli_path
        .lock()
        .unwrap()
        .as_ref()
        .map(|p| p.display().to_string());
    Json(ServerStatus {
        paired: state.config.lock().unwrap().is_some(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        cli_path,
    })
}

async fn pairing_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PairingRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    verify_pairing_response(&state.pairing_key, &req.challenge_response)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let server_challenge =
        pairing_challenge(&state.pairing_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((
        StatusCode::OK,
        Json(PairingResponse {
            paired: true,
            server_challenge,
        }),
    ))
}

async fn cli_download_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CliDownloadRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match download::download_cli(&state, &req.url, req.checksum.as_deref()).await {
        Ok(path) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"path": path.display().to_string()})),
        )),
        Err(e) => {
            log::error!("cli download failed: {e}");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn config_upload_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ConfigUploadRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match pairing::update_config_from_encrypted_payload(&state, &req.encrypted_payload) {
        Ok(_) => Ok((StatusCode::OK, Json(serde_json::json!({"ok": true})))),
        Err(e) => {
            log::error!("config upload failed: {e}");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

async fn secrets_upload_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SecretUploadRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match pairing::update_secrets_from_encrypted_payload(&state, &req.encrypted_payload) {
        Ok(_) => Ok((StatusCode::OK, Json(serde_json::json!({"ok": true})))),
        Err(e) => {
            log::error!("secrets upload failed: {e}");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

async fn agent_run_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AgentRunRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if state.cli_path.lock().unwrap().is_none() {
        return Err(StatusCode::PRECONDITION_FAILED);
    }
    let task_id = AppState::new_task_id();
    let prompt = req.prompt.clone();
    let cwd = req.cwd.clone();
    let harness = req.harness.clone();
    let model = req.model.clone();
    match download::launch_agent(
        &state,
        &task_id,
        &prompt,
        cwd.as_deref(),
        harness.as_deref(),
        model.as_deref(),
    ) {
        Ok(child) => {
            let running = RunningTask {
                task_id: task_id.clone(),
                prompt,
                started_at: chrono::Utc::now().to_rfc3339(),
                child: Arc::new(std::sync::Mutex::new(child)),
                log_dir: state.task_log_dir(&task_id),
            };
            state.tasks.lock().unwrap().insert(task_id.clone(), running);
            Ok((StatusCode::OK, Json(AgentRunResponse { task_id })))
        }
        Err(e) => {
            log::error!("failed to start agent: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn agent_schedule_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AgentScheduleRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if state.cli_path.lock().unwrap().is_none() {
        return Err(StatusCode::PRECONDITION_FAILED);
    }
    let task_id = AppState::new_task_id();
    match download::create_cron_job(&state, &task_id, &req.cron_expression, &req.run.prompt) {
        Ok(_) => Ok((StatusCode::OK, Json(AgentRunResponse { task_id }))),
        Err(e) => {
            log::error!("failed to schedule agent: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_agents_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let summaries = tasks::list_tasks(&state);
    Ok((StatusCode::OK, Json(AgentListResponse { tasks: summaries })))
}

async fn agent_status_handler(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match tasks::task_status(&state, &task_id) {
        Some(summary) => Ok((StatusCode::OK, Json(summary))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn agent_logs_handler(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let log_dir = state.task_log_dir(&task_id);
    let stdout_path = log_dir.join("stdout.log");
    let stderr_path = log_dir.join("stderr.log");
    let stdout = env::read_tail(&stdout_path, 256_000).unwrap_or_default();
    let stderr = env::read_tail(&stderr_path, 256_000).unwrap_or_default();
    Ok((
        StatusCode::OK,
        Json(AgentLogResponse {
            task_id,
            stdout,
            stderr,
        }),
    ))
}

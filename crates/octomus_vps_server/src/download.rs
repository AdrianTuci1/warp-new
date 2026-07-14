use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use command::blocking::Command;
use tokio::fs;

use crate::state::AppState;

pub async fn download_cli(state: &AppState, url: &str, _checksum: Option<&str>) -> Result<PathBuf> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .context("failed to download cli binary")?;
    if !response.status().is_success() {
        anyhow::bail!("download failed with status {}", response.status());
    }
    let bytes = response.bytes().await.context("failed to read cli bytes")?;
    let cli_path = state.cli_path();
    fs::write(&cli_path, &bytes)
        .await
        .context("failed to write cli binary")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&cli_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&cli_path, perms).await?;
    }
    let mut guard = state.cli_path.lock().unwrap();
    *guard = Some(cli_path.clone());
    Ok(cli_path)
}

pub async fn install_systemd_service(
    _state: &AppState,
    _port: u16,
    _pairing_code: &str,
) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let exe = std::env::current_exe().context("could not get current executable path")?;
        let service = format!(
            "[Unit]\nDescription=Octomus VPS Server\nAfter=network.target\n\n[Service]\nType=simple\nExecStart={} --port {} --pairing-code \"{}\"\nRestart=on-failure\n\n[Install]\nWantedBy=default.target\n",
            exe.display(),
            _port,
            _pairing_code
        );
        let user_dir = directories::BaseDirs::new()
            .map(|b| b.config_dir().join("systemd").join("user"))
            .context("no base dirs")?;
        fs::create_dir_all(&user_dir).await?;
        let service_path = user_dir.join("octomus-vps-server.service");
        let mut f = std::fs::File::create(&service_path)?;
        f.write_all(service.as_bytes())?;
    }
    Ok(())
}

pub fn launch_agent(
    state: &AppState,
    task_id: &str,
    prompt: &str,
    cwd: Option<&str>,
    harness: Option<&str>,
    _model: Option<&str>,
) -> Result<std::process::Child> {
    let cli_path = state
        .cli_path
        .lock()
        .unwrap()
        .clone()
        .context("cli binary not downloaded")?;
    let log_dir = state.task_log_dir(task_id);
    std::fs::create_dir_all(&log_dir)?;
    let stdout_path = log_dir.join("stdout.log");
    let stderr_path = log_dir.join("stderr.log");
    let stdout = std::fs::File::create(&stdout_path)?;
    let stderr = std::fs::File::create(&stderr_path)?;
    let config = state.config.lock().unwrap().clone();

    let mut cmd = Command::new(cli_path);
    cmd.arg("agent").arg("run");
    if let Some(h) = harness {
        cmd.arg("--harness").arg(h);
    }
    if let Some(ref cfg) = config {
        if let Some(model) = &cfg.default_model {
            cmd.arg("--model").arg(model);
        }
        if let Some(cwd_default) = &cfg.default_cwd {
            cmd.arg("--cwd").arg(cwd_default);
        }
        for (k, v) in &cfg.env_vars {
            cmd.env(k, v);
        }
    }
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    cmd.arg("--prompt").arg(prompt);
    cmd.env("OZ_STANDALONE", "1");
    cmd.env("OZ_CLI", "1");
    if let Some(api_key) = config.as_ref().and_then(|c| c.api_key.as_ref()) {
        cmd.env("WARP_API_KEY", api_key);
    }
    if let Some(server_url) = config.as_ref().and_then(|c| c.server_root_url.as_ref()) {
        cmd.env("WARP_SERVER_ROOT_URL", server_url);
    }
    cmd.stdout(stdout)
        .stderr(stderr)
        .spawn()
        .map_err(anyhow::Error::from)
}

pub fn create_cron_job(
    state: &AppState,
    task_id: &str,
    cron_expression: &str,
    prompt: &str,
) -> Result<()> {
    #[cfg(unix)]
    {
        let cli_path = state
            .cli_path
            .lock()
            .unwrap()
            .clone()
            .context("cli binary not downloaded")?;
        let entry = format!(
            "{} {}   {} agent run --prompt \"{}\" && echo \"{} completed\"\n",
            cron_expression,
            cli_path.display(),
            cli_path.display(),
            prompt.replace('\"', "\\\""),
            task_id
        );
        let temp =
            std::env::temp_dir().join(format!("octomus-cron-{}.{}", task_id, std::process::id()));
        std::fs::write(&temp, entry)?;
        let _ = Command::new("crontab").arg("-l").spawn();
    }
    Ok(())
}

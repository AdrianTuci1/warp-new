// Octomus — SSH remote executor.

use std::process::Stdio;

use async_stream::stream;
use futures::stream::Stream;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use super::{RemoteBackendConfig, RemoteExecutor, RemoteOutput, RemoteTask};

pub struct SshExecutor;

impl SshExecutor {
    pub fn new() -> Self {
        Self
    }

    fn ssh_args(host: &str, credential: &str) -> Vec<String> {
        vec![
            "-o".to_string(),
            "ConnectTimeout=5".to_string(),
            "-o".to_string(),
            "StrictHostKeyChecking=no".to_string(),
            "-o".to_string(),
            "UserKnownHostsFile=/dev/null".to_string(),
            "-i".to_string(),
            credential.to_string(),
            host.to_string(),
        ]
    }
}

#[async_trait::async_trait]
impl RemoteExecutor for SshExecutor {
    async fn test_connection(&self, config: &RemoteBackendConfig) -> Result<(), String> {
        let args = Self::ssh_args(&config.host, &config.credential);
        let output = Command::new("ssh")
            .args(&args)
            .arg("echo octomus-ssh-ok")
            .output()
            .await
            .map_err(|e| format!("ssh test_connection failed: {e}"))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("ssh test_connection failed: {stderr}"));
        }
        Ok(())
    }

    async fn execute(
        &self,
        config: &RemoteBackendConfig,
        task: RemoteTask,
    ) -> Result<Box<dyn Stream<Item = RemoteOutput> + Send>, String> {
        let mkdir_args = Self::ssh_args(&config.host, &config.credential);
        let mkdir_status = Command::new("ssh")
            .args(&mkdir_args)
            .arg(format!("mkdir -p {}", task.remote_work_dir))
            .status()
            .await
            .map_err(|e| format!("ssh mkdir failed: {e}"))?;
        if !mkdir_status.success() {
            return Err("ssh mkdir failed".into());
        }

        for file in &task.files {
            if let Some(file_name) = file.file_name().and_then(|n| n.to_str()) {
                let dest = format!("{}:{}/{}", config.host, task.remote_work_dir, file_name);
                let scp_status = Command::new("scp")
                    .args([
                        "-o",
                        "ConnectTimeout=5",
                        "-o",
                        "StrictHostKeyChecking=no",
                        "-o",
                        "UserKnownHostsFile=/dev/null",
                        "-i",
                        &config.credential,
                    ])
                    .arg(file.as_os_str())
                    .arg(&dest)
                    .status()
                    .await
                    .map_err(|e| format!("scp failed: {e}"))?;
                if !scp_status.success() {
                    return Err(format!("scp failed for {}", file.display()));
                }
            }
        }

        let host = config.host.clone();
        let credential = config.credential.clone();
        let remote_dir = task.remote_work_dir.clone();
        let command = task.command.clone();
        let env = task.env.clone();

        let stream = stream! {
            let mut ssh_cmd = Command::new("ssh");
            ssh_cmd.args(Self::ssh_args(&host, &credential));
            for (k, v) in env {
                ssh_cmd.arg(format!("export {k}={v};"));
            }
            ssh_cmd.arg(format!("cd {remote_dir} && {command}"));
            ssh_cmd.stdout(Stdio::piped());
            ssh_cmd.stderr(Stdio::piped());

            let mut child = match ssh_cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    yield RemoteOutput::Error(format!("ssh spawn failed: {e}"));
                    yield RemoteOutput::Exit(-1);
                    return;
                }
            };
            let stdout = match child.stdout.take() {
                Some(s) => s,
                None => {
                    yield RemoteOutput::Error("failed to capture stdout".into());
                    yield RemoteOutput::Exit(-1);
                    return;
                }
            };
            let stderr = match child.stderr.take() {
                Some(s) => s,
                None => {
                    yield RemoteOutput::Error("failed to capture stderr".into());
                    yield RemoteOutput::Exit(-1);
                    return;
                }
            };

            let mut stdout_reader = BufReader::new(stdout).lines();
            let mut stderr_reader = BufReader::new(stderr).lines();

            loop {
                tokio::select! {
                    line = stdout_reader.next_line() => match line {
                        Ok(Some(l)) => yield RemoteOutput::Stdout(l),
                        Ok(None) => break,
                        Err(e) => {
                            yield RemoteOutput::Error(format!("stdout read error: {e}"));
                            break;
                        }
                    },
                    line = stderr_reader.next_line() => match line {
                        Ok(Some(l)) => yield RemoteOutput::Stderr(l),
                        Ok(None) => break,
                        Err(e) => {
                            yield RemoteOutput::Error(format!("stderr read error: {e}"));
                            break;
                        }
                    },
                }
            }

            match child.wait().await {
                Ok(status) => {
                    let code = status.code().unwrap_or(-1);
                    yield RemoteOutput::Exit(code);
                }
                Err(e) => yield RemoteOutput::Error(format!("wait failed: {e}")),
            }
        };

        Ok(Box::new(stream))
    }
}

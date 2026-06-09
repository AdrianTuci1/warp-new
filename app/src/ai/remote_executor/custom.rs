// Octomus — Custom HTTP remote executor.

use async_stream::stream;
use futures::stream::Stream;
use serde_json::json;

use super::{RemoteBackendConfig, RemoteExecutor, RemoteOutput, RemoteTask};

pub struct CustomExecutor;

impl CustomExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl RemoteExecutor for CustomExecutor {
    async fn test_connection(&self, config: &RemoteBackendConfig) -> Result<(), String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| e.to_string())?;
        let resp = client
            .get(&config.host)
            .header("x-api-key", &config.credential)
            .send()
            .await
            .map_err(|e| format!("custom http test_connection failed: {e}"))?;
        if !resp.status().is_success() {
            return Err(format!(
                "custom http test_connection failed: {}",
                resp.status()
            ));
        }
        Ok(())
    }

    async fn execute(
        &self,
        config: &RemoteBackendConfig,
        task: RemoteTask,
    ) -> Result<Box<dyn Stream<Item = RemoteOutput> + Send>, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_seconds.unwrap_or(300),
            ))
            .build()
            .map_err(|e| e.to_string())?;

        let files: Vec<String> = task
            .files
            .iter()
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect();

        let payload = json!({
            "command": task.command,
            "env": task.env,
            "remote_work_dir": task.remote_work_dir,
            "files": files,
            "image": config.image,
        });

        let host = config.host.clone();
        let credential = config.credential.clone();

        let response = client
            .post(format!("{host}/run"))
            .header("x-api-key", credential)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("custom http execute failed: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("custom http execute failed: {status} {body}"));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("custom http read body failed: {e}"))?;
        let text = String::from_utf8_lossy(&bytes).to_string();

        let stream = stream! {
            for line in text.lines() {
                if let Some(json) = line.strip_prefix("data: ") {
                    match serde_json::from_str::<RemoteOutput>(json) {
                        Ok(out) => yield out,
                        Err(_) => yield RemoteOutput::Stdout(line.to_string()),
                    }
                } else {
                    yield RemoteOutput::Stdout(line.to_string());
                }
            }
            yield RemoteOutput::Exit(0);
        };

        Ok(Box::new(stream))
    }
}

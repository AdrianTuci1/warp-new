// Octomus — Modal remote executor (stub).

use async_stream::stream;
use futures::stream::Stream;

use super::{RemoteBackendConfig, RemoteExecutor, RemoteOutput, RemoteTask};

pub struct ModalExecutor;

impl ModalExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl RemoteExecutor for ModalExecutor {
    async fn test_connection(&self, config: &RemoteBackendConfig) -> Result<(), String> {
        if config.credential.is_empty() {
            return Err("Modal token not configured".into());
        }
        // Stub: validate token format (starts with "ak-") like Modal tokens do.
        if !config.credential.starts_with("ak-") {
            return Err("Modal token appears invalid".into());
        }
        Ok(())
    }

    async fn execute(
        &self,
        _config: &RemoteBackendConfig,
        _task: RemoteTask,
    ) -> Result<Box<dyn Stream<Item = RemoteOutput> + Send + Unpin>, String> {
        let s = stream! {
            yield RemoteOutput::Stdout("Modal executor: full implementation pending".to_string());
            yield RemoteOutput::Exit(0);
        };
        Ok(Box::new(s))
    }
}

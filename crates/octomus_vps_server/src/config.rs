use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VpsConfig {
    pub server_root_url: Option<String>,
    pub api_key: Option<String>,
    pub env_vars: HashMap<String, String>,
    pub default_cwd: Option<String>,
    pub default_harness: Option<String>,
    pub default_model: Option<String>,
}

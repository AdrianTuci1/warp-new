use std::time::Duration;

pub mod error;
pub mod install_tmux;
pub mod octomusify;
pub mod root_access;
pub mod ssh_detection;
pub mod util;

pub const SSH_WARPIFY_TIMEOUT_DURATION: Duration = Duration::from_secs(8);

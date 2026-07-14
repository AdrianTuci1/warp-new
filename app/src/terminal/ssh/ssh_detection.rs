use octomus_core::features::FeatureFlag;
use octomus_core::settings::Setting;
use octomus_util::path::ShellFamily;
use serde::{Deserialize, Serialize};

use crate::terminal::octomusify::settings::OctomusifySettings;

/// The different possible outcomes of detecting an interactive SSH session.
/// Also the payload for the [`crate::server::telemetry::TelemetryEvent::SshInteractiveSessionDetected`] event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SshInteractiveSessionDetected {
    #[serde(rename = "feature_disabled")]
    FeatureDisabled,
    #[serde(rename = "host_denylisted")]
    HostDenylisted,
    #[serde(rename = "octomusify_prompt")]
    ShouldPromptOctomusification {
        #[serde(skip)]
        command: String,
        #[serde(skip)]
        host: Option<String>,
    },
}

/// Determines whether a host could be warpified.
pub fn evaluate_octomusify_ssh_host(
    command: &str,
    ssh_host: Option<&str>,
    shell_family: ShellFamily,
    octomusify_settings: &OctomusifySettings,
) -> SshInteractiveSessionDetected {
    let should_prompt_ssh_tmux_wrapper = *octomusify_settings.enable_ssh_octomusification.value()
        && *octomusify_settings.use_ssh_tmux_wrapper.value();
    let matches_subshell = octomusify_settings.is_denylisted_subshell_command(command)
        || octomusify_settings.is_compatible_subshell_command(command, shell_family);
    if !should_prompt_ssh_tmux_wrapper
        || matches_subshell
        || !FeatureFlag::SSHTmuxWrapper.is_enabled()
    {
        return SshInteractiveSessionDetected::FeatureDisabled;
    }

    if let Some(ssh_host) = ssh_host {
        if octomusify_settings.is_ssh_host_denylisted(ssh_host) {
            return SshInteractiveSessionDetected::HostDenylisted;
        }
    }

    SshInteractiveSessionDetected::ShouldPromptOctomusification {
        host: ssh_host.map(|host| host.to_owned()),
        command: command.to_string(),
    }
}

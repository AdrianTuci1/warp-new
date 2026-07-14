use octomus_core::features::FeatureFlag;
use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

use super::DriveSortOrder;

pub const HAS_AUTO_OPENED_WELCOME_FOLDER: &str = "HasAutoOpenedWelcomeFolder";

define_settings_group!(OctomusDriveSettings, settings: [
    sorting_choice: OctomusDriveSortingChoice {
        type: DriveSortOrder,
        default: DriveSortOrder::ByObjectType,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "octomus_drive.sorting_choice",
        description: "The sort order for items in Octomus Drive.",
    },
    sharing_onboarding_block_shown: OctomusDriveSharingOnboardingBlockShown {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: true,
    },
    // Controls whether Octomus Drive appears in the tools panel, command palette, and command search.
    enable_octomus_drive: EnableOctomusDrive {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        private: false,
        toml_path: "octomus_drive.enabled",
        description: "Whether Octomus Drive is enabled.",
    },
]);

impl OctomusDriveSettings {
    /// Returns whether Octomus Drive should be considered enabled.
    pub fn is_octomus_drive_enabled(app: &octomusui::AppContext) -> bool {
        use octomusui::SingletonEntity as _;
        *Self::as_ref(app).enable_octomus_drive
    }
}

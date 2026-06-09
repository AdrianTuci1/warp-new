#[path = "channel_config.rs"]
mod channel_config;

use anyhow::Result;
use warp_core::channel::{Channel, ChannelState};
use warp_core::features;

fn main() -> Result<()> {
    let config = channel_config::load_config!("local");

    let mut state = ChannelState::new(Channel::Local, config)
        .with_additional_features(features::DEBUG_FLAGS)
        .with_additional_features(features::DOGFOOD_FLAGS)
        .with_additional_features(features::PREVIEW_FLAGS);

    if std::env::var("WITH_SANDBOX_TELEMETRY").is_ok() {
        state = state.with_additional_features(&[features::FeatureFlag::WithSandboxTelemetry]);
    }

    ChannelState::set(state);

    warp::run()
}

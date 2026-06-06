use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TelemetryEvent {
    AppOpened,
    AppClosed,
    AppActive,
    ModelUsed,
    FeatureUsed,
}

pub struct CLIAgentType;
pub struct MCPTemplateInstallationSource;
pub struct AICommandSearchEntrypoint;
pub struct AgentModeAutoDetectionFalsePositivePayload;
pub struct AgentModeAutoDetectionSettingOrigin;
pub struct AnonymousUserSignupEntrypoint;
pub struct CommandXRayTrigger;
pub struct EnvVarTelemetryMetadata;
pub struct PaletteSource;
pub struct SlashCommandAcceptedDetails;
pub struct SlashMenuSource;
pub struct WorkflowTelemetryMetadata;
pub mod telemetry_context {}

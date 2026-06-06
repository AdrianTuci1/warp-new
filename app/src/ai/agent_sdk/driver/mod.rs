pub struct Driver;
pub struct AgentDriverError;
pub mod harness {
    pub struct Harness;
    pub fn upload_snapshot_for_handoff() {}
}
pub mod environment {}
pub mod terminal {}
pub const WARP_DRIVE_SYNC_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(1);

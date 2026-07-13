use octomusui::elements::MouseStateHandle;
use octomusui::{AppContext, Element};

use super::{OctomusDriveItem, OctomusDriveItemId};
use crate::appearance::Appearance;
use crate::cloud_object::CloudObjectMetadata;
use crate::drive::index::DriveIndexAction;
use crate::drive::DriveObjectType;
use crate::server::ids::ClientId;
use crate::themes::theme::Fill;

#[derive(Clone)]
pub struct OctomusDriveAIFactCollection {
    id: ClientId,
}

impl OctomusDriveAIFactCollection {
    pub fn new(id: ClientId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> ClientId {
        self.id
    }
}

impl OctomusDriveItem for OctomusDriveAIFactCollection {
    fn display_name(&self) -> Option<String> {
        Some("Rules".to_string())
    }

    fn metadata(&self) -> Option<&CloudObjectMetadata> {
        None
    }

    fn object_type(&self) -> Option<DriveObjectType> {
        Some(DriveObjectType::AIFactCollection)
    }

    fn secondary_icon(&self, _color: Option<Fill>) -> Option<Box<dyn Element>> {
        None
    }

    fn click_action(&self) -> Option<DriveIndexAction> {
        Some(DriveIndexAction::OpenAIFactCollection)
    }

    fn preview(&self, _appearance: &Appearance) -> Option<Box<dyn Element>> {
        None
    }

    fn octomus_drive_id(&self) -> OctomusDriveItemId {
        OctomusDriveItemId::AIFactCollection
    }

    fn sync_status_icon(
        &self,
        _sync_queue_is_dequeueing: bool,
        _hover_state: MouseStateHandle,
        _appearance: &Appearance,
    ) -> Option<Box<dyn Element>> {
        None
    }

    fn action_summary(&self, _app: &AppContext) -> Option<String> {
        None
    }

    fn clone_box(&self) -> Box<dyn OctomusDriveItem> {
        Box::new(self.clone())
    }
}

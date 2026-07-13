use octomusui::elements::MouseStateHandle;
use octomusui::Element;

use super::{OctomusDriveItem, OctomusDriveItemId};
use crate::appearance::Appearance;
use crate::cloud_object::{CloudObjectMetadata, Space};
use crate::drive::index::DriveIndexAction;
use crate::drive::DriveObjectType;
use crate::themes::theme::Fill;

#[derive(Clone)]
pub struct OctomusDriveSpace {
    space: Space,
}

impl OctomusDriveSpace {
    #[allow(dead_code)]
    pub fn new(space: Space) -> Self {
        Self { space }
    }
}

impl OctomusDriveItem for OctomusDriveSpace {
    fn display_name(&self) -> Option<String> {
        None
    }

    fn metadata(&self) -> Option<&CloudObjectMetadata> {
        None
    }

    fn object_type(&self) -> Option<DriveObjectType> {
        None
    }

    fn secondary_icon(&self, _color: Option<Fill>) -> Option<Box<dyn Element>> {
        None
    }

    fn click_action(&self) -> Option<DriveIndexAction> {
        None
    }

    fn preview(&self, _appearance: &Appearance) -> Option<Box<dyn Element>> {
        None
    }

    fn octomus_drive_id(&self) -> OctomusDriveItemId {
        OctomusDriveItemId::Space(self.space)
    }

    fn sync_status_icon(
        &self,
        _sync_queue_is_dequeueing: bool,
        _hover_state: MouseStateHandle,
        _appearance: &Appearance,
    ) -> Option<Box<dyn Element>> {
        None
    }

    fn clone_box(&self) -> Box<dyn OctomusDriveItem> {
        Box::new(self.clone())
    }

    fn action_summary(&self, _app: &octomusui::AppContext) -> Option<String> {
        None
    }
}

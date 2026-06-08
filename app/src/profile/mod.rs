use std::sync::Arc;

use parking_lot::RwLock;
use warpui_core::{AppContext, Entity, ModelContext, SingletonEntity};

const DEFAULT_NAMES: &[&str] = &[
    "Alex", "Morgan", "Jordan", "Casey", "Riley", "Quinn", "Avery", "Skyler", "Dakota", "Reese",
];

/// Simple user profile that doesn't require server authentication.
/// Users can set a display name and photo, or use a randomly assigned default name.
#[derive(Clone, Debug)]
pub struct Profile {
    pub display_name: String,
    pub photo_path: Option<String>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            display_name: Self::random_default_name(),
            photo_path: None,
        }
    }
}

impl Profile {
    fn random_default_name() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let index = (seed % DEFAULT_NAMES.len() as u64) as usize;
        DEFAULT_NAMES[index].to_string()
    }
}

/// ProfileModel is a singleton that holds the current user's profile.
pub struct ProfileModel {
    profile: Arc<RwLock<Profile>>,
}

impl ProfileModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            profile: Arc::new(RwLock::new(Profile::default())),
        }
    }

    pub fn get_profile(&self) -> Profile {
        self.profile.read().clone()
    }

    pub fn display_name(&self) -> String {
        self.profile.read().display_name.clone()
    }

    pub fn photo_path(&self) -> Option<String> {
        self.profile.read().photo_path.clone()
    }

    pub fn set_display_name(&self, name: String) {
        self.profile.write().display_name = name;
    }

    pub fn set_photo_path(&self, path: Option<String>) {
        self.profile.write().photo_path = path;
    }

    pub fn user_id(&self) -> String {
        // Return a stable ID based on the display name
        // This is used where a user identifier is needed (e.g. creator_uid)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let name = self.display_name();
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        format!("user-{}", hasher.finish())
    }
}

impl Entity for ProfileModel {
    type Event = ProfileModelEvent;
}

impl SingletonEntity for ProfileModel {}

#[derive(Clone, Debug)]
pub enum ProfileModelEvent {
    ProfileUpdated,
}

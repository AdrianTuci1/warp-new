// LLM preferences - starts empty, user configures their own provider/key.
use warpui::AppContext;
use warpui::SingletonEntity;

#[derive(Clone, Debug, Default)]
pub struct LLMPreferences;

impl LLMPreferences {
    pub fn new() -> Self {
        Self
    }
}

impl SingletonEntity for LLMPreferences {
    fn new(_ctx: &mut AppContext) -> Self {
        Self::new()
    }
}

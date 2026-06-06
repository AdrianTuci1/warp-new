use warpui::{AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle};
use crate::editor::EditorView;

#[derive(Clone, Debug, PartialEq)]
pub enum NewEnvironmentButtonAction {}

#[derive(Clone, Debug, PartialEq)]
pub enum NewEnvironmentButtonEvent {}

pub struct NewEnvironmentButtonView {
    search_editor: ViewHandle<EditorView>,
}

impl NewEnvironmentButtonView {
    pub fn new(search_editor: ViewHandle<EditorView>, _ctx: &mut ViewContext<Self>) -> Self {
        Self { search_editor }
    }
}

impl View for NewEnvironmentButtonView {
    fn ui_name() -> &'static str {
        "NewEnvironmentButtonView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        warpui::elements::Empty::new().finish()
    }
}

impl Entity for NewEnvironmentButtonView {
    type Event = NewEnvironmentButtonEvent;
}

impl TypedActionView for NewEnvironmentButtonView {
    type Action = NewEnvironmentButtonAction;

    fn handle_action(&mut self, _action: &NewEnvironmentButtonAction, _ctx: &mut ViewContext<Self>) {}
}

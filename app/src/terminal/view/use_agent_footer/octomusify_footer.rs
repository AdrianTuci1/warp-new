use std::sync::Arc;

use parking_lot::FairMutex;
use octomusui::elements::{
    ChildView, Container, CrossAxisAlignment, Expanded, Flex, MainAxisSize, ParentElement,
};
use octomusui::prelude::Empty;
use octomusui::{AppContext, Element, Entity, TypedActionView, View, ViewContext, ViewHandle};

use super::{AgentFooterButtonTheme, USE_AGENT_KEYSTROKE};
use crate::terminal::view::block_banner::OctomusificationMode;
use crate::terminal::view::{TerminalModel, PADDING_LEFT};
use crate::ui_components::icons::Icon;
use crate::view_components::action_button::{
    ActionButton, ButtonSize, KeystrokeSource, TooltipAlignment,
};

/// Footer view rendered for detected subshell/SSH commands, offering both
/// "Octofy" and "Use agent" buttons in a horizontal row.
pub(super) struct OctomusifyFooterView {
    terminal_model: Arc<FairMutex<TerminalModel>>,
    octomusify_button: ViewHandle<ActionButton>,
    use_agent_button: ViewHandle<ActionButton>,
    dismiss_button: ViewHandle<ActionButton>,
    mode: Option<OctomusificationMode>,
}

impl OctomusifyFooterView {
    pub fn new(terminal_model: Arc<FairMutex<TerminalModel>>, ctx: &mut ViewContext<Self>) -> Self {
        let button_size = ButtonSize::XSmall;

        let octomusify_button = ctx.add_typed_action_view(|_ctx| {
            ActionButton::new("Octomusify subshell", AgentFooterButtonTheme::new(None))
                .with_icon(Icon::Octomus)
                .with_size(button_size)
                .with_tooltip("Enable Octomus shell integration in this session")
                .with_tooltip_alignment(TooltipAlignment::Left)
                .on_click(|ctx| {
                    ctx.dispatch_typed_action(OctomusifyFooterViewAction::Octomusify);
                })
        });

        let use_agent_button = ctx.add_typed_action_view(|ctx| {
            ActionButton::new("Use agent", AgentFooterButtonTheme::new(None))
                .with_icon(Icon::Oz)
                .with_keybinding(KeystrokeSource::Fixed(USE_AGENT_KEYSTROKE.clone()), ctx)
                .with_size(button_size)
                .with_tooltip("Ask the Octomus agent to assist")
                .with_tooltip_alignment(TooltipAlignment::Left)
                .on_click(|ctx| {
                    ctx.dispatch_typed_action(OctomusifyFooterViewAction::UseAgent);
                })
        });

        let dismiss_button = ctx.add_typed_action_view(|_ctx| {
            ActionButton::new("Dismiss", AgentFooterButtonTheme::new(None))
                .with_size(button_size)
                .on_click(|ctx| {
                    ctx.dispatch_typed_action(OctomusifyFooterViewAction::Dismiss);
                })
        });

        Self {
            terminal_model,
            octomusify_button,
            use_agent_button,
            dismiss_button,
            mode: None,
        }
    }

    /// Updates the octomusify button label, keybinding, and stores the current octomusification mode.
    pub fn set_mode(&mut self, mode: OctomusificationMode, ctx: &mut ViewContext<Self>) {
        let (label, binding_name) = match mode {
            OctomusificationMode::Ssh { .. } => {
                ("Octomusify SSH session", "terminal:octomusify_ssh_session")
            }
            OctomusificationMode::Subshell { .. } => ("Octomusify subshell", "terminal:octomusify_subshell"),
        };
        self.octomusify_button.update(ctx, |button, ctx| {
            button.set_label(label, ctx);
            button.set_keybinding(Some(KeystrokeSource::Binding(binding_name)), ctx);
        });
        self.mode = Some(mode);
        ctx.notify();
    }

    /// Returns the current octomusification mode, if set.
    pub fn mode(&self) -> Option<&OctomusificationMode> {
        self.mode.as_ref()
    }

    /// Clears the octomusification mode.
    pub fn clear_mode(&mut self, ctx: &mut ViewContext<Self>) {
        self.mode = None;
        self.octomusify_button.update(ctx, |button, ctx| {
            button.set_keybinding(None, ctx);
        });
        ctx.notify();
    }
}

#[derive(Debug, Clone)]
pub enum OctomusifyFooterViewAction {
    Octomusify,
    UseAgent,
    Dismiss,
}

pub enum OctomusifyFooterViewEvent {
    Octomusify { mode: OctomusificationMode },
    UseAgent,
    Dismiss,
}

impl Entity for OctomusifyFooterView {
    type Event = OctomusifyFooterViewEvent;
}

impl View for OctomusifyFooterView {
    fn ui_name() -> &'static str {
        "OctomusifyFooterView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        let terminal_model = self.terminal_model.lock();

        let button_row = Flex::row()
            .with_spacing(4.)
            .with_main_axis_size(MainAxisSize::Max)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(ChildView::new(&self.octomusify_button).finish())
            .with_child(ChildView::new(&self.use_agent_button).finish())
            .with_child(Expanded::new(1., Empty::new().finish()).finish())
            .with_child(ChildView::new(&self.dismiss_button).finish());

        let mut container = Container::new(button_row.finish())
            .with_horizontal_padding(*PADDING_LEFT)
            .with_vertical_padding(4.);

        if terminal_model.is_alt_screen_active() {
            if let Some(bg_color) = terminal_model.alt_screen().inferred_bg_color() {
                container = container.with_background(bg_color);
            }
        }

        container.finish()
    }
}

impl TypedActionView for OctomusifyFooterView {
    type Action = OctomusifyFooterViewAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            OctomusifyFooterViewAction::Octomusify => {
                if let Some(mode) = self.mode.clone() {
                    self.clear_mode(ctx);
                    ctx.emit(OctomusifyFooterViewEvent::Octomusify { mode });
                }
            }
            OctomusifyFooterViewAction::UseAgent => {
                self.clear_mode(ctx);
                ctx.emit(OctomusifyFooterViewEvent::UseAgent);
            }
            OctomusifyFooterViewAction::Dismiss => {
                self.clear_mode(ctx);
                ctx.emit(OctomusifyFooterViewEvent::Dismiss);
            }
        }
    }
}

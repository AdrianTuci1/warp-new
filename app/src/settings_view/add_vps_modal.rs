use warpui::elements::{
    ChildView, ConstrainedBox, Container, CornerRadius, Flex, MainAxisSize, MouseStateHandle,
    ParentElement, Radius, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::UiComponent;
use warpui::{
    AppContext, Element as WarpuiElement, Entity, SingletonEntity, TypedActionView, View,
    ViewContext, ViewHandle,
};

use crate::appearance::Appearance;
use crate::editor::{
    EditorView, Event as EditorEvent, PropagateAndNoOpNavigationKeys, SingleLineEditorOptions,
    TextOptions,
};
use crate::modal::{Modal, ModalViewState};

const LABEL_FONT_SIZE: f32 = 12.;
const INPUT_WIDTH: f32 = 480.;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddVpsModalEvent {
    Close,
    AddVps {
        name: String,
        host: String,
        username: String,
        ssh_key: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddVpsModalAction {
    Cancel,
    Save,
}

pub struct AddVpsModal {
    name_editor: ViewHandle<EditorView>,
    host_editor: ViewHandle<EditorView>,
    username_editor: ViewHandle<EditorView>,
    ssh_key_editor: ViewHandle<EditorView>,
    cancel_button_mouse_state: MouseStateHandle,
    save_button_mouse_state: MouseStateHandle,
}

impl AddVpsModal {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let font_family = Appearance::as_ref(ctx).ui_font_family();
        let text_colors = crate::settings_view::editor_text_colors(Appearance::as_ref(ctx));
        let text_colors_2 = text_colors.clone();
        let text_colors_3 = text_colors.clone();
        let text_colors_4 = text_colors.clone();

        let name_editor = ctx.add_typed_action_view(move |ctx| {
            let options = SingleLineEditorOptions {
                text: TextOptions {
                    font_family_override: Some(font_family),
                    text_colors_override: Some(text_colors.clone()),
                    ..Default::default()
                },
                propagate_and_no_op_vertical_navigation_keys:
                    PropagateAndNoOpNavigationKeys::Always,
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text("e.g., Production VPS", ctx);
            editor
        });

        let host_editor = ctx.add_typed_action_view(move |ctx| {
            let options = SingleLineEditorOptions {
                text: TextOptions {
                    font_family_override: Some(font_family),
                    text_colors_override: Some(text_colors_2.clone()),
                    ..Default::default()
                },
                propagate_and_no_op_vertical_navigation_keys:
                    PropagateAndNoOpNavigationKeys::Always,
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text("e.g., server.example.com", ctx);
            editor
        });

        let username_editor = ctx.add_typed_action_view(move |ctx| {
            let options = SingleLineEditorOptions {
                text: TextOptions {
                    font_family_override: Some(font_family),
                    text_colors_override: Some(text_colors_3.clone()),
                    ..Default::default()
                },
                propagate_and_no_op_vertical_navigation_keys:
                    PropagateAndNoOpNavigationKeys::Always,
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text("e.g., root", ctx);
            editor
        });

        let ssh_key_editor = ctx.add_typed_action_view(move |ctx| {
            let options = SingleLineEditorOptions {
                text: TextOptions {
                    font_family_override: Some(font_family),
                    text_colors_override: Some(text_colors_4.clone()),
                    ..Default::default()
                },
                propagate_and_no_op_vertical_navigation_keys:
                    PropagateAndNoOpNavigationKeys::Always,
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text("Paste SSH private key here", ctx);
            editor
        });

        ctx.subscribe_to_view(&name_editor, |me, _, event, ctx| {
            me.handle_name_event(event, ctx);
        });
        ctx.subscribe_to_view(&host_editor, |me, _, event, ctx| {
            me.handle_host_event(event, ctx);
        });
        ctx.subscribe_to_view(&username_editor, |me, _, event, ctx| {
            me.handle_username_event(event, ctx);
        });
        ctx.subscribe_to_view(&ssh_key_editor, |me, _, event, ctx| {
            me.handle_ssh_key_event(event, ctx);
        });

        Self {
            name_editor,
            host_editor,
            username_editor,
            ssh_key_editor,
            cancel_button_mouse_state: Default::default(),
            save_button_mouse_state: Default::default(),
        }
    }

    pub fn on_open(&mut self, ctx: &mut ViewContext<Self>) {
        self.name_editor
            .update(ctx, |editor, ctx| editor.set_buffer_text("", ctx));
        self.host_editor
            .update(ctx, |editor, ctx| editor.set_buffer_text("", ctx));
        self.username_editor
            .update(ctx, |editor, ctx| editor.set_buffer_text("", ctx));
        self.ssh_key_editor
            .update(ctx, |editor, ctx| editor.set_buffer_text("", ctx));
    }

    pub fn on_close(&mut self, _ctx: &mut ViewContext<Self>) {}

    fn handle_name_event(&mut self, event: &EditorEvent, ctx: &mut ViewContext<Self>) {
        if let EditorEvent::Edited(_) = event {
            ctx.notify();
        }
    }

    fn handle_host_event(&mut self, event: &EditorEvent, ctx: &mut ViewContext<Self>) {
        if let EditorEvent::Edited(_) = event {
            ctx.notify();
        }
    }

    fn handle_username_event(&mut self, event: &EditorEvent, ctx: &mut ViewContext<Self>) {
        if let EditorEvent::Edited(_) = event {
            ctx.notify();
        }
    }

    fn handle_ssh_key_event(&mut self, event: &EditorEvent, ctx: &mut ViewContext<Self>) {
        if let EditorEvent::Edited(_) = event {
            ctx.notify();
        }
    }

    fn is_valid(&self, ctx: &AppContext) -> bool {
        let host = self.host_editor.as_ref(ctx).buffer_text(ctx);
        !host.trim().is_empty()
    }

    fn emit_add_vps(&self, ctx: &mut ViewContext<Self>) {
        let name = self.name_editor.as_ref(ctx).buffer_text(ctx);
        let host = self.host_editor.as_ref(ctx).buffer_text(ctx);
        let username = self.username_editor.as_ref(ctx).buffer_text(ctx);
        let ssh_key = self.ssh_key_editor.as_ref(ctx).buffer_text(ctx);

        ctx.emit(AddVpsModalEvent::AddVps {
            name,
            host,
            username,
            ssh_key,
        });
    }
}

impl Entity for AddVpsModal {
    type Event = AddVpsModalEvent;
}

impl SingletonEntity for AddVpsModal {}

impl TypedActionView for AddVpsModal {
    type Action = AddVpsModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            AddVpsModalAction::Cancel => {
                ctx.emit(AddVpsModalEvent::Close);
            }
            AddVpsModalAction::Save => {
                if self.is_valid(ctx) {
                    self.emit_add_vps(ctx);
                }
            }
        }
    }
}

impl View for AddVpsModal {
    fn ui_name() -> &'static str {
        "AddVpsModal"
    }

    fn render(&self, app: &AppContext) -> Box<dyn WarpuiElement> {
        let appearance = Appearance::as_ref(app);
        let theme = appearance.theme();

        let mut column = Flex::column().with_spacing(16.);

        // Title
        column.add_child(
            Text::new_inline(
                "Add VPS Host",
                appearance.ui_font_family(),
                appearance.header_font_size(),
            )
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(theme.active_ui_text_color().into())
            .finish(),
        );

        // Name field
        column.add_child(render_input_row(
            "Name",
            "Display name for this VPS host",
            &self.name_editor,
            appearance,
        ));

        // Host field
        column.add_child(render_input_row(
            "Host",
            "Hostname or IP address",
            &self.host_editor,
            appearance,
        ));

        // Username field
        column.add_child(render_input_row(
            "Username",
            "SSH username (e.g., root)",
            &self.username_editor,
            appearance,
        ));

        // SSH Key field
        column.add_child(render_input_row(
            "SSH Key",
            "Private key for SSH authentication",
            &self.ssh_key_editor,
            appearance,
        ));

        // Buttons
        let is_valid = self.is_valid(app);
        let save_button = appearance
            .ui_builder()
            .button(ButtonVariant::Accent, self.save_button_mouse_state.clone())
            .with_text_label("Add VPS".to_string())
            .build()
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(AddVpsModalAction::Save);
            })
            .finish();

        let cancel_button = appearance
            .ui_builder()
            .button(ButtonVariant::Secondary, self.cancel_button_mouse_state.clone())
            .with_text_label("Cancel".to_string())
            .build()
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(AddVpsModalAction::Cancel);
            })
            .finish();

        let button_row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(warpui::elements::MainAxisAlignment::End)
            .with_spacing(8.)
            .with_child(cancel_button)
            .with_child(save_button)
            .finish();

        column.add_child(button_row);

        let content = column.finish();

        Container::new(content)
            .with_uniform_padding(24.)
            .with_background(theme.surface_1())
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
            .finish()
    }
}

fn render_input_row(
    label: &str,
    description: &str,
    editor: &ViewHandle<EditorView>,
    appearance: &Appearance,
) -> Box<dyn WarpuiElement> {
    let theme = appearance.theme();

    let mut column = Flex::column().with_spacing(4.);

    column.add_child(
        Text::new(label.to_string(), appearance.ui_font_family(), LABEL_FONT_SIZE)
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(theme.active_ui_text_color().into())
            .finish(),
    );

    column.add_child(
        Text::new(description.to_string(), appearance.ui_font_family(), LABEL_FONT_SIZE)
            .with_color(theme.nonactive_ui_text_color().into())
            .finish(),
    );

    column.add_child(
        ConstrainedBox::new(ChildView::new(editor).finish())
            .with_max_width(INPUT_WIDTH)
            .finish(),
    );

    column.finish()
}

pub struct AddVpsModalViewState {
    state: ModalViewState<Modal<AddVpsModal>>,
}

impl AddVpsModalViewState {
    pub fn new(state: ModalViewState<Modal<AddVpsModal>>) -> Self {
        Self { state }
    }

    pub fn view(&self) -> &ViewHandle<Modal<AddVpsModal>> {
        &self.state.view
    }

    pub fn is_open(&self) -> bool {
        self.state.is_open()
    }

    pub fn render(&self) -> Box<dyn WarpuiElement> {
        self.state.render()
    }

    pub fn open<T: View>(&mut self, ctx: &mut ViewContext<T>) {
        self.state.open();
        self.state.view.update(ctx, |modal, ctx| {
            modal.body().update(ctx, |body, ctx| {
                body.on_open(ctx);
            });
        });
        ctx.notify();
    }

    pub fn close<T: View>(&mut self, ctx: &mut ViewContext<T>) {
        self.state.close();
        self.state.view.update(ctx, |modal, ctx| {
            modal.body().update(ctx, |body, ctx| {
                body.on_close(ctx);
            });
        });
        ctx.notify();
    }
}

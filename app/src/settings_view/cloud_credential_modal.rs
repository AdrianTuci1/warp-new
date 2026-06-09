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
use crate::view_components::{Dropdown, DropdownItem};

const LABEL_FONT_SIZE: f32 = 12.;
const INPUT_WIDTH: f32 = 480.;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudCredentialModalEvent {
    Close,
    AddEntry {
        platform: ai::cloud_credentials::CloudPlatform,
        name: String,
        host_or_key: String,
        vps_username: Option<String>,
        vps_ssh_key: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudCredentialModalAction {
    Cancel,
    Save,
    SelectPlatform(ai::cloud_credentials::CloudPlatform),
}

pub struct CloudCredentialModal {
    platform_dropdown: ViewHandle<Dropdown<CloudCredentialModalAction>>,
    name_editor: ViewHandle<EditorView>,
    host_or_key_editor: ViewHandle<EditorView>,
    vps_username_editor: ViewHandle<EditorView>,
    vps_ssh_key_editor: ViewHandle<EditorView>,
    selected_platform: ai::cloud_credentials::CloudPlatform,
    cancel_button_mouse_state: MouseStateHandle,
    save_button_mouse_state: MouseStateHandle,
}

impl CloudCredentialModal {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        use ai::cloud_credentials::CloudPlatform;

        let font_family = Appearance::as_ref(ctx).ui_font_family();
        let text_colors = crate::settings_view::editor_text_colors(Appearance::as_ref(ctx));
        let text_colors_2 = text_colors.clone();
        let text_colors_3 = text_colors.clone();
        let text_colors_4 = text_colors.clone();

        let platform_dropdown = ctx.add_typed_action_view(|ctx| {
            let mut dropdown = Dropdown::new(ctx);
            dropdown.set_items(
                vec![
                    DropdownItem::new(
                        CloudPlatform::Modal.label(),
                        CloudCredentialModalAction::SelectPlatform(CloudPlatform::Modal),
                    ),
                    DropdownItem::new(
                        CloudPlatform::Vps.label(),
                        CloudCredentialModalAction::SelectPlatform(CloudPlatform::Vps),
                    ),
                ],
                ctx,
            );
            dropdown.set_selected_by_name(CloudPlatform::Modal.label(), ctx);
            dropdown
        });

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

        let host_or_key_editor = ctx.add_typed_action_view(move |ctx| {
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
            editor.set_placeholder_text("Modal API key or VPS host", ctx);
            editor
        });

        let vps_username_editor = ctx.add_typed_action_view(move |ctx| {
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
            editor.set_placeholder_text("VPS username", ctx);
            editor
        });

        let vps_ssh_key_editor = ctx.add_typed_action_view(move |ctx| {
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
            editor.set_placeholder_text("VPS SSH private key", ctx);
            editor
        });

        ctx.subscribe_to_view(&name_editor, |me, _, event, ctx| {
            me.handle_editor_event(event, ctx);
        });
        ctx.subscribe_to_view(&host_or_key_editor, |me, _, event, ctx| {
            me.handle_editor_event(event, ctx);
        });
        ctx.subscribe_to_view(&vps_username_editor, |me, _, event, ctx| {
            me.handle_editor_event(event, ctx);
        });
        ctx.subscribe_to_view(&vps_ssh_key_editor, |me, _, event, ctx| {
            me.handle_editor_event(event, ctx);
        });

        Self {
            platform_dropdown,
            name_editor,
            host_or_key_editor,
            vps_username_editor,
            vps_ssh_key_editor,
            selected_platform: ai::cloud_credentials::CloudPlatform::Modal,
            cancel_button_mouse_state: Default::default(),
            save_button_mouse_state: Default::default(),
        }
    }

    fn sync_platform_controls(&mut self, ctx: &mut ViewContext<Self>) {
        let selected_label = self.selected_platform.label();
        self.platform_dropdown.update(ctx, |dropdown, ctx| {
            dropdown.set_selected_by_name(selected_label, ctx);
        });

        let name_placeholder = match self.selected_platform {
            ai::cloud_credentials::CloudPlatform::Modal => "e.g., Modal build pool",
            ai::cloud_credentials::CloudPlatform::Vps => "e.g., Production VPS",
        };
        self.name_editor.update(ctx, |editor, ctx| {
            editor.set_placeholder_text(name_placeholder, ctx);
        });

        let host_placeholder = match self.selected_platform {
            ai::cloud_credentials::CloudPlatform::Modal => "ak-...",
            ai::cloud_credentials::CloudPlatform::Vps => "server.example.com",
        };
        self.host_or_key_editor.update(ctx, |editor, ctx| {
            editor.set_placeholder_text(host_placeholder, ctx);
        });
    }

    pub fn on_open(&mut self, ctx: &mut ViewContext<Self>) {
        self.sync_platform_controls(ctx);
        ctx.focus(&self.name_editor);
        ctx.notify();
    }

    pub fn on_close(&mut self, ctx: &mut ViewContext<Self>) {
        self.name_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer_and_reset_undo_stack(ctx);
        });
        self.host_or_key_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer_and_reset_undo_stack(ctx);
        });
        self.vps_username_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer_and_reset_undo_stack(ctx);
        });
        self.vps_ssh_key_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer_and_reset_undo_stack(ctx);
        });
        self.selected_platform = ai::cloud_credentials::CloudPlatform::Modal;
        self.sync_platform_controls(ctx);
        ctx.notify();
    }

    fn save(&mut self, ctx: &mut ViewContext<Self>) {
        let name = self.name_editor.as_ref(ctx).buffer_text(ctx);
        let host_or_key = self.host_or_key_editor.as_ref(ctx).buffer_text(ctx);
        let vps_username = self.vps_username_editor.as_ref(ctx).buffer_text(ctx);
        let vps_ssh_key = self.vps_ssh_key_editor.as_ref(ctx).buffer_text(ctx);

        if !self.is_valid(ctx) {
            return;
        }

        ctx.emit(CloudCredentialModalEvent::AddEntry {
            platform: self.selected_platform,
            name,
            host_or_key,
            vps_username: if vps_username.trim().is_empty() {
                None
            } else {
                Some(vps_username)
            },
            vps_ssh_key: if vps_ssh_key.trim().is_empty() {
                None
            } else {
                Some(vps_ssh_key)
            },
        });
    }

    fn cancel(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(CloudCredentialModalEvent::Close);
    }

    fn handle_editor_event(&mut self, event: &EditorEvent, ctx: &mut ViewContext<Self>) {
        use warp_editor::editor::NavigationKey;
        match event {
            EditorEvent::Navigate(NavigationKey::Tab) => {
                ctx.focus(&self.host_or_key_editor);
            }
            EditorEvent::Enter => {
                self.save(ctx);
            }
            EditorEvent::Escape => {
                self.cancel(ctx);
            }
            EditorEvent::Edited(_) => {
                ctx.notify();
            }
            _ => {}
        }
    }

    fn is_valid(&self, app: &AppContext) -> bool {
        let host_or_key = self.host_or_key_editor.as_ref(app).buffer_text(app);
        if host_or_key.trim().is_empty() {
            return false;
        }

        if self.selected_platform == ai::cloud_credentials::CloudPlatform::Vps {
            let username = self.vps_username_editor.as_ref(app).buffer_text(app);
            let ssh_key = self.vps_ssh_key_editor.as_ref(app).buffer_text(app);
            return !username.trim().is_empty() && !ssh_key.trim().is_empty();
        }

        true
    }
}

impl Entity for CloudCredentialModal {
    type Event = CloudCredentialModalEvent;
}

impl TypedActionView for CloudCredentialModal {
    type Action = CloudCredentialModalAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CloudCredentialModalAction::Cancel => {
                self.cancel(ctx);
            }
            CloudCredentialModalAction::Save => {
                self.save(ctx);
            }
            CloudCredentialModalAction::SelectPlatform(platform) => {
                self.selected_platform = *platform;
                self.sync_platform_controls(ctx);
                ctx.notify();
            }
        }
    }
}

impl View for CloudCredentialModal {
    fn ui_name() -> &'static str {
        "CloudCredentialModal"
    }

    fn render(&self, app: &AppContext) -> Box<dyn WarpuiElement> {
        use ai::cloud_credentials::CloudPlatform;
        let appearance = Appearance::as_ref(app);
        let mut col = Flex::column();

        let platform_label = Text::new_inline(
            "Credential type",
            appearance.ui_font_family(),
            LABEL_FONT_SIZE,
        )
        .with_style(Properties::default().weight(Weight::Semibold))
        .with_color(appearance.theme().active_ui_text_color().into())
        .finish();
        col.add_child(
            Container::new(platform_label)
                .with_margin_bottom(4.)
                .finish(),
        );
        col.add_child(
            ConstrainedBox::new(ChildView::new(&self.platform_dropdown).finish())
                .with_width(INPUT_WIDTH)
                .finish(),
        );
        col.add_child(
            Container::new(
                Text::new_inline(
                    platform_description(self.selected_platform),
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_color(appearance.theme().nonactive_ui_text_color().into())
                .finish(),
            )
            .with_margin_top(6.)
            .finish(),
        );

        col.add_child(render_form_label(appearance, "Display name"));
        col.add_child(render_editor_container(appearance, &self.name_editor));

        let host_label = match self.selected_platform {
            CloudPlatform::Modal => "Modal API Key",
            CloudPlatform::Vps => "VPS Host (IP or hostname)",
        };
        col.add_child(render_form_label(appearance, host_label));
        col.add_child(render_editor_container(
            appearance,
            &self.host_or_key_editor,
        ));

        // VPS-only fields
        if self.selected_platform == CloudPlatform::Vps {
            col.add_child(render_form_label(appearance, "VPS Username"));
            col.add_child(render_editor_container(
                appearance,
                &self.vps_username_editor,
            ));
            col.add_child(render_form_label(appearance, "VPS SSH Private Key"));
            col.add_child(render_editor_container(
                appearance,
                &self.vps_ssh_key_editor,
            ));
        }

        // Buttons
        let mut button_row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(warpui::elements::MainAxisAlignment::End)
            .with_spacing(8.);

        let cancel_btn = appearance
            .ui_builder()
            .button(ButtonVariant::Basic, self.cancel_button_mouse_state.clone())
            .with_text_label("Cancel".to_string())
            .build()
            .on_click(|ctx: &mut warpui::elements::EventContext, _, _| {
                ctx.dispatch_typed_action(CloudCredentialModalAction::Cancel);
            })
            .finish();
        button_row.add_child(cancel_btn);

        let mut save_button = appearance
            .ui_builder()
            .button(ButtonVariant::Accent, self.save_button_mouse_state.clone())
            .with_text_label("Save".to_string());
        if !self.is_valid(app) {
            save_button = save_button.disabled();
        }
        let save_btn = save_button
            .build()
            .on_click(|ctx: &mut warpui::elements::EventContext, _, _| {
                ctx.dispatch_typed_action(CloudCredentialModalAction::Save);
            })
            .finish();
        button_row.add_child(save_btn);

        col.add_child(
            Container::new(button_row.finish())
                .with_margin_top(16.)
                .finish(),
        );

        col.finish()
    }
}

fn render_form_label(appearance: &Appearance, label: &str) -> Box<dyn WarpuiElement> {
    Container::new(
        Text::new_inline(label.to_string(), appearance.ui_font_family(), 12.)
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(appearance.theme().active_ui_text_color().into())
            .finish(),
    )
    .with_margin_top(8.)
    .with_margin_bottom(4.)
    .finish()
}

fn render_editor_container(
    appearance: &Appearance,
    editor: &ViewHandle<EditorView>,
) -> Box<dyn WarpuiElement> {
    Container::new(
        ConstrainedBox::new(ChildView::new(editor).finish())
            .with_width(INPUT_WIDTH)
            .finish(),
    )
    .with_padding(warpui::elements::Padding::uniform(8.))
    .with_background(appearance.theme().surface_1())
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(6.)))
    .finish()
}

fn platform_description(platform: ai::cloud_credentials::CloudPlatform) -> &'static str {
    match platform {
        ai::cloud_credentials::CloudPlatform::Modal => {
            "Use a Modal API key for managed cloud workloads."
        }
        ai::cloud_credentials::CloudPlatform::Vps => {
            "Use a host, username, and SSH key for your own server."
        }
    }
}

pub struct CloudCredentialModalViewState {
    state: ModalViewState<Modal<CloudCredentialModal>>,
}

impl CloudCredentialModalViewState {
    pub fn new(state: ModalViewState<Modal<CloudCredentialModal>>) -> Self {
        Self { state }
    }

    pub fn view(&self) -> &ViewHandle<Modal<CloudCredentialModal>> {
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

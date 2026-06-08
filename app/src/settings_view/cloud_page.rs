use std::sync::Arc;

use warp_core::report_if_error;
use warpui::elements::{
    Container, CornerRadius, Element, Flex, MainAxisAlignment, MouseStateHandle, ParentElement,
    Radius, Shrinkable, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::button::{Button, ButtonVariant};
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::{
    Action, AppContext, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle,
};

use super::settings_page::{
    render_body_item, AdditionalInfo, MatchData, PageType, SettingsPageMeta,
    SettingsPageViewHandle, SettingsWidget, ToggleState,
};
use super::{LocalOnlyIconState, SettingsAction, SettingsSection};
use crate::appearance::Appearance;
use crate::editor::{
    EditorView, Event as EditorEvent, PropagateAndNoOpNavigationKeys, SingleLineEditorOptions,
    TextOptions,
};
use crate::view_components::dropdown::{Dropdown, DropdownAction, DropdownItem};

#[derive(Debug, Clone)]
pub enum CloudSettingsPageAction {
    AddEntry,
    RemoveEntry(String),
    SaveEntry,
    SelectPlatform(ai::cloud_credentials::CloudPlatform),
}

#[derive(Debug, Clone)]
pub enum CloudSettingsPageEvent {
    CredentialsUpdated,
}

pub struct CloudSettingsPageView {
    page: PageType<Self>,
    /// Editor for entry name/label
    name_editor: ViewHandle<EditorView>,
    /// Editor for host or API key
    host_or_key_editor: ViewHandle<EditorView>,
    /// Editor for VPS username
    vps_username_editor: ViewHandle<EditorView>,
    /// Editor for VPS SSH key
    vps_ssh_key_editor: ViewHandle<EditorView>,
    /// Currently selected platform for the "add" form
    selected_platform: ai::cloud_credentials::CloudPlatform,
    /// Whether the add form is visible
    show_add_form: bool,
}

impl CloudSettingsPageView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let font_family = Appearance::as_ref(ctx).ui_font_family();

        let create_editor = |ctx: &mut ViewContext<EditorView>, placeholder: &str| {
            let options = SingleLineEditorOptions {
                text: TextOptions {
                    font_family_override: Some(font_family),
                    ..Default::default()
                },
                propagate_and_no_op_vertical_navigation_keys:
                    PropagateAndNoOpNavigationKeys::Always,
                ..Default::default()
            };
            let mut editor = EditorView::single_line(options, ctx);
            editor.set_placeholder_text(placeholder, ctx);
            editor
        };

        let name_editor = ctx.add_typed_action_view(|ctx| create_editor(ctx, "Name (e.g. Production VPS)"));
        let host_or_key_editor =
            ctx.add_typed_action_view(|ctx| create_editor(ctx, "Modal API key or VPS host"));
        let vps_username_editor = ctx.add_typed_action_view(|ctx| create_editor(ctx, "VPS username"));
        let vps_ssh_key_editor = ctx.add_typed_action_view(|ctx| create_editor(ctx, "VPS SSH private key"));

        ctx.subscribe_to_view(&name_editor, |me, _, event, ctx| {
            if let EditorEvent::Blurred = event {
                me.handle_action(&CloudSettingsPageAction::SaveEntry, ctx);
            }
        });
        ctx.subscribe_to_view(&host_or_key_editor, |me, _, event, ctx| {
            if let EditorEvent::Blurred = event {
                me.handle_action(&CloudSettingsPageAction::SaveEntry, ctx);
            }
        });
        ctx.subscribe_to_view(&vps_username_editor, |me, _, event, ctx| {
            if let EditorEvent::Blurred = event {
                me.handle_action(&CloudSettingsPageAction::SaveEntry, ctx);
            }
        });
        ctx.subscribe_to_view(&vps_ssh_key_editor, |me, _, event, ctx| {
            if let EditorEvent::Blurred = event {
                me.handle_action(&CloudSettingsPageAction::SaveEntry, ctx);
            }
        });

        let mut view = Self {
            page: PageType::new_uncategorized(
                vec![Box::new(CloudCredentialsWidget::default())],
                Some("Cloud"),
            ),
            name_editor,
            host_or_key_editor,
            vps_username_editor,
            vps_ssh_key_editor,
            selected_platform: ai::cloud_credentials::CloudPlatform::Modal,
            show_add_form: false,
        };

        ctx.subscribe_to_model(
            &ai::cloud_credentials::CloudCredentialsManager::handle(ctx),
            |me, _model, _event, ctx| {
                ctx.notify();
            },
        );

        view
    }

    fn clear_form(&mut self, ctx: &mut ViewContext<Self>) {
        self.name_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer(ctx);
        });
        self.host_or_key_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer(ctx);
        });
        self.vps_username_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer(ctx);
        });
        self.vps_ssh_key_editor.update(ctx, |editor, ctx| {
            editor.clear_buffer(ctx);
        });
        self.selected_platform = ai::cloud_credentials::CloudPlatform::Modal;
        self.show_add_form = false;
    }

    fn read_editor_text(&self, editor: &ViewHandle<EditorView>, ctx: &ViewContext<Self>) -> String {
        editor.read(ctx, |editor, ctx| editor.buffer_text(ctx).to_string())
    }
}

impl Entity for CloudSettingsPageView {
    type Event = CloudSettingsPageEvent;
}

impl TypedActionView for CloudSettingsPageView {
    type Action = CloudSettingsPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        use ai::cloud_credentials::{
            CloudCredentialEntry, CloudCredentialsManager, CloudPlatform,
        };
        match action {
            CloudSettingsPageAction::AddEntry => {
                self.show_add_form = true;
                ctx.notify();
            }
            CloudSettingsPageAction::RemoveEntry(id) => {
                CloudCredentialsManager::handle(ctx).update(ctx, |manager, ctx| {
                    manager.remove_entry(id, ctx);
                });
                ctx.notify();
            }
            CloudSettingsPageAction::SaveEntry => {
                let name = self.read_editor_text(&self.name_editor, ctx);
                let host_or_key = self.read_editor_text(&self.host_or_key_editor, ctx);
                let vps_username = self.read_editor_text(&self.vps_username_editor, ctx);
                let vps_ssh_key = self.read_editor_text(&self.vps_ssh_key_editor, ctx);

                if host_or_key.trim().is_empty() {
                    return;
                }

                let entry = CloudCredentialEntry {
                    id: format!("{}", uuid::Uuid::new_v4()),
                    platform: self.selected_platform,
                    name: if name.trim().is_empty() {
                        None
                    } else {
                        Some(name)
                    },
                    host_or_key: Some(host_or_key),
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
                };

                CloudCredentialsManager::handle(ctx).update(ctx, |manager, ctx| {
                    manager.add_entry(entry, ctx);
                });
                self.clear_form(ctx);
                ctx.notify();
            }
            CloudSettingsPageAction::SelectPlatform(platform) => {
                self.selected_platform = *platform;
                ctx.notify();
            }
        }
    }
}

impl View for CloudSettingsPageView {
    fn ui_name() -> &'static str {
        "CloudSettingsPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

impl SettingsPageMeta for CloudSettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::CloudPlatform
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        true
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<CloudSettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<CloudSettingsPageView>) -> Self {
        SettingsPageViewHandle::CloudPlatform(view_handle)
    }
}

#[derive(Default)]
struct CloudCredentialsWidget {
    add_button_mouse_state: MouseStateHandle,
    remove_button_mouse_states: Vec<(String, MouseStateHandle)>,
    platform_dropdown: Option<ViewHandle<Dropdown>>,
}

impl SettingsWidget for CloudCredentialsWidget {
    type View = CloudSettingsPageView;

    fn search_terms(&self) -> &str {
        "cloud vps modal credentials ssh api key"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        use ai::cloud_credentials::{CloudCredentialsManager, CloudPlatform};
        let mut col = Flex::column();

        // Description
        col.add_child(
            Container::new(
                Text::new_inline(
                    "Configure credentials to launch Cloud Agents or Subagents on your VPS or Modal.",
                    appearance.ui_font_family(),
                    12.,
                )
                .with_color(appearance.theme().nonactive_ui_text_color().into())
                .finish(),
            )
            .with_margin_bottom(16.)
            .finish(),
        );

        // List existing entries
        let manager = CloudCredentialsManager::as_ref(app);
        let creds = manager.credentials();

        if !creds.entries().is_empty() {
            for entry in creds.entries() {
                col.add_child(render_entry_row(appearance, entry));
            }
        }

        // Add button
        let add_button = appearance
            .ui_builder()
            .button(ButtonVariant::Accent, self.add_button_mouse_state.clone())
            .with_text_label("+ Add credential".to_string())
            .build()
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(CloudSettingsPageAction::AddEntry);
            })
            .finish();
        col.add_child(Container::new(add_button).with_margin_top(8.).finish());

        // Add form (shown when show_add_form is true)
        if view.show_add_form {
            col.add_child(render_add_form(appearance, view));
        }

        col.finish()
    }
}

fn render_entry_row(
    appearance: &Appearance,
    entry: &ai::cloud_credentials::CloudCredentialEntry,
) -> Box<dyn Element> {
    use ai::cloud_credentials::CloudPlatform;
    let mut row = Flex::row()
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_cross_axis_alignment(warpui::elements::CrossAxisAlignment::Center);

    let label = entry
        .name
        .as_deref()
        .unwrap_or_else(|| match entry.platform {
            CloudPlatform::Modal => "Modal",
            CloudPlatform::Vps => "VPS",
        });

    let platform_icon = match entry.platform {
        CloudPlatform::Modal => "☁️",
        CloudPlatform::Vps => "🖥️",
    };

    let info_text = format!(
        "{} {} — {}",
        platform_icon,
        entry.platform.label(),
        label
    );

    row.add_child(
        Text::new_inline(info_text, appearance.ui_font_family(), 12.)
            .with_color(appearance.theme().active_ui_text_color().into())
            .finish(),
    );

    let host_display = entry
        .host_or_key
        .as_deref()
        .unwrap_or("")
        .chars()
        .take(20)
        .collect::<String>();
    if !host_display.is_empty() {
        row.add_child(
            Text::new_inline(
                format!("{}", host_display),
                appearance.ui_font_family(),
                11.,
            )
            .with_color(appearance.theme().nonactive_ui_text_color().into())
            .finish(),
        );
    }

    Container::new(row.finish())
        .with_padding(warpui::elements::Padding::uniform(10.))
        .with_background(appearance.theme().surface_2())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(6.)))
        .with_margin_bottom(8.)
        .finish()
}

fn render_add_form(appearance: &Appearance, view: &CloudSettingsPageView) -> Box<dyn Element> {
    use ai::cloud_credentials::CloudPlatform;
    let mut col = Flex::column();

    // Platform selector
    let platform_label = Text::new_inline("Platform", appearance.ui_font_family(), 12.)
        .with_style(Properties::default().weight(Weight::Semibold))
        .with_color(appearance.theme().active_ui_text_color().into())
        .finish();
    col.add_child(Container::new(platform_label).with_margin_bottom(4.).finish());

    let platform_buttons = Flex::row().with_spacing(8.);
    let modal_selected = view.selected_platform == CloudPlatform::Modal;
    let vps_selected = view.selected_platform == CloudPlatform::Vps;

    let modal_btn = render_platform_button(
        appearance,
        "Modal",
        modal_selected,
        CloudSettingsPageAction::SelectPlatform(CloudPlatform::Modal),
    );
    let vps_btn = render_platform_button(
        appearance,
        "VPS",
        vps_selected,
        CloudSettingsPageAction::SelectPlatform(CloudPlatform::Vps),
    );

    col.add_child(
        Flex::row()
            .with_spacing(8.)
            .with_child(modal_btn)
            .with_child(vps_btn)
            .finish(),
    );

    // Name field
    col.add_child(render_form_label(appearance, "Name"));
    col.add_child(render_editor_container(appearance, &view.name_editor));

    // Host / API Key field
    let host_label = match view.selected_platform {
        CloudPlatform::Modal => "Modal API Key",
        CloudPlatform::Vps => "VPS Host (IP or hostname)",
    };
    col.add_child(render_form_label(appearance, host_label));
    col.add_child(render_editor_container(appearance, &view.host_or_key_editor));

    // VPS-only fields
    if view.selected_platform == CloudPlatform::Vps {
        col.add_child(render_form_label(appearance, "VPS Username"));
        col.add_child(render_editor_container(appearance, &view.vps_username_editor));
        col.add_child(render_form_label(appearance, "VPS SSH Private Key"));
        col.add_child(render_editor_container(appearance, &view.vps_ssh_key_editor));
    }

    // Save button
    let save_btn = appearance
        .ui_builder()
        .button(ButtonVariant::Accent, Default::default())
        .with_text_label("Save".to_string())
        .build()
        .on_click(|ctx, _, _| {
            ctx.dispatch_typed_action(CloudSettingsPageAction::SaveEntry);
        })
        .finish();
    col.add_child(Container::new(save_btn).with_margin_top(8.).finish());

    Container::new(col.finish())
        .with_padding(warpui::elements::Padding::uniform(12.))
        .with_background(appearance.theme().surface_2())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
        .with_margin_top(12.)
        .finish()
}

fn render_platform_button(
    appearance: &Appearance,
    label: &str,
    selected: bool,
    action: CloudSettingsPageAction,
) -> Box<dyn Element> {
    let bg = if selected {
        appearance.theme().accent()
    } else {
        appearance.theme().surface_3()
    };
    let text_color = if selected {
        appearance.theme().background()
    } else {
        appearance.theme().active_ui_text_color()
    };

    Container::new(
        Text::new_inline(label.to_string(), appearance.ui_font_family(), 12.)
            .with_color(text_color.into())
            .finish(),
    )
    .with_padding(warpui::elements::Padding::uniform(6.))
    .with_background(bg)
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
    .finish()
}

fn render_form_label(appearance: &Appearance, label: &str) -> Box<dyn Element> {
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
) -> Box<dyn Element> {
    Container::new(warpui::elements::ChildView::new(editor).finish())
        .with_padding(warpui::elements::Padding::uniform(8.))
        .with_background(appearance.theme().surface_1())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(6.)))
        .finish()
}

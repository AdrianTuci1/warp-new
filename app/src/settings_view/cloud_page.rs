use warp_core::report_if_error;
use warpui::elements::{
    Container, CrossAxisAlignment, Element, Flex, MainAxisAlignment, MainAxisSize,
    MouseStateHandle, ParentElement, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::{
    id, Action, AppContext, Element as WarpuiElement, Entity, SingletonEntity, TypedActionView,
    View, ViewContext, ViewHandle,
};

use super::cloud_credential_modal::{
    CloudCredentialModal, CloudCredentialModalEvent, CloudCredentialModalViewState,
};
use super::settings_page::{
    MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle, SettingsWidget,
};
use super::SettingsSection;
use crate::appearance::Appearance;
use crate::modal::{Modal, ModalEvent, ModalViewState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CloudSettingsPageAction {
    AddEntry,
    RemoveEntry { index: usize },
    CloseModal,
}

pub struct CloudSettingsPageView {
    page: PageType<Self>,
    add_button_mouse_state: MouseStateHandle,
    modal_state: CloudCredentialModalViewState,
}

impl CloudSettingsPageView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let modal_view = ctx.add_typed_action_view(CloudCredentialModal::new);
        let modal_state = CloudCredentialModalViewState::new(ModalViewState::new(
            ctx.add_typed_action_view(|ctx| {
                Modal::new(
                    Some("Add Cloud Credential".to_string()),
                    modal_view.clone(),
                    ctx,
                )
                .with_body_style(UiComponentStyles {
                    height: Some(420.),
                    ..Default::default()
                })
            }),
        ));

        ctx.subscribe_to_view(&modal_state.view(), |me, _, event, ctx| {
            if let ModalEvent::Close = event {
                me.modal_state.close(ctx);
            }
        });

        ctx.subscribe_to_view(&modal_view, |me, _, event, ctx| {
            me.handle_modal_event(event, ctx);
        });

        ctx.subscribe_to_model(
            &ai::cloud_credentials::CloudCredentialsManager::handle(ctx),
            |me, _model, _event, ctx| {
                ctx.notify();
            },
        );

        Self {
            page: PageType::new_uncategorized(
                vec![Box::new(CloudCredentialsListWidget::default())],
                None,
            ),
            add_button_mouse_state: Default::default(),
            modal_state,
        }
    }

    fn handle_modal_event(
        &mut self,
        event: &CloudCredentialModalEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            CloudCredentialModalEvent::Close => {
                self.modal_state.close(ctx);
            }
            CloudCredentialModalEvent::AddEntry {
                platform,
                name,
                host_or_key,
                vps_username,
                vps_ssh_key,
            } => {
                let manager = ai::cloud_credentials::CloudCredentialsManager::handle(ctx);
                manager.update(ctx, |manager, ctx| {
                    manager.add_entry(
                        ai::cloud_credentials::CloudCredentialEntry {
                            id: uuid::Uuid::new_v4().to_string(),
                            platform: *platform,
                            name: Some(name.clone()),
                            host_or_key: Some(host_or_key.clone()),
                            vps_username: vps_username.clone(),
                            vps_ssh_key: vps_ssh_key.clone(),
                        },
                        ctx,
                    );
                });
                self.modal_state.close(ctx);
            }
        }
    }

    pub fn open_add_modal(&mut self, ctx: &mut ViewContext<Self>) {
        self.modal_state.open(ctx);
    }
}

impl Entity for CloudSettingsPageView {
    type Event = ();
}

impl SingletonEntity for CloudSettingsPageView {}

impl TypedActionView for CloudSettingsPageView {
    type Action = CloudSettingsPageAction;

    fn handle_action(
        &mut self, action: &Self::Action, ctx: &mut ViewContext<Self>,
    ) {
        match action {
            CloudSettingsPageAction::AddEntry => {
                self.open_add_modal(ctx);
            }
            CloudSettingsPageAction::RemoveEntry { index } => {
                let manager = ai::cloud_credentials::CloudCredentialsManager::handle(ctx);
                let credentials = manager.as_ref(ctx).credentials().clone();
                if let Some(entry) = credentials.entries.get(*index) {
                    let id = entry.id.clone();
                    manager.update(ctx, |manager, ctx| {
                        manager.remove_entry(&id, ctx);
                    });
                }
            }
            CloudSettingsPageAction::CloseModal => {
                self.modal_state.close(ctx);
            }
        }
    }
}

impl View for CloudSettingsPageView {
    fn ui_name() -> &'static str {
        "CloudSettingsPageView"
    }

    fn render(&self, app: &AppContext) -> Box<dyn WarpuiElement> {
        let mut stack = warpui::elements::Stack::new();
        stack.add_child(self.page.render(self, app));

        if self.modal_state.is_open() {
            stack.add_child(self.modal_state.render());
        }

        stack.finish()
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
struct CloudCredentialsListWidget;

impl SettingsWidget for CloudCredentialsListWidget {
    type View = CloudSettingsPageView;

    fn search_terms(&self) -> &str {
        "cloud platform credential modal vps ssh api key"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn WarpuiElement> {
        let manager = ai::cloud_credentials::CloudCredentialsManager::as_ref(app);
        let credentials = manager.credentials();

        let mut col = Flex::column();

        // Header with add button
        let mut header = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);

        header.add_child(
            Text::new_inline(
                "Cloud Credentials",
                appearance.ui_font_family(),
                appearance.header_font_size(),
            )
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(appearance.theme().active_ui_text_color().into())
            .finish(),
        );

        let add_button = appearance
            .ui_builder()
            .button(ButtonVariant::Accent, view.add_button_mouse_state.clone())
            .with_text_label("+ Add credential".to_string())
            .build()
            .on_click(|ctx, _, _| {
                ctx.dispatch_typed_action(CloudSettingsPageAction::AddEntry);
            })
            .finish();
        header.add_child(add_button);

        col.add_child(header.finish());
        col.add_child(super::settings_page::render_separator(appearance));

        if credentials.entries.is_empty() {
            col.add_child(
                Text::new_inline(
                    "No credentials configured. Click \"+ Add credential\" to add one.",
                    appearance.ui_font_family(),
                    12.,
                )
                .with_color(appearance.theme().disabled_ui_text_color().into())
                .finish(),
            );
        } else {
            for (index, entry) in credentials.entries.iter().enumerate() {
                col.add_child(render_credential_row(view, entry, index, appearance));
            }
        }

        col.finish()
    }
}

fn render_credential_row(
    _view: &CloudSettingsPageView,
    entry: &ai::cloud_credentials::CloudCredentialEntry,
    index: usize,
    appearance: &Appearance,
) -> Box<dyn WarpuiElement> {
    use ai::cloud_credentials::CloudPlatform;

    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_cross_axis_alignment(CrossAxisAlignment::Center);

    let platform_icon = match entry.platform {
        CloudPlatform::Modal => "☁️",
        CloudPlatform::Vps => "🖥️",
    };

    let label = if entry.name.as_deref().unwrap_or("").trim().is_empty() {
        entry.host_or_key.as_deref().unwrap_or("").to_string()
    } else {
        entry.name.as_deref().unwrap_or("").to_string()
    };

    let info_text = format!("{} {} — {}", platform_icon, entry.platform.label(), label);

    row.add_child(
        Text::new_inline(info_text, appearance.ui_font_family(), 12.)
            .with_color(appearance.theme().active_ui_text_color().into())
            .finish(),
    );

    let remove_btn = appearance
        .ui_builder()
        .button(ButtonVariant::Basic, Default::default())
        .with_text_label("Remove".to_string())
        .build()
        .on_click(move |ctx, _, _| {
            ctx.dispatch_typed_action(CloudSettingsPageAction::RemoveEntry { index });
        })
        .finish();
    row.add_child(remove_btn);

    Container::new(row.finish())
        .with_padding_top(8.)
        .with_padding_bottom(8.)
        .finish()
}

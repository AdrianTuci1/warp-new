use warpui::elements::{
    Border, Container, CornerRadius, CrossAxisAlignment, Flex, MainAxisAlignment, MainAxisSize,
    MouseStateHandle, ParentElement, Radius, Shrinkable, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::{
    AppContext, Element as WarpuiElement, Entity, SingletonEntity, TypedActionView, View,
    ViewContext, ViewHandle,
};

use super::SettingsSection;
use super::cloud_credential_modal::{
    CloudCredentialModal, CloudCredentialModalEvent, CloudCredentialModalViewState,
};
use super::settings_page::{
    MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle, SettingsWidget,
};
use crate::appearance::Appearance;
use crate::modal::{Modal, ModalEvent, ModalViewState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CloudSettingsPageAction {
    AddEntry,
    RemoveEntry { index: usize },
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
                .with_dismiss_on_click()
                .with_body_style(UiComponentStyles {
                    height: Some(460.),
                    ..Default::default()
                })
            }),
        ));

        ctx.subscribe_to_view(&modal_state.view(), |me, _, event, ctx| {
            let ModalEvent::Close = event;
            me.modal_state.close(ctx);
        });

        ctx.subscribe_to_view(&modal_view, |me, _, event, ctx| {
            me.handle_modal_event(event, ctx);
        });

        ctx.subscribe_to_model(
            &ai::cloud_credentials::CloudCredentialsManager::handle(ctx),
            |_me, _model, _event, ctx| {
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

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
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
        "cloud platform credential modal vps ssh api key octo octomus"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn WarpuiElement> {
        let manager = ai::cloud_credentials::CloudCredentialsManager::as_ref(app);
        let credentials = manager.credentials();
        let modal_count = credentials.modal_entries().count();
        let vps_count = credentials.vps_entries().count();

        let mut column = Flex::column().with_spacing(16.);
        column.add_child(render_overview_card(
            view,
            appearance,
            credentials.entries.len(),
            modal_count,
            vps_count,
        ));

        if credentials.entries.is_empty() {
            column.add_child(render_empty_state_card(appearance));
        } else {
            let mut list = Flex::column().with_spacing(12.);
            list.add_child(
                Text::new_inline(
                    format!("Saved credentials ({})", credentials.entries.len()),
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(appearance.theme().active_ui_text_color().into())
                .finish(),
            );

            for (index, entry) in credentials.entries.iter().enumerate() {
                list.add_child(render_credential_card(entry, index, appearance));
            }

            column.add_child(list.finish());
        }

        column.finish()
    }
}

fn render_overview_card(
    view: &CloudSettingsPageView,
    appearance: &Appearance,
    total_count: usize,
    modal_count: usize,
    vps_count: usize,
) -> Box<dyn WarpuiElement> {
    let theme = appearance.theme();
    let add_button = appearance
        .ui_builder()
        .button(ButtonVariant::Accent, view.add_button_mouse_state.clone())
        .with_text_label("Add credential".to_string())
        .build()
        .on_click(|ctx, _, _| {
            ctx.dispatch_typed_action(CloudSettingsPageAction::AddEntry);
        })
        .finish();

    let title_block = Flex::column()
        .with_spacing(8.)
        .with_child(
            Text::new_inline(
                "Cloud Platform",
                appearance.ui_font_family(),
                appearance.header_font_size(),
            )
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(theme.active_ui_text_color().into())
            .finish(),
        )
        .with_child(
            Text::new_inline(
                "Store Modal and VPS credentials for Octo cloud workflows in one place.",
                appearance.ui_font_family(),
                appearance.ui_font_size(),
            )
            .with_color(theme.nonactive_ui_text_color().into())
            .finish(),
        )
        .finish();

    let stats = Flex::row()
        .with_spacing(8.)
        .with_child(render_stat_pill(appearance, format!("{total_count} total")))
        .with_child(render_stat_pill(appearance, format!("{modal_count} Modal")))
        .with_child(render_stat_pill(appearance, format!("{vps_count} VPS")))
        .finish();

    let header = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Shrinkable::new(1., title_block).finish())
        .with_child(add_button)
        .finish();

    render_card(
        appearance,
        Flex::column()
            .with_spacing(16.)
            .with_child(header)
            .with_child(stats)
            .finish(),
    )
}

fn render_empty_state_card(appearance: &Appearance) -> Box<dyn WarpuiElement> {
    render_card(
        appearance,
        Flex::column()
            .with_spacing(8.)
            .with_child(
                Text::new_inline(
                    "No cloud credentials yet",
                    appearance.ui_font_family(),
                    appearance.ui_font_size() + 1.,
                )
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(appearance.theme().active_ui_text_color().into())
                .finish(),
            )
            .with_child(
                Text::new_inline(
                    "Add a Modal API key or a VPS host profile to start wiring Octo into your cloud setup.",
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_color(appearance.theme().nonactive_ui_text_color().into())
                .finish(),
            )
            .finish(),
    )
}

fn render_credential_card(
    entry: &ai::cloud_credentials::CloudCredentialEntry,
    index: usize,
    appearance: &Appearance,
) -> Box<dyn WarpuiElement> {
    let title = credential_title(entry);
    let detail = credential_detail(entry);

    let mut header = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_cross_axis_alignment(CrossAxisAlignment::Center);
    header.add_child(
        Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(10.)
            .with_child(render_platform_badge(appearance, entry.platform.label()))
            .with_child(
                Text::new_inline(
                    title,
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(appearance.theme().active_ui_text_color().into())
                .finish(),
            )
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
    header.add_child(remove_btn);

    render_card(
        appearance,
        Flex::column()
            .with_spacing(10.)
            .with_child(header.finish())
            .with_child(
                Text::new_inline(
                    detail,
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_color(appearance.theme().nonactive_ui_text_color().into())
                .finish(),
            )
            .finish(),
    )
}

fn render_stat_pill(appearance: &Appearance, label: String) -> Box<dyn WarpuiElement> {
    Container::new(
        Text::new_inline(
            label,
            appearance.ui_font_family(),
            appearance.ui_font_size() - 1.,
        )
        .with_style(Properties::default().weight(Weight::Semibold))
        .with_color(appearance.theme().active_ui_text_color().into())
        .finish(),
    )
    .with_horizontal_padding(10.)
    .with_vertical_padding(6.)
    .with_background(appearance.theme().surface_2())
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(999.)))
    .finish()
}

fn render_platform_badge(appearance: &Appearance, label: &str) -> Box<dyn WarpuiElement> {
    Container::new(
        Text::new_inline(
            label.to_string(),
            appearance.ui_font_family(),
            appearance.ui_font_size() - 1.,
        )
        .with_style(Properties::default().weight(Weight::Semibold))
        .with_color(appearance.theme().active_ui_text_color().into())
        .finish(),
    )
    .with_horizontal_padding(8.)
    .with_vertical_padding(4.)
    .with_background(appearance.theme().surface_2())
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(999.)))
    .finish()
}

fn render_card(appearance: &Appearance, body: Box<dyn WarpuiElement>) -> Box<dyn WarpuiElement> {
    Container::new(body)
        .with_uniform_padding(16.)
        .with_background(appearance.theme().surface_1())
        .with_border(Border::all(1.).with_border_fill(appearance.theme().outline()))
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
        .finish()
}

fn credential_title(entry: &ai::cloud_credentials::CloudCredentialEntry) -> String {
    let name = entry.name.as_deref().unwrap_or("").trim();
    if !name.is_empty() {
        return name.to_string();
    }

    match entry.platform {
        ai::cloud_credentials::CloudPlatform::Modal => "Modal credential".to_string(),
        ai::cloud_credentials::CloudPlatform::Vps => "VPS credential".to_string(),
    }
}

fn credential_detail(entry: &ai::cloud_credentials::CloudCredentialEntry) -> String {
    match entry.platform {
        ai::cloud_credentials::CloudPlatform::Modal => {
            let api_key = entry.host_or_key.as_deref().unwrap_or("").trim();
            if api_key.is_empty() {
                "Modal API key not set".to_string()
            } else {
                format!("API key ending in {}", trailing_chars(api_key, 4))
            }
        }
        ai::cloud_credentials::CloudPlatform::Vps => {
            let host = entry.host_or_key.as_deref().unwrap_or("").trim();
            let username = entry.vps_username.as_deref().unwrap_or("").trim();

            match (host.is_empty(), username.is_empty()) {
                (false, false) => format!("{host} • {username}"),
                (false, true) => host.to_string(),
                (true, false) => username.to_string(),
                (true, true) => "VPS host not set".to_string(),
            }
        }
    }
}

fn trailing_chars(value: &str, count: usize) -> String {
    value
        .chars()
        .rev()
        .take(count)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{credential_detail, credential_title, trailing_chars};
    use ai::cloud_credentials::{CloudCredentialEntry, CloudPlatform};

    #[test]
    fn trailing_chars_returns_suffix() {
        assert_eq!(trailing_chars("abcdef", 4), "cdef");
        assert_eq!(trailing_chars("abc", 4), "abc");
    }

    #[test]
    fn modal_credentials_mask_api_key_in_detail() {
        let entry = CloudCredentialEntry {
            platform: CloudPlatform::Modal,
            host_or_key: Some("ak-modal-1234".to_string()),
            ..Default::default()
        };

        assert_eq!(credential_title(&entry), "Modal credential");
        assert_eq!(credential_detail(&entry), "API key ending in 1234");
    }

    #[test]
    fn vps_credentials_show_host_and_username() {
        let entry = CloudCredentialEntry {
            platform: CloudPlatform::Vps,
            host_or_key: Some("server.example.com".to_string()),
            vps_username: Some("root".to_string()),
            ..Default::default()
        };

        assert_eq!(credential_title(&entry), "VPS credential");
        assert_eq!(credential_detail(&entry), "server.example.com • root");
    }
}

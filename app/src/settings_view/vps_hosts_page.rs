use warpui::elements::{
    Border, ChildView, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, Flex,
    MainAxisAlignment, MainAxisSize, MouseStateHandle, ParentElement, Radius, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::{UiComponent, UiComponentStyles};
use warpui::{
    AppContext, Element as WarpuiElement, Entity, SingletonEntity, TypedActionView, View,
    ViewContext, ViewHandle,
};

use super::SettingsSection;
use super::add_vps_modal::{AddVpsModal, AddVpsModalEvent, AddVpsModalViewState};
use super::settings_page::{
    MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle, SettingsWidget,
};
use crate::appearance::Appearance;
use crate::modal::{Modal, ModalEvent, ModalViewState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VpsHostsPageAction {
    AddEntry,
    RemoveEntry { index: usize },
}

pub struct VpsHostsPageView {
    page: PageType<Self>,
    add_button_mouse_state: MouseStateHandle,
    modal_state: AddVpsModalViewState,
}

impl VpsHostsPageView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let modal_view = ctx.add_typed_action_view(AddVpsModal::new);
        let modal_state = AddVpsModalViewState::new(ModalViewState::new(
            ctx.add_typed_action_view(|ctx| {
                Modal::new(
                    Some("Add VPS Host".to_string()),
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
                vec![Box::new(VpsHostsListWidget::default())],
                None,
            ),
            add_button_mouse_state: Default::default(),
            modal_state,
        }
    }

    fn handle_modal_event(
        &mut self,
        event: &AddVpsModalEvent,
        ctx: &mut ViewContext<Self>,
    ) {
        match event {
            AddVpsModalEvent::Close => {
                self.modal_state.close(ctx);
            }
            AddVpsModalEvent::AddVps { name, host, username, ssh_key } => {
                let manager = ai::cloud_credentials::CloudCredentialsManager::handle(ctx);
                manager.update(ctx, |manager, ctx| {
                    manager.add_entry(
                        ai::cloud_credentials::CloudCredentialEntry {
                            id: uuid::Uuid::new_v4().to_string(),
                            platform: ai::cloud_credentials::CloudPlatform::Vps,
                            name: Some(name.clone()),
                            host_or_key: Some(host.clone()),
                            vps_username: Some(username.clone()).filter(|s| !s.is_empty()),
                            vps_ssh_key: Some(ssh_key.clone()).filter(|s| !s.is_empty()),
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

impl Entity for VpsHostsPageView {
    type Event = ();
}

impl SingletonEntity for VpsHostsPageView {}

impl TypedActionView for VpsHostsPageView {
    type Action = VpsHostsPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            VpsHostsPageAction::AddEntry => {
                self.open_add_modal(ctx);
            }
            VpsHostsPageAction::RemoveEntry { index } => {
                let manager = ai::cloud_credentials::CloudCredentialsManager::handle(ctx);
                let credentials = manager.as_ref(ctx).credentials().clone();
                let vps_entries: Vec<_> = credentials.vps_entries().collect();
                if let Some(entry) = vps_entries.get(*index) {
                    let id = entry.id.clone();
                    manager.update(ctx, |manager, ctx| {
                        manager.remove_entry(&id, ctx);
                    });
                }
            }
        }
    }
}

impl View for VpsHostsPageView {
    fn ui_name() -> &'static str {
        "VpsHostsPageView"
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

impl SettingsPageMeta for VpsHostsPageView {
    fn section() -> SettingsSection {
        SettingsSection::VpsHosts
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

impl From<ViewHandle<VpsHostsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<VpsHostsPageView>) -> Self {
        SettingsPageViewHandle::VpsHosts(view_handle)
    }
}

#[derive(Default)]
struct VpsHostsListWidget;

impl SettingsWidget for VpsHostsListWidget {
    type View = VpsHostsPageView;

    fn search_terms(&self) -> &str {
        "vps host ssh server cloud platform"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn WarpuiElement> {
        let manager = ai::cloud_credentials::CloudCredentialsManager::as_ref(app);
        let credentials = manager.credentials();
        let vps_entries: Vec<_> = credentials.vps_entries().collect();

        let mut column = Flex::column().with_spacing(16.);

        // Header
        let header = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .with_cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(
                Flex::column()
                    .with_spacing(8.)
                    .with_child(
                        Text::new_inline(
                            "VPS Hosts",
                            appearance.ui_font_family(),
                            appearance.header_font_size(),
                        )
                        .with_style(Properties::default().weight(Weight::Semibold))
                        .with_color(appearance.theme().active_ui_text_color().into())
                        .finish(),
                    )
                    .with_child(
                        Text::new_inline(
                            "Manage your VPS hosts for running ambient agents.",
                            appearance.ui_font_family(),
                            appearance.ui_font_size(),
                        )
                        .with_color(appearance.theme().nonactive_ui_text_color().into())
                        .finish(),
                    )
                    .finish(),
            )
            .with_child(
                appearance
                    .ui_builder()
                    .button(ButtonVariant::Accent, view.add_button_mouse_state.clone())
                    .with_text_label("Add VPS".to_string())
                    .build()
                    .on_click(|ctx, _, _| {
                        ctx.dispatch_typed_action(VpsHostsPageAction::AddEntry);
                    })
                    .finish(),
            )
            .finish();

        column.add_child(header);

        if vps_entries.is_empty() {
            column.add_child(render_empty_state(appearance));
        } else {
            let mut list = Flex::column().with_spacing(12.);
            list.add_child(
                Text::new_inline(
                    format!("Saved VPS hosts ({})", vps_entries.len()),
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(appearance.theme().active_ui_text_color().into())
                .finish(),
            );

            for (index, entry) in vps_entries.iter().enumerate() {
                list.add_child(render_vps_card(entry, index, appearance));
            }

            column.add_child(list.finish());
        }

        column.finish()
    }
}

fn render_empty_state(appearance: &Appearance) -> Box<dyn WarpuiElement> {
    Container::new(
        Flex::column()
            .with_spacing(12.)
            .with_main_axis_alignment(MainAxisAlignment::Center)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(
                Text::new_inline(
                    "No VPS hosts configured",
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_color(appearance.theme().nonactive_ui_text_color().into())
                .finish(),
            )
            .with_child(
                Text::new_inline(
                    "Add a VPS host to run ambient agents on your own infrastructure.",
                    appearance.ui_font_family(),
                    appearance.ui_font_size(),
                )
                .with_color(appearance.theme().nonactive_ui_text_color().into())
                .finish(),
            )
            .finish(),
    )
    .with_uniform_padding(24.)
    .with_background(appearance.theme().surface_1())
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
    .with_border(Border::all(1.).with_border_fill(appearance.theme().outline().into_solid()))
    .finish()
}

fn render_vps_card(
    entry: &ai::cloud_credentials::CloudCredentialEntry,
    index: usize,
    appearance: &Appearance,
) -> Box<dyn WarpuiElement> {
    let host = entry.host_or_key.as_deref().unwrap_or("Unknown host");
    let username = entry.vps_username.as_deref().unwrap_or("root");
    let label = entry.display_label();

    let mut row = Flex::row()
        .with_main_axis_size(MainAxisSize::Max)
        .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_spacing(12.);

    let info = Flex::column()
        .with_spacing(4.)
        .with_child(
            Text::new_inline(
                label,
                appearance.ui_font_family(),
                appearance.ui_font_size(),
            )
            .with_style(Properties::default().weight(Weight::Semibold))
            .with_color(appearance.theme().active_ui_text_color().into())
            .finish(),
        )
        .with_child(
            Text::new_inline(
                format!("{}@{}", username, host),
                appearance.ui_font_family(),
                appearance.ui_font_size(),
            )
            .with_color(appearance.theme().nonactive_ui_text_color().into())
            .finish(),
        )
        .finish();

    row.add_child(info);

    let remove_button = appearance
        .ui_builder()
        .button(ButtonVariant::Secondary, MouseStateHandle::default())
        .with_text_label("Remove".to_string())
        .build()
        .on_click(move |ctx, _, _| {
            ctx.dispatch_typed_action(VpsHostsPageAction::RemoveEntry { index });
        })
        .finish();

    row.add_child(remove_button);

    Container::new(row.finish())
        .with_uniform_padding(16.)
        .with_background(appearance.theme().surface_1())
        .with_corner_radius(CornerRadius::with_all(Radius::Pixels(8.)))
        .with_border(Border::all(1.).with_border_fill(appearance.theme().outline().into_solid()))
        .finish()
}

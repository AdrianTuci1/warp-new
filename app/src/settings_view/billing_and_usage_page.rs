use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use chrono::Local;
use itertools::Itertools;
use markdown_parser::{FormattedText, FormattedTextFragment, FormattedTextLine};
use pathfinder_color::ColorU;
use pathfinder_geometry::vector::vec2f;
use settings::Setting;
use thousands::Separable;
use warp_core::features::FeatureFlag;
use warp_core::ui::appearance::Appearance;
use warp_core::ui::theme::Fill;
use warp_graphql::billing::AddonCreditsOption;
use warpui::elements::{
    Align, Border, ChildAnchor, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, Empty,
    Flex, FormattedTextElement, HighlightedHyperlink, Hoverable, HyperlinkUrl, MainAxisAlignment,
    MainAxisSize, MouseStateHandle, OffsetPositioning, ParentAnchor, ParentElement,
    ParentOffsetBounds, Radius, Shrinkable, Text, Wrap,
};
use warpui::fonts::{Properties, Weight};
use warpui::platform::Cursor;
use warpui::prelude::ChildView;
use warpui::ui_components::button::{ButtonVariant, TextAndIcon, TextAndIconAlignment};
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::ui_components::switch::SwitchStateHandle;
use warpui::{
    AppContext, Element, Entity, ModelHandle, SingletonEntity, TypedActionView, UpdateView, View,
    ViewContext, ViewHandle,
};

use super::admin_actions::AdminActions;
use super::settings_page::{
    build_sub_header, render_body_item, render_customer_type_badge, render_info_icon,
    AdditionalInfo, HEADER_PADDING,
};
use super::SettingsSection;
use crate::ai::AIRequestUsageModel;
use crate::auth::auth_manager::LoginGatedFeature;
use crate::auth::auth_state::AuthState;
use crate::auth::auth_view_modal::AuthViewVariant;
use crate::auth::{AuthManager, AuthStateProvider, UserUid};
use crate::menu::{Event as MenuEvent, Menu, MenuItem, MenuItemFields};
use crate::modal::{Modal, ModalEvent, ModalViewState};
use crate::pricing::{PricingInfoModel, PricingInfoModelEvent};
use crate::server::ids::ServerId;
use crate::server::telemetry::TelemetryEvent;
use crate::settings::ai::AISettings;
use crate::settings_view::settings_page::TOGGLE_BUTTON_RIGHT_PADDING;
use crate::ui_components::blended_colors;
use crate::ui_components::buttons::icon_button;
use crate::ui_components::icons::Icon;
use crate::ui_components::menu_button::{icon_button_with_context_menu, MenuDirection};
use crate::ui_components::tab_selector::{self, SettingsTab};
use crate::view_components::action_button::{ActionButton, PrimaryTheme, SecondaryTheme};
use crate::view_components::ToastFlavor;
use crate::workspaces::team::Team;
use crate::workspaces::update_manager::TeamUpdateManager;
use crate::workspaces::user_profiles::UserProfiles;
use crate::workspaces::user_workspaces::{UserWorkspaces, UserWorkspacesEvent};
use crate::workspaces::workspace::{CustomerType, Workspace};
use crate::{send_telemetry_from_ctx, WorkspaceAction};

const HEADER_FONT_SIZE: f32 = 16.;
const OVERAGE_USAGE_LINK_TEXT: &str = "View details on overage usage";
const OVERAGE_TOGGLE_ADMIN_HEADER: &str = "Enable premium model usage overages";
const OVERAGE_TOGGLE_USER_HEADER_ENABLED: &str = "Premium model usage overages are enabled";
const OVERAGE_TOGGLE_USER_HEADER_DISABLED: &str = "Premium model usage overages are not enabled";
const OVERAGE_TOGGLE_DESCRIPTION: &str = "Continue using premium models beyond your plan's limits. Usage is charged in $20 increments up to your spending limit, with any remaining balance charged on your scheduled billing date.";
const OVERAGE_TOGGLE_USER_DESCRIPTION: &str =
    "Ask a team admin to enable overages for more AI usage.";

const SORT_MENU_ITEM_DISPLAY_NAME_A_Z_LABEL: &str = "A to Z";
const SORT_MENU_ITEM_DISPLAY_NAME_Z_A_LABEL: &str = "Z to A";
const SORT_MENU_ITEM_REQUEST_USAGE_ASCENDING_LABEL: &str = "Usage ascending";
const SORT_MENU_ITEM_REQUEST_USAGE_DESCENDING_LABEL: &str = "Usage descending";

const AUTO_RELOAD_EXCEED_LIMIT_WARNING_STRING: &str =
    "Auto reload is disabled, as the next reload would exceed your monthly spend limit. Increase your limit to use auto reload.";
const AUTO_RELOAD_DELINQUENT_WARNING_STRING: &str =
    "Restricted due to billing issue. Update your payment method to purchase add-on credits.";
const RESTRICTED_BILLING_USAGE_WARNING_STRING: &str =
    "Auto reload is disabled due to recent failed reload. Please update your payment method and try again.";

const OVERVIEW_TAB_TEXT: &str = "Overview";
const USAGE_HISTORY_TAB_TEXT: &str = "Usage History";

const ENTERPRISE_USAGE_CALLOUT_HEADER: &str = "Usage reporting is currently limited";
const ENTERPRISE_USAGE_CALLOUT_BODY_ADMIN_PREFIX: &str =
    "Enterprise credit usage isn't fully available in this view yet. For the most accurate spend tracking, ";
const ENTERPRISE_USAGE_CALLOUT_BODY_ADMIN_LINK: &str = "visit the admin panel";
const ENTERPRISE_USAGE_CALLOUT_BODY_ADMIN_SUFFIX: &str = ".";
const ENTERPRISE_USAGE_CALLOUT_BODY_NON_ADMIN: &str =
    "Enterprise credit usage isn't fully available in this view yet. Contact a team admin for detailed usage reporting.";

const ADDON_CREDITS_DESCRIPTION: &str = "Add-on credits are purchased in prepaid packages that roll over each billing cycle and expire after one year. The more you purchase, the better the per-credit rate. Once your base plan credits are used, add-on credits will be consumed.";
const ADDITIONAL_ADDON_CREDITS_DESCRIPTION_FOR_TEAM: &str =
    "Purchased add-on credits are shared across your team.";

// Cloud agent trial widget constants.
const AMBIENT_AGENT_TRIAL_TITLE: &str = "Cloud agent trial";
/// The threshold below which we only show the "Buy more" button (not "New agent").
use crate::ai::request_usage_model::AMBIENT_AGENT_TRIAL_CREDIT_THRESHOLD;

// Stub types for removed billing_and_usage modules
pub struct SpendingLimitModal;
pub struct SpendingLimitModalEvent;
pub struct UsageHistoryModel;
pub struct UsageHistoryEntry;

pub fn create_discount_badge(discount: u32, appearance: &Appearance) -> Box<dyn Element> {
    if discount == 0 {
        return Empty::new().finish();
    }

    let theme = appearance.theme();
    let bg_color: Fill = theme.terminal_colors().normal.green.into();

    Container::new(
        Text::new_inline(format!("{discount}% off"), appearance.ui_font_family(), 10.)
            .with_color(theme.main_text_color(bg_color).into())
            .finish(),
    )
    .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
    .with_background(bg_color)
    .with_uniform_padding(4.)
    .finish()
}
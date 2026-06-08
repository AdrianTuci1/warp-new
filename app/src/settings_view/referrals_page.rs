1|use std::ops::Deref;
2|use std::sync::Arc;
3|
4|use lazy_static::lazy_static;
5|use markdown_parser::{FormattedText, FormattedTextFragment, FormattedTextLine};
6|use pathfinder_color::ColorU;
7|use pathfinder_geometry::vector::Vector2F;
8|use thiserror::Error;
9|use validator::ValidateEmail;
10|use warpui::clipboard::ClipboardContent;
11|use warpui::elements::{
12|    Align, Border, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, Element, Fill,
13|    Flex, FormattedTextElement, HighlightedHyperlink, Icon, MainAxisSize, MouseStateHandle,
14|    ParentElement, Radius, Rect, Shrinkable,
15|};
16|use warpui::fonts::Weight;
17|use warpui::ui_components::button::ButtonVariant;
18|use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
19|use warpui::{
20|    AppContext, Entity, EventContext, FocusContext, SingletonEntity, TypedActionView, View,
21|    ViewContext, ViewHandle,
22|};
23|
24|use super::settings_page::{
25|    MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle, SettingsWidget, PAGE_PADDING,
26|};
27|use super::SettingsSection;
28|use crate::appearance::Appearance;
29|use crate::auth::AuthStateProvider;
30|use crate::editor::{EditorView, Event as EditorEvent, SingleLineEditorOptions, TextOptions};
31|use crate::server::server_api::referral::{ReferralInfo, ReferralsClient};
32|use crate::server::telemetry::TelemetryEvent;
33|use crate::ui_components::blended_colors;
34|use crate::view_components::ToastFlavor;
35|use crate::{safe_info, send_telemetry_from_ctx};
36|
37|const HEADER_FONT_SIZE: f32 = 18.;
38|const HEADER_MARGIN_BOTTOM: f32 = 32.;
39|const HEADER_TEXT: &str = "Invite a friend to Warp";
40|const ANONYMOUS_USER_HEADER_TEXT: &str = "Sign up to participate in Octomus' referral program";
41|
42|const INVITE_FIELD_LABEL_BOTTOM_MARGIN: f32 = 8.;
43|
44|const LINK_BOTTOM_MARGIN: f32 = 12.;
45|const LINK_TEXT_PADDING: f32 = 10.;
46|const LINK_CORNER_RADIUS: Radius = Radius::Pixels(4.);
47|const LINK_ERROR_TEXT: &str = "Failed to load referral code.";
48|
49|const BUTTON_WIDTH: f32 = 98.;
50|const BUTTON_HEIGHT: f32 = 36.;
51|const BUTTON_LEFT_MARGIN: f32 = 8.;
52|const BUTTON_FONT_SIZE: f32 = 12.;
53|const LINK_BUTTON_TEXT: &str = "Copy link";
54|const EMAIL_BUTTON_TEXT: &str = "Send";
55|const EMAIL_BUTTON_SENDING_TEXT: &str = "Sending...";
56|const LOADING_TEXT: &str = "Loading...";
57|
58|const LINK_COPIED_TOAST: &str = "Link copied.";
59|const EMAIL_SUCCESS_TOAST: &str = "Successfully sent emails.";
60|const EMAIL_FAILURE_TOAST: &str = "Failed to send emails. Please try again.";
61|
62|const REWARD_INTRO: &str = "Get exclusive Warp goodies when you refer someone*";
63|const REWARD_INTRO_FONT_SIZE: f32 = 14.;
64|const REWARD_SECTION_VERTICAL_SPACING: f32 = 24.;
65|
66|const REFERRAL_ICON_BOX_VERTICAL_SPACING: f32 = 8.;
67|const REWARD_ICON_BOX_HEIGHT: f32 = 60.;
68|const REWARD_ICON_BOX_WIDTH: f32 = 80.;
69|const REWARD_ICON_BORDER_CORNER_RADIUS: Radius = Radius::Pixels(8.);
70|const REWARD_ICON_BOX_DESCRIPTION_HORIZONTAL_SPACING: f32 = 12.;
71|const REWARD_ICON_BOX_BORDER_WIDTH: f32 = 1.;
72|
73|const METER_LEVEL_BORDER_WIDTH: f32 = 2.;
74|const METER_LEVEL_CIRCLE_HEIGHT: f32 = 28.;
75|const METER_LEVEL_FONT_SIZE: f32 = 11.;
76|const METER_LINE_WIDTH: f32 = 2.;
77|const METER_LINE_HEIGHT: f32 = 26.;
78|const METER_ICON_SEPARATOR_VERTICAL_MARGIN: f32 = 7.;
79|const METER_DOT_SPACING: f32 = 2.;
80|const METER_TOP_MARGIN: f32 = 16.;
81|const METER_RIGHT_MARGIN: f32 = 12.;
82|
83|const CLAIMED_REFERRALS_LABEL_HORIZONTAL_SPACING: f32 = 4.;
84|const CLAIMED_REFERRALS_COUNT_LABEL_SINGULAR: &str = "Current referral";
85|const CLAIMED_REFERRALS_COUNT_LABEL_PLURAL: &str = "Current referrals";
86|const CLAIMED_REFERRALS_LABEL_WIDTH: f32 = 52.;
87|const CLAIMED_REFERRALS_LABEL_FONT_SIZE: f32 = 14.;
88|const CLAIMED_REFERRALS_COUNT_FONT_SIZE: f32 = 48.;
89|const CLAIMED_REFERRAL_COUNT_LEFT_MARGIN: f32 = 40.;
90|
91|const CLAIMED_REFERRAL_CLIP: usize = 999;
92|
93|const TERMS_LINK_TEXT: &str = "Certain restrictions apply.";
94|const TERMS_URL: &str =
95|    "https://docs.warp.dev/support-and-community/community/refer-a-friend#referral-program-terms-and-conditions";
96|const TERMS_CONTACT_TEXT: &str =
97|    " If you have any questions about the referral program, please contact referrals@warp.dev.";
98|
99|enum ApiState {
100|    Loading,
101|    Ready {
102|        referral_info: ReferralInfo,
103|        email_state: SendEmailState,
104|    },
105|    Failed,
106|}
107|
108|#[derive(Debug)]
109|pub enum ReferralsPageAction {
110|    CopyLink,
111|    SendEmailInvite,
112|    SignupAnonymousUser,
113|}
114|
115|pub enum ReferralsPageEvent {
116|    SignupAnonymousUser,
117|    FocusModal,
118|    ShowToast {
119|        message: String,
120|        flavor: ToastFlavor,
121|    },
122|}
123|
124|enum SendEmailState {
125|    Idle,
126|    Sending,
127|}
128|
129|pub struct ReferralsPageView {
130|    page: PageType<Self>,
131|    email_editor: ViewHandle<EditorView>,
132|    referrals_client: Arc<dyn ReferralsClient>,
133|    api_state: ApiState,
134|}
135|
136|#[derive(Clone)]
137|struct Reward {
138|    required_referral_count: usize,
139|    icon_path: &'static str,
140|    icon_height: f32,
141|    icon_width: f32,
142|    label: String,
143|}
144|
145|lazy_static! {
146|    static ref REWARDS: Vec<Reward> = vec![
147|        Reward {
148|            required_referral_count: 1,
149|            icon_path: "bundled/svg/referral-theme.svg",
150|            icon_width: 64.,
151|            icon_height: 64.,
152|            label: "Exclusive theme".to_owned(),
153|        },
154|        Reward {
155|            required_referral_count: 5,
156|            icon_path: "bundled/svg/referral-keycaps.svg",
157|            icon_width: 56.,
158|            icon_height: 56.,
159|            label: "Keycaps + stickers".to_owned(),
160|        },
161|        Reward {
162|            required_referral_count: 10,
163|            icon_path: "bundled/svg/referral-tshirt.svg",
164|            icon_width: 64.,
165|            icon_height: 64.,
166|            label: "T-shirt".to_owned(),
167|        },
168|        Reward {
169|            required_referral_count: 20,
170|            icon_path: "bundled/svg/referral-notebook.svg",
171|            icon_width: 64.,
172|            icon_height: 64.,
173|            label: "Notebook".to_owned(),
174|        },
175|        Reward {
176|            required_referral_count: 35,
177|            icon_path: "bundled/svg/referral-hat.svg",
178|            icon_width: 64.,
179|            icon_height: 64.,
180|            label: "Baseball cap".to_owned(),
181|        },
182|        Reward {
183|            required_referral_count: 50,
184|            icon_path: "bundled/svg/referral-hoodie.svg",
185|            icon_width: 64.,
186|            icon_height: 64.,
187|            label: "Hoodie".to_owned(),
188|        },
189|        Reward {
190|            required_referral_count: 75,
191|            icon_path: "bundled/svg/referral-hydroflask.svg",
192|            icon_width: 48.,
193|            icon_height: 48.,
194|            label: "Premium Hydro Flask".to_owned(),
195|        },
196|        Reward {
197|            required_referral_count: 100,
198|            icon_path: "bundled/svg/referral-backpack.svg",
199|            icon_width: 50.,
200|            icon_height: 50.,
201|            label: "Backpack".to_owned(),
202|        },
203|    ];
204|}
205|
206|impl ReferralsPageView {
207|    pub fn new(referrals_client: Arc<dyn ReferralsClient>, ctx: &mut ViewContext<Self>) -> Self {
208|        let email_editor = ctx.add_typed_action_view(|ctx| {
209|            let options = SingleLineEditorOptions {
210|                text: TextOptions::ui_font_size(Appearance::as_ref(ctx)),
211|                ..Default::default()
212|            };
213|            EditorView::single_line(options, ctx)
214|        });
215|
216|        ctx.subscribe_to_view(&email_editor, |me, _, event, ctx| {
217|            me.handle_editor_event(event, ctx);
218|        });
219|
220|        let page = PageType::new_monolith(ReferralsWidget::default(), Some(HEADER_TEXT), true);
221|        Self {
222|            page,
223|            referrals_client,
224|            api_state: ApiState::Loading,
225|            email_editor,
226|        }
227|    }
228|
229|    /// Make a request to get the referral status
230|    ///
231|    /// If the status has already been fetched, the information will be kept while the request
232|    /// is in flight.
233|    fn fetch_referral_status(&mut self, ctx: &mut ViewContext<Self>) {
234|        // If we already have data, we fire another request to make sure it is up-to-date,
235|        // however, we don't want to update the state and lose the existing data until the
236|        // request completes.
237|        if matches!(self.api_state, ApiState::Failed) {
238|            self.api_state = ApiState::Loading;
239|        }
240|
241|        let referrals_client = self.referrals_client.clone();
242|        let _ = ctx.spawn(
243|            async move { referrals_client.get_referral_info().await },
244|            Self::handle_referral_status_response,
245|        );
246|    }
247|
248|    fn handle_referral_status_response(
249|        &mut self,
250|        response: anyhow::Result<ReferralInfo>,
251|        ctx: &mut ViewContext<Self>,
252|    ) {
253|        match response {
254|            Ok(info) => match &mut self.api_state {
255|                state @ ApiState::Loading | state @ ApiState::Failed => {
256|                    *state = ApiState::Ready {
257|                        referral_info: info,
258|                        email_state: SendEmailState::Idle,
259|                    };
260|                }
261|                ApiState::Ready { referral_info, .. } => {
262|                    *referral_info = info;
263|                }
264|            },
265|            Err(err) => {
266|                self.api_state = ApiState::Failed;
267|                log::warn!("Error loading referral info from server: {err}");
268|            }
269|        }
270|        ctx.notify();
271|    }
272|
273|    fn copy_link(&mut self, ctx: &mut ViewContext<Self>) {
274|        match &self.api_state {
275|            ApiState::Loading | ApiState::Failed => {
276|                // Shouldn't happen as the buttons will be disabled
277|                log::warn!("Attempting to copy link before API request is complete");
278|            }
279|            ApiState::Ready { referral_info, .. } => {
280|                send_telemetry_from_ctx!(TelemetryEvent::CopyInviteLink, ctx);
281|                ctx.clipboard()
282|                    .write(ClipboardContent::plain_text(referral_info.url.to_string()));
283|                ctx.emit(ReferralsPageEvent::ShowToast {
284|                    message: LINK_COPIED_TOAST.to_owned(),
285|                    flavor: ToastFlavor::Default,
286|                });
287|            }
288|        }
289|    }
290|
291|    fn send_email_invite(&mut self, ctx: &mut ViewContext<Self>) {
292|        let emails = self.recipient_emails_from_editor(ctx);
293|        match &mut self.api_state {
294|            ApiState::Ready {
295|                email_state: state @ SendEmailState::Idle,
296|                ..
297|            } => match emails.iter().map(Deref::deref).try_for_each(validate_email) {
298|                Ok(_) => {
299|                    *state = SendEmailState::Sending;
300|                    let referrals_client = self.referrals_client.clone();
301|                    let _ = ctx.spawn(
302|                        async move { referrals_client.send_invite(emails).await },
303|                        Self::handle_send_email_invite_response,
304|                    );
305|                }
306|                Err(error) => {
307|                    ctx.emit(ReferralsPageEvent::ShowToast {
308|                        message: error.ui_message(),
309|                        flavor: ToastFlavor::Error,
310|                    });
311|                    log::warn!("Emails entered are invalid: {error}");
312|                }
313|            },
314|            _ => {
315|                // Shouldn't happen as the buttons will be disabled
316|                log::warn!("Attempting to send email referrals before API is available");
317|            }
318|        }
319|    }
320|
321|    fn handle_send_email_invite_response(
322|        &mut self,
323|        response: anyhow::Result<Vec<String>>,
324|        ctx: &mut ViewContext<Self>,
325|    ) {
326|        match response {
327|            Ok(successful) => {
328|                self.email_editor.update(ctx, |view, ctx| {
329|                    view.clear_buffer_and_reset_undo_stack(ctx);
330|                    ctx.notify();
331|                });
332|                safe_info!(
333|                    safe: ("Successfully sent {} invites", successful.len()),
334|                    full: ("Successfully sent invites to: {:?}", successful)
335|                );
336|                ctx.emit(ReferralsPageEvent::ShowToast {
337|                    message: EMAIL_SUCCESS_TOAST.to_owned(),
338|                    flavor: ToastFlavor::Success,
339|                });
340|            }
341|            Err(err) => {
342|                log::error!("Error sending referral emails: {err}");
343|                ctx.emit(ReferralsPageEvent::ShowToast {
344|                    message: EMAIL_FAILURE_TOAST.to_owned(),
345|                    flavor: ToastFlavor::Error,
346|                });
347|            }
348|        }
349|
350|        if let ApiState::Ready { email_state, .. } = &mut self.api_state {
351|            *email_state = SendEmailState::Idle;
352|        }
353|        ctx.notify();
354|    }
355|
356|    fn recipient_emails_from_editor(&self, ctx: &mut ViewContext<Self>) -> Vec<String> {
357|        let editor_text = self.email_editor.as_ref(ctx).buffer_text(ctx);
358|        editor_text
359|            .split(',')
360|            .map(|email| email.trim().to_string())
361|            .collect()
362|    }
363|
364|    fn handle_editor_event(&mut self, event: &EditorEvent, ctx: &mut ViewContext<Self>) {
365|        match event {
366|            EditorEvent::Enter => {
367|                self.send_email_invite(ctx);
368|            }
369|            EditorEvent::Escape => ctx.emit(ReferralsPageEvent::FocusModal),
370|            _ => (),
371|        }
372|    }
373|
374|    fn referral_claimed_count(&self) -> Option<usize> {
375|        match &self.api_state {
376|            ApiState::Ready { referral_info, .. } => Some(referral_info.number_claimed),
377|            _ => None,
378|        }
379|    }
380|}
381|
382|impl Entity for ReferralsPageView {
383|    type Event = ReferralsPageEvent;
384|}
385|
386|impl View for ReferralsPageView {
387|    fn ui_name() -> &'static str {
388|        "ReferralsPageView"
389|    }
390|
391|    fn render(&self, app: &AppContext) -> Box<dyn Element> {
392|        self.page.render(self, app)
393|    }
394|
395|    fn on_focus(&mut self, focus_ctx: &FocusContext, ctx: &mut ViewContext<Self>) {
396|        if focus_ctx.is_self_focused() {
397|            self.fetch_referral_status(ctx);
398|        }
399|    }
400|}
401|
402|impl SettingsPageMeta for ReferralsPageView {
403|    fn section() -> SettingsSection {
404|        SettingsSection::Account
405|    }
406|
407|    fn should_render(&self, _ctx: &AppContext) -> bool {
408|        true
409|    }
410|
411|    fn on_page_selected(&mut self, _: bool, ctx: &mut ViewContext<Self>) {
412|        self.fetch_referral_status(ctx);
413|    }
414|
415|    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
416|        self.page.update_filter(query, ctx)
417|    }
418|
419|    fn scroll_to_widget(&mut self, widget_id: &'static str) {
420|        self.page.scroll_to_widget(widget_id)
421|    }
422|
423|    fn clear_highlighted_widget(&mut self) {
424|        self.page.clear_highlighted_widget();
425|    }
426|}
427|
428|impl TypedActionView for ReferralsPageView {
429|    type Action = ReferralsPageAction;
430|
431|    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
432|        match action {
433|            ReferralsPageAction::CopyLink => self.copy_link(ctx),
434|            ReferralsPageAction::SendEmailInvite => self.send_email_invite(ctx),
435|            ReferralsPageAction::SignupAnonymousUser => {
436|                ctx.emit(ReferralsPageEvent::SignupAnonymousUser)
437|            }
438|        }
439|    }
440|}
441|
442|impl From<ViewHandle<ReferralsPageView>> for SettingsPageViewHandle {
443|    fn from(view_handle: ViewHandle<ReferralsPageView>) -> Self {
444|        SettingsPageViewHandle::Referrals(view_handle)
445|    }
446|}
447|#[derive(Error, Debug)]
448|enum EmailValidationError {
449|    #[error("Email is empty")]
450|    Empty,
451|    #[error("Email is invalid: {0}")]
452|    Invalid(String),
453|}
454|
455|impl EmailValidationError {
456|    /// The user-readable error descriptions.
457|    fn ui_message(&self) -> String {
458|        match self {
459|            EmailValidationError::Empty => "Please enter an email.".to_owned(),
460|            EmailValidationError::Invalid(invalid_email) => {
461|                format!("Please ensure the following email is valid: {invalid_email}")
462|            }
463|        }
464|    }
465|}
466|
467|fn validate_email(email: &str) -> anyhow::Result<(), EmailValidationError> {
468|    if email.is_empty() {
469|        Err(EmailValidationError::Empty)
470|    } else if !email.validate_email() {
471|        Err(EmailValidationError::Invalid(email.to_owned()))
472|    } else {
473|        Ok(())
474|    }
475|}
476|
477|#[derive(Default)]
478|struct ReferralsWidget {
479|    copy_link_mouse_state: MouseStateHandle,
480|    send_email_mouse_state: MouseStateHandle,
481|    sign_up_button_mouse_state: MouseStateHandle,
482|    term_docs_highlighted_hyperlink: HighlightedHyperlink,
483|}
484|
485|impl ReferralsWidget {
486|    fn render_page_body(
487|        &self,
488|        view: &ReferralsPageView,
489|        appearance: &Appearance,
490|        app: &AppContext,
491|    ) -> Box<dyn Element> {
492|        let is_anonymous = AuthStateProvider::as_ref(app)
493|            .get()
494|            .is_anonymous_or_logged_out();
495|
496|        let invite_or_signup_section = if is_anonymous {
497|            self.render_signup_section(appearance)
498|        } else {
499|            self.render_send_invite_section(view, appearance)
500|        };
501|
502|        Flex::column()
503|            .with_child(
504|                Container::new(invite_or_signup_section)
505|                    .with_padding_bottom(PAGE_PADDING)
506|                    .finish(),
507|            )
508|            .with_child(
509|                Container::new(self.render_rewards_section(is_anonymous, view, appearance))
510|                    .with_padding_bottom(PAGE_PADDING)
511|                    .finish(),
512|            )
513|            .finish()
514|    }
515|
516|    fn render_link_row(
517|        &self,
518|        view: &ReferralsPageView,
519|        appearance: &Appearance,
520|    ) -> Box<dyn Element> {
521|        let (link_text, button_enabled) = match &view.api_state {
522|            ApiState::Ready { referral_info, .. } => (referral_info.url.clone(), true),
523|            ApiState::Loading => (LOADING_TEXT.into(), false),
524|            ApiState::Failed => (LINK_ERROR_TEXT.into(), false),
525|        };
526|        let theme = appearance.theme();
527|
528|        Container::new(
529|            Flex::row()
530|                .with_child(
531|                    Shrinkable::new(
532|                        1.0,
533|                        Container::new(
534|                            Align::new(
535|                                appearance
536|                                    .ui_builder()
537|                                    .span(link_text)
538|                                    .with_style(UiComponentStyles {
539|                                        font_color: Some(
540|                                            theme.main_text_color(theme.background()).into_solid(),
541|                                        ),
542|                                        ..Default::default()
543|                                    })
544|                                    .build()
545|                                    .finish(),
546|                            )
547|                            .left()
548|                            .finish(),
549|                        )
550|                        .with_background(theme.background())
551|                        .with_uniform_padding(LINK_TEXT_PADDING)
552|                        .with_corner_radius(CornerRadius::with_all(LINK_CORNER_RADIUS))
553|                        .with_border(Border::all(1.).with_border_fill(theme.outline()))
554|                        .finish(),
555|                    )
556|                    .finish(),
557|                )
558|                .with_child(self.render_button(
559|                    button_enabled,
560|                    LINK_BUTTON_TEXT,
561|                    self.copy_link_mouse_state.clone(),
562|                    |ctx, _, _| ctx.dispatch_typed_action(ReferralsPageAction::CopyLink),
563|                    appearance,
564|                ))
565|                .with_main_axis_size(MainAxisSize::Max)
566|                .finish(),
567|        )
568|        .with_margin_bottom(LINK_BOTTOM_MARGIN)
569|        .finish()
570|    }
571|
572|    fn render_email_row(
573|        &self,
574|        view: &ReferralsPageView,
575|        appearance: &Appearance,
576|    ) -> Box<dyn Element> {
577|        let (button_text, button_enabled) = match &view.api_state {
578|            ApiState::Ready {
579|                email_state: SendEmailState::Idle,
580|                ..
581|            } => (EMAIL_BUTTON_TEXT, true),
582|            ApiState::Ready {
583|                email_state: SendEmailState::Sending,
584|                ..
585|            } => (EMAIL_BUTTON_SENDING_TEXT, false),
586|            _ => (EMAIL_BUTTON_TEXT, false),
587|        };
588|
589|        Flex::row()
590|            .with_child(
591|                Shrinkable::new(
592|                    1.0,
593|                    Align::new(
594|                        appearance
595|                            .ui_builder()
596|                            .text_input(view.email_editor.clone())
597|                            .with_style(UiComponentStyles::default())
598|                            .build()
599|                            .finish(),
600|                    )
601|                    .left()
602|                    .finish(),
603|                )
604|                .finish(),
605|            )
606|            .with_child(self.render_button(
607|                button_enabled,
608|                button_text,
609|                self.send_email_mouse_state.clone(),
610|                |ctx, _, _| ctx.dispatch_typed_action(ReferralsPageAction::SendEmailInvite),
611|                appearance,
612|            ))
613|            .finish()
614|    }
615|
616|    fn render_send_invite_section(
617|        &self,
618|        view: &ReferralsPageView,
619|        appearance: &Appearance,
620|    ) -> Box<dyn Element> {
621|        Flex::column()
622|            .with_child(
623|                Container::new(self.render_label("Link", appearance))
624|                    .with_padding_top(PAGE_PADDING)
625|                    .finish(),
626|            )
627|            .with_child(self.render_link_row(view, appearance))
628|            .with_child(self.render_label("Email", appearance))
629|            .with_child(self.render_email_row(view, appearance))
630|            .finish()
631|    }
632|
633|    fn render_signup_section(&self, appearance: &Appearance) -> Box<dyn Element> {
634|        let button_styles = UiComponentStyles {
635|            font_size: Some(14.),
636|            font_weight: Some(Weight::Semibold),
637|            border_radius: Some(CornerRadius::with_all(Radius::Pixels(4.))),
638|            padding: Some(Coords {
639|                top: 12.,
640|                bottom: 12.,
641|                left: 40.,
642|                right: 40.,
643|            }),
644|            ..Default::default()
645|        };
646|
647|        let sign_up_button = appearance
648|            .ui_builder()
649|            .button(
650|                ButtonVariant::Accent,
651|                self.sign_up_button_mouse_state.clone(),
652|            )
653|            .with_style(button_styles)
654|            .with_text_label("Sign up".to_owned())
655|            .build()
656|            .on_click(move |ctx, _, _| {
657|                ctx.dispatch_typed_action(ReferralsPageAction::SignupAnonymousUser);
658|            })
659|            .finish();
660|
661|        Flex::column()
662|            .with_child(
663|                Container::new(
664|                    appearance
665|                        .ui_builder()
666|                        .span(ANONYMOUS_USER_HEADER_TEXT)
667|                        .with_style(UiComponentStyles {
668|                            font_size: Some(HEADER_FONT_SIZE),
669|                            ..Default::default()
670|                        })
671|                        .build()
672|                        .finish(),
673|                )
674|                .with_margin_bottom(HEADER_MARGIN_BOTTOM)
675|                .finish(),
676|            )
677|            .with_child(Flex::row().with_child(sign_up_button).finish())
678|            .finish()
679|    }
680|
681|    /// Render submit buttons for the email and link fields.
682|    fn render_button<F>(
683|        &self,
684|        button_enabled: bool,
685|        button_text: &str,
686|        mouse_state_handle: MouseStateHandle,
687|        on_click: F,
688|        appearance: &Appearance,
689|    ) -> Box<dyn Element>
690|    where
691|        F: 'static + FnMut(&mut EventContext, &AppContext, Vector2F),
692|    {
693|        let button = appearance
694|            .ui_builder()
695|            .button(ButtonVariant::Accent, mouse_state_handle)
696|            .with_centered_text_label(button_text.to_owned())
697|            .with_style(UiComponentStyles {
698|                font_size: Some(BUTTON_FONT_SIZE),
699|                font_weight: Some(Weight::Semibold),
700|                width: Some(BUTTON_WIDTH),
701|                height: Some(BUTTON_HEIGHT),
702|                ..Default::default()
703|            });
704|
705|        Container::new({
706|            if button_enabled {
707|                button.build().on_click(on_click).finish()
708|            } else {
709|                button.disabled().build().finish()
710|            }
711|        })
712|        .with_margin_left(BUTTON_LEFT_MARGIN)
713|        .finish()
714|    }
715|
716|    /// Render text labels for the email and link fields.
717|    fn render_label<S>(&self, text: S, appearance: &Appearance) -> Box<dyn Element>
718|    where
719|        S: Into<String>,
720|    {
721|        Container::new(appearance.ui_builder().span(text.into()).build().finish())
722|            .with_margin_bottom(INVITE_FIELD_LABEL_BOTTOM_MARGIN)
723|            .finish()
724|    }
725|
726|    fn render_rewards_section(
727|        &self,
728|        is_anonymous: bool,
729|        view: &ReferralsPageView,
730|        appearance: &Appearance,
731|    ) -> Box<dyn Element> {
732|        let mut rewards_section = Flex::column();
733|
734|        rewards_section.add_child(
735|            Container::new(
736|                appearance
737|                    .ui_builder()
738|                    .span(REWARD_INTRO)
739|                    .with_style(UiComponentStyles {
740|                        font_size: Some(REWARD_INTRO_FONT_SIZE),
741|                        ..Default::default()
742|                    })
743|                    .build()
744|                    .finish(),
745|            )
746|            .with_margin_bottom(REWARD_SECTION_VERTICAL_SPACING)
747|            .finish(),
748|        );
749|
750|        let mut reward_status_row = Flex::row()
751|            .with_child(
752|                Container::new(self.render_meter(view, appearance))
753|                    .with_margin_top(METER_TOP_MARGIN)
754|                    .with_margin_bottom(METER_TOP_MARGIN)
755|                    .with_margin_right(METER_RIGHT_MARGIN)
756|                    .finish(),
757|            )
758|            .with_child(self.render_rewards_list(view, appearance));
759|
760|        if !is_anonymous {
761|            if let Some(count) = self.render_claimed_referrals_count(view, appearance) {
762|                reward_status_row.add_child(
763|                    Container::new(count)
764|                        .with_margin_left(CLAIMED_REFERRAL_COUNT_LEFT_MARGIN)
765|                        .finish(),
766|                );
767|            }
768|        };
769|
770|        rewards_section.add_child(reward_status_row.finish());
771|
772|        rewards_section.add_child(
773|            Container::new(
774|                Align::new(
775|                    FormattedTextElement::new(
776|                        FormattedText::new([FormattedTextLine::Line(vec![
777|                            FormattedTextFragment::plain_text("*"),
778|                            FormattedTextFragment::hyperlink(TERMS_LINK_TEXT, TERMS_URL),
779|                            FormattedTextFragment::plain_text(TERMS_CONTACT_TEXT),
780|                        ])]),
781|                        12.,
782|                        appearance.ui_font_family(),
783|                        appearance.ui_font_family(),
784|                        blended_colors::text_sub(
785|                            appearance.theme(),
786|                            appearance.theme().surface_1(),
787|                        ),
788|                        self.term_docs_highlighted_hyperlink.clone(),
789|                    )
790|                    .with_hyperlink_font_color(appearance.theme().accent().into_solid())
791|                    .register_default_click_handlers(|url, _, ctx| {
792|                        ctx.open_url(&url.url);
793|                    })
794|                    .finish(),
795|                )
796|                .left()
797|                .finish(),
798|            )
799|            .with_margin_top(REWARD_SECTION_VERTICAL_SPACING)
800|            .finish(),
801|        );
802|
803|        rewards_section.finish()
804|    }
805|
806|    fn render_rewards_list(
807|        &self,
808|        view: &ReferralsPageView,
809|        appearance: &Appearance,
810|    ) -> Box<dyn Element> {
811|        Container::new(
812|            Flex::column()
813|                .with_children(REWARDS.iter().map(|reward| {
814|                    Container::new(self.render_reward(reward, view, appearance))
815|                        .with_margin_bottom(REFERRAL_ICON_BOX_VERTICAL_SPACING)
816|                        .finish()
817|                }))
818|                .finish(),
819|        )
820|        .finish()
821|    }
822|
823|    fn render_reward(
824|        &self,
825|        reward: &Reward,
826|        view: &ReferralsPageView,
827|        appearance: &Appearance,
828|    ) -> Box<dyn Element> {
829|        let (icon_color, label_color, label_font_weight): (ColorU, ColorU, Option<Weight>) =
830|            match view.referral_claimed_count() {
831|                Some(claimed_referrals) if claimed_referrals >= reward.required_referral_count => (
832|                    blended_colors::accent(appearance.theme()).into(),
833|                    blended_colors::text_main(appearance.theme(), appearance.theme().background()),
834|                    Some(Weight::Bold),
835|                ),
836|
837|                _ => (
838|                    blended_colors::text_sub(appearance.theme(), appearance.theme().background()),
839|                    blended_colors::text_sub(appearance.theme(), appearance.theme().background()),
840|                    None,
841|                ),
842|            };
843|
844|        Flex::row()
845|            .with_child(
846|                ConstrainedBox::new(
847|                    Container::new(
848|                        Align::new(
849|                            ConstrainedBox::new(Icon::new(reward.icon_path, icon_color).finish())
850|                                .with_height(reward.icon_height)
851|                                .with_width(reward.icon_width)
852|                                .finish(),
853|                        )
854|                        .finish(),
855|                    )
856|                    .with_corner_radius(CornerRadius::with_all(REWARD_ICON_BORDER_CORNER_RADIUS))
857|                    .with_border(
858|                        Border::all(REWARD_ICON_BOX_BORDER_WIDTH)
859|                            .with_border_color(appearance.theme().surface_3().into()),
860|                    )
861|                    .finish(),
862|                )
863|                .with_width(REWARD_ICON_BOX_WIDTH)
864|                .with_height(REWARD_ICON_BOX_HEIGHT)
865|                .finish(),
866|            )
867|            .with_child(
868|                Container::new(
869|                    appearance
870|                        .ui_builder()
871|                        .span(reward.label.clone())
872|                        .with_style(UiComponentStyles {
873|                            font_color: Some(label_color),
874|                            font_weight: label_font_weight,
875|                            ..Default::default()
876|                        })
877|                        .build()
878|                        .finish(),
879|                )
880|                .with_margin_left(REWARD_ICON_BOX_DESCRIPTION_HORIZONTAL_SPACING)
881|                .finish(),
882|            )
883|            .with_cross_axis_alignment(CrossAxisAlignment::Center)
884|            .finish()
885|    }
886|
887|    /// Render the meter tracking how many claimed referrals the user has sent.
888|    fn render_meter(&self, view: &ReferralsPageView, appearance: &Appearance) -> Box<dyn Element> {
889|        let referral_count = view.referral_claimed_count().unwrap_or_default();
890|
891|        let mut column = Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Center);
892|
893|        for (index, reward) in REWARDS.iter().enumerate() {
894|            let lower_threshold = reward.required_referral_count;
895|
896|            let count_indicator =
897|                self.render_referral_meter_count(lower_threshold, referral_count, appearance);
898|
899|            if index < (REWARDS.len() - 1) {
900|                column.add_child(
901|                    Container::new(count_indicator)
902|                        .with_margin_bottom(METER_ICON_SEPARATOR_VERTICAL_MARGIN)
903|                        .finish(),
904|                );
905|
906|                let higher_threshold = REWARDS[index + 1].required_referral_count;
907|
908|                column.add_child(
909|                    Container::new(self.render_meter_separator(
910|                        lower_threshold,
911|                        higher_threshold,
912|                        referral_count,
913|                        appearance,
914|                    ))
915|                    .with_margin_bottom(METER_ICON_SEPARATOR_VERTICAL_MARGIN)
916|                    .finish(),
917|                )
918|            } else {
919|                column.add_child(Container::new(count_indicator).finish());
920|            }
921|        }
922|
923|        column.finish()
924|    }
925|
926|    /// For the reward meter, render the count needed for a reward or an
927|    /// indicator that the user has met the required count.
928|    fn render_referral_meter_count(
929|        &self,
930|        required_referral_count: usize,
931|        referral_count: usize,
932|        appearance: &Appearance,
933|    ) -> Box<dyn Element> {
934|        if referral_count >= required_referral_count {
935|            ConstrainedBox::new(
936|                Icon::new(
937|                    "bundled/svg/check-circle-broken.svg",
938|                    appearance.theme().accent(),
939|                )
940|                .finish(),
941|            )
942|            .with_height(METER_LEVEL_CIRCLE_HEIGHT)
943|            .with_width(METER_LEVEL_CIRCLE_HEIGHT)
944|            .finish()
945|        } else {
946|            let gray: ColorU =
947|                blended_colors::text_sub(appearance.theme(), appearance.theme().background());
948|
949|            Container::new(
950|                ConstrainedBox::new(
951|                    Align::new(
952|                        appearance
953|                            .ui_builder()
954|                            .span(required_referral_count.to_string())
955|                            .with_style(UiComponentStyles {
956|                                font_size: Some(METER_LEVEL_FONT_SIZE),
957|                                font_color: Some(gray),
958|                                font_weight: Some(Weight::Bold),
959|                                ..Default::default()
960|                            })
961|                            .build()
962|                            .finish(),
963|                    )
964|                    .finish(),
965|                )
966|                .with_height(METER_LEVEL_CIRCLE_HEIGHT)
967|                .with_width(METER_LEVEL_CIRCLE_HEIGHT)
968|                .finish(),
969|            )
970|            .with_border(Border::all(METER_LEVEL_BORDER_WIDTH).with_border_color(gray))
971|            .with_corner_radius(CornerRadius::with_all(Radius::Percentage(50.)))
972|            .finish()
973|        }
974|    }
975|
976|    /// Render the solid or dotted lines that indicate completed or partial progress towards a reward's referral requirements.
977|    fn render_meter_separator(
978|        &self,
979|        lower_count: usize,
980|        higher_count: usize,
981|        current_count: usize,
982|        appearance: &Appearance,
983|    ) -> Box<dyn Element> {
984|        let completed_color = blended_colors::accent(appearance.theme());
985|
986|        let dot_color = if current_count > lower_count {
987|            completed_color
988|        } else {
989|            blended_colors::text_sub(appearance.theme(), appearance.theme().background()).into()
990|        };
991|
992|        let line = ConstrainedBox::new(
993|            Rect::new()
994|                .with_corner_radius(CornerRadius::with_all(Radius::Percentage(50.)))
995|                .with_background(completed_color)
996|                .finish(),
997|        )
998|        .with_width(METER_LINE_WIDTH)
999|        .with_height(METER_LINE_HEIGHT)
1000|        .finish();
1001|
1002|        if current_count > higher_count {
1003|            line
1004|        } else {
1005|            self.render_meter_dotted_line(dot_color)
1006|        }
1007|    }
1008|
1009|    fn render_meter_dotted_line<F>(&self, color: F) -> Box<dyn Element>
1010|    where
1011|        F: Into<Fill> + Clone,
1012|    {
1013|        let mut dot_column = Flex::column();
1014|
1015|        for _ in 0..5 {
1016|            dot_column.add_child(
1017|                Container::new(
1018|                    ConstrainedBox::new(
1019|                        Rect::new()
1020|                            .with_corner_radius(CornerRadius::with_all(Radius::Percentage(50.)))
1021|                            .with_background(color.clone())
1022|                            .finish(),
1023|                    )
1024|                    .with_width(METER_LINE_WIDTH)
1025|                    .with_height(METER_LINE_WIDTH)
1026|                    .finish(),
1027|                )
1028|                .with_margin_bottom(METER_DOT_SPACING)
1029|                .finish(),
1030|            );
1031|        }
1032|        dot_column.add_child(
1033|            ConstrainedBox::new(
1034|                Rect::new()
1035|                    .with_corner_radius(CornerRadius::with_all(Radius::Percentage(50.)))
1036|                    .with_background(color)
1037|                    .finish(),
1038|            )
1039|            .with_width(METER_LINE_WIDTH)
1040|            .with_height(METER_LINE_WIDTH)
1041|            .finish(),
1042|        );
1043|
1044|        dot_column.finish()
1045|    }
1046|
1047|    fn render_claimed_referrals_count(
1048|        &self,
1049|        view: &ReferralsPageView,
1050|        appearance: &Appearance,
1051|    ) -> Option<Box<dyn Element>> {
1052|        let claimed_count = view.referral_claimed_count()?;
1053|
1054|        let claimed_count_text = if claimed_count <= CLAIMED_REFERRAL_CLIP {
1055|            claimed_count.to_string()
1056|        } else {
1057|            format!("{claimed_count}+")
1058|        };
1059|
1060|        let current_referrals_label = match claimed_count {
1061|            1 => CLAIMED_REFERRALS_COUNT_LABEL_SINGULAR,
1062|            _ => CLAIMED_REFERRALS_COUNT_LABEL_PLURAL,
1063|        };
1064|
1065|        Some(
1066|            Flex::row()
1067|                .with_child(
1068|                    Container::new(
1069|                        appearance
1070|                            .ui_builder()
1071|                            .span(claimed_count_text)
1072|                            .with_style(UiComponentStyles {
1073|                                font_size: Some(CLAIMED_REFERRALS_COUNT_FONT_SIZE),
1074|                                font_color: Some(blended_colors::text_sub(
1075|                                    appearance.theme(),
1076|                                    appearance.theme().background(),
1077|                                )),
1078|                                ..Default::default()
1079|                            })
1080|                            .build()
1081|                            .finish(),
1082|                    )
1083|                    .with_margin_right(CLAIMED_REFERRALS_LABEL_HORIZONTAL_SPACING)
1084|                    .finish(),
1085|                )
1086|                .with_child(
1087|                    ConstrainedBox::new(
1088|                        appearance
1089|                            .ui_builder()
1090|                            .wrappable_text(current_referrals_label.to_string(), true)
1091|                            .with_style(UiComponentStyles {
1092|                                font_size: Some(CLAIMED_REFERRALS_LABEL_FONT_SIZE),
1093|                                font_color: Some(blended_colors::text_sub(
1094|                                    appearance.theme(),
1095|                                    appearance.theme().background(),
1096|                                )),
1097|                                ..Default::default()
1098|                            })
1099|                            .build()
1100|                            .finish(),
1101|                    )
1102|                    .with_width(CLAIMED_REFERRALS_LABEL_WIDTH)
1103|                    .finish(),
1104|                )
1105|                .with_cross_axis_alignment(CrossAxisAlignment::Center)
1106|                .finish(),
1107|        )
1108|    }
1109|}
1110|
1111|impl SettingsWidget for ReferralsWidget {
1112|    type View = ReferralsPageView;
1113|
1114|    fn search_terms(&self) -> &str {
1115|        "referrals invites"
1116|    }
1117|
1118|    fn render(
1119|        &self,
1120|        view: &Self::View,
1121|        appearance: &Appearance,
1122|        app: &AppContext,
1123|    ) -> Box<dyn Element> {
1124|        self.render_page_body(view, appearance, app)
1125|    }
1126|}
1127|
    fn render_cloud_credentials_section(
        &self,
        appearance: &Appearance,
        view: &PlatformPageView,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        let mut col = Flex::column();
        col.add_child(
            Text::new_inline("Cloud Credentials", appearance.ui_font_family(), 16.)
                .with_style(Properties::default().weight(Weight::Bold))
                .with_color(appearance.theme().active_ui_text_color().into())
                .with_clip(ClipConfig::end())
                .finish(),
        );
        col.add_child(
            Container::new(
                Text::new_inline(
                    "Configure credentials to launch Cloud Agents or Subagents on your VPS or Modal.",
                    appearance.ui_font_family(),
                    CONTENT_FONT_SIZE,
                )
                .with_color(appearance.theme().nonactive_ui_text_color().into())
                .finish(),
            )
            .with_margin_top(8.)
            .finish(),
        );

        // Modal API Key
        col.add_child(
            Container::new(
                self.render_credential_input(appearance, "Modal API Key", view.modal_api_key_editor.clone()),
            )
            .with_margin_top(16.)
            .finish(),
        );

        // VPS Host
        col.add_child(
            Container::new(
                self.render_credential_input(appearance, "VPS Host", view.vps_host_editor.clone()),
            )
            .with_margin_top(12.)
            .finish(),
        );

        // VPS Username
        col.add_child(
            Container::new(
                self.render_credential_input(appearance, "VPS Username", view.vps_username_editor.clone()),
            )
            .with_margin_top(12.)
            .finish(),
        );

        // VPS SSH Key
        col.add_child(
            Container::new(
                self.render_credential_input(appearance, "VPS SSH Key", view.vps_ssh_key_editor.clone()),
            )
            .with_margin_top(12.)
            .finish(),
        );

        col.finish()
    }

    fn render_credential_input(
        &self,
        appearance: &Appearance,
        label: &str,
        editor: ViewHandle<EditorView>,
    ) -> Box<dyn Element> {
        let mut col = Flex::column();
        col.add_child(
            Text::new_inline(label, appearance.ui_font_family(), CONTENT_FONT_SIZE)
                .with_style(Properties::default().weight(Weight::Semibold))
                .with_color(appearance.theme().active_ui_text_color().into())
                .finish(),
        );
        col.add_child(
            Container::new(
                ConstrainedBox::new(ChildView::new(&editor).finish())
                    .with_height(36.)
                    .finish(),
            )
            .with_margin_top(6.)
            .with_padding(Padding::uniform(8.))
            .with_background(appearance.theme().surface_2().into())
            .with_corner_radius(CornerRadius::uniform(6.))
            .finish(),
        );
        col.finish()
    }

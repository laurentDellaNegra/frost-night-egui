use ui_theme::components::checkbox;
use ui_theme::Theme;

pub struct CheckboxStoryState {
    pub checked: bool,
    pub label: String,
    pub enabled: bool,
}

impl Default for CheckboxStoryState {
    fn default() -> Self {
        Self {
            checked: false,
            label: "Accept terms".into(),
            enabled: true,
        }
    }
}

pub fn checkbox_story(ui: &mut egui::Ui, theme: &Theme, state: &mut CheckboxStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Label")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.text_edit_singleline(&mut state.label);
        });
        ui.checkbox(&mut state.enabled, "Enabled");
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        ui.add_enabled_ui(state.enabled, |ui| {
            checkbox(ui, theme, &mut state.checked, &state.label);
        });
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("Unchecked")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        let mut unchecked = false;
        checkbox(ui, theme, &mut unchecked, "Unchecked option");

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("Checked")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        let mut checked = true;
        checkbox(ui, theme, &mut checked, "Checked option");

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("Disabled")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        ui.add_enabled_ui(false, |ui| {
            let mut disabled = false;
            checkbox(ui, theme, &mut disabled, "Disabled option");
        });
    });
}

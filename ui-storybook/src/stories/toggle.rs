use ui_theme::components::toggle;
use ui_theme::Theme;

pub struct ToggleStoryState {
    pub on: bool,
    pub enabled: bool,
}

impl Default for ToggleStoryState {
    fn default() -> Self {
        Self {
            on: false,
            enabled: true,
        }
    }
}

pub fn toggle_story(ui: &mut egui::Ui, theme: &Theme, state: &mut ToggleStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.checkbox(&mut state.enabled, "Enabled");
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        ui.add_enabled_ui(state.enabled, |ui| {
            ui.horizontal(|ui| {
                toggle(ui, theme, &mut state.on);
                ui.label(
                    egui::RichText::new(if state.on { "On" } else { "Off" })
                        .size(13.0)
                        .color(theme.palette.foreground),
                );
            });
        });
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("Off")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        let mut off = false;
        ui.horizontal(|ui| {
            toggle(ui, theme, &mut off);
            ui.label(
                egui::RichText::new("Toggle is off")
                    .size(13.0)
                    .color(theme.palette.foreground),
            );
        });

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("On")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        let mut on = true;
        ui.horizontal(|ui| {
            toggle(ui, theme, &mut on);
            ui.label(
                egui::RichText::new("Toggle is on")
                    .size(13.0)
                    .color(theme.palette.foreground),
            );
        });

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("Disabled")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        ui.add_enabled_ui(false, |ui| {
            let mut disabled = false;
            toggle(ui, theme, &mut disabled);
        });
    });
}

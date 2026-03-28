use ui_theme::components::card;
use ui_theme::Theme;

pub struct CardStoryState {
    pub content: String,
}

impl Default for CardStoryState {
    fn default() -> Self {
        Self {
            content: "This is card content. Cards are themed containers with borders and rounded corners.".into(),
        }
    }
}

pub fn card_story(ui: &mut egui::Ui, theme: &Theme, state: &mut CardStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Content")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.text_edit_singleline(&mut state.content);
        });
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        card(ui, theme, |ui| {
            ui.label(
                egui::RichText::new(&state.content)
                    .size(13.0)
                    .color(theme.palette.foreground),
            );
        });
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("Basic card")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        card(ui, theme, |ui| {
            ui.label(
                egui::RichText::new("Card Title")
                    .size(14.0)
                    .color(theme.palette.foreground)
                    .strong(),
            );
            ui.add_space(theme.spacing.xs);
            ui.label(
                egui::RichText::new("This is a basic card with a title and body text.")
                    .size(13.0)
                    .color(theme.palette.muted_foreground),
            );
        });

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("Card with mixed content")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        card(ui, theme, |ui| {
            ui.label(
                egui::RichText::new("Flight Information")
                    .size(14.0)
                    .color(theme.palette.foreground)
                    .strong(),
            );
            ui.add_space(theme.spacing.xs);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Callsign:")
                        .size(12.0)
                        .color(theme.palette.muted_foreground),
                );
                ui.label(
                    egui::RichText::new("SWR 1234")
                        .size(12.0)
                        .color(theme.palette.foreground),
                );
            });
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Altitude:")
                        .size(12.0)
                        .color(theme.palette.muted_foreground),
                );
                ui.label(
                    egui::RichText::new("FL350")
                        .size(12.0)
                        .color(theme.palette.foreground),
                );
            });
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Speed:")
                        .size(12.0)
                        .color(theme.palette.muted_foreground),
                );
                ui.label(
                    egui::RichText::new("450 kts")
                        .size(12.0)
                        .color(theme.palette.foreground),
                );
            });
        });
    });
}

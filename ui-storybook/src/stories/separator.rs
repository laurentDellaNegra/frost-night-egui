use ui_theme::components::separator;
use ui_theme::Theme;

pub struct SeparatorStoryState {
    pub count: usize,
}

impl Default for SeparatorStoryState {
    fn default() -> Self {
        Self { count: 3 }
    }
}

pub fn separator_story(ui: &mut egui::Ui, theme: &Theme, state: &mut SeparatorStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Number of separators")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.add(egui::Slider::new(&mut state.count, 1..=10));
        });
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        for i in 0..=state.count {
            ui.label(
                egui::RichText::new(format!("Block {}", i + 1))
                    .size(13.0)
                    .color(theme.palette.foreground),
            );
            ui.label(
                egui::RichText::new("Content for this block.")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            if i < state.count {
                ui.add_space(theme.spacing.xs);
                separator(ui, theme);
                ui.add_space(theme.spacing.xs);
            }
        }
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("Section One")
                .size(13.0)
                .color(theme.palette.foreground),
        );
        ui.label(
            egui::RichText::new("Some content in the first section.")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );

        ui.add_space(theme.spacing.xs);
        separator(ui, theme);
        ui.add_space(theme.spacing.xs);

        ui.label(
            egui::RichText::new("Section Two")
                .size(13.0)
                .color(theme.palette.foreground),
        );
        ui.label(
            egui::RichText::new("Some content in the second section.")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );

        ui.add_space(theme.spacing.xs);
        separator(ui, theme);
        ui.add_space(theme.spacing.xs);

        ui.label(
            egui::RichText::new("Section Three")
                .size(13.0)
                .color(theme.palette.foreground),
        );
        ui.label(
            egui::RichText::new("Some content in the third section.")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
    });
}

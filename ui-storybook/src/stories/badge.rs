use ui_theme::components::{badge, BadgeVariant};
use ui_theme::Theme;

const ALL_VARIANTS: [BadgeVariant; 4] = [
    BadgeVariant::Primary,
    BadgeVariant::Accent,
    BadgeVariant::Outline,
    BadgeVariant::Destructive,
];

pub struct BadgeStoryState {
    pub variant: BadgeVariant,
    pub label: String,
}

impl Default for BadgeStoryState {
    fn default() -> Self {
        Self {
            variant: BadgeVariant::Primary,
            label: "Badge".into(),
        }
    }
}

pub fn badge_story(ui: &mut egui::Ui, theme: &Theme, state: &mut BadgeStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Variant")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            egui::ComboBox::from_id_salt("badge_variant_combo")
                .selected_text(format!("{:?}", state.variant))
                .show_ui(ui, |ui| {
                    for v in &ALL_VARIANTS {
                        ui.selectable_value(&mut state.variant, *v, format!("{:?}", v));
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Label")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.text_edit_singleline(&mut state.label);
        });
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        badge(ui, theme, &state.label, state.variant);
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.horizontal(|ui| {
            for variant in &ALL_VARIANTS {
                badge(ui, theme, &format!("{:?}", variant), *variant);
                ui.add_space(theme.spacing.sm);
            }
        });

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("In context")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Flight status:")
                    .size(13.0)
                    .color(theme.palette.foreground),
            );
            badge(ui, theme, "Active", BadgeVariant::Primary);
            badge(ui, theme, "En Route", BadgeVariant::Accent);
            badge(ui, theme, "Delayed", BadgeVariant::Destructive);
            badge(ui, theme, "Scheduled", BadgeVariant::Outline);
        });
    });
}

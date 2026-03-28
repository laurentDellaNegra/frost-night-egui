use ui_theme::components::button;
use ui_theme::{ControlSize, ControlVariant, Theme};

const ALL_VARIANTS: [ControlVariant; 6] = [
    ControlVariant::Primary,
    ControlVariant::Secondary,
    ControlVariant::Ghost,
    ControlVariant::Outline,
    ControlVariant::Destructive,
    ControlVariant::Link,
];

const ALL_SIZES: [ControlSize; 3] = [ControlSize::Sm, ControlSize::Md, ControlSize::Lg];

pub struct ButtonStoryState {
    pub variant: ControlVariant,
    pub size: ControlSize,
    pub label: String,
    pub enabled: bool,
}

impl Default for ButtonStoryState {
    fn default() -> Self {
        Self {
            variant: ControlVariant::Primary,
            size: ControlSize::Md,
            label: "Click me".into(),
            enabled: true,
        }
    }
}

pub fn button_story(ui: &mut egui::Ui, theme: &Theme, state: &mut ButtonStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Variant")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            egui::ComboBox::from_id_salt("variant_combo")
                .selected_text(format!("{:?}", state.variant))
                .show_ui(ui, |ui| {
                    for v in &ALL_VARIANTS {
                        ui.selectable_value(&mut state.variant, *v, format!("{:?}", v));
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Size")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            egui::ComboBox::from_id_salt("size_combo")
                .selected_text(format!("{:?}", state.size))
                .show_ui(ui, |ui| {
                    for s in &ALL_SIZES {
                        ui.selectable_value(&mut state.size, *s, format!("{:?}", s));
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
        ui.checkbox(&mut state.enabled, "Enabled");
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        ui.add_enabled_ui(state.enabled, |ui| {
            button(ui, theme, &state.label, state.variant, state.size);
        });
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        for size in &ALL_SIZES {
            ui.label(
                egui::RichText::new(format!("Size: {:?}", size))
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.add_space(theme.spacing.xs);
            ui.horizontal(|ui| {
                for variant in &ALL_VARIANTS {
                    button(ui, theme, format!("{:?}", variant), *variant, *size);
                    ui.add_space(theme.spacing.xs);
                }
            });
            ui.add_space(theme.spacing.sm);
        }

        ui.add_space(theme.spacing.sm);
        ui.label(
            egui::RichText::new("Disabled")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        ui.add_enabled_ui(false, |ui| {
            ui.horizontal(|ui| {
                for variant in &ALL_VARIANTS {
                    button(
                        ui,
                        theme,
                        format!("{:?}", variant),
                        *variant,
                        ControlSize::Md,
                    );
                    ui.add_space(theme.spacing.xs);
                }
            });
        });
    });
}

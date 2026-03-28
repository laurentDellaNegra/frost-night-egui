use ui_theme::components::text_input;
use ui_theme::{ControlSize, Theme};

const ALL_SIZES: [ControlSize; 3] = [ControlSize::Sm, ControlSize::Md, ControlSize::Lg];

pub struct InputStoryState {
    pub text: String,
    pub size: ControlSize,
    pub enabled: bool,
}

impl Default for InputStoryState {
    fn default() -> Self {
        Self {
            text: "Hello, world".into(),
            size: ControlSize::Md,
            enabled: true,
        }
    }
}

pub fn input_story(ui: &mut egui::Ui, theme: &Theme, state: &mut InputStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Size")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            egui::ComboBox::from_id_salt("input_size_combo")
                .selected_text(format!("{:?}", state.size))
                .show_ui(ui, |ui| {
                    for s in &ALL_SIZES {
                        ui.selectable_value(&mut state.size, *s, format!("{:?}", s));
                    }
                });
        });
        ui.checkbox(&mut state.enabled, "Enabled");
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        ui.add_enabled_ui(state.enabled, |ui| {
            text_input(ui, theme, &mut state.text, state.size);
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
            let mut placeholder = String::new();
            text_input(ui, theme, &mut placeholder, *size);
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
            let mut disabled_text = "Disabled input".to_string();
            text_input(ui, theme, &mut disabled_text, ControlSize::Md);
        });
    });
}

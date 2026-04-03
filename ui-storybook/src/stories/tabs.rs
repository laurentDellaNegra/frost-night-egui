use ui_theme::components::tabs;
use ui_theme::Theme;

pub struct TabsStoryState {
    pub selected: usize,
    pub tab_count: usize,
    pub variant_selected_a: usize,
    pub variant_selected_b: usize,
}

impl Default for TabsStoryState {
    fn default() -> Self {
        Self {
            selected: 0,
            tab_count: 3,
            variant_selected_a: 0,
            variant_selected_b: 1,
        }
    }
}

pub fn tabs_story(ui: &mut egui::Ui, theme: &Theme, state: &mut TabsStoryState) {
    // Controls
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Tab count")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            let mut count = state.tab_count as f32;
            ui.add(egui::Slider::new(&mut count, 2.0..=6.0).step_by(1.0));
            state.tab_count = count as usize;
        });
    });

    let all_labels = ["Layers", "Filters", "Settings", "Weather", "Routes", "Status"];
    let labels: Vec<&str> = all_labels.iter().take(state.tab_count).copied().collect();

    if state.selected >= state.tab_count {
        state.selected = 0;
    }

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        tabs(ui, theme, &mut state.selected, &labels);
        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new(format!("Selected tab: {} (index {})", labels[state.selected], state.selected))
                .size(13.0)
                .color(theme.palette.muted_foreground),
        );
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("2 tabs")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        tabs(ui, theme, &mut state.variant_selected_a, &["Active", "Inactive"]);

        ui.add_space(theme.spacing.lg);
        ui.label(
            egui::RichText::new("5 tabs")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        tabs(ui, theme, &mut state.variant_selected_b, &["Maps", "Data", "Tools", "History", "Export"]);
    });
}

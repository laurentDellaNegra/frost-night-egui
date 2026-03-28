use ui_theme::components::segmented;
use ui_theme::Theme;

pub struct SegmentedStoryState {
    pub selected: usize,
    pub labels_text: String,
    pub enabled: bool,
}

impl Default for SegmentedStoryState {
    fn default() -> Self {
        Self {
            selected: 0,
            labels_text: "One,Two,Three".into(),
            enabled: true,
        }
    }
}

pub fn segmented_story(ui: &mut egui::Ui, theme: &Theme, state: &mut SegmentedStoryState) {
    // Compute max_idx from current labels (before controls modify them)
    let label_count = state.labels_text.split(',').count();
    let max_idx = if label_count == 0 { 0 } else { label_count - 1 };

    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Labels (comma-separated)")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.text_edit_singleline(&mut state.labels_text);
        });
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Selected index")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.add(egui::Slider::new(&mut state.selected, 0..=max_idx));
        });
        ui.checkbox(&mut state.enabled, "Enabled");
    });

    // Recompute labels after controls may have changed them
    let labels: Vec<String> = state
        .labels_text
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
    let current_max = if label_refs.is_empty() {
        0
    } else {
        label_refs.len() - 1
    };
    if state.selected > current_max {
        state.selected = current_max;
    }

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        ui.add_enabled_ui(state.enabled, |ui| {
            if !label_refs.is_empty() {
                segmented(ui, theme, &label_refs, &mut state.selected);
            }
        });
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("Two segments")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        let mut sel2 = 0;
        segmented(ui, theme, &["Map", "Satellite"], &mut sel2);

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("Three segments")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        let mut sel3 = 1;
        segmented(ui, theme, &["Day", "Night", "Auto"], &mut sel3);

        ui.add_space(theme.spacing.md);
        ui.label(
            egui::RichText::new("Four segments")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        let mut sel4 = 2;
        segmented(ui, theme, &["All", "Active", "Pending", "Closed"], &mut sel4);
    });
}

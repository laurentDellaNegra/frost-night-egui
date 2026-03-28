use ui_theme::Theme;

/// Section header label (e.g. "Controls", "Playground", "All Variants")
pub fn section_header(ui: &mut egui::Ui, theme: &Theme, label: &str) {
    ui.label(
        egui::RichText::new(label)
            .size(11.0)
            .strong()
            .color(theme.palette.muted_foreground),
    );
    ui.add_space(theme.spacing.sm);
}

/// Controls panel — muted background with border
pub fn controls_panel(ui: &mut egui::Ui, theme: &Theme, add_controls: impl FnOnce(&mut egui::Ui)) {
    section_header(ui, theme, "Controls");
    egui::Frame::NONE
        .fill(theme.palette.muted)
        .inner_margin(egui::Margin::same(theme.spacing.md as i8))
        .corner_radius(theme.radius.md)
        .stroke(egui::Stroke::new(1.0, theme.palette.border))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing =
                egui::vec2(theme.spacing.sm, theme.spacing.xs + 2.0);
            add_controls(ui);
        });
}

/// Spacing between sections
pub fn section_divider(ui: &mut egui::Ui, theme: &Theme) {
    ui.add_space(theme.spacing.xl * 2.0);
}

/// Labeled section for playground or gallery content
pub fn section_frame(
    ui: &mut egui::Ui,
    theme: &Theme,
    label: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    section_header(ui, theme, label);
    add_contents(ui);
}

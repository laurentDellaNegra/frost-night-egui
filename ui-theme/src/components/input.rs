//! Themed text input component.

use egui::{Response, TextBuffer, TextEdit, Ui};

use crate::theme::{ControlSize, Theme};

/// Apply Skyscope Design System text input styling to a caller-provided text edit.
pub fn text_edit(
    ui: &mut Ui,
    theme: &Theme,
    text_edit: TextEdit<'_>,
    size: ControlSize,
) -> Response {
    text_edit_enabled(ui, theme, true, text_edit, size)
}

/// Apply Skyscope Design System text input styling to a caller-provided text edit.
pub fn text_edit_enabled(
    ui: &mut Ui,
    theme: &Theme,
    enabled: bool,
    text_edit: TextEdit<'_>,
    size: ControlSize,
) -> Response {
    let vis = theme.input(size);

    ui.scope(|ui| {
        let style = ui.style_mut();
        style.visuals.extreme_bg_color = vis.bg;
        style.visuals.widgets.inactive.bg_stroke = vis.border;
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, theme.palette.ring);
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, theme.palette.ring);

        ui.add_enabled(
            enabled,
            text_edit
                .font(vis.font)
                .text_color(vis.text_color)
                .margin(egui::Margin::symmetric(theme.spacing.sm as i8, 6)),
        )
    })
    .inner
}

/// A themed single-line text input.
pub fn text_input<S: TextBuffer>(
    ui: &mut Ui,
    theme: &Theme,
    text: &mut S,
    size: ControlSize,
) -> Response {
    text_edit(
        ui,
        theme,
        TextEdit::singleline(text).desired_width(f32::INFINITY),
        size,
    )
}

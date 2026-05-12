//! Generic Skyscope Design System surface container.

use egui::{Response, Ui};

use crate::theme::Theme;

/// A translucent Skyscope Design System surface with border and rounded corners.
pub fn surface(ui: &mut Ui, theme: &Theme, add_contents: impl FnOnce(&mut Ui)) -> Response {
    egui::Frame::new()
        .fill(theme.palette.surface_blur)
        .stroke(egui::Stroke::new(1.0, theme.palette.border))
        .corner_radius(egui::CornerRadius::same(theme.radius.lg))
        .inner_margin(egui::Margin::same(theme.spacing.md as i8))
        .show(ui, add_contents)
        .response
}

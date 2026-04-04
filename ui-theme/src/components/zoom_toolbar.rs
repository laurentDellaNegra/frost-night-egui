//! Zoom control toolbar (bottom-right fixed position).
//!
//! A small vertical strip with plus/minus icon buttons, a separator,
//! and a "Reset" text button. Same glassmorphism backdrop as the main toolbar.

use egui::{CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};
use egui_flex::{Flex, FlexAlignContent, item};

use crate::icons::icon_font;
use crate::theme::Theme;

/// Response from the zoom toolbar.
pub struct ZoomToolbarResponse {
    /// The plus (zoom in) button was clicked.
    pub zoom_in: bool,
    /// The minus (zoom out) button was clicked.
    pub zoom_out: bool,
    /// The reset button was clicked.
    pub reset: bool,
}

/// Paint an icon button and return true if clicked.
fn icon_button(ui: &mut Ui, theme: &Theme, icon: char, size: f32, icon_size: f32) -> bool {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());
    let inner_cr = CornerRadius::same(theme.radius.md);
    if response.hovered() {
        let inset = rect.shrink(theme.control_gap);
        ui.painter()
            .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
    }
    let color = if response.hovered() {
        theme.palette.foreground
    } else {
        theme.palette.muted_foreground
    };
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        icon.to_string(),
        icon_font(icon_size),
        color,
    );
    response.clicked()
}

/// A vertical zoom control toolbar with +, −, and Reset buttons.
///
/// Paints its own semi-transparent backdrop and border, matching the
/// main toolbar visual style.
pub fn zoom_toolbar(
    ui: &mut Ui,
    theme: &Theme,
    rect: Rect,
    plus_icon: char,
    minus_icon: char,
) -> ZoomToolbarResponse {
    let icon_size = 18.0;
    let button_size = 36.0;
    let padding = theme.spacing.xs;
    let reset_height = 28.0;

    let mut result = ZoomToolbarResponse {
        zoom_in: false,
        zoom_out: false,
        reset: false,
    };

    if ui.is_rect_visible(rect) {
        let cr = CornerRadius::same(theme.radius.lg);

        // Backdrop
        ui.painter()
            .rect_filled(rect, cr, theme.palette.surface_blur);
        ui.painter().rect_stroke(
            rect,
            cr,
            Stroke::new(1.0, theme.palette.border),
            StrokeKind::Inside,
        );

        let inner_rect = rect.shrink(padding);
        let mut inner_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("zoom_toolbar")
                .max_rect(inner_rect),
        );

        Flex::vertical()
            .gap(Vec2::ZERO)
            .align_content(FlexAlignContent::Center)
            .show(&mut inner_ui, |flex| {
                // Plus button
                flex.add_ui(item(), |ui| {
                    result.zoom_in = icon_button(ui, theme, plus_icon, button_size, icon_size);
                });

                // Minus button
                flex.add_ui(item(), |ui| {
                    result.zoom_out = icon_button(ui, theme, minus_icon, button_size, icon_size);
                });

                // Separator
                flex.add_ui(item(), |ui| {
                    let sep_w = button_size - theme.spacing.xs * 2.0;
                    let (rect, _) = ui.allocate_exact_size(
                        Vec2::new(sep_w, 1.0 + theme.spacing.xs * 2.0),
                        Sense::hover(),
                    );
                    ui.painter().line_segment(
                        [rect.left_center(), rect.right_center()],
                        Stroke::new(1.0, theme.palette.border),
                    );
                });

                // Reset button
                flex.add_ui(item(), |ui| {
                    let (rect, response) =
                        ui.allocate_exact_size(Vec2::new(button_size, reset_height), Sense::click());
                    let inner_cr = CornerRadius::same(theme.radius.md);
                    if response.hovered() {
                        let inset = rect.shrink(theme.control_gap);
                        ui.painter()
                            .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
                    }
                    let color = if response.hovered() {
                        theme.palette.foreground
                    } else {
                        theme.palette.muted_foreground
                    };
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Reset",
                        egui::FontId::proportional(10.0),
                        color,
                    );
                    result.reset = response.clicked();
                });
            });
    }

    result
}

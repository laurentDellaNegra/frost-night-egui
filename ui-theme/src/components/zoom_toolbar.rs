//! Zoom control toolbar (bottom-right fixed position).
//!
//! A small vertical strip with plus/minus icon buttons, a separator,
//! and a "Reset" text button. Same glassmorphism backdrop as the main toolbar.

use egui::{CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

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

/// A vertical zoom control toolbar with +, −, and Reset buttons.
///
/// Paints its own semi-transparent backdrop and border, matching the
/// main toolbar visual style.
pub fn zoom_toolbar(
    ui: &mut Ui,
    theme: &Theme,
    plus_icon: char,
    minus_icon: char,
) -> ZoomToolbarResponse {
    let icon_size = 18.0;
    let button_size = 36.0;
    let padding = theme.spacing.xs;
    let separator_margin = theme.spacing.xs;
    let reset_height = 28.0;
    let font = egui::FontId::proportional(10.0);

    // Layout: padding + button + button + sep_margin + 1px + sep_margin + reset_height + padding
    let total_height =
        padding * 2.0 + button_size * 2.0 + separator_margin * 2.0 + 1.0 + reset_height;
    let total_width = button_size + padding * 2.0;

    let (outer_rect, _response) =
        ui.allocate_exact_size(Vec2::new(total_width, total_height), Sense::hover());

    let mut result = ZoomToolbarResponse {
        zoom_in: false,
        zoom_out: false,
        reset: false,
    };

    if ui.is_rect_visible(outer_rect) {
        let cr = CornerRadius::same(theme.radius.lg);

        // Backdrop
        ui.painter()
            .rect_filled(outer_rect, cr, theme.palette.surface_blur);
        ui.painter().rect_stroke(
            outer_rect,
            cr,
            Stroke::new(1.0, theme.palette.border),
            StrokeKind::Inside,
        );

        let center_x = outer_rect.center().x;
        let mut y = outer_rect.top() + padding;

        // --- Plus button ---
        let plus_rect = Rect::from_min_size(
            egui::pos2(center_x - button_size / 2.0, y),
            Vec2::splat(button_size),
        );
        let plus_id = ui.id().with("zoom_plus");
        let plus_response = ui.interact(plus_rect, plus_id, Sense::click());
        if plus_response.clicked() {
            result.zoom_in = true;
        }

        let inner_cr = CornerRadius::same(theme.radius.md);
        if plus_response.hovered() {
            let inset = plus_rect.shrink(theme.control_gap);
            ui.painter()
                .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
        }

        let plus_color = if plus_response.hovered() {
            theme.palette.foreground
        } else {
            theme.palette.muted_foreground
        };
        ui.painter().text(
            plus_rect.center(),
            egui::Align2::CENTER_CENTER,
            plus_icon.to_string(),
            icon_font(icon_size),
            plus_color,
        );

        y += button_size;

        // --- Minus button ---
        let minus_rect = Rect::from_min_size(
            egui::pos2(center_x - button_size / 2.0, y),
            Vec2::splat(button_size),
        );
        let minus_id = ui.id().with("zoom_minus");
        let minus_response = ui.interact(minus_rect, minus_id, Sense::click());
        if minus_response.clicked() {
            result.zoom_out = true;
        }

        if minus_response.hovered() {
            let inset = minus_rect.shrink(theme.control_gap);
            ui.painter()
                .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
        }

        let minus_color = if minus_response.hovered() {
            theme.palette.foreground
        } else {
            theme.palette.muted_foreground
        };
        ui.painter().text(
            minus_rect.center(),
            egui::Align2::CENTER_CENTER,
            minus_icon.to_string(),
            icon_font(icon_size),
            minus_color,
        );

        y += button_size;

        // --- Separator ---
        y += separator_margin;
        let sep_x0 = outer_rect.left() + padding + theme.spacing.xs;
        let sep_x1 = outer_rect.right() - padding - theme.spacing.xs;
        ui.painter().line_segment(
            [egui::pos2(sep_x0, y), egui::pos2(sep_x1, y)],
            Stroke::new(1.0, theme.palette.border),
        );
        y += 1.0 + separator_margin;

        // --- Reset button ---
        let reset_rect = Rect::from_min_size(
            egui::pos2(outer_rect.left() + padding, y),
            Vec2::new(button_size, reset_height),
        );
        let reset_id = ui.id().with("zoom_reset");
        let reset_response = ui.interact(reset_rect, reset_id, Sense::click());
        if reset_response.clicked() {
            result.reset = true;
        }

        if reset_response.hovered() {
            let inset = reset_rect.shrink(theme.control_gap);
            ui.painter()
                .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
        }

        let reset_color = if reset_response.hovered() {
            theme.palette.foreground
        } else {
            theme.palette.muted_foreground
        };
        ui.painter().text(
            reset_rect.center(),
            egui::Align2::CENTER_CENTER,
            "Reset",
            font,
            reset_color,
        );
    }

    result
}

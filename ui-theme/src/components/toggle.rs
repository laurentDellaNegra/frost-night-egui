//! Themed toggle/switch component.

use egui::{Color32, CornerRadius, Response, Sense, Ui, Vec2};

use crate::theme::Theme;
use crate::tokens::mix;

// Same colors as checkbox
const OUTER_BORDER: Color32 = Color32::from_rgb(0x3C, 0x46, 0x56);
const INNER_FILL_OFF: Color32 = Color32::from_rgb(0x0E, 0x1A, 0x38);
const INNER_FILL_ON: Color32 = Color32::from_rgb(0x16, 0x2C, 0x59);

/// A themed toggle switch matching the SkyScope design.
///
/// Pill-shaped outer border with a sliding rounded-rect thumb inside.
/// Same border, gap, and inner radius as the checkbox.
pub fn toggle(ui: &mut Ui, theme: &Theme, on: &mut bool) -> Response {
    let desired_size = Vec2::new(40.0, 22.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool_with_time(response.id, *on, 0.15);

        // Outer border (same color / style as checkbox)
        let outer_cr = CornerRadius::same(theme.radius.lg);
        let border_color = mix(OUTER_BORDER, theme.palette.muted_foreground, how_on * 0.3);
        ui.painter().rect_stroke(
            rect,
            outer_cr,
            egui::Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        // Inner thumb (3px gap, md radius — same as checkbox inner)
        let gap = 3.0;
        let inner_cr = CornerRadius::same(theme.radius.md);
        let thumb_size = rect.height() - gap * 2.0;
        let thumb_half = thumb_size / 2.0;
        let thumb_x = egui::lerp(
            rect.left() + thumb_half + gap..=rect.right() - thumb_half - gap,
            how_on,
        );
        let thumb_rect = egui::Rect::from_center_size(
            egui::pos2(thumb_x, rect.center().y),
            Vec2::splat(thumb_size),
        );

        let thumb_color = mix(INNER_FILL_OFF, INNER_FILL_ON, how_on);
        ui.painter().rect_filled(thumb_rect, inner_cr, thumb_color);

        // Checkmark on thumb when ON (fade in)
        if how_on > 0.01 {
            let alpha = (how_on * 255.0) as u8;
            let check_color = Color32::from_rgba_unmultiplied(0xFF, 0xFF, 0xFF, alpha);
            let center = thumb_rect.center();
            let s = thumb_size * 0.22;
            let painter = ui.painter();

            let p1 = center + egui::vec2(-s * 1.2, 0.0);
            let p2 = center + egui::vec2(-s * 0.2, s * 0.9);
            let p3 = center + egui::vec2(s * 1.2, -s * 0.8);
            let stroke = egui::Stroke::new(1.5, check_color);
            painter.line_segment([p1, p2], stroke);
            painter.line_segment([p2, p3], stroke);
        }
    }

    response
}

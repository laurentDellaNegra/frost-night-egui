//! Underline-style tab bar matching the Figma card mockup.

use egui::{Response, Sense, Stroke, Ui, Vec2};

use crate::theme::Theme;
use crate::tokens::mix;

/// A horizontal tab bar with underline indicator.
///
/// Returns the `Response` for the tab bar area. The selected index is
/// updated in-place via `selected`.
pub fn tabs(
    ui: &mut Ui,
    theme: &Theme,
    labels: &[&str],
    selected: &mut usize,
) -> Response {
    let tab_height = theme.spacing.sm * 2.0 + 14.0;
    let underline_thickness = 1.5;
    let anim_duration = 0.12;
    let tab_bar_id = ui.auto_id_with("tabs");

    let width = ui.available_width();
    let (bar_rect, bar_response) = ui.allocate_exact_size(
        Vec2::new(width, tab_height + underline_thickness),
        Sense::hover(),
    );

    if !ui.is_rect_visible(bar_rect) {
        return bar_response;
    }

    // Bottom border across full width
    ui.painter().hline(
        bar_rect.left()..=bar_rect.right(),
        bar_rect.bottom(),
        Stroke::new(1.0, theme.palette.border),
    );

    // Layout tabs
    let tab_padding_h = theme.spacing.md;
    let tab_gap = theme.spacing.xs;

    // Measure tab widths
    let galleys: Vec<_> = labels
        .iter()
        .map(|&label| {
            ui.painter().layout_no_wrap(
                label.to_string(),
                egui::FontId::proportional(13.0),
                theme.palette.foreground,
            )
        })
        .collect();

    let mut x = bar_rect.left();

    for (i, galley) in galleys.iter().enumerate() {
        let tab_id = tab_bar_id.with(i);
        let is_selected = *selected == i;

        let tab_w = galley.size().x + tab_padding_h * 2.0;
        let tab_rect = egui::Rect::from_min_size(
            egui::pos2(x, bar_rect.top()),
            Vec2::new(tab_w, tab_height + underline_thickness),
        );

        // Click detection
        let tab_response = ui.interact(tab_rect, tab_id, Sense::click());
        if tab_response.clicked() {
            *selected = i;
        }

        // Animate selection
        let sel_t = ui.ctx().animate_bool_with_time(
            tab_id.with("sel"),
            *selected == i,
            anim_duration,
        );

        // Hover
        let hovered = tab_response.hovered() && !is_selected;

        // Text color: muted → foreground
        let text_color = if hovered {
            mix(theme.palette.muted_foreground, theme.palette.foreground, 0.5)
        } else {
            mix(theme.palette.muted_foreground, theme.palette.foreground, sel_t)
        };

        // Draw text
        let text_galley = ui.painter().layout_no_wrap(
            labels[i].to_string(),
            egui::FontId::proportional(13.0),
            text_color,
        );
        let text_x = tab_rect.left() + tab_padding_h;
        let text_y = bar_rect.top() + (tab_height - text_galley.size().y) / 2.0;
        ui.painter().galley(
            egui::pos2(text_x, text_y),
            text_galley,
            text_color,
        );

        // Underline indicator (animated)
        if sel_t > 0.0 {
            let underline_color = mix(
                egui::Color32::TRANSPARENT,
                theme.palette.ring,
                sel_t,
            );
            let underline_y = bar_rect.bottom() - underline_thickness / 2.0;
            // Animate width from center
            let center_x = tab_rect.center().x;
            let half_w = (tab_w * sel_t) / 2.0;
            ui.painter().hline(
                (center_x - half_w)..=(center_x + half_w),
                underline_y,
                Stroke::new(underline_thickness, underline_color),
            );
        }

        x += tab_w + tab_gap;
    }

    bar_response
}

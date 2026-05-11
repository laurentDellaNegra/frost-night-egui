//! Themed segmented control (toggle buttons).

use egui::{CornerRadius, Response, Sense, Ui, Vec2};

use crate::theme::mix;
use crate::theme::Theme;

/// Per-segment styling for the active segment state.
#[derive(Clone, Copy, Debug)]
pub struct SegmentStyle {
    pub active_fill: egui::Color32,
    pub active_text: Option<egui::Color32>,
}

/// A horizontal segmented control. Returns the newly selected index if changed.
///
/// Same outer border, gap, and inner radius as checkbox/toggle.
/// Active segment has a navy-filled inset rect; inactive segments are transparent.
pub fn segmented(ui: &mut Ui, theme: &Theme, labels: &[&str], selected: &mut usize) -> Response {
    let default_style = SegmentStyle {
        active_fill: theme.palette.control_fill_on,
        active_text: Some(theme.palette.foreground),
    };

    segmented_styled(ui, theme, labels, &[default_style], selected)
}

/// A horizontal segmented control with per-segment active fills.
pub fn segmented_with_fills(
    ui: &mut Ui,
    theme: &Theme,
    labels: &[&str],
    active_fills: &[egui::Color32],
    selected: &mut usize,
) -> Response {
    let styles: Vec<_> = active_fills
        .iter()
        .copied()
        .map(|active_fill| SegmentStyle {
            active_fill,
            active_text: None,
        })
        .collect();

    segmented_styled(ui, theme, labels, &styles, selected)
}

/// A horizontal segmented control with per-segment active styling.
pub fn segmented_styled(
    ui: &mut Ui,
    theme: &Theme,
    labels: &[&str],
    styles: &[SegmentStyle],
    selected: &mut usize,
) -> Response {
    let default_style = SegmentStyle {
        active_fill: theme.palette.control_fill_on,
        active_text: Some(theme.palette.foreground),
    };
    let font = egui::FontId::proportional(12.0);
    let pad = Vec2::new(theme.spacing.lg, theme.spacing.xs + 2.0);
    let gap = theme.control_gap;

    // Measure total width
    let galleys: Vec<_> = labels
        .iter()
        .map(|l| {
            ui.painter()
                .layout_no_wrap(l.to_string(), font.clone(), theme.palette.foreground)
        })
        .collect();
    let segment_widths: Vec<f32> = galleys.iter().map(|g| g.size().x + pad.x * 2.0).collect();
    let total_width: f32 = segment_widths.iter().sum();
    let height = galleys.iter().map(|g| g.size().y).fold(0.0_f32, f32::max) + pad.y * 2.0;

    let (outer_rect, mut response) =
        ui.allocate_exact_size(Vec2::new(total_width, height), Sense::click());

    if ui.is_rect_visible(outer_rect) {
        let outer_cr = CornerRadius::same(theme.radius.lg);
        let inner_cr = CornerRadius::same(theme.radius.md);

        // Outer border (same as checkbox/toggle)
        ui.painter().rect_stroke(
            outer_rect,
            outer_cr,
            egui::Stroke::new(1.0, theme.palette.control_border),
            egui::StrokeKind::Inside,
        );

        let mut x = outer_rect.left();
        for (i, galley) in galleys.into_iter().enumerate() {
            let seg_rect = egui::Rect::from_min_size(
                egui::pos2(x, outer_rect.top()),
                Vec2::new(segment_widths[i], height),
            );

            let is_active = i == *selected;
            let hovered =
                response.hovered() && seg_rect.contains(response.hover_pos().unwrap_or_default());

            // Handle click on this segment
            if ui.is_enabled() && response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    if seg_rect.contains(pos) && !is_active {
                        *selected = i;
                        response.mark_changed();
                    }
                }
            }

            // Active segment: inset filled rect (3px gap, md radius)
            if is_active {
                let style = styles.get(i).copied().unwrap_or(default_style);
                let inset = seg_rect.shrink(gap);
                ui.painter().rect_filled(inset, inner_cr, style.active_fill);
            }

            // Text
            let text_color = if is_active {
                let style = styles.get(i).copied().unwrap_or(default_style);
                style
                    .active_text
                    .unwrap_or_else(|| contrast_text_color(style.active_fill, theme))
            } else if hovered {
                mix(
                    theme.palette.muted_foreground,
                    theme.palette.foreground,
                    0.3,
                )
            } else {
                theme.palette.muted_foreground
            };

            let text_pos = egui::pos2(
                seg_rect.center().x - galley.size().x / 2.0,
                seg_rect.center().y - galley.size().y / 2.0,
            );
            ui.painter().galley(text_pos, galley, text_color);

            x += segment_widths[i];
        }
    }

    response
}

fn contrast_text_color(fill: egui::Color32, theme: &Theme) -> egui::Color32 {
    let luminance = (0.2126 * f32::from(fill.r())
        + 0.7152 * f32::from(fill.g())
        + 0.0722 * f32::from(fill.b()))
        / 255.0;

    if luminance > 0.62 {
        theme.palette.background
    } else {
        egui::Color32::WHITE
    }
}

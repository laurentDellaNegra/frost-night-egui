//! Horizontal top toolbar with title, clock, aviation data, error indicator, and icon buttons.
//!
//! Paints its own semi-transparent backdrop. Positioned at the top of the screen,
//! to the right of the left sidebar toolbar.

use egui::{CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::icons::{icon_font, ICON_CIRCLE_X};
use crate::theme::Theme;

/// Response from the top toolbar.
pub struct TopToolbarResponse {
    /// Index of the icon button that was clicked, if any.
    pub icon_clicked: Option<usize>,
}

/// A horizontal top toolbar with app title, clock, QNH/TL data, error indicator, and icon buttons.
///
/// Paints its own semi-transparent backdrop and border.
pub fn top_toolbar(
    ui: &mut Ui,
    theme: &Theme,
    title: &str,
    clock: &str,
    qnh: &str,
    tl: &str,
    error: Option<&str>,
    icons: &[char],
) -> TopToolbarResponse {
    let height = 36.0;
    let pad_h = theme.spacing.sm;
    let section_gap = theme.spacing.md;
    let icon_btn_size = 28.0;
    let icon_size = 16.0;
    let sep_margin_v = theme.spacing.sm;

    let title_font = egui::FontId::new(18.0, egui::FontFamily::Proportional);
    let clock_font = egui::FontId::monospace(13.0);
    let label_font = egui::FontId::proportional(12.0);
    let value_font = egui::FontId::monospace(12.0);
    let error_font = egui::FontId::proportional(12.0);

    // Measure text widths for layout
    let painter = ui.painter();
    let snowflake_font = crate::icons::icon_font(18.0);
    let snowflake_galley = painter.layout_no_wrap(
        crate::icons::ICON_SNOWFLAKE.to_string(),
        snowflake_font,
        theme.palette.ring,
    );
    let snowflake_gap = theme.spacing.xs + 2.0;
    let mut title_job = egui::text::LayoutJob::single_section(
        title.to_string(),
        egui::TextFormat {
            font_id: title_font.clone(),
            color: theme.palette.foreground,
            ..Default::default()
        },
    );
    title_job.wrap = egui::text::TextWrapping::no_max_width();
    let title_galley = painter.layout_job(title_job);
    let clock_galley = painter.layout_no_wrap(
        clock.to_string(),
        clock_font.clone(),
        theme.palette.foreground,
    );
    let qnh_label_galley = painter.layout_no_wrap(
        "QNH".to_string(),
        label_font.clone(),
        theme.palette.muted_foreground,
    );
    let qnh_val_galley = painter.layout_no_wrap(
        qnh.to_string(),
        value_font.clone(),
        theme.palette.foreground,
    );
    let tl_label_galley = painter.layout_no_wrap(
        "TL".to_string(),
        label_font.clone(),
        theme.palette.muted_foreground,
    );
    let tl_val_galley =
        painter.layout_no_wrap(tl.to_string(), value_font.clone(), theme.palette.foreground);

    let error_icon_size = 14.0;
    let error_width = if let Some(err_text) = error {
        let err_galley = painter.layout_no_wrap(
            err_text.to_string(),
            error_font.clone(),
            theme.palette.destructive,
        );
        // circle-x icon + gap + text
        error_icon_size + theme.spacing.xs + err_galley.size().x
    } else {
        0.0
    };

    let label_value_gap = theme.spacing.xs;
    let pair_gap = theme.spacing.md;

    // Total width calculation
    let mut total_w = pad_h; // left padding
    total_w += snowflake_galley.size().x + snowflake_gap + title_galley.size().x;
    total_w += section_gap; // gap before separator
    total_w += 1.0; // separator
    total_w += section_gap; // gap after separator
    total_w += clock_galley.size().x;
    total_w += section_gap + 1.0 + section_gap; // separator
    total_w += qnh_label_galley.size().x + label_value_gap + qnh_val_galley.size().x;
    total_w += pair_gap;
    total_w += tl_label_galley.size().x + label_value_gap + tl_val_galley.size().x;
    total_w += section_gap + 1.0 + section_gap; // separator after TL
    if error.is_some() {
        total_w += error_width;
    }
    if !icons.is_empty() {
        total_w += section_gap + 1.0 + section_gap; // separator
        total_w += icons.len() as f32 * icon_btn_size;
    }
    total_w += pad_h; // right padding

    let (outer_rect, _response) =
        ui.allocate_exact_size(Vec2::new(total_w, height), Sense::hover());

    let mut result = TopToolbarResponse { icon_clicked: None };

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

        let center_y = outer_rect.center().y;
        let mut x = outer_rect.left() + pad_h;

        // --- Snowflake + Title (vertically centered as a group) ---
        let group_h = snowflake_galley.size().y.max(title_galley.size().y);
        let group_top = center_y - group_h / 2.0;
        ui.painter().galley(
            egui::pos2(x, group_top + (group_h - snowflake_galley.size().y) / 2.0),
            snowflake_galley.clone(),
            theme.palette.ring,
        );
        x += snowflake_galley.size().x + snowflake_gap;
        ui.painter().galley(
            egui::pos2(x, group_top + (group_h - title_galley.size().y) / 2.0),
            title_galley.clone(),
            theme.palette.foreground,
        );
        x += title_galley.size().x + section_gap;

        // --- Vertical separator ---
        let sep_top = outer_rect.top() + sep_margin_v;
        let sep_bot = outer_rect.bottom() - sep_margin_v;
        ui.painter().line_segment(
            [egui::pos2(x, sep_top), egui::pos2(x, sep_bot)],
            Stroke::new(1.0, theme.palette.border),
        );
        x += 1.0 + section_gap;

        // --- Clock ---
        ui.painter().galley(
            egui::pos2(x, center_y - clock_galley.size().y / 2.0),
            clock_galley.clone(),
            theme.palette.foreground,
        );
        x += clock_galley.size().x + section_gap;

        // --- Vertical separator ---
        ui.painter().line_segment(
            [egui::pos2(x, sep_top), egui::pos2(x, sep_bot)],
            Stroke::new(1.0, theme.palette.border),
        );
        x += 1.0 + section_gap;

        // --- QNH label + value ---
        ui.painter().galley(
            egui::pos2(x, center_y - qnh_label_galley.size().y / 2.0),
            qnh_label_galley.clone(),
            theme.palette.muted_foreground,
        );
        x += qnh_label_galley.size().x + label_value_gap;
        ui.painter().galley(
            egui::pos2(x, center_y - qnh_val_galley.size().y / 2.0),
            qnh_val_galley.clone(),
            theme.palette.foreground,
        );
        x += qnh_val_galley.size().x + pair_gap;

        // --- TL label + value ---
        ui.painter().galley(
            egui::pos2(x, center_y - tl_label_galley.size().y / 2.0),
            tl_label_galley.clone(),
            theme.palette.muted_foreground,
        );
        x += tl_label_galley.size().x + label_value_gap;
        ui.painter().galley(
            egui::pos2(x, center_y - tl_val_galley.size().y / 2.0),
            tl_val_galley.clone(),
            theme.palette.foreground,
        );
        x += tl_val_galley.size().x + section_gap;

        // --- Vertical separator after TL ---
        ui.painter().line_segment(
            [egui::pos2(x, sep_top), egui::pos2(x, sep_bot)],
            Stroke::new(1.0, theme.palette.border),
        );
        x += 1.0 + section_gap;

        // --- Error indicator ---
        if let Some(err_text) = error {
            // Lucide circle-x icon
            ui.painter().text(
                egui::pos2(x + error_icon_size / 2.0, center_y),
                egui::Align2::CENTER_CENTER,
                ICON_CIRCLE_X.to_string(),
                icon_font(error_icon_size),
                theme.palette.destructive,
            );
            x += error_icon_size + theme.spacing.xs;

            // Error text
            let err_galley = ui.painter().layout_no_wrap(
                err_text.to_string(),
                error_font,
                theme.palette.destructive,
            );
            ui.painter().galley(
                egui::pos2(x, center_y - err_galley.size().y / 2.0),
                err_galley.clone(),
                theme.palette.destructive,
            );
            x += err_galley.size().x;
        }

        // --- Icon buttons ---
        if !icons.is_empty() {
            x += section_gap;

            // Vertical separator before icons
            ui.painter().line_segment(
                [egui::pos2(x, sep_top), egui::pos2(x, sep_bot)],
                Stroke::new(1.0, theme.palette.border),
            );
            x += 1.0 + section_gap;

            let inner_cr = CornerRadius::same(theme.radius.md);
            for (i, &icon) in icons.iter().enumerate() {
                let btn_rect = Rect::from_min_size(
                    egui::pos2(x, center_y - icon_btn_size / 2.0),
                    Vec2::splat(icon_btn_size),
                );
                let btn_id = ui.id().with(("top_tb_icon", i));
                let btn_response = ui.interact(btn_rect, btn_id, Sense::click());

                if btn_response.clicked() {
                    result.icon_clicked = Some(i);
                }

                if btn_response.hovered() {
                    let inset = btn_rect.shrink(theme.control_gap);
                    ui.painter()
                        .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
                }

                let icon_color = if btn_response.hovered() {
                    theme.palette.foreground
                } else {
                    theme.palette.muted_foreground
                };
                ui.painter().text(
                    btn_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    icon.to_string(),
                    icon_font(icon_size),
                    icon_color,
                );

                x += icon_btn_size;
            }
        }
    }

    result
}

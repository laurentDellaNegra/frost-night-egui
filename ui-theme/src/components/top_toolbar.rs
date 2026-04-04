//! Horizontal top toolbar with title, clock, aviation data, error indicator, and icon buttons.
//!
//! Paints its own semi-transparent backdrop. Positioned at the top of the screen,
//! to the right of the left sidebar toolbar.

use egui::{CornerRadius, Sense, Stroke, StrokeKind, Ui, Vec2};
use egui_flex::{Flex, FlexAlign, item};

use crate::icons::{icon_font, ICON_CIRCLE_X, ICON_SNOWFLAKE};
use crate::theme::Theme;

/// Response from the top toolbar.
pub struct TopToolbarResponse {
    /// Index of the icon button that was clicked, if any.
    pub icon_clicked: Option<usize>,
}

/// Paint a vertical separator line (1px wide, with vertical margins).
fn separator(ui: &mut Ui, theme: &Theme, height: f32, margin_v: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(1.0, height), Sense::hover());
    ui.painter().line_segment(
        [
            rect.center_top() + egui::vec2(0.0, margin_v),
            rect.center_bottom() - egui::vec2(0.0, margin_v),
        ],
        Stroke::new(1.0, theme.palette.border),
    );
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

    let label_value_gap = theme.spacing.xs;
    let pair_gap = theme.spacing.md;
    let snowflake_gap = theme.spacing.xs + 2.0;
    let error_icon_size = 14.0;

    // Pre-compute width so backdrop is content-sized
    let painter = ui.painter();
    let snowflake_w = painter
        .layout_no_wrap(
            ICON_SNOWFLAKE.to_string(),
            icon_font(18.0),
            theme.palette.ring,
        )
        .size()
        .x;
    let title_w = painter
        .layout_no_wrap(
            title.to_string(),
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
            theme.palette.foreground,
        )
        .size()
        .x;
    let clock_w = painter
        .layout_no_wrap(
            clock.to_string(),
            egui::FontId::monospace(13.0),
            theme.palette.foreground,
        )
        .size()
        .x;
    let qnh_label_w = painter
        .layout_no_wrap(
            "QNH".to_string(),
            egui::FontId::proportional(12.0),
            theme.palette.muted_foreground,
        )
        .size()
        .x;
    let qnh_val_w = painter
        .layout_no_wrap(
            qnh.to_string(),
            egui::FontId::monospace(12.0),
            theme.palette.foreground,
        )
        .size()
        .x;
    let tl_label_w = painter
        .layout_no_wrap(
            "TL".to_string(),
            egui::FontId::proportional(12.0),
            theme.palette.muted_foreground,
        )
        .size()
        .x;
    let tl_val_w = painter
        .layout_no_wrap(
            tl.to_string(),
            egui::FontId::monospace(12.0),
            theme.palette.foreground,
        )
        .size()
        .x;
    let error_w = if let Some(err_text) = error {
        let err_w = painter
            .layout_no_wrap(
                err_text.to_string(),
                egui::FontId::proportional(12.0),
                theme.palette.destructive,
            )
            .size()
            .x;
        section_gap + 1.0 + section_gap + error_icon_size + theme.spacing.xs + err_w
    } else {
        0.0
    };
    let icons_w = if !icons.is_empty() {
        section_gap + 1.0 + section_gap + icons.len() as f32 * icon_btn_size
    } else {
        0.0
    };

    let total_w = pad_h
        + snowflake_w + snowflake_gap + title_w
        + section_gap + 1.0 + section_gap  // separator
        + clock_w
        + section_gap + 1.0 + section_gap  // separator
        + qnh_label_w + label_value_gap + qnh_val_w + pair_gap + tl_label_w + label_value_gap + tl_val_w
        + section_gap + 1.0 + section_gap  // separator
        + error_w
        + icons_w
        + pad_h;

    let (outer_rect, _) = ui.allocate_exact_size(Vec2::new(total_w, height), Sense::hover());

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

        let inner_rect = outer_rect.shrink2(Vec2::new(pad_h, 0.0));
        let mut inner_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("top_tb_inner")
                .max_rect(inner_rect),
        );

        Flex::horizontal()
            .gap(Vec2::new(section_gap, 0.0))
            .align_items(FlexAlign::Center)
            .show(&mut inner_ui, |flex| {
                // --- Snowflake + Title ---
                flex.add_ui(item(), |ui| {
                    Flex::horizontal()
                        .gap(Vec2::new(snowflake_gap, 0.0))
                        .align_items(FlexAlign::Center)
                        .show(ui, |flex| {
                            flex.add_ui(item(), |ui| {
                                let (rect, _) = ui.allocate_exact_size(
                                    Vec2::new(snowflake_w, height),
                                    Sense::hover(),
                                );
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    ICON_SNOWFLAKE.to_string(),
                                    icon_font(18.0),
                                    theme.palette.ring,
                                );
                            });
                            flex.add_ui(item(), |ui| {
                                let (rect, _) = ui.allocate_exact_size(
                                    Vec2::new(title_w, height),
                                    Sense::hover(),
                                );
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    title,
                                    egui::FontId::new(18.0, egui::FontFamily::Proportional),
                                    theme.palette.foreground,
                                );
                            });
                        });
                });

                // Separator
                flex.add_ui(item(), |ui| separator(ui, theme, height, sep_margin_v));

                // --- Clock ---
                flex.add_ui(item(), |ui| {
                    ui.label(
                        egui::RichText::new(clock)
                            .font(egui::FontId::monospace(13.0))
                            .color(theme.palette.foreground),
                    );
                });

                // Separator
                flex.add_ui(item(), |ui| separator(ui, theme, height, sep_margin_v));

                // --- QNH + TL ---
                flex.add_ui(item(), |ui| {
                    Flex::horizontal()
                        .gap(Vec2::new(pair_gap, 0.0))
                        .align_items(FlexAlign::Center)
                        .show(ui, |flex| {
                            // QNH pair
                            flex.add_ui(item(), |ui| {
                                Flex::horizontal()
                                    .gap(Vec2::new(label_value_gap, 0.0))
                                    .align_items(FlexAlign::Center)
                                    .show(ui, |flex| {
                                        flex.add_ui(item(), |ui| {
                                            ui.label(
                                                egui::RichText::new("QNH")
                                                    .font(egui::FontId::proportional(12.0))
                                                    .color(theme.palette.muted_foreground),
                                            );
                                        });
                                        flex.add_ui(item(), |ui| {
                                            ui.label(
                                                egui::RichText::new(qnh)
                                                    .font(egui::FontId::monospace(12.0))
                                                    .color(theme.palette.foreground),
                                            );
                                        });
                                    });
                            });

                            // TL pair
                            flex.add_ui(item(), |ui| {
                                Flex::horizontal()
                                    .gap(Vec2::new(label_value_gap, 0.0))
                                    .align_items(FlexAlign::Center)
                                    .show(ui, |flex| {
                                        flex.add_ui(item(), |ui| {
                                            ui.label(
                                                egui::RichText::new("TL")
                                                    .font(egui::FontId::proportional(12.0))
                                                    .color(theme.palette.muted_foreground),
                                            );
                                        });
                                        flex.add_ui(item(), |ui| {
                                            ui.label(
                                                egui::RichText::new(tl)
                                                    .font(egui::FontId::monospace(12.0))
                                                    .color(theme.palette.foreground),
                                            );
                                        });
                                    });
                            });
                        });
                });

                // Separator
                flex.add_ui(item(), |ui| separator(ui, theme, height, sep_margin_v));

                // --- Error indicator (optional) ---
                if let Some(err_text) = error {
                    flex.add_ui(item(), |ui| {
                        Flex::horizontal()
                            .gap(Vec2::new(theme.spacing.xs, 0.0))
                            .align_items(FlexAlign::Center)
                            .show(ui, |flex| {
                                flex.add_ui(item(), |ui| {
                                    ui.label(
                                        egui::RichText::new(ICON_CIRCLE_X.to_string())
                                            .font(icon_font(error_icon_size))
                                            .color(theme.palette.destructive),
                                    );
                                });
                                flex.add_ui(item(), |ui| {
                                    ui.label(
                                        egui::RichText::new(err_text)
                                            .font(egui::FontId::proportional(12.0))
                                            .color(theme.palette.destructive),
                                    );
                                });
                            });
                    });
                }

                // --- Icon buttons (optional) ---
                if !icons.is_empty() {
                    // Separator before icons
                    flex.add_ui(item(), |ui| separator(ui, theme, height, sep_margin_v));

                    let inner_cr = CornerRadius::same(theme.radius.md);
                    for (i, &icon) in icons.iter().enumerate() {
                        let idx = i;
                        flex.add_ui(item(), |ui| {
                            let (rect, response) = ui.allocate_exact_size(
                                Vec2::splat(icon_btn_size),
                                Sense::click(),
                            );

                            if response.clicked() {
                                result.icon_clicked = Some(idx);
                            }

                            if response.hovered() {
                                let inset = rect.shrink(theme.control_gap);
                                ui.painter().rect_filled(
                                    inset,
                                    inner_cr,
                                    theme.palette.control_fill_off,
                                );
                            }

                            let icon_color = if response.hovered() {
                                theme.palette.foreground
                            } else {
                                theme.palette.muted_foreground
                            };
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                icon.to_string(),
                                icon_font(icon_size),
                                icon_color,
                            );
                        });
                    }
                }
            });
    }

    result
}

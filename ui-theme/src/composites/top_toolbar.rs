//! Generic horizontal status toolbar.
//!
//! Paints its own semi-transparent backdrop. Product/domain semantics are
//! supplied by the caller as status fields and action definitions.

use std::hash::Hash;

use egui::{CornerRadius, Sense, Stroke, StrokeKind, Ui, Vec2};
use egui_flex::{item, Flex, FlexAlign};

use crate::icons::icon_font;
use crate::theme::Theme;

/// Visual state for a toolbar status field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusFieldKind {
    Normal,
    Warning,
    Error,
    Muted,
}

/// A label/value status field rendered in the top toolbar.
#[derive(Clone, Copy, Debug)]
pub struct StatusField<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub kind: StatusFieldKind,
}

/// A clickable icon action rendered at the end of the toolbar.
#[derive(Clone, Copy, Debug)]
pub struct ToolbarAction {
    pub icon: char,
    pub selected: bool,
    pub disabled: bool,
}

/// Response from the top toolbar.
pub struct TopToolbarResponse {
    /// Index of the action button that was clicked this frame, if any.
    pub icon_clicked: Option<usize>,
}

fn field_color(theme: &Theme, kind: StatusFieldKind) -> egui::Color32 {
    match kind {
        StatusFieldKind::Normal => theme.palette.foreground,
        StatusFieldKind::Warning => theme.palette.primary,
        StatusFieldKind::Error => theme.palette.destructive,
        StatusFieldKind::Muted => theme.palette.muted_foreground,
    }
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

/// A horizontal top toolbar with title, generic status fields, and icon actions.
///
/// Use [`top_toolbar_with_id`] when rendering multiple toolbars under the same
/// parent `Ui`.
pub fn top_toolbar(
    ui: &mut Ui,
    theme: &Theme,
    title: &str,
    fields: &[StatusField<'_>],
    actions: &[ToolbarAction],
) -> TopToolbarResponse {
    top_toolbar_with_id(ui, theme, "top_toolbar", title, fields, actions)
}

/// A horizontal top toolbar with a caller-provided ID salt.
pub fn top_toolbar_with_id(
    ui: &mut Ui,
    theme: &Theme,
    id_salt: impl Hash,
    title: &str,
    fields: &[StatusField<'_>],
    actions: &[ToolbarAction],
) -> TopToolbarResponse {
    let height = 36.0;
    let pad_h = theme.spacing.sm;
    let section_gap = theme.spacing.md;
    let icon_btn_size = 28.0;
    let icon_size = 16.0;
    let sep_margin_v = theme.spacing.sm;
    let label_value_gap = theme.spacing.xs;
    let field_gap = theme.spacing.md;

    let painter = ui.painter();
    let title_w = painter
        .layout_no_wrap(
            title.to_owned(),
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
            theme.palette.foreground,
        )
        .size()
        .x;

    let fields_w: f32 = fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let label_w = painter
                .layout_no_wrap(
                    field.label.to_owned(),
                    egui::FontId::proportional(12.0),
                    theme.palette.muted_foreground,
                )
                .size()
                .x;
            let value_w = painter
                .layout_no_wrap(
                    field.value.to_owned(),
                    egui::FontId::monospace(12.0),
                    field_color(theme, field.kind),
                )
                .size()
                .x;
            label_w + label_value_gap + value_w + if i > 0 { field_gap } else { 0.0 }
        })
        .sum();

    let fields_section_w = if fields.is_empty() {
        0.0
    } else {
        section_gap + 1.0 + section_gap + fields_w
    };
    let actions_w = if actions.is_empty() {
        0.0
    } else {
        section_gap + 1.0 + section_gap + actions.len() as f32 * icon_btn_size
    };
    let total_w = pad_h + title_w + fields_section_w + actions_w + pad_h;

    let (outer_rect, _) = ui.allocate_exact_size(Vec2::new(total_w, height), Sense::hover());

    let mut result = TopToolbarResponse { icon_clicked: None };

    if ui.is_rect_visible(outer_rect) {
        let cr = CornerRadius::same(theme.radius.lg);

        ui.painter()
            .rect_filled(outer_rect, cr, theme.palette.surface_blur);
        ui.painter().rect_stroke(
            outer_rect,
            cr,
            Stroke::new(1.0, theme.palette.border),
            StrokeKind::Inside,
        );

        let inner_rect = outer_rect.shrink2(Vec2::new(pad_h, 0.0));
        let mut inner_ui =
            ui.new_child(egui::UiBuilder::new().id_salt(id_salt).max_rect(inner_rect));

        Flex::horizontal()
            .gap(Vec2::new(section_gap, 0.0))
            .align_items(FlexAlign::Center)
            .show(&mut inner_ui, |flex| {
                flex.add_ui(item(), |ui| {
                    ui.label(
                        egui::RichText::new(title)
                            .font(egui::FontId::new(18.0, egui::FontFamily::Proportional))
                            .color(theme.palette.foreground),
                    );
                });

                if !fields.is_empty() {
                    flex.add_ui(item(), |ui| separator(ui, theme, height, sep_margin_v));
                    flex.add_ui(item(), |ui| {
                        Flex::horizontal()
                            .gap(Vec2::new(field_gap, 0.0))
                            .align_items(FlexAlign::Center)
                            .show(ui, |flex| {
                                for field in fields {
                                    flex.add_ui(item(), |ui| {
                                        Flex::horizontal()
                                            .gap(Vec2::new(label_value_gap, 0.0))
                                            .align_items(FlexAlign::Center)
                                            .show(ui, |flex| {
                                                flex.add_ui(item(), |ui| {
                                                    ui.label(
                                                        egui::RichText::new(field.label)
                                                            .font(egui::FontId::proportional(12.0))
                                                            .color(theme.palette.muted_foreground),
                                                    );
                                                });
                                                flex.add_ui(item(), |ui| {
                                                    ui.label(
                                                        egui::RichText::new(field.value)
                                                            .font(egui::FontId::monospace(12.0))
                                                            .color(field_color(theme, field.kind)),
                                                    );
                                                });
                                            });
                                    });
                                }
                            });
                    });
                }

                if !actions.is_empty() {
                    flex.add_ui(item(), |ui| separator(ui, theme, height, sep_margin_v));

                    let inner_cr = CornerRadius::same(theme.radius.md);
                    for (i, action) in actions.iter().enumerate() {
                        flex.add_ui(item(), |ui| {
                            let (rect, response) =
                                ui.allocate_exact_size(Vec2::splat(icon_btn_size), Sense::click());

                            if !action.disabled && response.clicked() {
                                result.icon_clicked = Some(i);
                            }

                            if action.selected || (response.hovered() && !action.disabled) {
                                let inset = rect.shrink(theme.control_gap);
                                let fill = if action.selected {
                                    theme.palette.control_fill_on
                                } else {
                                    theme.palette.control_fill_off
                                };
                                ui.painter().rect_filled(inset, inner_cr, fill);
                            }

                            let icon_color = if action.disabled {
                                theme.palette.muted_foreground.gamma_multiply(0.5)
                            } else if action.selected || response.hovered() {
                                theme.palette.foreground
                            } else {
                                theme.palette.muted_foreground
                            };
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                action.icon.to_string(),
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

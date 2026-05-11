//! Compact labeled action toolbar.
//!
//! Paints a small glass toolbar where each action is represented by a Lucide
//! icon and text label. Product semantics are supplied entirely by callers.

use std::hash::Hash;

use egui::{CornerRadius, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::icons::icon_font;
use crate::theme::Theme;

/// A labeled action rendered in the action toolbar.
#[derive(Clone, Copy, Debug)]
pub struct ActionToolbarItem<'a> {
    pub icon: char,
    pub label: &'a str,
    pub tooltip: &'a str,
    pub selected: bool,
    pub disabled: bool,
}

/// Response from the action toolbar.
pub struct ActionToolbarResponse {
    /// Index of the action button that was clicked this frame, if any.
    pub clicked: Option<usize>,
}

/// A compact labeled action toolbar.
///
/// Use [`action_toolbar_with_id`] when rendering multiple toolbars under the
/// same parent `Ui`.
pub fn action_toolbar(
    ui: &mut Ui,
    theme: &Theme,
    actions: &[ActionToolbarItem<'_>],
) -> ActionToolbarResponse {
    action_toolbar_with_id(ui, theme, "action_toolbar", actions)
}

/// A compact labeled action toolbar with a caller-provided ID salt.
pub fn action_toolbar_with_id(
    ui: &mut Ui,
    theme: &Theme,
    id_salt: impl Hash,
    actions: &[ActionToolbarItem<'_>],
) -> ActionToolbarResponse {
    let height = 36.0;
    let pad_h = theme.spacing.sm;
    let action_gap = theme.spacing.xs;
    let button_height = 28.0;
    let icon_size = 14.0;
    let icon_gap = theme.spacing.xs;
    let label_pad_h = theme.spacing.sm + 4.0;
    let label_font = egui::FontId::proportional(12.0);

    let label_galleys: Vec<_> = actions
        .iter()
        .map(|action| {
            ui.painter().layout_no_wrap(
                action.label.to_owned(),
                label_font.clone(),
                theme.palette.foreground,
            )
        })
        .collect();
    let button_widths: Vec<f32> = label_galleys
        .iter()
        .map(|galley| galley.size().x + icon_size + icon_gap + label_pad_h * 2.0)
        .collect();
    let total_w = pad_h * 2.0
        + button_widths.iter().sum::<f32>()
        + action_gap * actions.len().saturating_sub(1) as f32;

    let (outer_rect, _) = ui.allocate_exact_size(Vec2::new(total_w, height), Sense::hover());
    let mut result = ActionToolbarResponse { clicked: None };

    if ui.is_rect_visible(outer_rect) {
        let outer_cr = CornerRadius::same(theme.radius.lg);

        ui.painter()
            .rect_filled(outer_rect, outer_cr, theme.palette.surface_blur);
        ui.painter().rect_stroke(
            outer_rect,
            outer_cr,
            Stroke::new(1.0, theme.palette.border),
            StrokeKind::Inside,
        );

        let inner_rect = outer_rect.shrink2(Vec2::new(pad_h, 0.0));
        let mut inner_ui =
            ui.new_child(egui::UiBuilder::new().id_salt(id_salt).max_rect(inner_rect));

        inner_ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = action_gap;
            let inner_cr = CornerRadius::same(theme.radius.md);

            for (index, action) in actions.iter().enumerate() {
                let (rect, response) = ui.allocate_exact_size(
                    Vec2::new(button_widths[index], button_height),
                    Sense::click(),
                );
                let response = response.on_hover_text(action.tooltip);
                let disabled = action.disabled || !ui.is_enabled();

                if !disabled && response.clicked() {
                    result.clicked = Some(index);
                }

                if action.selected || (response.hovered() && !disabled) {
                    let inset = rect.shrink(theme.control_gap);
                    let fill = if action.selected {
                        theme.palette.control_fill_on
                    } else {
                        theme.palette.control_fill_off
                    };
                    ui.painter().rect_filled(inset, inner_cr, fill);
                }

                let text_color = if disabled {
                    theme.palette.muted_foreground.gamma_multiply(0.5)
                } else if action.selected || response.hovered() {
                    theme.palette.foreground
                } else {
                    theme.palette.muted_foreground
                };

                let label_galley = &label_galleys[index];
                let group_width = icon_size + icon_gap + label_galley.size().x;
                let icon_pos = egui::pos2(
                    rect.center().x - group_width / 2.0 + icon_size / 2.0,
                    rect.center().y,
                );
                ui.painter().text(
                    icon_pos,
                    egui::Align2::CENTER_CENTER,
                    action.icon.to_string(),
                    icon_font(icon_size),
                    text_color,
                );

                let text_pos = egui::pos2(
                    rect.center().x - group_width / 2.0 + icon_size + icon_gap,
                    rect.center().y - label_galley.size().y / 2.0,
                );
                ui.painter()
                    .galley(text_pos, label_galley.clone(), text_color);
            }
        });
    }

    result
}

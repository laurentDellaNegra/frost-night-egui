//! Vertical icon toolbar matching the Figma left sidebar.
//!
//! A dark vertical strip with icon buttons, separators between groups,
//! and an active-item highlight using the shared control fill pattern.

use egui::{Color32, CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};
use egui_flex::{Flex, item};

use crate::icons::icon_font;
use crate::theme::Theme;

/// A single toolbar item.
pub struct ToolbarItem {
    /// Lucide icon codepoint (e.g. `ICON_MAP`).
    pub icon: char,
    /// Optional small notification dot color.
    pub badge: Option<Color32>,
}

impl ToolbarItem {
    pub fn new(icon: char) -> Self {
        Self { icon, badge: None }
    }

    pub fn with_badge(mut self, color: Color32) -> Self {
        self.badge = Some(color);
        self
    }
}

/// A group of toolbar items separated by dividers.
pub type ToolbarGroup = Vec<ToolbarItem>;

/// Response from the toolbar.
pub struct ToolbarResponse {
    /// The button index that was clicked this frame, if any.
    pub clicked: Option<usize>,
    /// The outer rect of the toolbar (for positioning attached panels).
    pub rect: Rect,
    /// The Y position of each button center (flat index), for aligning cards.
    pub button_centers_y: Vec<f32>,
}

/// A vertical icon toolbar.
///
/// Items are organized in groups separated by thin horizontal dividers.
/// The active item gets a filled background using `control_fill_on`.
/// `selected` is `Option<usize>` — `None` means no button is active.
/// `floating` lists button indices that have detached floating cards —
/// these get a `ring`-colored icon to indicate the card is open somewhere.
pub fn toolbar(
    ui: &mut Ui,
    theme: &Theme,
    groups: &[ToolbarGroup],
    selected: Option<usize>,
    floating: &[usize],
) -> ToolbarResponse {
    let icon_size = 18.0;
    let button_size = 36.0;
    let padding = theme.spacing.xs;
    let separator_margin = theme.spacing.xs;

    // Pre-compute size so we can paint the backdrop before layout
    let total_items: usize = groups.iter().map(|g| g.len()).sum();
    let total_separators = groups.len().saturating_sub(1);
    let total_height = padding * 2.0
        + total_items as f32 * button_size
        + total_separators as f32 * (separator_margin * 2.0 + 1.0);
    let total_width = button_size + padding * 2.0;

    let (outer_rect, _response) =
        ui.allocate_exact_size(Vec2::new(total_width, total_height), Sense::hover());

    let mut result = ToolbarResponse {
        clicked: None,
        rect: outer_rect,
        button_centers_y: Vec::with_capacity(total_items),
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

        let inner_rect = outer_rect.shrink(padding);
        let mut inner_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("toolbar_inner")
                .max_rect(inner_rect),
        );

        let sep_inset = theme.spacing.xs;
        let inner_cr = CornerRadius::same(theme.radius.md);

        Flex::vertical().gap(Vec2::ZERO).show(&mut inner_ui, |flex| {
            let mut flat_idx = 0;

            for (gi, group) in groups.iter().enumerate() {
                // Separator between groups
                if gi > 0 {
                    flex.add_ui(item(), |ui| {
                        let sep_w = button_size - sep_inset * 2.0;
                        let h = 1.0 + separator_margin * 2.0;
                        let (rect, _) = ui.allocate_exact_size(Vec2::new(sep_w, h), Sense::hover());
                        ui.painter().line_segment(
                            [rect.left_center(), rect.right_center()],
                            Stroke::new(1.0, theme.palette.border),
                        );
                    });
                }

                for toolbar_item in group {
                    let idx = flat_idx;
                    let is_active = selected == Some(idx);
                    let is_floating = floating.contains(&idx);
                    let icon = toolbar_item.icon;
                    let badge = toolbar_item.badge;

                    flex.add_ui(item(), |ui| {
                        let (rect, response) =
                            ui.allocate_exact_size(Vec2::splat(button_size), Sense::click());

                        result.button_centers_y.push(rect.center().y);

                        if response.clicked() {
                            result.clicked = Some(idx);
                        }

                        // Active / hover background
                        if is_active {
                            let inset = rect.shrink(theme.control_gap);
                            ui.painter()
                                .rect_filled(inset, inner_cr, theme.palette.control_fill_on);
                        } else if response.hovered() {
                            let inset = rect.shrink(theme.control_gap);
                            ui.painter()
                                .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
                        }

                        // Icon color
                        let icon_color = if is_floating {
                            theme.palette.ring
                        } else if is_active || response.hovered() {
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

                        // Notification badge dot
                        if let Some(badge_color) = badge {
                            let dot_pos = egui::pos2(
                                rect.right() - theme.spacing.sm,
                                rect.top() + theme.spacing.sm,
                            );
                            ui.painter().circle_filled(dot_pos, 3.0, badge_color);
                        }
                    });

                    flat_idx += 1;
                }
            }
        });
    }

    result
}

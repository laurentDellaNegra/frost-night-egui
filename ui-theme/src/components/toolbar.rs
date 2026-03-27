//! Vertical icon toolbar matching the Figma left sidebar.
//!
//! A dark vertical strip with icon buttons, separators between groups,
//! and an active-item highlight using the shared control fill pattern.

use egui::{Color32, CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

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
pub fn toolbar(
    ui: &mut Ui,
    theme: &Theme,
    groups: &[ToolbarGroup],
    selected: Option<usize>,
) -> ToolbarResponse {
    let icon_size = 18.0;
    let button_size = 36.0;
    let padding = theme.spacing.xs;
    let separator_margin = theme.spacing.xs;

    // Count total items for flat indexing
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

        // Toolbar background (semi-transparent like card)
        ui.painter()
            .rect_filled(outer_rect, cr, theme.palette.surface_blur);
        ui.painter().rect_stroke(
            outer_rect,
            cr,
            Stroke::new(1.0, theme.palette.border),
            StrokeKind::Inside,
        );

        let mut y = outer_rect.top() + padding;
        let center_x = outer_rect.center().x;
        let mut flat_idx = 0;

        for (gi, group) in groups.iter().enumerate() {
            // Separator before group (except first)
            if gi > 0 {
                y += separator_margin;
                let sep_x0 = outer_rect.left() + padding + theme.spacing.xs;
                let sep_x1 = outer_rect.right() - padding - theme.spacing.xs;
                ui.painter().line_segment(
                    [egui::pos2(sep_x0, y), egui::pos2(sep_x1, y)],
                    Stroke::new(1.0, theme.palette.border),
                );
                y += 1.0 + separator_margin;
            }

            for item in group {
                let btn_rect = Rect::from_min_size(
                    egui::pos2(center_x - button_size / 2.0, y),
                    Vec2::splat(button_size),
                );

                result.button_centers_y.push(btn_rect.center().y);

                let is_active = selected == Some(flat_idx);

                // Interaction
                let btn_id = ui.id().with(("toolbar_btn", flat_idx));
                let btn_response = ui.interact(btn_rect, btn_id, Sense::click());

                if btn_response.clicked() {
                    result.clicked = Some(flat_idx);
                }

                let hovered = btn_response.hovered();

                // Active / hover background
                let inner_cr = CornerRadius::same(theme.radius.md);
                if is_active {
                    let inset = btn_rect.shrink(theme.control_gap);
                    ui.painter()
                        .rect_filled(inset, inner_cr, theme.palette.control_fill_on);
                } else if hovered {
                    let inset = btn_rect.shrink(theme.control_gap);
                    ui.painter()
                        .rect_filled(inset, inner_cr, theme.palette.control_fill_off);
                }

                // Icon
                let icon_color = if is_active || hovered {
                    theme.palette.foreground
                } else {
                    theme.palette.muted_foreground
                };

                ui.painter().text(
                    btn_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    item.icon.to_string(),
                    icon_font(icon_size),
                    icon_color,
                );

                // Notification badge dot
                if let Some(badge_color) = item.badge {
                    let dot_pos = egui::pos2(
                        btn_rect.right() - theme.spacing.sm,
                        btn_rect.top() + theme.spacing.sm,
                    );
                    ui.painter().circle_filled(dot_pos, 3.0, badge_color);
                }

                y += button_size;
                flat_idx += 1;
            }
        }
    }

    result
}

//! Sidebar card that can be docked to the toolbar or dragged freely.
//!
//! Features the same drag handle (3 dots → grab bar) as `drag_card`.
//! When docked, slides out from the toolbar. When dragged, detaches
//! and becomes a free-floating card with border glow.

use egui::{Color32, CornerRadius, Id, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::theme::Theme;
use crate::tokens::mix;

/// Response from a sidebar card.
pub struct SidebarCardResponse {
    /// The close button was clicked.
    pub closed: bool,
    /// The card is currently being dragged.
    pub dragging: bool,
    /// The drag delta this frame (for the caller to update floating position).
    pub drag_delta: Vec2,
}

/// A sidebar card with a drag handle, title bar, close button, and scrollable body.
///
/// The `open_t` parameter (0.0–1.0) drives the open/close animation.
/// The card renders at `rect` and includes a drag handle zone at the top.
/// When dragged, returns `dragging: true` and `drag_delta` so the caller
/// can switch to floating mode.
pub fn sidebar_card(
    ui: &mut Ui,
    theme: &Theme,
    id: Id,
    rect: Rect,
    open_t: f32,
    title: &str,
    highlight: bool,
    add_contents: impl FnOnce(&mut Ui),
) -> SidebarCardResponse {
    let cr = CornerRadius::same(theme.radius.lg);
    let pad = theme.spacing.lg;
    // Extra inset so title aligns with tab/accordion text inside the body
    let title_inset = theme.spacing.md;
    let dots_zone_h = theme.spacing.md;
    let title_h = theme.spacing.xl;
    let header_gap = theme.spacing.sm;
    let handle_h = dots_zone_h + header_gap + title_h;

    // Card position is fixed — animation only affects opacity, not position.
    // (Moving the rect between passes causes egui "widget rect changed id" warnings.)
    let card_rect = rect;

    // --- Drag interaction on the handle zone ---
    // Exclude the close button area on the right to avoid overlapping interactions.
    let close_zone_w = title_inset;
    let handle_rect = Rect::from_min_max(
        card_rect.min,
        egui::pos2(card_rect.right() - close_zone_w, card_rect.top() + handle_h),
    );
    let drag_response = ui.interact(handle_rect, id.with("drag"), Sense::drag());
    let dragging = drag_response.dragged();
    let drag_delta = drag_response.drag_delta();

    // --- Drag / highlight animation: border glow ---
    let drag_t = ui
        .ctx()
        .animate_bool_with_time(id.with("drag_anim"), dragging, 0.2);
    let highlight_t = ui
        .ctx()
        .animate_bool_with_time(id.with("highlight_anim"), highlight, 0.25);
    let glow_t = drag_t.max(highlight_t);

    // Opacity via alpha modulation
    let alpha = (open_t * 255.0) as u8;

    // --- Backdrop ---
    let backdrop = Color32::from_rgba_unmultiplied(
        theme.palette.surface_blur.r(),
        theme.palette.surface_blur.g(),
        theme.palette.surface_blur.b(),
        ((theme.palette.surface_blur.a() as f32 / 255.0) * open_t * 255.0) as u8,
    );
    ui.painter().rect_filled(card_rect, cr, backdrop);

    // Border: glow when dragging
    let base_border = Color32::from_rgba_unmultiplied(
        theme.palette.border.r(),
        theme.palette.border.g(),
        theme.palette.border.b(),
        alpha,
    );
    let border_color = if glow_t > 0.01 {
        mix(base_border, theme.palette.ring, glow_t)
    } else {
        base_border
    };
    let border_width = egui::lerp(1.0..=1.8, glow_t);
    ui.painter().rect_stroke(
        card_rect,
        cr,
        Stroke::new(border_width, border_color),
        StrokeKind::Outside,
    );

    // Outer glow halo when dragging or highlighted
    if glow_t > 0.01 {
        let glow_alpha = (glow_t * open_t * 60.0) as u8;
        let glow_color = Color32::from_rgba_unmultiplied(
            theme.palette.ring.r(),
            theme.palette.ring.g(),
            theme.palette.ring.b(),
            glow_alpha,
        );
        let glow_expand = egui::lerp(0.0..=4.0, glow_t);
        ui.painter().rect_stroke(
            card_rect.expand(glow_expand),
            cr,
            Stroke::new(2.0, glow_color),
            StrokeKind::Outside,
        );
    }

    // --- Handle indicator: 3 dots → grab bar ---
    let hovered = drag_response.hovered() || dragging;
    let hover_t = ui
        .ctx()
        .animate_bool_with_time(id.with("handle_hover"), hovered, 0.15);

    let dots_center_y = card_rect.top() + pad + dots_zone_h / 2.0;
    let dots_center_x = card_rect.center().x;
    let dot_radius = 2.0;
    let dot_spacing = 6.0;
    let bar_half_w = theme.spacing.lg;
    let bar_h = 3.0;
    let bar_cr = CornerRadius::same(2);

    let handle_base = mix(
        theme.palette.muted_foreground,
        theme.palette.foreground,
        hover_t * 0.5,
    );
    if hover_t < 0.99 {
        let dot_alpha = ((1.0 - hover_t) * open_t * 255.0) as u8;
        let dot_color = Color32::from_rgba_unmultiplied(
            handle_base.r(),
            handle_base.g(),
            handle_base.b(),
            dot_alpha,
        );
        for i in [-1.0, 0.0, 1.0] {
            let spread = egui::lerp(1.0..=2.0, hover_t);
            let x = dots_center_x + i * dot_spacing * spread;
            ui.painter()
                .circle_filled(egui::pos2(x, dots_center_y), dot_radius, dot_color);
        }
    }

    if hover_t > 0.01 {
        let bar_alpha = (hover_t * open_t * 255.0) as u8;
        let bar_color = Color32::from_rgba_unmultiplied(
            handle_base.r(),
            handle_base.g(),
            handle_base.b(),
            bar_alpha,
        );
        let w = egui::lerp(dot_spacing..=bar_half_w, hover_t);
        let bar_rect = Rect::from_center_size(
            egui::pos2(dots_center_x, dots_center_y),
            Vec2::new(w * 2.0, bar_h),
        );
        ui.painter().rect_filled(bar_rect, bar_cr, bar_color);
    }

    // --- Title row (below dots zone + gap) ---
    let title_center_y = card_rect.top() + pad + dots_zone_h + header_gap + title_h / 2.0;

    let title_color = Color32::from_rgba_unmultiplied(
        theme.palette.foreground.r(),
        theme.palette.foreground.g(),
        theme.palette.foreground.b(),
        alpha,
    );
    ui.painter().text(
        egui::pos2(card_rect.left() + pad + title_inset, title_center_y),
        egui::Align2::LEFT_CENTER,
        title,
        egui::FontId::proportional(13.0),
        title_color,
    );

    // Close button (X)
    let close_size = theme.spacing.lg;
    let close_center = egui::pos2(
        card_rect.right() - pad - title_inset - close_size / 2.0,
        title_center_y,
    );
    let close_rect =
        Rect::from_center_size(close_center, Vec2::splat(close_size + theme.spacing.sm));
    let close_response = ui.interact(close_rect, id.with("sidebar_close"), Sense::click());

    let x_base = if close_response.hovered() {
        theme.palette.foreground
    } else {
        theme.palette.muted_foreground
    };
    let x_color = Color32::from_rgba_unmultiplied(x_base.r(), x_base.g(), x_base.b(), alpha);
    let s = close_size / 2.0 - 2.0;
    ui.painter().line_segment(
        [
            close_center + egui::vec2(-s, -s),
            close_center + egui::vec2(s, s),
        ],
        Stroke::new(1.5, x_color),
    );
    ui.painter().line_segment(
        [
            close_center + egui::vec2(s, -s),
            close_center + egui::vec2(-s, s),
        ],
        Stroke::new(1.5, x_color),
    );

    // Cursor hint
    if drag_response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
    }
    if dragging {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
    }

    // --- Body content ---
    let body_top = card_rect.top() + pad + handle_h + theme.spacing.xs;
    let body_rect = Rect::from_min_max(
        egui::pos2(card_rect.left() + pad, body_top),
        egui::pos2(card_rect.right() - pad, card_rect.bottom() - pad),
    );

    if open_t > 0.3 {
        let mut body_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(id)
                .global_scope(true)
                .max_rect(body_rect),
        );
        body_ui.set_opacity(ui.opacity() * open_t);
        egui::ScrollArea::vertical().show(&mut body_ui, |ui| {
            add_contents(ui);
        });
    }

    SidebarCardResponse {
        closed: close_response.clicked(),
        dragging,
        drag_delta,
    }
}

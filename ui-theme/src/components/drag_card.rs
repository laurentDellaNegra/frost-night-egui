//! Draggable card with a title bar handle.
//!
//! Paints its own semi-transparent backdrop (`surface_blur`) and handles
//! drag-to-fade opacity animation internally.

use egui::{Color32, CornerRadius, Id, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::theme::Theme;

/// Persistent state for a draggable card.
#[derive(Clone, Debug)]
pub struct DragCardState {
    pub pos: Pos2,
    pub size: Vec2,
}

/// Response from a drag card.
pub struct DragCardResponse {
    /// The close button was clicked.
    pub closed: bool,
    /// The card is currently being dragged.
    pub dragging: bool,
}

/// A draggable floating card with built-in backdrop and drag-to-fade animation.
///
/// The card paints a semi-transparent `surface_blur` backdrop, a border,
/// a title bar with drag handle and close button, and a scrollable body.
/// When dragged, the entire card fades to 15% opacity.
pub fn drag_card(
    ui: &mut Ui,
    theme: &Theme,
    id: Id,
    state: &mut DragCardState,
    title: &str,
    add_contents: impl FnOnce(&mut Ui),
) -> DragCardResponse {
    let cr = CornerRadius::same(theme.radius.lg);
    let handle_h = 32.0;
    let padding = 12.0;

    // --- Drag interaction (detect first so we know opacity) ---
    let handle_rect = Rect::from_min_size(state.pos, Vec2::new(state.size.x, handle_h));
    let drag_response = ui.interact(handle_rect, id.with("drag"), Sense::drag());
    if drag_response.dragged() {
        state.pos += drag_response.drag_delta();
    }
    // Recompute after potential drag delta
    let card_rect = Rect::from_min_size(state.pos, state.size);
    let handle_rect = Rect::from_min_size(state.pos, Vec2::new(state.size.x, handle_h));

    // --- Opacity animation: fade to 15% when dragging ---
    let target_opacity = if drag_response.dragged() { 0.15 } else { 1.0 };
    let opacity = ui.ctx().animate_value_with_time(
        id.with("card_opacity"),
        target_opacity,
        0.15,
    );

    // --- Semi-transparent backdrop ---
    let [r, g, b, a] = theme.palette.surface_blur.to_array();
    let drag_alpha = (a as f32 * opacity) as u8;
    ui.painter().rect_filled(
        card_rect,
        cr,
        Color32::from_rgba_unmultiplied(r, g, b, drag_alpha),
    );

    // Apply opacity to all subsequent painting
    ui.set_opacity(opacity);

    // Border
    ui.painter().rect_stroke(
        card_rect,
        cr,
        Stroke::new(1.0, theme.palette.border),
        StrokeKind::Outside,
    );

    // --- Title bar ---
    // Title text
    ui.painter().text(
        egui::pos2(handle_rect.left() + padding, handle_rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        egui::FontId::proportional(13.0),
        theme.palette.foreground,
    );

    // Close button (X)
    let close_size = 16.0;
    let close_center = egui::pos2(
        handle_rect.right() - padding - close_size / 2.0,
        handle_rect.center().y,
    );
    let close_rect = Rect::from_center_size(close_center, Vec2::splat(close_size + 8.0));
    let close_response = ui.interact(close_rect, id.with("close"), Sense::click());

    let x_color = if close_response.hovered() {
        theme.palette.foreground
    } else {
        theme.palette.muted_foreground
    };
    let s = close_size / 2.0 - 2.0;
    ui.painter().line_segment(
        [close_center + egui::vec2(-s, -s), close_center + egui::vec2(s, s)],
        Stroke::new(1.5, x_color),
    );
    ui.painter().line_segment(
        [close_center + egui::vec2(s, -s), close_center + egui::vec2(-s, s)],
        Stroke::new(1.5, x_color),
    );

    // Cursor hint
    if drag_response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
    }
    if drag_response.dragged() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
    }

    // --- Body content ---
    let body_rect = Rect::from_min_max(
        egui::pos2(card_rect.left() + padding, handle_rect.bottom() + 8.0),
        egui::pos2(card_rect.right() - padding, card_rect.bottom() - padding),
    );

    let mut body_ui = ui.new_child(egui::UiBuilder::new().max_rect(body_rect));
    egui::ScrollArea::vertical().show(&mut body_ui, add_contents);

    // Restore full opacity for anything drawn after the card
    ui.set_opacity(1.0);

    DragCardResponse {
        closed: close_response.clicked(),
        dragging: drag_response.dragged(),
    }
}

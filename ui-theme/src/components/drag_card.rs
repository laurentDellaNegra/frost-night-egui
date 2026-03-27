//! Draggable card with a title bar handle.
//!
//! The caller is responsible for painting the backdrop (e.g. glassmorphism
//! blur) before calling [`drag_card`]. This component only draws the border,
//! title bar, close button, and scrollable body.

use egui::{CornerRadius, Id, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::theme::Theme;

/// Persistent state for a draggable card.
#[derive(Clone, Debug)]
pub struct DragCardState {
    pub pos: Pos2,
    pub size: Vec2,
}

/// A draggable floating card.
///
/// The top bar acts as a drag handle showing the title and a close button.
/// The body is scrollable.
///
/// **Backdrop**: not drawn by this component — paint your own blur/fill
/// before calling this function so it appears behind the card content.
///
/// Response from a drag card.
pub struct DragCardResponse {
    /// The close button was clicked.
    pub closed: bool,
    /// The card is currently being dragged.
    pub dragging: bool,
}

/// Returns close and dragging state.
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

    let card_rect = Rect::from_min_size(state.pos, state.size);

    // Border
    ui.painter().rect_stroke(
        card_rect,
        cr,
        Stroke::new(1.0, theme.palette.border),
        StrokeKind::Outside,
    );

    // --- Title bar / drag handle ---
    let handle_rect = Rect::from_min_size(state.pos, Vec2::new(state.size.x, handle_h));

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

    // Drag interaction on the handle
    let drag_response = ui.interact(handle_rect, id.with("drag"), Sense::drag());
    if drag_response.dragged() {
        state.pos += drag_response.drag_delta();
    }

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

    DragCardResponse {
        closed: close_response.clicked(),
        dragging: drag_response.dragged(),
    }
}

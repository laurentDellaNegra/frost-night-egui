//! egui integration helpers.

use egui::style::WidgetVisuals;
use egui::CornerRadius;

use crate::theme::Theme;
use crate::tokens::{filled_tokens, with_alpha, StateColors, VariantTokens};

/// Convert a [`StateColors`] into an egui [`WidgetVisuals`].
fn to_widget_visuals(state: &StateColors, corner_radius: CornerRadius, expansion: f32) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: state.bg_fill,
        weak_bg_fill: state.bg_fill,
        bg_stroke: state.border,
        corner_radius,
        fg_stroke: state.fg_stroke,
        expansion,
    }
}

/// Convert [`VariantTokens`] to egui's native [`Widgets`](egui::style::Widgets) struct.
pub fn to_egui_widgets(
    tokens: &VariantTokens,
    corner_radius: CornerRadius,
    expansion: f32,
) -> egui::style::Widgets {
    egui::style::Widgets {
        noninteractive: to_widget_visuals(&tokens.idle, corner_radius, 0.0),
        inactive: to_widget_visuals(&tokens.idle, corner_radius, 0.0),
        hovered: to_widget_visuals(&tokens.hovered, corner_radius, expansion),
        active: to_widget_visuals(&tokens.active, corner_radius, expansion),
        open: to_widget_visuals(&tokens.active, corner_radius, 0.0),
    }
}

/// Apply the theme globally to an egui context.
///
/// Sets dark mode as the base, then overrides colors from the palette.
/// Call once at app startup, or whenever the theme changes.
pub fn apply_theme(ctx: &egui::Context, theme: &Theme) {
    let p = &theme.palette;
    let mut visuals = egui::Visuals::dark();

    // Base surface colors
    visuals.window_fill = p.card;
    visuals.panel_fill = p.card;
    visuals.extreme_bg_color = p.background;
    visuals.faint_bg_color = p.muted;
    visuals.window_stroke = egui::Stroke::new(1.0, p.border);

    // Selection
    visuals.selection.bg_fill = with_alpha(p.primary, 80);
    visuals.selection.stroke = egui::Stroke::new(1.0, p.primary);

    // Hyperlinks
    visuals.hyperlink_color = p.accent;

    // Widget visuals — neutral filled defaults (no special border)
    let default_tokens = filled_tokens(p.secondary, p.secondary_foreground, p);
    let corner_radius = CornerRadius::same(theme.radius.md);
    visuals.widgets = to_egui_widgets(&default_tokens, corner_radius, 1.0);

    ctx.set_visuals(visuals);
}

//! Lucide icon font integration.
//!
//! Embeds the Lucide icon font (TTF) and provides named constants for
//! icon codepoints. Call [`load_icon_font`] once at startup to register
//! the font with egui, then use the constants with [`icon_text`] to
//! render icons.

use egui::{Context, FontData, FontDefinitions, FontFamily, FontId, RichText};

/// The Lucide icon font family name.
pub const ICON_FONT_FAMILY: &str = "lucide";

/// Embedded Lucide TTF font data.
const LUCIDE_TTF: &[u8] = include_bytes!("fonts/lucide.ttf");

/// Register the Lucide icon font with an egui context.
///
/// Call this once during app initialization (e.g. in `CreationContext`).
pub fn load_icon_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        ICON_FONT_FAMILY.to_owned(),
        FontData::from_static(LUCIDE_TTF).into(),
    );
    fonts
        .families
        .entry(FontFamily::Name(ICON_FONT_FAMILY.into()))
        .or_default()
        .push(ICON_FONT_FAMILY.to_owned());
    ctx.set_fonts(fonts);
}

/// Create a [`FontId`] for the icon font at a given size.
pub fn icon_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(ICON_FONT_FAMILY.into()))
}

/// Create a [`RichText`] rendering an icon at the given size.
pub fn icon_text(icon: char, size: f32) -> RichText {
    RichText::new(icon.to_string()).font(icon_font(size))
}

// ---------------------------------------------------------------------------
// Named icon codepoints (Lucide font 1.7.0)
// ---------------------------------------------------------------------------

pub const ICON_MAP: char = '\u{E110}';
pub const ICON_MAP_PIN: char = '\u{E111}';
pub const ICON_LAYERS: char = '\u{E529}';
pub const ICON_LAYERS_2: char = '\u{E52A}';
pub const ICON_PLUS: char = '\u{E13D}';
pub const ICON_MINUS: char = '\u{E11C}';
pub const ICON_SEARCH: char = '\u{E151}';
pub const ICON_SETTINGS: char = '\u{E154}';
pub const ICON_COMPASS: char = '\u{E09B}';
pub const ICON_NAVIGATION: char = '\u{E123}';
pub const ICON_NAVIGATION_2: char = '\u{E124}';
pub const ICON_RULER: char = '\u{E14B}';
pub const ICON_FILTER: char = '\u{E0DC}';
pub const ICON_EYE: char = '\u{E0BA}';
pub const ICON_EYE_OFF: char = '\u{E0BB}';
pub const ICON_BOOK_OPEN: char = '\u{E05F}';
pub const ICON_PANEL_LEFT: char = '\u{E12A}';
pub const ICON_GRID: char = '\u{E4FF}';
pub const ICON_GLOBE: char = '\u{E0E8}';
pub const ICON_RADAR: char = '\u{E497}';
pub const ICON_CROSSHAIR: char = '\u{E0AC}';
pub const ICON_PLANE: char = '\u{E1DE}';
pub const ICON_LOCATE: char = '\u{E1DA}';
pub const ICON_ZOOM_IN: char = '\u{E1B6}';
pub const ICON_ZOOM_OUT: char = '\u{E1B7}';
pub const ICON_CIRCLE_X: char = '\u{E084}';
pub const ICON_SNOWFLAKE: char = '\u{E165}';

//! Frost Night UI theme — a minimal egui design system.
//!
//! Dark theme extracted from Figma mockups.
//! Architecture inspired by shadcn patterns, written from scratch.

pub mod blur;
pub mod components;
pub mod helpers;
pub mod oklch;
pub mod palette;
pub mod scale;
pub mod theme;
pub mod tokens;

// Convenience re-exports
pub use blur::BlurRect;
pub use helpers::apply_theme;
pub use palette::ColorPalette;
pub use scale::{ControlSize, RadiusScale};
pub use theme::Theme;
pub use tokens::{mix, ControlVariant, StateColors, VariantTokens};

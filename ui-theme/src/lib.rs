//! Frost Night UI theme — a reusable egui design system.
//!
//! Dark theme extracted from Figma mockups.
//! Architecture inspired by shadcn patterns, written from scratch.

pub mod components;
pub mod containers;
pub mod effects;
#[cfg(feature = "icons")]
pub mod icons;
pub mod theme;

#[cfg(feature = "composites")]
pub mod composites;

#[cfg(feature = "demo")]
pub mod demo;

pub use components::FrostUiExt;
pub use effects::BlurRect;
#[cfg(feature = "icons")]
pub use icons::{add_icon_font_to, install_icon_font};
#[allow(deprecated)]
pub use theme::apply_theme;
pub use theme::{
    apply_visuals, install_theme, ColorPalette, ControlSize, ControlVariant, InstallThemeOptions,
    RadiusScale, SpacingScale, StateColors, Theme, VariantTokens,
};

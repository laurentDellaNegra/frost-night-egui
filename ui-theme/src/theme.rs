//! Main theme struct and control/input visual resolvers.

use egui::CornerRadius;

use crate::palette::ColorPalette;
use crate::scale::{ControlSize, RadiusScale};
use crate::tokens::{variant_tokens, ControlVariant, VariantTokens};

/// Resolved visual properties for rendering a control.
#[derive(Clone, Debug)]
pub struct ControlVisuals {
    pub tokens: VariantTokens,
    pub corner_radius: CornerRadius,
    pub font: egui::FontId,
    pub padding: egui::Vec2,
}

/// Resolved visual properties for rendering a text input.
#[derive(Clone, Debug)]
pub struct InputVisuals {
    pub bg: egui::Color32,
    pub border: egui::Stroke,
    pub corner_radius: CornerRadius,
    pub font: egui::FontId,
    pub text_color: egui::Color32,
    pub placeholder_color: egui::Color32,
}

/// The main theme, combining palette and scales.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Theme {
    pub palette: ColorPalette,
    pub radius: RadiusScale,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Dark theme.
    pub fn dark() -> Self {
        Self {
            palette: ColorPalette::dark(),
            radius: RadiusScale::default(),
        }
    }

    /// Resolve widget visuals for a variant + size combination.
    pub fn control(&self, variant: ControlVariant, size: ControlSize) -> ControlVisuals {
        ControlVisuals {
            tokens: variant_tokens(&self.palette, variant),
            corner_radius: size.corner_radius(&self.radius),
            font: size.font(),
            padding: size.padding(),
        }
    }

    /// Resolve input visuals for a given size.
    pub fn input(&self, size: ControlSize) -> InputVisuals {
        InputVisuals {
            bg: self.palette.muted,
            border: egui::Stroke::new(1.0, self.palette.input),
            corner_radius: size.corner_radius(&self.radius),
            font: size.font(),
            text_color: self.palette.foreground,
            placeholder_color: self.palette.muted_foreground,
        }
    }

    /// Create a [`BlurRect`](crate::blur::BlurRect) for a glassmorphism surface.
    pub fn surface_blur(&self, rect: egui::Rect, corner_radius: CornerRadius) -> crate::blur::BlurRect {
        crate::blur::BlurRect {
            rect,
            radius: self.palette.surface_blur_radius,
            tint: self.palette.surface_blur,
            corner_radius,
        }
    }
}

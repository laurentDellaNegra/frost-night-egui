//! Radius, spacing, and typography scales.

use egui::{CornerRadius, FontId, Vec2};

/// Corner radius scale derived from Figma measurements.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadiusScale {
    /// Small radius — badges, tags (4px).
    pub sm: u8,
    /// Medium radius — buttons, inputs (6px).
    pub md: u8,
    /// Large radius — cards, panels, dialogs (8px).
    pub lg: u8,
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            sm: 4,
            md: 6,
            lg: 8,
        }
    }
}

/// Control size variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ControlSize {
    Sm,
    Md,
    Lg,
}

impl ControlSize {
    /// Button/control inner padding for this size.
    pub fn padding(self) -> Vec2 {
        match self {
            Self::Sm => Vec2::new(8.0, 4.0),
            Self::Md => Vec2::new(12.0, 6.0),
            Self::Lg => Vec2::new(16.0, 8.0),
        }
    }

    /// Corner radius for this size.
    pub fn corner_radius(self, scale: &RadiusScale) -> CornerRadius {
        match self {
            Self::Sm => CornerRadius::same(scale.sm),
            Self::Md => CornerRadius::same(scale.md),
            Self::Lg => CornerRadius::same(scale.lg),
        }
    }

    /// Font for this control size.
    pub fn font(self) -> FontId {
        match self {
            Self::Sm => FontId::proportional(11.0),
            Self::Md => FontId::proportional(13.0),
            Self::Lg => FontId::proportional(15.0),
        }
    }
}

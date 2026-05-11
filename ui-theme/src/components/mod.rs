//! Thin component wrappers that use the theme for styling.

mod badge;
mod button;
mod checkbox;
mod input;
mod segmented;
mod separator;
mod toggle;

pub use badge::{badge, BadgeVariant};
pub use button::button;
pub use checkbox::{checkbox, checkbox_small};
pub use input::{text_edit, text_edit_enabled, text_input};
pub use segmented::{segmented, segmented_styled, segmented_with_fills, SegmentStyle};
pub use separator::separator;
pub use toggle::toggle;

use crate::theme::{ControlSize, ControlVariant, Theme};

/// Ergonomic extension methods for adding Frost Night controls to an `egui::Ui`.
pub trait FrostUiExt {
    fn frost_button(
        &mut self,
        theme: &Theme,
        label: impl Into<egui::WidgetText>,
        variant: ControlVariant,
        size: ControlSize,
    ) -> egui::Response;

    fn frost_text_input<S: egui::TextBuffer>(
        &mut self,
        theme: &Theme,
        text: &mut S,
        size: ControlSize,
    ) -> egui::Response;

    fn frost_text_edit(
        &mut self,
        theme: &Theme,
        editor: egui::TextEdit<'_>,
        size: ControlSize,
    ) -> egui::Response;

    fn frost_text_edit_enabled(
        &mut self,
        theme: &Theme,
        enabled: bool,
        editor: egui::TextEdit<'_>,
        size: ControlSize,
    ) -> egui::Response;
}

impl FrostUiExt for egui::Ui {
    fn frost_button(
        &mut self,
        theme: &Theme,
        label: impl Into<egui::WidgetText>,
        variant: ControlVariant,
        size: ControlSize,
    ) -> egui::Response {
        button(self, theme, label, variant, size)
    }

    fn frost_text_input<S: egui::TextBuffer>(
        &mut self,
        theme: &Theme,
        text: &mut S,
        size: ControlSize,
    ) -> egui::Response {
        text_input(self, theme, text, size)
    }

    fn frost_text_edit(
        &mut self,
        theme: &Theme,
        editor: egui::TextEdit<'_>,
        size: ControlSize,
    ) -> egui::Response {
        text_edit(self, theme, editor, size)
    }

    fn frost_text_edit_enabled(
        &mut self,
        theme: &Theme,
        enabled: bool,
        editor: egui::TextEdit<'_>,
        size: ControlSize,
    ) -> egui::Response {
        text_edit_enabled(self, theme, enabled, editor, size)
    }
}

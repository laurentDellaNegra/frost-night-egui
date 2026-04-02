//! Maps menu widget — composition of tabs, search input, checkbox grid, and accordions.

use egui::{Ui, Vec2};

use crate::components::{accordion, checkbox_small, tabs};
use crate::icons::{icon_text, ICON_CIRCLE_X, ICON_SEARCH};
use crate::theme::Theme;

/// Persistent state for the maps menu widget.
pub struct MapsMenuState {
    pub tab: usize,
    pub search: String,
    pub favorites: Vec<(String, bool)>,
    pub accordion_open: Vec<bool>,
    pub accordion_nested: Vec<Vec<bool>>,
}

impl Default for MapsMenuState {
    fn default() -> Self {
        Self {
            tab: 0,
            search: String::new(),
            favorites: vec![
                ("MVA LSGG Cold".into(), true),
                ("MVA LSGG > 7C".into(), false),
                ("22 ILS".into(), true),
                ("22 SRA EMG".into(), false),
                ("04 ILS".into(), false),
                ("04 SRA EMG".into(), false),
                ("CTC CTR TERR CHART".into(), false),
                ("LFN EMG".into(), false),
            ],
            accordion_open: vec![false, false, false, false],
            accordion_nested: vec![
                vec![false, false, false, false, false],
                vec![false, false, false],
                vec![false, false],
                vec![false, false],
            ],
        }
    }
}

/// Renders the maps menu widget.
pub fn maps_menu(ui: &mut Ui, theme: &Theme, state: &mut MapsMenuState) {
    // Tabs
    let tab_labels = ["★ Favorites", "All Maps"];
    tabs(ui, theme, &tab_labels[..], &mut state.tab);
    ui.add_space(theme.spacing.lg);

    // Search bar with icon
    search_input(ui, theme, &mut state.search);
    ui.add_space(theme.spacing.lg);

    match state.tab {
        0 => favorites_tab(ui, theme, state),
        1 => all_maps_tab(ui, theme),
        _ => {}
    }
}

/// Search input with a search icon prefix and clear button suffix.
fn search_input(ui: &mut Ui, theme: &Theme, text: &mut String) {
    let row_height = 28.0;
    let icon_size = 16.0;
    let clear_btn_w = icon_size + theme.spacing.sm;

    ui.horizontal(|ui| {
        ui.set_height(row_height);
        ui.spacing_mut().item_spacing.x = theme.spacing.sm;

        // Search icon — vertically centered
        ui.add_sized(
            Vec2::new(icon_size, row_height),
            egui::Label::new(
                icon_text(ICON_SEARCH, icon_size)
                    .color(theme.palette.muted_foreground),
            ),
        );

        // Text input — takes remaining width, taller
        let remaining = ui.available_width()
            - if text.is_empty() { 0.0 } else { clear_btn_w + theme.spacing.sm };
        ui.add_sized(
            Vec2::new(remaining, row_height),
            egui::TextEdit::singleline(text)
                .hint_text(
                    egui::RichText::new("Search...")
                        .color(theme.palette.muted_foreground)
                        .size(12.0),
                )
                .font(egui::FontId::proportional(12.0))
                .text_color(theme.palette.foreground)
                .margin(egui::Margin::symmetric(
                    theme.spacing.xs as i8,
                    theme.spacing.sm as i8,
                )),
        );

        // Clear button — vertically centered
        if !text.is_empty() {
            let clear = ui.add_sized(
                Vec2::new(clear_btn_w, row_height),
                egui::Button::new(
                    icon_text(ICON_CIRCLE_X, icon_size)
                        .color(theme.palette.muted_foreground),
                )
                .frame(false),
            );
            if clear.clicked() {
                text.clear();
            }
        }
    });
}

fn favorites_tab(ui: &mut Ui, theme: &Theme, state: &mut MapsMenuState) {
    // Favorites checkbox grid (3 columns, fixed width per column for alignment)
    let col_count = 3;
    let available = ui.available_width();
    let col_width = (available / col_count as f32).floor();

    egui::Grid::new("favorites_grid")
        .num_columns(col_count)
        .min_col_width(col_width)
        .max_col_width(col_width)
        .spacing(egui::vec2(theme.spacing.sm, theme.spacing.xs))
        .show(ui, |ui| {
            for (idx, (label, checked)) in state.favorites.iter_mut().enumerate() {
                checkbox_small(ui, theme, checked, label);
                if (idx + 1) % col_count == 0 {
                    ui.end_row();
                }
            }
        });

    ui.add_space(theme.spacing.xl);

    // "Favorites" title
    ui.label(
        egui::RichText::new("Favorites")
            .size(13.0)
            .color(theme.palette.foreground),
    );
    ui.add_space(theme.spacing.md);

    // Accordions with nested content
    let sections = ["Basic (5)", "LSGG Geneva (3)", "LSZB Bern (2)", "Airspace (2)"];

    accordion(
        ui,
        theme,
        &sections,
        &mut state.accordion_open,
        false,
        |ui, i| {
            let nested = &mut state.accordion_nested[i];
            match i {
                0 => {
                    let items = ["MVA LSGG Cold", "22 ILS", "04 ILS", "CTC CTR TERR CHART", "LFN EMG"];
                    render_checkbox_list(ui, theme, nested, &items);
                }
                1 => {
                    let items = ["SID/STAR", "Approach Charts", "Ground Movement"];
                    render_checkbox_list(ui, theme, nested, &items);
                }
                2 => {
                    let items = ["SID/STAR", "Approach Charts"];
                    render_checkbox_list(ui, theme, nested, &items);
                }
                3 => {
                    let items = ["TMA Zurich", "FIR Switzerland"];
                    render_checkbox_list(ui, theme, nested, &items);
                }
                _ => {}
            }
        },
    );
}

fn all_maps_tab(ui: &mut Ui, theme: &Theme) {
    ui.label(
        egui::RichText::new("Browse all available maps")
            .size(13.0)
            .color(theme.palette.muted_foreground),
    );
}

fn render_checkbox_list(ui: &mut Ui, theme: &Theme, checked: &mut Vec<bool>, labels: &[&str]) {
    while checked.len() < labels.len() {
        checked.push(false);
    }
    for (j, &label) in labels.iter().enumerate() {
        checkbox_small(ui, theme, &mut checked[j], label);
        ui.add_space(theme.spacing.xs);
    }
}

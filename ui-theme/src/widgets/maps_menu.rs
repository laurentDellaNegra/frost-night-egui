//! Maps menu widget — composition of tabs, search input, checkbox grid, and accordions.

use egui::{Sense, Ui, Vec2};

use crate::components::{accordion, checkbox_small, tabs};
use crate::icons::{icon_text, ICON_CIRCLE_X, ICON_SEARCH};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

/// A single map entry.
pub struct MapEntry {
    pub name: String,
    pub favorite: bool,
    pub selected: bool,
}

/// A category that can contain maps and/or sub-categories.
pub struct MapCategory {
    pub name: String,
    pub maps: Vec<MapEntry>,
    pub children: Vec<MapCategory>,
    /// Open state per context (0 = favorites tab, 1 = all maps tab, 2 = search).
    pub open: [bool; 3],
}

impl MapCategory {
    fn new(name: &str, maps: &[&str]) -> Self {
        Self {
            name: name.into(),
            maps: maps.iter().map(|&m| MapEntry { name: m.into(), favorite: false, selected: false }).collect(),
            children: vec![],
            open: [false; 3],
        }
    }

    fn with_children(name: &str, children: Vec<MapCategory>) -> Self {
        Self { name: name.into(), maps: vec![], children, open: [false; 3] }
    }

    fn favorite_count(&self) -> usize {
        let own = self.maps.iter().filter(|m| m.favorite).count();
        let child: usize = self.children.iter().map(|c| c.favorite_count()).count();
        own + child
    }

    fn total_count(&self) -> usize {
        self.maps.len() + self.children.iter().map(|c| c.total_count()).sum::<usize>()
    }

    fn search_count(&self, query: &str) -> usize {
        let own = self.maps.iter().filter(|m| m.name.to_lowercase().contains(query)).count();
        let child: usize = self.children.iter().map(|c| c.search_count(query)).sum();
        own + child
    }

    fn has_favorites(&self) -> bool {
        self.maps.iter().any(|m| m.favorite) || self.children.iter().any(|c| c.has_favorites())
    }

    fn has_search_match(&self, query: &str) -> bool {
        self.maps.iter().any(|m| m.name.to_lowercase().contains(query))
            || self.children.iter().any(|c| c.has_search_match(query))
    }
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Fixed quick-access maps shown in the grid (always visible, independent of favorites).
const GRID_MAPS: &[&str] = &[
    "MVA LSGG Cold", "MVA LSGG > 7C", "22 ILS", "22 SRA EMG",
    "04 ILS", "04 SRA EMG", "CTC CTR TERR CHART", "LFN EMG",
    "TMA Zurich",
];

pub struct MapsMenuState {
    pub tab: usize,
    pub search: String,
    pub categories: Vec<MapCategory>,
}

impl Default for MapsMenuState {
    fn default() -> Self {
        let mut categories = vec![
            MapCategory::new("Basic", &["MVA LSGG Cold", "MVA LSGG > 7C", "22 ILS", "22 SRA EMG", "04 ILS", "04 SRA EMG", "CTC CTR TERR CHART", "LFN EMG"]),
            MapCategory::with_children("LSGG Geneva", vec![
                MapCategory::new("SID/STAR", &["SID RWY 22", "SID RWY 04", "STAR KONIL", "STAR MOLUS"]),
                MapCategory::new("Approach", &["ILS RWY 22", "ILS RWY 04", "VOR RWY 22"]),
                MapCategory::new("Ground", &["Taxi Chart", "Parking Stands"]),
            ]),
            MapCategory::with_children("LSZB Bern", vec![
                MapCategory::new("SID/STAR", &["SID RWY 14", "STAR KINES"]),
                MapCategory::new("Approach", &["RNAV RWY 14", "VOR RWY 32"]),
            ]),
            MapCategory::new("Airspace", &["TMA Zurich", "TMA Geneva", "CTR Zurich", "CTR Geneva", "FIR Switzerland"]),
        ];

        // Pre-favorite some maps (these also appear in the grid)
        categories[0].maps[0].favorite = true; // MVA LSGG Cold
        categories[0].maps[0].selected = true;
        categories[0].maps[2].favorite = true; // 22 ILS
        categories[0].maps[4].favorite = true; // 04 ILS
        categories[3].maps[0].favorite = true; // TMA Zurich

        Self {
            tab: 0,
            search: String::new(),
            categories,
        }
    }
}

// ---------------------------------------------------------------------------
// Main render
// ---------------------------------------------------------------------------

pub fn maps_menu(ui: &mut Ui, theme: &Theme, state: &mut MapsMenuState) {
    // Deferred tab switching: store (frame_nr, tab) so we only apply
    // the click on a NEW frame, not on the second pass of the same frame.
    let pending_key = ui.id().with("pending_tab");
    let pass_nr = ui.ctx().cumulative_pass_nr();
    let pending: Option<(u64, usize)> = ui.ctx().data_mut(|d| d.get_temp(pending_key));
    if let Some((stored_pass, t)) = pending {
        // Apply only when at least 2 passes have elapsed (= new frame).
        if pass_nr >= stored_pass + 2 {
            state.tab = t;
            ui.ctx().data_mut(|d| d.remove_temp::<(u64, usize)>(pending_key));
        }
    }

    let active_tab = state.tab;
    let tab_labels = ["★  Favorites", "All Maps"];
    let mut tab_local = active_tab;
    tabs(ui, theme, &mut tab_local, &tab_labels[..]);
    if tab_local != active_tab {
        ui.ctx().data_mut(|d| d.insert_temp(pending_key, (pass_nr, tab_local)));
    }
    ui.add_space(theme.spacing.lg);

    search_input(ui, theme, &mut state.search);
    ui.add_space(theme.spacing.lg);

    let is_searching = state.search.len() >= 3;

    // Both push_id scopes always exist so the auto-ID counter is stable
    // regardless of which tab is active.
    ui.push_id("tab_fav", |ui| {
        if active_tab == 0 {
            favorites_tab(ui, theme, state, is_searching);
        }
    });
    ui.push_id("tab_all", |ui| {
        if active_tab == 1 {
            all_maps_tab(ui, theme, state, is_searching);
        }
    });
}

// ---------------------------------------------------------------------------
// Search input
// ---------------------------------------------------------------------------

fn search_input(ui: &mut Ui, theme: &Theme, text: &mut String) {
    let row_height = 28.0;
    let icon_size = 16.0;
    let clear_btn_w = icon_size + theme.spacing.sm;

    ui.horizontal(|ui| {
        ui.set_height(row_height);
        ui.spacing_mut().item_spacing.x = theme.spacing.sm;

        ui.add_sized(
            Vec2::new(icon_size, row_height),
            egui::Label::new(icon_text(ICON_SEARCH, icon_size).color(theme.palette.muted_foreground)),
        );

        let remaining = ui.available_width()
            - if text.is_empty() { 0.0 } else { clear_btn_w + theme.spacing.sm };
        ui.add_sized(
            Vec2::new(remaining, row_height),
            egui::TextEdit::singleline(text)
                .hint_text(egui::RichText::new("Search...").color(theme.palette.muted_foreground).size(12.0))
                .font(egui::FontId::proportional(12.0))
                .text_color(theme.palette.foreground)
                .margin(egui::Margin::symmetric(theme.spacing.xs as i8, theme.spacing.sm as i8)),
        );

        if !text.is_empty() {
            let clear = ui.add_sized(
                Vec2::new(clear_btn_w, row_height),
                egui::Button::new(icon_text(ICON_CIRCLE_X, icon_size).color(theme.palette.muted_foreground)).frame(false),
            );
            if clear.clicked() {
                text.clear();
            }
        }
    });
}

// ---------------------------------------------------------------------------
// Star button helper
// ---------------------------------------------------------------------------

fn star_button(ui: &mut Ui, theme: &Theme, favorite: &mut bool) {
    let color = if *favorite {
        theme.palette.ring
    } else {
        theme.palette.muted_foreground
    };
    let icon = if *favorite { "★" } else { "☆" };
    let resp = ui.add(
        egui::Button::new(
            egui::RichText::new(icon).size(14.0).color(color),
        )
        .frame(false),
    );
    if resp.clicked() {
        *favorite = !*favorite;
    }
}

/// Render a map row: small checkbox on left, star on right.
fn map_row(ui: &mut Ui, theme: &Theme, entry: &mut MapEntry) {
    ui.horizontal(|ui| {
        checkbox_small(ui, theme, &mut entry.selected, &entry.name.clone());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            star_button(ui, theme, &mut entry.favorite);
        });
    });
}

// ---------------------------------------------------------------------------
// Favorites tab
// ---------------------------------------------------------------------------

fn favorites_tab(ui: &mut Ui, theme: &Theme, state: &mut MapsMenuState, is_searching: bool) {
    // Animate grid collapse
    let grid_t = ui.ctx().animate_bool_with_time(
        ui.id().with("grid_collapse"),
        !is_searching,
        0.2,
    );

    // Quick-access grid of favorite maps — isolated push_id so it doesn't shift IDs below
    ui.push_id("grid_section", |ui| { if grid_t > 0.0 {
        let grid_id = ui.id().with("grid_h");
        let prev_h: f32 = ui.ctx().data_mut(|d| d.get_temp(grid_id).unwrap_or(0.0));
        let anim_h = prev_h * grid_t;
        let width = ui.available_width();

        if anim_h > 0.0 {
            let top = ui.available_rect_before_wrap().min;
            let clip = egui::Rect::from_min_size(top, Vec2::new(width, anim_h));
            ui.allocate_exact_size(Vec2::new(width, anim_h), Sense::hover());

            let mut grid_ui = ui.new_child(
                egui::UiBuilder::new().max_rect(
                    egui::Rect::from_min_size(top, Vec2::new(width, f32::INFINITY)),
                ),
            );
            grid_ui.set_clip_rect(clip.intersect(ui.clip_rect()));
            grid_ui.set_opacity(grid_ui.opacity() * grid_t);

            render_quick_grid(&mut grid_ui, theme, &mut state.categories);

            let actual_h = grid_ui.min_size().y;
            ui.ctx().data_mut(|d| d.insert_temp(grid_id, actual_h));
        } else {
            // Measure
            let top = ui.available_rect_before_wrap().min;
            let mut measure = ui.new_child(
                egui::UiBuilder::new().max_rect(
                    egui::Rect::from_min_size(top, Vec2::new(width, f32::INFINITY)),
                ),
            );
            measure.set_clip_rect(egui::Rect::from_min_size(top, Vec2::ZERO));
            measure.set_invisible();
            render_quick_grid(&mut measure, theme, &mut state.categories);
            let actual_h = measure.min_size().y;
            ui.ctx().data_mut(|d| d.insert_temp(grid_id, actual_h));
        }

        ui.add_space(theme.spacing.lg * grid_t);
    } });

    // Content section — always in its own push_id scope
    ui.push_id("content_section", |ui| { if is_searching {
        ui.push_id("fav_search", |ui| {
            search_results_view(ui, theme, &mut state.categories, &state.search);
        });
    } else {
        ui.push_id("fav_accordions", |ui| {
            ui.label(
                egui::RichText::new("Favorites")
                    .size(13.0)
                    .color(theme.palette.foreground),
            );
            ui.add_space(theme.spacing.md);

            render_category_accordions(ui, theme, &mut state.categories, Filter::FavoritesOnly, 0);
        });
    } });
}

/// Render the fixed quick-access grid. Each checkbox references a real MapEntry
/// in the categories tree by name, so selected state stays in sync.
fn render_quick_grid(ui: &mut Ui, theme: &Theme, categories: &mut Vec<MapCategory>) {
    let col_count = 3;
    let width = ui.available_width();
    let col_width = (width / col_count as f32).floor();

    egui::Grid::new("quick_grid")
        .num_columns(col_count)
        .min_col_width(col_width)
        .max_col_width(col_width)
        .spacing(egui::vec2(theme.spacing.sm, theme.spacing.xs))
        .show(ui, |ui| {
            for (idx, &map_name) in GRID_MAPS.iter().enumerate() {
                if let Some(entry) = find_entry_mut(categories, map_name) {
                    checkbox_small(ui, theme, &mut entry.selected, &entry.name.clone());
                }
                if (idx + 1) % col_count == 0 {
                    ui.end_row();
                }
            }
        });
}

/// Find a map entry by name anywhere in the category tree.
fn find_entry_mut<'a>(categories: &'a mut Vec<MapCategory>, name: &str) -> Option<&'a mut MapEntry> {
    for cat in categories.iter_mut() {
        for entry in cat.maps.iter_mut() {
            if entry.name == name {
                return Some(entry);
            }
        }
        if let Some(e) = find_entry_mut(&mut cat.children, name) {
            return Some(e);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// All Maps tab
// ---------------------------------------------------------------------------

fn all_maps_tab(ui: &mut Ui, theme: &Theme, state: &mut MapsMenuState, is_searching: bool) {
    if is_searching {
        ui.push_id("all_search", |ui| {
            search_results_view(ui, theme, &mut state.categories, &state.search);
        });
    } else {
        ui.push_id("all_accordions", |ui| {
            render_category_accordions(ui, theme, &mut state.categories, Filter::All, 1);
        });
    }
}

// ---------------------------------------------------------------------------
// Search results
// ---------------------------------------------------------------------------

fn search_results_view(ui: &mut Ui, theme: &Theme, categories: &mut Vec<MapCategory>, search: &str) {
    let query = search.to_lowercase();
    let total: usize = categories.iter().map(|c| c.search_count(&query)).sum();

    ui.label(
        egui::RichText::new(format!("Search results ({})", total))
            .size(13.0)
            .color(theme.palette.foreground),
    );
    ui.add_space(theme.spacing.md);

    if total == 0 {
        ui.label(
            egui::RichText::new("No results found")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        return;
    }

    // Auto-open all matching categories in the search context
    open_matching_categories(categories, &query, 2);

    render_category_accordions(ui, theme, categories, Filter::Search(query), 2);
}

// ---------------------------------------------------------------------------
// Shared accordion renderer
// ---------------------------------------------------------------------------

enum Filter {
    All,
    FavoritesOnly,
    Search(String),
}

impl Filter {
    fn matches_entry(&self, entry: &MapEntry) -> bool {
        match self {
            Filter::All => true,
            Filter::FavoritesOnly => entry.favorite,
            Filter::Search(q) => entry.name.to_lowercase().contains(q),
        }
    }

    fn matches_category(&self, cat: &MapCategory) -> bool {
        match self {
            Filter::All => true,
            Filter::FavoritesOnly => cat.has_favorites(),
            Filter::Search(q) => cat.has_search_match(q),
        }
    }

    fn count_in(&self, cat: &MapCategory) -> usize {
        match self {
            Filter::All => cat.total_count(),
            Filter::FavoritesOnly => cat.favorite_count(),
            Filter::Search(q) => cat.search_count(q),
        }
    }
}

fn render_category_accordions(
    ui: &mut Ui,
    theme: &Theme,
    categories: &mut Vec<MapCategory>,
    filter: Filter,
    ctx: usize,
) {
    // Collect visible categories
    let visible_indices: Vec<usize> = categories
        .iter()
        .enumerate()
        .filter(|(_, c)| filter.matches_category(c))
        .map(|(i, _)| i)
        .collect();

    if visible_indices.is_empty() {
        return;
    }

    let titles: Vec<String> = visible_indices
        .iter()
        .map(|&i| {
            let c = &categories[i];
            let count = filter.count_in(c);
            format!("{} ({})", c.name, count)
        })
        .collect();
    let title_refs: Vec<&str> = titles.iter().map(|s| s.as_str()).collect();

    let mut open_states: Vec<bool> = visible_indices.iter().map(|&i| categories[i].open[ctx]).collect();

    accordion(
        ui,
        theme,
        &title_refs,
        &mut open_states,
        false,
        |ui, vi| {
            let cat_idx = visible_indices[vi];
            let cat = &mut categories[cat_idx];

            ui.push_id(cat_idx, |ui| {
                // Render matching maps
                for (ei, entry) in cat.maps.iter_mut().enumerate() {
                    if filter.matches_entry(entry) {
                        ui.push_id(ei, |ui| {
                            map_row(ui, theme, entry);
                        });
                        ui.add_space(theme.spacing.xs);
                    }
                }

                // Render matching children as nested accordions
                if !cat.children.is_empty() {
                    render_category_accordions(ui, theme, &mut cat.children, filter_ref(&filter), ctx);
                }
            });
        },
    );

    // Write back open states
    for (vi, &cat_idx) in visible_indices.iter().enumerate() {
        categories[cat_idx].open[ctx] = open_states[vi];
    }
}

/// Force-open all categories that have search matches in a given context.
fn open_matching_categories(categories: &mut Vec<MapCategory>, query: &str, ctx: usize) {
    for cat in categories.iter_mut() {
        if cat.has_search_match(query) {
            cat.open[ctx] = true;
        }
        open_matching_categories(&mut cat.children, query, ctx);
    }
}

/// Clone-like helper for filter (can't Clone because of String).
fn filter_ref(f: &Filter) -> Filter {
    match f {
        Filter::All => Filter::All,
        Filter::FavoritesOnly => Filter::FavoritesOnly,
        Filter::Search(q) => Filter::Search(q.clone()),
    }
}

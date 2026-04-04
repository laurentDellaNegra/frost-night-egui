//! Shared demo app used by both the native example and the web-demo crate.
//!
//! Gated behind the `demo` feature (requires `eframe`).

use eframe::egui;

use crate::components::*;
use crate::icons::*;
use crate::widgets::MapsMenuState;
use crate::{apply_theme, ControlSize, ControlVariant, Theme};

// ---------------------------------------------------------------------------
// Track data
// ---------------------------------------------------------------------------

struct Track {
    nx: f32,
    ny: f32,
    callsign: &'static str,
    vx: f32,
    vy: f32,
}

#[derive(Clone)]
struct LiveTrack {
    nx: f32,
    ny: f32,
    callsign: &'static str,
    vx: f32,
    vy: f32,
    dnx: f32,
    dny: f32,
}

const TRACKS: &[Track] = &[
    Track { nx: 0.15, ny: 0.12, callsign: "SWR142", vx: 30.0, vy: 15.0 },
    Track { nx: 0.38, ny: 0.23, callsign: "BAW73C", vx: -20.0, vy: 25.0 },
    Track { nx: 0.63, ny: 0.46, callsign: "EZY18P", vx: 25.0, vy: -10.0 },
    Track { nx: 0.23, ny: 0.62, callsign: "DLH4N", vx: 15.0, vy: 20.0 },
    Track { nx: 0.75, ny: 0.18, callsign: "AFR61", vx: -10.0, vy: 30.0 },
    Track { nx: 0.53, ny: 0.69, callsign: "TAP9K", vx: 20.0, vy: -15.0 },
    Track { nx: 0.09, ny: 0.38, callsign: "RYR3F", vx: 35.0, vy: 5.0 },
    Track { nx: 0.69, ny: 0.77, callsign: "AUA22", vx: -15.0, vy: -20.0 },
    Track { nx: 0.44, ny: 0.08, callsign: "FIN8B", vx: 10.0, vy: 25.0 },
    Track { nx: 0.85, ny: 0.58, callsign: "KLM56", vx: -25.0, vy: 10.0 },
    Track { nx: 0.11, ny: 0.80, callsign: "THY4A", vx: 28.0, vy: -8.0 },
    Track { nx: 0.93, ny: 0.14, callsign: "SAS91", vx: -18.0, vy: 22.0 },
    Track { nx: 0.25, ny: 0.31, callsign: "IBE34", vx: 22.0, vy: 18.0 },
    Track { nx: 0.58, ny: 0.12, callsign: "AZA7F", vx: -12.0, vy: 28.0 },
    Track { nx: 0.79, ny: 0.83, callsign: "LOT3B", vx: 15.0, vy: -25.0 },
    Track { nx: 0.43, ny: 0.49, callsign: "CSA52", vx: -30.0, vy: 5.0 },
    Track { nx: 0.19, ny: 0.89, callsign: "BEL9C", vx: 20.0, vy: -12.0 },
    Track { nx: 0.65, ny: 0.29, callsign: "AAL88", vx: -8.0, vy: 32.0 },
    Track { nx: 0.88, ny: 0.40, callsign: "UAL15", vx: -22.0, vy: -15.0 },
    Track { nx: 0.35, ny: 0.74, callsign: "DAL67", vx: 18.0, vy: 12.0 },
    Track { nx: 0.06, ny: 0.22, callsign: "JAL02", vx: 32.0, vy: 10.0 },
    Track { nx: 0.73, ny: 0.65, callsign: "QFA8R", vx: -14.0, vy: -28.0 },
    Track { nx: 0.50, ny: 0.86, callsign: "SIA32", vx: 10.0, vy: -18.0 },
    Track { nx: 0.31, ny: 0.05, callsign: "CPA71", vx: -5.0, vy: 30.0 },
    Track { nx: 0.95, ny: 0.74, callsign: "ANZ6D", vx: -20.0, vy: -10.0 },
    Track { nx: 0.16, ny: 0.52, callsign: "ETH5A", vx: 25.0, vy: 15.0 },
    Track { nx: 0.61, ny: 0.94, callsign: "RAM44", vx: 12.0, vy: -22.0 },
    Track { nx: 0.83, ny: 0.28, callsign: "TAR12", vx: -28.0, vy: 8.0 },
    Track { nx: 0.48, ny: 0.37, callsign: "VIR9B", vx: 16.0, vy: 20.0 },
    Track { nx: 0.05, ny: 0.69, callsign: "EIN3G", vx: 30.0, vy: -5.0 },
];

const WAYPOINTS: &[(f32, f32, &str)] = &[
    (0.31, 0.34, "MOLUS"),
    (0.56, 0.28, "DEGES"),
    (0.19, 0.54, "KELIP"),
    (0.73, 0.40, "ARBOS"),
    (0.88, 0.62, "TITIX"),
    (0.40, 0.77, "LUPEN"),
    (0.13, 0.15, "NARAK"),
    (0.69, 0.08, "VEBIT"),
    (0.50, 0.62, "RIPUS"),
    (0.25, 0.85, "ODINA"),
];

// ---------------------------------------------------------------------------
// Background
// ---------------------------------------------------------------------------

const MAP_AREAS: &[(&[(f32, f32)], [u8; 3])] = &[
    (&[(0.10, 0.15), (0.35, 0.10), (0.45, 0.25), (0.40, 0.45), (0.15, 0.40)], [0x08, 0x12, 0x22]),
    (&[(0.50, 0.05), (0.75, 0.08), (0.80, 0.30), (0.60, 0.35), (0.48, 0.20)], [0x0A, 0x16, 0x2A]),
    (&[(0.55, 0.50), (0.80, 0.45), (0.90, 0.65), (0.85, 0.80), (0.60, 0.75)], [0x06, 0x0E, 0x1C]),
    (&[(0.02, 0.55), (0.25, 0.50), (0.35, 0.70), (0.30, 0.90), (0.05, 0.85)], [0x09, 0x14, 0x26]),
    (&[(0.35, 0.25), (0.55, 0.30), (0.58, 0.50), (0.40, 0.55), (0.30, 0.40)], [0x07, 0x10, 0x20]),
];

fn paint_background(ui: &egui::Ui, live_tracks: &[LiveTrack]) {
    let rect = ui.max_rect();
    let painter = ui.painter();
    let w = rect.width();
    let h = rect.height();
    let green = egui::Color32::from_rgb(0x00, 0xCC, 0x44);
    let dim_green = egui::Color32::from_rgb(0x00, 0x66, 0x22);
    let grid_color = egui::Color32::from_rgb(0x08, 0x18, 0x28);

    for &(poly, col) in MAP_AREAS {
        let points: Vec<egui::Pos2> = poly
            .iter()
            .map(|&(nx, ny)| egui::pos2(rect.left() + nx * w, rect.top() + ny * h))
            .collect();
        let fill = egui::Color32::from_rgb(col[0], col[1], col[2]);
        painter.add(egui::Shape::convex_polygon(
            points.clone(),
            fill,
            egui::Stroke::NONE,
        ));
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            painter.line_segment(
                [points[i], points[j]],
                egui::Stroke::new(0.5, egui::Color32::from_rgb(col[0] + 4, col[1] + 8, col[2] + 12)),
            );
        }
    }

    let step = 40.0;
    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            egui::Stroke::new(0.5, grid_color),
        );
        x += step;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            egui::Stroke::new(0.5, grid_color),
        );
        y += step;
    }

    let center = rect.center();
    for r in [80.0, 160.0, 240.0, 320.0, 400.0] {
        painter.circle_stroke(center, r, egui::Stroke::new(0.5, grid_color));
    }

    for t in live_tracks {
        let pos = egui::pos2(rect.left() + t.nx * w, rect.top() + t.ny * h);
        let s = 4.0;
        painter.add(egui::Shape::convex_polygon(
            vec![
                egui::pos2(pos.x, pos.y - s),
                egui::pos2(pos.x + s, pos.y),
                egui::pos2(pos.x, pos.y + s),
                egui::pos2(pos.x - s, pos.y),
            ],
            green,
            egui::Stroke::NONE,
        ));
        painter.line_segment(
            [pos, egui::pos2(pos.x + t.vx, pos.y + t.vy)],
            egui::Stroke::new(1.0, green),
        );
        painter.text(
            egui::pos2(pos.x + 8.0, pos.y - 6.0),
            egui::Align2::LEFT_CENTER,
            t.callsign,
            egui::FontId::monospace(10.0),
            green,
        );
        for i in 1..=4 {
            let f = i as f32;
            painter.circle_filled(
                egui::pos2(pos.x - t.vx * f * 0.3, pos.y - t.vy * f * 0.3),
                1.5,
                dim_green,
            );
        }
    }

    for &(nx, ny, name) in WAYPOINTS {
        let pos = egui::pos2(rect.left() + nx * w, rect.top() + ny * h);
        let s = 3.5;
        painter.add(egui::Shape::convex_polygon(
            vec![
                egui::pos2(pos.x, pos.y - s),
                egui::pos2(pos.x + s, pos.y + s * 0.7),
                egui::pos2(pos.x - s, pos.y + s * 0.7),
            ],
            egui::Color32::TRANSPARENT,
            egui::Stroke::new(1.0, dim_green),
        ));
        painter.text(
            egui::pos2(pos.x + 6.0, pos.y),
            egui::Align2::LEFT_CENTER,
            name,
            egui::FontId::monospace(9.0),
            dim_green,
        );
    }
}

// ---------------------------------------------------------------------------
// Demo card content
// ---------------------------------------------------------------------------

fn demo_card_content(
    ui: &mut egui::Ui,
    theme: &Theme,
    card_salt: usize,
    input_text: &mut String,
    toggle_on: &mut bool,
    check_a: &mut bool,
    check_b: &mut bool,
    check_c: &mut bool,
    segment_idx: &mut usize,
) {
    ui.push_id(card_salt, |ui| {
        ui.label(egui::RichText::new("Buttons").size(13.0).strong());
        ui.add_space(4.0);
        egui::Grid::new("sidebar_buttons_grid")
            .spacing([6.0, 4.0])
            .show(ui, |ui| {
                for variant in [
                    ControlVariant::Primary,
                    ControlVariant::Secondary,
                    ControlVariant::Ghost,
                    ControlVariant::Outline,
                    ControlVariant::Destructive,
                    ControlVariant::Link,
                ] {
                    for size in [ControlSize::Sm, ControlSize::Md, ControlSize::Lg] {
                        button(ui, theme, format!("{variant:?}"), variant, size);
                    }
                    ui.end_row();
                }
            });

        ui.add_space(8.0);
        separator(ui, theme);
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Text Input").size(13.0).strong());
        ui.add_space(4.0);
        text_input(ui, theme, input_text, ControlSize::Md);

        ui.add_space(8.0);
        separator(ui, theme);
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Badges").size(13.0).strong());
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            badge(ui, theme, "Primary", BadgeVariant::Primary);
            badge(ui, theme, "Accent", BadgeVariant::Accent);
            badge(ui, theme, "Outline", BadgeVariant::Outline);
            badge(ui, theme, "Destructive", BadgeVariant::Destructive);
        });

        ui.add_space(8.0);
        separator(ui, theme);
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Toggle Switch").size(13.0).strong());
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            toggle(ui, theme, toggle_on);
            ui.label(if *toggle_on { "ON" } else { "OFF" });
        });

        ui.add_space(8.0);
        separator(ui, theme);
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Checkbox").size(13.0).strong());
        ui.add_space(4.0);
        checkbox(ui, theme, check_a, "Label (checked)");
        ui.add_space(2.0);
        checkbox(ui, theme, check_b, "Label");
        ui.add_space(2.0);
        checkbox(ui, theme, check_c, "Label");

        ui.add_space(8.0);
        separator(ui, theme);
        ui.add_space(8.0);

        ui.label(egui::RichText::new("Segmented Control").size(13.0).strong());
        ui.add_space(4.0);
        segmented(ui, theme, &["Active", "Inactive"], segment_idx);
    });
}


fn demo_accordion_content(
    ui: &mut egui::Ui,
    theme: &Theme,
    map_tab: &mut usize,
    acc_open: &mut Vec<bool>,
    nested: &mut [Vec<bool>; 4],
) {
    tabs(ui, theme, map_tab, &["Layers", "Filters", "Settings"]);
    ui.add_space(theme.spacing.sm);

    accordion(
        ui,
        theme,
        &["Base Maps", "Overlays", "Airspace", "Weather"],
        acc_open,
        false,
        |ui, i| match i {
            0 => {
                accordion(
                    ui, theme,
                    &["Standard", "Satellite", "Terrain"],
                    &mut nested[0], true,
                    |ui, j| match j {
                        0 => { ui.label("OpenStreetMap tiles"); ui.label("Light/dark variants"); }
                        1 => { ui.label("High-res imagery"); ui.label("Cloud-free composite"); }
                        2 => { ui.label("Elevation contours"); ui.label("Hillshade overlay"); }
                        _ => {}
                    },
                );
            }
            1 => {
                accordion(
                    ui, theme,
                    &["Airways", "Waypoints", "Navaids"],
                    &mut nested[1], true,
                    |ui, j| match j {
                        0 => { ui.label("Upper: UL/UT routes"); ui.label("Lower: L routes"); }
                        1 => { ui.label("RNAV fixes"); ui.label("Conventional intersections"); }
                        2 => { ui.label("VOR/DME stations"); ui.label("NDB beacons"); }
                        _ => {}
                    },
                );
            }
            2 => {
                accordion(
                    ui, theme,
                    &["TMA", "CTR", "FIR"],
                    &mut nested[2], true,
                    |ui, j| match j {
                        0 => { ui.label("TMA Zurich"); ui.label("TMA Geneva"); }
                        1 => { ui.label("CTR Zurich 1/2"); ui.label("CTR Geneva"); }
                        2 => { ui.label("LSAS Switzerland"); ui.label("Adjacent FIRs"); }
                        _ => {}
                    },
                );
            }
            3 => {
                accordion(
                    ui, theme,
                    &["Surface", "Upper air"],
                    &mut nested[3], true,
                    |ui, j| match j {
                        0 => { ui.label("METAR / SPECI"); ui.label("TAF forecasts"); }
                        1 => { ui.label("SIGMET / AIRMET"); ui.label("Winds & temps aloft"); }
                        _ => {}
                    },
                );
            }
            _ => {}
        },
    );
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct FloatingCard {
    pos: egui::Pos2,
    from_button: usize,
    highlight_time: f64,
}

pub struct DemoApp {
    theme: Theme,
    input_text: String,
    toggle_on: bool,
    check_a: bool,
    check_b: bool,
    check_c: bool,
    segment_idx: usize,
    docked_button: Option<usize>,
    floating_cards: Vec<FloatingCard>,
    last_docked_pos: Option<(egui::Pos2, usize)>,
    docked_detached: bool,
    docked_drag_offset: egui::Vec2,
    any_card_dragging: bool,
    live_tracks: Vec<LiveTrack>,
    map_tab: usize,
    accordion_open: Vec<bool>,
    accordion_nested: [Vec<bool>; 4],
    maps_menu: MapsMenuState,
}

impl DemoApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let theme = Theme::dark();
        apply_theme(&cc.egui_ctx, &theme);

        let ref_w = 800.0f32;
        let ref_h = 650.0f32;
        let live_tracks: Vec<LiveTrack> = TRACKS
            .iter()
            .map(|t| LiveTrack {
                nx: t.nx,
                ny: t.ny,
                callsign: t.callsign,
                vx: t.vx,
                vy: t.vy,
                dnx: t.vx / ref_w,
                dny: t.vy / ref_h,
            })
            .collect();

        Self {
            theme,
            input_text: String::new(),
            toggle_on: false,
            check_a: true,
            check_b: false,
            check_c: false,
            segment_idx: 0,
            docked_button: None,
            floating_cards: Vec::new(),
            last_docked_pos: None,
            docked_detached: false,
            docked_drag_offset: egui::Vec2::ZERO,
            any_card_dragging: false,
            live_tracks,
            map_tab: 0,
            accordion_open: vec![true, false, false, false],
            accordion_nested: [
                vec![true, false, false],
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false],
            ],
            maps_menu: MapsMenuState::default(),
        }
    }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let full_rect = ui.max_rect();

        let dt = ui.input(|i| i.stable_dt).min(0.1);
        let speed = 0.25;
        for t in &mut self.live_tracks {
            t.nx += t.dnx * speed * dt;
            t.ny += t.dny * speed * dt;
            if t.nx > 1.0 { t.nx -= 1.0; }
            if t.nx < 0.0 { t.nx += 1.0; }
            if t.ny > 1.0 { t.ny -= 1.0; }
            if t.ny < 0.0 { t.ny += 1.0; }
        }
        ui.ctx().request_repaint();

        ui.painter()
            .rect_filled(full_rect, 0.0, self.theme.palette.background);
        paint_background(ui, &self.live_tracks);

        // Global drag fade (based on PREVIOUS frame's drag state)
        let drag_fade_t = ui.ctx().animate_bool_with_time(
            egui::Id::new("global_drag_fade"),
            self.any_card_dragging,
            0.15,
        );
        if drag_fade_t > 0.01 {
            ui.set_opacity(egui::lerp(1.0..=0.15, drag_fade_t));
        }
        let mut any_dragging_this_frame = false;

        // Left toolbar (fixed)
        let toolbar_groups: Vec<ToolbarGroup> = vec![
            vec![
                ToolbarItem::new(ICON_PANEL_LEFT),
            ],
            vec![
                ToolbarItem::new(ICON_MAP).with_badge(egui::Color32::from_rgb(0xE0, 0x5A, 0x7A)),
                ToolbarItem::new(ICON_LAYERS).with_badge(egui::Color32::from_rgb(0x4A, 0x90, 0xCF)),
                ToolbarItem::new(ICON_GLOBE),
                ToolbarItem::new(ICON_PLUS),
                ToolbarItem::new(ICON_RADAR),
                ToolbarItem::new(ICON_NAVIGATION),
                ToolbarItem::new(ICON_CROSSHAIR),
            ],
            vec![
                ToolbarItem::new(ICON_FILTER),
                ToolbarItem::new(ICON_SETTINGS),
            ],
        ];

        let toolbar_margin = 12.0;
        let top_tb_height = 36.0;

        // Top toolbar
        let top_tb_x = full_rect.left() + toolbar_margin;
        let top_tb_y = full_rect.top() + toolbar_margin;
        let mut top_tb_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("top_toolbar")
                .max_rect(
                    egui::Rect::from_min_size(
                        egui::pos2(top_tb_x, top_tb_y),
                        egui::vec2(full_rect.width() - toolbar_margin * 2.0, top_tb_height),
                    ),
                ),
        );
        let _top_response = top_toolbar(
            &mut top_tb_ui,
            &self.theme,
            "Frost Night egui",
            "23:14:20",
            "1016",
            "80",
            Some("ERROR"),
            &[ICON_GRID, ICON_COMPASS, ICON_EYE],
        );

        // Left toolbar (below top toolbar)
        let tb_x = full_rect.left() + toolbar_margin;
        let tb_y = top_tb_y + top_tb_height + self.theme.spacing.xl;
        let mut toolbar_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("left_toolbar")
                .max_rect(
                    egui::Rect::from_min_size(
                        egui::pos2(tb_x, tb_y),
                        egui::vec2(60.0, full_rect.height() - tb_y - toolbar_margin),
                    ),
                ),
        );
        let floating_buttons: Vec<usize> = self.floating_cards.iter().map(|f| f.from_button).collect();
        let tb_response = toolbar(&mut toolbar_ui, &self.theme, &toolbar_groups, self.docked_button, &floating_buttons);

        // Sidebar / floating card constants
        let left_tb_width = 36.0 + self.theme.spacing.xs * 2.0;
        let sidebar_card_width = 420.0;
        let sidebar_card_height = 560.0;
        let dock_x = tb_x + left_tb_width + self.theme.spacing.lg;

        // Handle toolbar button clicks
        if let Some(clicked) = tb_response.clicked {
            if let Some(idx) = self.floating_cards.iter().position(|f| f.from_button == clicked) {
                self.floating_cards[idx].highlight_time = ui.input(|i| i.time);
                let card = self.floating_cards.remove(idx);
                self.floating_cards.push(card);
            } else if self.docked_button == Some(clicked) {
                self.last_docked_pos = Some((egui::pos2(dock_x, tb_y), clicked));
                self.docked_button = None;
            } else {
                self.docked_button = Some(clicked);
                self.docked_detached = false;
            }
        }

        let docked_button_this_frame = self.docked_button;

        let panel_titles = [
            "Panel", "", "Map", "Globe", "Add", "Radar", "Navigation",
            "Crosshair", "Filter", "Settings",
        ];

        // --- Docked card (with open/close animation) ---
        // Wrapped in push_id to isolate auto-ID counters from floating section.
        let mut pending_float: Option<FloatingCard> = None;
        let is_docked_open = docked_button_this_frame.is_some();
        let docked_open_t = ui.ctx().animate_bool_with_time(
            egui::Id::new("sidebar_card_anim"),
            is_docked_open,
            0.15,
        );

        ui.push_id("docked_section", |ui| {
            if docked_open_t > 0.01 && !self.docked_detached {
                let (base_pos, button_idx) = if let Some(idx) = docked_button_this_frame {
                    let pos = egui::pos2(dock_x, tb_y);
                    self.last_docked_pos = Some((pos, idx));
                    (pos, idx)
                } else {
                    self.last_docked_pos.unwrap_or((egui::pos2(dock_x, tb_y), 0))
                };

                let card_rect = egui::Rect::from_min_size(
                    base_pos + self.docked_drag_offset,
                    egui::vec2(sidebar_card_width, sidebar_card_height),
                );
                let title = panel_titles.get(button_idx).copied().unwrap_or("Panel");

                let theme = &self.theme;
                let card_resp = match button_idx {
                    0 => {
                        let (it, to, ca, cb, cc, si) = (
                            &mut self.input_text, &mut self.toggle_on,
                            &mut self.check_a, &mut self.check_b, &mut self.check_c,
                            &mut self.segment_idx,
                        );
                        sidebar_card(ui, theme, egui::Id::new("docked_sidebar_card"),
                            card_rect, docked_open_t, title, false,
                            |ui| demo_card_content(ui, theme, 0, it, to, ca, cb, cc, si))
                    }
                    1 => {
                        let mm = &mut self.maps_menu;
                        sidebar_card(ui, theme, egui::Id::new("docked_sidebar_card"),
                            card_rect, docked_open_t, title, false,
                            |ui| crate::widgets::maps_menu(ui, theme, mm))
                    }
                    2 => {
                        let (mt, ao, an) = (&mut self.map_tab, &mut self.accordion_open, &mut self.accordion_nested);
                        sidebar_card(ui, theme, egui::Id::new("docked_sidebar_card"),
                            card_rect, docked_open_t, title, false,
                            |ui| demo_accordion_content(ui, theme, mt, ao, an))
                    }
                    _ => {
                        sidebar_card(ui, theme, egui::Id::new("docked_sidebar_card"),
                            card_rect, docked_open_t, title, false,
                            |ui| { ui.label(egui::RichText::new(format!("{title} panel content")).size(13.0).color(theme.palette.muted_foreground)); })
                    }
                };

                if card_resp.dragging {
                    self.docked_drag_offset += card_resp.drag_delta;
                    any_dragging_this_frame = true;
                } else if self.docked_drag_offset.length() > 1.0 {
                    if let Some(idx) = docked_button_this_frame {
                        self.docked_button = None;
                        pending_float = Some(FloatingCard {
                            pos: card_rect.min,
                            from_button: idx,
                            highlight_time: 0.0,
                        });
                        self.docked_detached = true;
                    }
                    self.docked_drag_offset = egui::Vec2::ZERO;
                } else {
                    self.docked_drag_offset = egui::Vec2::ZERO;
                    if card_resp.closed {
                        if let Some(idx) = docked_button_this_frame {
                            self.last_docked_pos = Some((egui::pos2(dock_x, tb_y), idx));
                        }
                        self.docked_button = None;
                    }
                }
            }
        });

        if docked_open_t <= 0.01 {
            self.docked_detached = false;
        }

        // --- Floating (parked) cards ---
        // Wrapped in push_id to isolate auto-ID counters from docked section.
        // Rendered in Vec order: last = on top (highest z-index).
        let mut floating_to_remove = vec![];
        let mut bring_to_front: Option<usize> = None;
        ui.push_id("floating_section", |ui| {
            for i in 0..self.floating_cards.len() {
                let pos = self.floating_cards[i].pos;
                let from_button = self.floating_cards[i].from_button;
                let now = ui.input(|i| i.time);
                let hl = (now - self.floating_cards[i].highlight_time) < 0.3;
                let card_rect = egui::Rect::from_min_size(
                    pos,
                    egui::vec2(sidebar_card_width, sidebar_card_height),
                );
                let title = panel_titles.get(from_button).copied().unwrap_or("Panel");

                let theme = &self.theme;
                let card_id = egui::Id::new(("sidebar_card", from_button));
                let card_resp = match from_button {
                    0 => {
                        let (it, to, ca, cb, cc, si) = (
                            &mut self.input_text, &mut self.toggle_on,
                            &mut self.check_a, &mut self.check_b, &mut self.check_c,
                            &mut self.segment_idx,
                        );
                        sidebar_card(ui, theme, card_id, card_rect, 1.0, title, hl,
                            |ui| demo_card_content(ui, theme, from_button, it, to, ca, cb, cc, si))
                    }
                    1 => {
                        let mm = &mut self.maps_menu;
                        sidebar_card(ui, theme, card_id, card_rect, 1.0, title, hl,
                            |ui| crate::widgets::maps_menu(ui, theme, mm))
                    }
                    2 => {
                        let (mt, ao, an) = (&mut self.map_tab, &mut self.accordion_open, &mut self.accordion_nested);
                        sidebar_card(ui, theme, card_id, card_rect, 1.0, title, hl,
                            |ui| demo_accordion_content(ui, theme, mt, ao, an))
                    }
                    _ => {
                        sidebar_card(ui, theme, card_id, card_rect, 1.0, title, hl,
                            |ui| { ui.label(egui::RichText::new(format!("{title} panel content")).size(13.0).color(theme.palette.muted_foreground)); })
                    }
                };

                if card_resp.dragging {
                    self.floating_cards[i].pos = pos + card_resp.drag_delta;
                    any_dragging_this_frame = true;
                    bring_to_front = Some(i);
                } else if ui.input(|i| i.pointer.any_pressed())
                    && card_rect.contains(
                        ui.input(|i| i.pointer.interact_pos().unwrap_or_default()),
                    )
                {
                    bring_to_front = Some(i);
                }
                if card_resp.closed {
                    floating_to_remove.push(i);
                }
            }
        });
        for idx in floating_to_remove.into_iter().rev() {
            self.floating_cards.remove(idx);
        }
        // Bring interacted card to front (move to end of Vec)
        if let Some(idx) = bring_to_front {
            if idx < self.floating_cards.len() {
                let card = self.floating_cards.remove(idx);
                self.floating_cards.push(card);
            }
        }

        if let Some(fc) = pending_float {
            self.floating_cards.push(fc);
        }

        self.any_card_dragging = any_dragging_this_frame;

        // Bottom-right zoom control toolbar
        let zoom_margin = 12.0;
        let zoom_w = 36.0 + self.theme.spacing.xs * 2.0;
        let zoom_h = self.theme.spacing.xs * 2.0
            + 36.0 * 2.0
            + self.theme.spacing.xs * 2.0
            + 1.0
            + 28.0;
        let zoom_pos = egui::pos2(
            full_rect.right() - zoom_margin - zoom_w,
            full_rect.bottom() - zoom_margin - zoom_h,
        );
        let zoom_rect = egui::Rect::from_min_size(zoom_pos, egui::vec2(zoom_w, zoom_h));
        let _zoom_response = zoom_toolbar(
            ui,
            &self.theme,
            zoom_rect,
            ICON_PLUS,
            ICON_MINUS,
        );
    }
}

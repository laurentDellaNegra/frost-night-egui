use eframe::egui;
use ui_theme::components::*;
use ui_theme::{apply_theme, ControlSize, ControlVariant, Theme};

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
        painter.add(egui::Shape::convex_polygon(points.clone(), fill, egui::Stroke::NONE));
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
        painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], egui::Stroke::new(0.5, grid_color));
        x += step;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], egui::Stroke::new(0.5, grid_color));
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
                egui::pos2(pos.x, pos.y - s), egui::pos2(pos.x + s, pos.y),
                egui::pos2(pos.x, pos.y + s), egui::pos2(pos.x - s, pos.y),
            ],
            green, egui::Stroke::NONE,
        ));
        painter.line_segment([pos, egui::pos2(pos.x + t.vx, pos.y + t.vy)], egui::Stroke::new(1.0, green));
        painter.text(egui::pos2(pos.x + 8.0, pos.y - 6.0), egui::Align2::LEFT_CENTER, t.callsign, egui::FontId::monospace(10.0), green);
        for i in 1..=4 {
            let f = i as f32;
            painter.circle_filled(egui::pos2(pos.x - t.vx * f * 0.3, pos.y - t.vy * f * 0.3), 1.5, dim_green);
        }
    }

    for &(nx, ny, name) in WAYPOINTS {
        let pos = egui::pos2(rect.left() + nx * w, rect.top() + ny * h);
        let s = 3.5;
        painter.add(egui::Shape::convex_polygon(
            vec![egui::pos2(pos.x, pos.y - s), egui::pos2(pos.x + s, pos.y + s * 0.7), egui::pos2(pos.x - s, pos.y + s * 0.7)],
            egui::Color32::TRANSPARENT, egui::Stroke::new(1.0, dim_green),
        ));
        painter.text(egui::pos2(pos.x + 6.0, pos.y), egui::Align2::LEFT_CENTER, name, egui::FontId::monospace(9.0), dim_green);
    }
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

struct DemoApp {
    theme: Theme,
    input_text: String,
    toggle_on: bool,
    check_a: bool,
    check_b: bool,
    check_c: bool,
    segment_idx: usize,
    card_state: DragCardState,
    card_open: bool,
    live_tracks: Vec<LiveTrack>,
}

impl DemoApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        let theme = Theme::dark();
        apply_theme(&cc.egui_ctx, &theme);

        let ref_w = 800.0f32;
        let ref_h = 650.0f32;
        let live_tracks: Vec<LiveTrack> = TRACKS
            .iter()
            .map(|t| LiveTrack {
                nx: t.nx, ny: t.ny,
                callsign: t.callsign, vx: t.vx, vy: t.vy,
                dnx: t.vx / ref_w, dny: t.vy / ref_h,
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
            card_state: DragCardState {
                pos: egui::pos2(190.0, 40.0),
                size: egui::vec2(420.0, 560.0),
            },
            card_open: true,
            live_tracks,
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

        ui.painter().rect_filled(full_rect, 0.0, self.theme.palette.background);
        paint_background(ui, &self.live_tracks);

        if self.card_open {
            let card_response = drag_card(
                ui, &self.theme, egui::Id::new("demo_card"),
                &mut self.card_state, "Frost Night UI Demo",
                |ui| {
                    ui.label(egui::RichText::new("Buttons").size(13.0).strong());
                    ui.add_space(4.0);
                    egui::Grid::new("buttons_grid").spacing([6.0, 4.0]).show(ui, |ui| {
                        for variant in [
                            ControlVariant::Primary, ControlVariant::Secondary,
                            ControlVariant::Ghost, ControlVariant::Outline,
                            ControlVariant::Destructive, ControlVariant::Link,
                        ] {
                            for size in [ControlSize::Sm, ControlSize::Md, ControlSize::Lg] {
                                button(ui, &self.theme, format!("{variant:?}"), variant, size);
                            }
                            ui.end_row();
                        }
                    });

                    ui.add_space(8.0);
                    separator(ui, &self.theme);
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new("Text Input").size(13.0).strong());
                    ui.add_space(4.0);
                    text_input(ui, &self.theme, &mut self.input_text, ControlSize::Md);

                    ui.add_space(8.0);
                    separator(ui, &self.theme);
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new("Badges").size(13.0).strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        badge(ui, &self.theme, "Primary", BadgeVariant::Primary);
                        badge(ui, &self.theme, "Accent", BadgeVariant::Accent);
                        badge(ui, &self.theme, "Outline", BadgeVariant::Outline);
                        badge(ui, &self.theme, "Destructive", BadgeVariant::Destructive);
                    });

                    ui.add_space(8.0);
                    separator(ui, &self.theme);
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new("Toggle Switch").size(13.0).strong());
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        toggle(ui, &self.theme, &mut self.toggle_on);
                        ui.label(if self.toggle_on { "ON" } else { "OFF" });
                    });

                    ui.add_space(8.0);
                    separator(ui, &self.theme);
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new("Checkbox").size(13.0).strong());
                    ui.add_space(4.0);
                    checkbox(ui, &self.theme, &mut self.check_a, "Label (checked)");
                    ui.add_space(2.0);
                    checkbox(ui, &self.theme, &mut self.check_b, "Label");
                    ui.add_space(2.0);
                    checkbox(ui, &self.theme, &mut self.check_c, "Label");

                    ui.add_space(8.0);
                    separator(ui, &self.theme);
                    ui.add_space(8.0);

                    ui.label(egui::RichText::new("Segmented Control").size(13.0).strong());
                    ui.add_space(4.0);
                    segmented(ui, &self.theme, &["Active", "Inactive"], &mut self.segment_idx);
                },
            );
            if card_response.closed {
                self.card_open = false;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Entry points
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Frost Night UI Demo",
        options,
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("the_canvas_id")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
            )
            .await;
        if let Err(e) = start_result {
            log::error!("Failed to start eframe: {e:?}");
        }
    });
}

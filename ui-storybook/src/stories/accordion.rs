use ui_theme::components::accordion;
use ui_theme::Theme;

pub struct AccordionStoryState {
    pub exclusive: bool,
    pub item_count: usize,
    pub open: Vec<bool>,
    pub exclusive_demo_open: Vec<bool>,
    pub multi_demo_open: Vec<bool>,
    pub nested_outer: Vec<bool>,
    pub nested_inner: [Vec<bool>; 3],
}

impl Default for AccordionStoryState {
    fn default() -> Self {
        Self {
            exclusive: false,
            item_count: 3,
            open: vec![true, false, false],
            exclusive_demo_open: vec![true, false, false],
            multi_demo_open: vec![true, true, false],
            nested_outer: vec![true, false, false],
            nested_inner: [
                vec![true, false],
                vec![false, false],
                vec![false, false],
            ],
        }
    }
}

pub fn accordion_story(ui: &mut egui::Ui, theme: &Theme, state: &mut AccordionStoryState) {
    // Controls
    super::controls::controls_panel(ui, theme, |ui| {
        ui.checkbox(&mut state.exclusive, "Exclusive (only one open)");
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Sections")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            let mut count = state.item_count as f32;
            ui.add(egui::Slider::new(&mut count, 2.0..=6.0).step_by(1.0));
            state.item_count = count as usize;
        });
    });

    // Sync open vec with item count
    while state.open.len() < state.item_count {
        state.open.push(false);
    }
    state.open.truncate(state.item_count);

    let titles: Vec<&str> = [
        "Flight Information",
        "Weather Data",
        "Navigation Settings",
        "Communication",
        "System Status",
        "Advanced Options",
    ]
    .iter()
    .take(state.item_count)
    .copied()
    .collect();

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        accordion(ui, theme, &titles, &mut state.open, state.exclusive, |ui, i| {
            match i {
                0 => {
                    ui.label("Flight: SWR 1234");
                    ui.label("Route: LSZH → LFPG");
                    ui.label("Status: En route");
                }
                1 => {
                    ui.label("METAR: LSZH 281450Z 24008KT CAVOK");
                    ui.label("QNH: 1016 hPa");
                    ui.label("Temperature: 12°C");
                }
                2 => {
                    ui.label("Active waypoint: DEGES");
                    ui.label("Distance: 142 NM");
                    ui.label("ETA: 14:32 UTC");
                }
                3 => {
                    ui.label("Frequency: 124.700 MHz");
                    ui.label("Squawk: 4521");
                }
                4 => {
                    ui.label("GPS: Operational");
                    ui.label("Radar: Active");
                    ui.label("TCAS: Normal");
                }
                _ => {
                    ui.label(format!("Content for section {}", i + 1));
                }
            }
        });
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("Exclusive mode")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);

        accordion(
            ui,
            theme,
            &["Section A", "Section B", "Section C"],
            &mut state.exclusive_demo_open,
            true,
            |ui, i| {
                ui.label(format!("Content for section {} (exclusive)", i + 1));
            },
        );

        ui.add_space(theme.spacing.lg);
        ui.label(
            egui::RichText::new("Multi-open mode")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);

        accordion(
            ui,
            theme,
            &["Section X", "Section Y", "Section Z"],
            &mut state.multi_demo_open,
            false,
            |ui, i| {
                ui.label(format!("Content for section {} (multi)", i + 1));
            },
        );

        ui.add_space(theme.spacing.lg);
        ui.label(
            egui::RichText::new("Nested (accordion inside accordion)")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);

        let nested_titles = &["Layers", "Filters", "Settings"];
        accordion(
            ui,
            theme,
            nested_titles,
            &mut state.nested_outer,
            false,
            |ui, i| {
                let inner = &mut state.nested_inner[i];
                match i {
                    0 => {
                        accordion(
                            ui, theme,
                            &["Base maps", "Overlays"],
                            inner, true,
                            |ui, j| match j {
                                0 => { ui.label("Standard"); ui.label("Satellite"); }
                                1 => { ui.label("Airways"); ui.label("Weather"); }
                                _ => {}
                            },
                        );
                    }
                    1 => {
                        accordion(
                            ui, theme,
                            &["Altitude", "Type"],
                            inner, true,
                            |ui, j| match j {
                                0 => { ui.label("FL100 – FL450"); }
                                1 => { ui.label("Commercial"); ui.label("Military"); }
                                _ => {}
                            },
                        );
                    }
                    2 => {
                        accordion(
                            ui, theme,
                            &["Display", "Units"],
                            inner, true,
                            |ui, j| match j {
                                0 => { ui.label("Label size: Medium"); ui.label("Trail length: 5 min"); }
                                1 => { ui.label("Altitude: Flight Level"); ui.label("Speed: Knots"); }
                                _ => {}
                            },
                        );
                    }
                    _ => {}
                }
            },
        );
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Frost Night UI Demo",
        options,
        Box::new(|cc| Ok(Box::new(frost_night_demo::DemoApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}

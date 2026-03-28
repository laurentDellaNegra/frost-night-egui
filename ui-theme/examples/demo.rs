fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 650.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Frost Night UI Demo",
        options,
        Box::new(|cc| Ok(Box::new(ui_theme::demo::DemoApp::new(cc)))),
    )
}

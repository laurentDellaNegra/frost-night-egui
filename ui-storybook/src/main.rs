#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

mod stories;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_story(canvas_id: &str, story_name: &str) {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let story = story_name.to_string();
    let id = canvas_id.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(&id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let web_options = eframe::WebOptions::default();
        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(move |cc| Ok(Box::new(StoryApp::new(cc, story.clone())))),
            )
            .await;

        if let Err(e) = start_result {
            log::error!("Failed to start story: {e:?}");
        }
    });
}

use ui_theme::{apply_theme, Theme};

struct StoryApp {
    theme: Theme,
    story: String,
    accordion_state: stories::AccordionStoryState,
    button_state: stories::ButtonStoryState,
    input_state: stories::InputStoryState,
    checkbox_state: stories::CheckboxStoryState,
    toggle_state: stories::ToggleStoryState,
    segmented_state: stories::SegmentedStoryState,
    badge_state: stories::BadgeStoryState,
    card_state: stories::CardStoryState,
    toolbar_state: stories::ToolbarStoryState,
    separator_state: stories::SeparatorStoryState,
    drag_card_state: stories::DragCardStoryState,
}

impl StoryApp {
    fn new(cc: &eframe::CreationContext, story: String) -> Self {
        let theme = Theme::dark();
        apply_theme(&cc.egui_ctx, &theme);
        Self {
            theme,
            story,
            accordion_state: Default::default(),
            button_state: Default::default(),
            input_state: Default::default(),
            checkbox_state: Default::default(),
            toggle_state: Default::default(),
            segmented_state: Default::default(),
            badge_state: Default::default(),
            card_state: Default::default(),
            toolbar_state: Default::default(),
            separator_state: Default::default(),
            drag_card_state: Default::default(),
        }
    }
}

impl eframe::App for StoryApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let rect = ui.max_rect();
        ui.painter()
            .rect_filled(rect, 0.0, self.theme.palette.background);

        let padding = self.theme.spacing.lg;

        egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(padding);
        let available = ui.available_width();
        ui.horizontal(|ui| {
            ui.add_space(padding);
            ui.vertical(|ui| {
                ui.set_width(available - padding * 2.0);
        match self.story.as_str() {
            "accordion" => stories::accordion_story(ui, &self.theme, &mut self.accordion_state),
            "button" => stories::button_story(ui, &self.theme, &mut self.button_state),
            "input" => stories::input_story(ui, &self.theme, &mut self.input_state),
            "checkbox" => stories::checkbox_story(ui, &self.theme, &mut self.checkbox_state),
            "toggle" => stories::toggle_story(ui, &self.theme, &mut self.toggle_state),
            "segmented" => stories::segmented_story(ui, &self.theme, &mut self.segmented_state),
            "badge" => stories::badge_story(ui, &self.theme, &mut self.badge_state),
            "card" => stories::card_story(ui, &self.theme, &mut self.card_state),
            "toolbar" => stories::toolbar_story(ui, &self.theme, &mut self.toolbar_state),
            "separator" => stories::separator_story(ui, &self.theme, &mut self.separator_state),
            "drag-card" => stories::drag_card_story(ui, &self.theme, &mut self.drag_card_state),
            _ => {
                ui.label("Unknown story");
            }
        }
            });
        });
        ui.add_space(padding);
        });
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    println!("Run via WASM with trunk serve, or use web-demo for the full demo.");
}

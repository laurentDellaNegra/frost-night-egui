use ui_theme::components::{drag_card, DragCardState};
use ui_theme::Theme;

pub struct DragCardStoryState {
    pub title: String,
    pub drag_state: DragCardState,
}

impl Default for DragCardStoryState {
    fn default() -> Self {
        Self {
            title: "Drag Card".into(),
            drag_state: DragCardState {
                pos: egui::pos2(20.0, 120.0),
                size: egui::vec2(220.0, 180.0),
            },
        }
    }
}

pub fn drag_card_story(ui: &mut egui::Ui, theme: &Theme, state: &mut DragCardStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui: &mut egui::Ui| {
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label(
                egui::RichText::new("Title")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.text_edit_singleline(&mut state.title);
        });
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
    let id = egui::Id::new("story_drag_card");
    let resp = drag_card(ui, theme, id, &mut state.drag_state, &state.title, |ui: &mut egui::Ui| {
        ui.label(
            egui::RichText::new("Drag the handle at the top to move this card.")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);
        ui.label(
            egui::RichText::new("The card features a close button, drag handle animation, and border glow effect during drag.")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
    });

    if resp.closed {
        // Reset position on close
        state.drag_state.pos = egui::pos2(20.0, 120.0);
    }
    });
}

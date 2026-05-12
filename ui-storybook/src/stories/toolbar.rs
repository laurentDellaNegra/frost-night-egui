use skyscope_design_system::composites::{
    action_toolbar_with_id, toolbar_with_id, top_toolbar_with_id, zoom_toolbar_with_id,
    ActionToolbarItem, StatusField, StatusFieldKind, ToolbarAction, ToolbarGroup, ToolbarItem,
};
use skyscope_design_system::icons::*;
use skyscope_design_system::Theme;

pub struct ToolbarStoryState {
    pub selected: Option<usize>,
    pub action_selected: usize,
    pub show_top: bool,
    pub show_zoom: bool,
}

impl Default for ToolbarStoryState {
    fn default() -> Self {
        Self {
            selected: Some(0),
            action_selected: 0,
            show_top: true,
            show_zoom: true,
        }
    }
}

fn demo_groups() -> Vec<ToolbarGroup> {
    vec![
        vec![
            ToolbarItem::new(ICON_MAP),
            ToolbarItem::new(ICON_LAYERS),
            ToolbarItem::new(ICON_COMPASS),
        ],
        vec![
            ToolbarItem::new(ICON_SEARCH),
            ToolbarItem::new(ICON_FILTER),
            ToolbarItem::new(ICON_EYE),
        ],
        vec![ToolbarItem::new(ICON_SETTINGS)],
    ]
}

pub fn toolbar_story(ui: &mut egui::Ui, theme: &Theme, state: &mut ToolbarStoryState) {
    // Controls panel
    super::controls::controls_panel(ui, theme, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("Selected button")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            let mut has_selected = state.selected.is_some();
            ui.checkbox(&mut has_selected, "Has selection");
            if has_selected {
                let mut idx = state.selected.unwrap_or(0);
                ui.add(egui::Slider::new(&mut idx, 0..=6));
                state.selected = Some(idx);
            } else {
                state.selected = None;
            }
        });
        ui.checkbox(&mut state.show_top, "Show Top Toolbar");
        ui.checkbox(&mut state.show_zoom, "Show Zoom Toolbar");
    });

    // Playground
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "Playground", |ui| {
        let groups = demo_groups();
        let resp = toolbar_with_id(
            ui,
            theme,
            "toolbar-playground",
            &groups,
            state.selected,
            &[],
        );
        if let Some(clicked) = resp.clicked {
            state.selected = Some(clicked);
        }

        if state.show_top {
            ui.add_space(theme.spacing.md);
            top_toolbar_with_id(
                ui,
                theme,
                "toolbar-top-playground",
                "Skyscope",
                &[
                    StatusField {
                        label: "UTC",
                        value: "14:32:05",
                        kind: StatusFieldKind::Normal,
                    },
                    StatusField {
                        label: "QNH",
                        value: "1013",
                        kind: StatusFieldKind::Normal,
                    },
                    StatusField {
                        label: "STATUS",
                        value: "GPS degraded",
                        kind: StatusFieldKind::Warning,
                    },
                ],
                &[
                    ToolbarAction {
                        icon: ICON_GRID,
                        selected: false,
                        disabled: false,
                    },
                    ToolbarAction {
                        icon: ICON_GLOBE,
                        selected: false,
                        disabled: false,
                    },
                    ToolbarAction {
                        icon: ICON_SETTINGS,
                        selected: false,
                        disabled: false,
                    },
                ],
            );
        }

        ui.add_space(theme.spacing.md);
        let action_response = action_toolbar_with_id(
            ui,
            theme,
            "toolbar-actions-playground",
            &[
                ActionToolbarItem {
                    icon: ICON_GRID,
                    label: "Grid",
                    tooltip: "Show grid",
                    selected: state.action_selected == 0,
                    disabled: false,
                },
                ActionToolbarItem {
                    icon: ICON_FILTER,
                    label: "Filter",
                    tooltip: "Filter visible items",
                    selected: state.action_selected == 1,
                    disabled: false,
                },
                ActionToolbarItem {
                    icon: ICON_SETTINGS,
                    label: "Settings",
                    tooltip: "Open settings",
                    selected: false,
                    disabled: true,
                },
            ],
        );
        if let Some(clicked) = action_response.clicked {
            state.action_selected = clicked;
        }

        if state.show_zoom {
            ui.add_space(theme.spacing.md);
            let zoom_rect = egui::Rect::from_min_size(
                ui.cursor().min + egui::vec2(0.0, theme.spacing.xs),
                egui::vec2(44.0, 140.0),
            );
            zoom_toolbar_with_id(
                ui,
                theme,
                "toolbar-zoom-playground",
                zoom_rect,
                ICON_PLUS,
                ICON_MINUS,
            );
            ui.allocate_space(egui::vec2(44.0, 140.0 + theme.spacing.xs));
        }
    });

    // All Variants
    super::controls::section_divider(ui, theme);
    super::controls::section_frame(ui, theme, "All Variants", |ui| {
        ui.label(
            egui::RichText::new("Vertical Toolbar")
                .size(12.0)
                .color(theme.palette.muted_foreground),
        );
        ui.add_space(theme.spacing.xs);

        let gallery_groups = demo_groups();
        let gallery_resp = toolbar_with_id(
            ui,
            theme,
            "toolbar-gallery",
            &gallery_groups,
            state.selected,
            &[],
        );
        if let Some(clicked) = gallery_resp.clicked {
            state.selected = Some(clicked);
        }

        ui.add_space(theme.spacing.lg);

        // Top toolbar
        if state.show_top {
            ui.label(
                egui::RichText::new("Top Toolbar")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.add_space(theme.spacing.xs);
            top_toolbar_with_id(
                ui,
                theme,
                "toolbar-top-gallery",
                "Skyscope",
                &[
                    StatusField {
                        label: "UTC",
                        value: "14:32:05",
                        kind: StatusFieldKind::Normal,
                    },
                    StatusField {
                        label: "QNH",
                        value: "1013",
                        kind: StatusFieldKind::Normal,
                    },
                    StatusField {
                        label: "TL",
                        value: "FL60",
                        kind: StatusFieldKind::Normal,
                    },
                ],
                &[
                    ToolbarAction {
                        icon: ICON_GRID,
                        selected: false,
                        disabled: false,
                    },
                    ToolbarAction {
                        icon: ICON_GLOBE,
                        selected: false,
                        disabled: false,
                    },
                    ToolbarAction {
                        icon: ICON_SETTINGS,
                        selected: false,
                        disabled: false,
                    },
                ],
            );
        }

        ui.add_space(theme.spacing.lg);

        // Zoom toolbar
        if state.show_zoom {
            ui.label(
                egui::RichText::new("Zoom Toolbar")
                    .size(12.0)
                    .color(theme.palette.muted_foreground),
            );
            ui.add_space(theme.spacing.xs);
            let zoom_rect = egui::Rect::from_min_size(
                ui.cursor().min + egui::vec2(0.0, theme.spacing.xs),
                egui::vec2(44.0, 140.0),
            );
            zoom_toolbar_with_id(
                ui,
                theme,
                "toolbar-zoom-gallery",
                zoom_rect,
                ICON_PLUS,
                ICON_MINUS,
            );
            ui.allocate_space(egui::vec2(44.0, 140.0 + theme.spacing.xs));
        }
    });
}

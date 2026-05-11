# frost-night-egui

Frost Night is an egui theme and component library for dark, glass-tinted operational interfaces.

The Rust package is `frost-night-egui`, imported from Rust as `frost_night_egui`.

## Overview

The reusable library owns the Frost Night visual language:

- semantic color palette and interaction tokens
- spacing, radius, control size, and typography scales
- egui `Visuals` integration
- optional Lucide icon font installation
- generic components and containers
- optional higher-level composites for demos and prototypes

Application-specific behavior belongs in the consuming app: domain logic, operational validation, keyboard shortcuts, map/menu data, docking orchestration, and renderer-specific real blur.

## Modules

| Module | Feature | Purpose |
| --- | --- | --- |
| `theme` | core | `Theme`, palette, tokens, scales, `apply_visuals`, `install_theme` |
| `components` | core | `button`, `text_input`, `text_edit`, `checkbox`, `toggle`, `segmented`, `segmented_styled`, `badge`, `separator`, `FrostUiExt` |
| `containers` | core | `card`, `surface`, `tabs`, `accordion`, `drag_card` |
| `effects` | core | `BlurRect` and fallback glass/tint painting |
| `icons` | `icons` | Lucide font data, install helpers, icon constants |
| `composites` | `composites` | optional toolbar/sidebar/zoom/action toolbar compositions |

## Features

```toml
[dependencies]
frost-night-egui = { path = "ui-theme" }
```

Available features:

- `default = ["icons"]`
- `icons`: embedded Lucide icon font helpers and constants
- `composites`: optional toolbar/sidebar/zoom/action toolbar compositions, depends on `egui_flex`
- `serde`: derives serde support for theme data where available

The core crate compiles without default features:

```sh
cargo check -p frost-night-egui --no-default-features
```

Common repository commands are available through the root script:

```sh
./run.sh ci
./run.sh demo
./run.sh wasm-demo
./run.sh wasm-storybook
```

## Integration

```rust
use frost_night_egui::{
    install_theme, ControlSize, ControlVariant, InstallThemeOptions, Theme,
};
use frost_night_egui::components::*;

pub struct App {
    theme: Theme,
    name: String,
}

impl App {
    pub fn new(ctx: &egui::Context) -> Self {
        let theme = Theme::dark();

        install_theme(
            ctx,
            &theme,
            InstallThemeOptions {
                install_visuals: true,
                install_fonts: true,
            },
        );

        Self {
            theme,
            name: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        button(
            ui,
            &self.theme,
            "Acknowledge",
            ControlVariant::Primary,
            ControlSize::Md,
        );

        text_input(ui, &self.theme, &mut self.name, ControlSize::Md);
    }
}
```

Install only visuals when the host application owns all fonts:

```rust
install_theme(
    ctx,
    &theme,
    InstallThemeOptions {
        install_visuals: true,
        install_fonts: false,
    },
);
```

Host apps with custom fonts should add Lucide to their own font definitions before calling `ctx.set_fonts`:

```rust
let mut fonts = egui::FontDefinitions::default();
frost_night_egui::add_icon_font_to(&mut fonts);
ctx.set_fonts(fonts);
```

`install_icon_font(ctx)` is also available with the `icons` feature, but egui does not expose the context's existing `FontDefinitions`; it installs into default font definitions and may replace host custom font setup.

## Components

```rust
use frost_night_egui::{ControlSize, ControlVariant, Theme};
use frost_night_egui::components::{
    badge, button, checkbox, segmented, segmented_with_fills, text_edit, text_input, toggle,
    BadgeVariant,
};
use frost_night_egui::containers::{accordion, accordion_with_id, card, tabs, tabs_with_id};

button(ui, &theme, "Primary", ControlVariant::Primary, ControlSize::Md);
text_input(ui, &theme, &mut text, ControlSize::Md);
text_edit(
    ui,
    &theme,
    egui::TextEdit::singleline(&mut text).hint_text("Search"),
    ControlSize::Md,
);
checkbox(ui, &theme, &mut checked, "Enable feature");
toggle(ui, &theme, &mut enabled);
segmented(ui, &theme, &["One", "Two"], &mut selected);
segmented_with_fills(
    ui,
    &theme,
    &["Low", "Medium", "High"],
    &[theme.palette.primary, theme.palette.accent, theme.palette.destructive],
    &mut selected,
);
badge(ui, &theme, "Online", BadgeVariant::Accent);

tabs(ui, &theme, &mut selected_tab, &["Layers", "Filters"]);
accordion(ui, &theme, &["Section"], &mut open, false, |ui, index| {
    ui.label(format!("Body {index}"));
});
card(ui, &theme, |ui| {
    ui.label("Panel content");
});
```

For multiple stateful instances under the same parent `Ui`, use the ID-salted variants:

```rust
tabs_with_id(ui, &theme, "left-panel-tabs", &mut selected, &labels);
accordion_with_id(ui, &theme, "settings-accordion", &items, &mut open, false, |ui, i| {
    ui.label(format!("Item {i}"));
});
```

## Optional Composites

Enable composites when you want the demo-style toolbar building blocks:

```toml
frost-night-egui = { path = "ui-theme", features = ["composites"] }
```

`top_toolbar` is generic. The consuming app supplies domain labels such as QNH or TL:

```rust
use frost_night_egui::composites::{top_toolbar, StatusField, StatusFieldKind, ToolbarAction};
use frost_night_egui::icons::{ICON_GRID, ICON_SETTINGS};

let fields = [
    StatusField { label: "UTC", value: "23:14:20", kind: StatusFieldKind::Normal },
    StatusField { label: "QNH", value: "1016", kind: StatusFieldKind::Normal },
];
let actions = [
    ToolbarAction { icon: ICON_GRID, selected: false, disabled: false },
    ToolbarAction { icon: ICON_SETTINGS, selected: false, disabled: false },
];

let response = top_toolbar(ui, &theme, "Frost Night", &fields, &actions);
```

`action_toolbar` renders compact labeled actions and requires `features = ["composites", "icons"]`:

```rust
use frost_night_egui::composites::{action_toolbar, ActionToolbarItem};
use frost_night_egui::icons::{ICON_FILTER, ICON_GRID};

let response = action_toolbar(
    ui,
    &theme,
    &[
        ActionToolbarItem {
            icon: ICON_GRID,
            label: "Grid",
            tooltip: "Show grid",
            selected: true,
            disabled: false,
        },
        ActionToolbarItem {
            icon: ICON_FILTER,
            label: "Filter",
            tooltip: "Filter items",
            selected: false,
            disabled: false,
        },
    ],
);
```

## Running The Demo

The demo is a separate private workspace crate so sample aviation data and demo
orchestration do not leak into the reusable `frost-night-egui` library API.

Native:

```sh
cargo run -p frost-night-demo
```

Web:

```sh
cd web-demo
trunk serve
```

The web launcher depends on `frost-night-demo`, which depends on
`frost-night-egui` with the `composites` feature enabled.

## Project Structure

```text
frost-night-demo/
  src/          # DemoApp and private sample aviation/menu data

ui-theme/src/
  theme/        # palette, tokens, scales, visuals/style integration
  components/   # generic controls
  containers/   # cards, tabs, accordion, surfaces
  effects/      # glass/tint/blur intent
  icons/        # optional Lucide integration
  composites/   # optional composed tool surfaces
```

## Design Tokens

See [DESIGN_TOKENS.md](DESIGN_TOKENS.md) for the extracted color palette, spacing, and component specifications.

## License

MIT. See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

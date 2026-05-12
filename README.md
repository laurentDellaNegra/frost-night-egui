# skyscope-design-system

Skyscope Design System is an egui theme and component library for dark, glass-tinted operational interfaces.

The Rust package is `skyscope-design-system`, imported from Rust as `skyscope_design_system`.

## Overview

The reusable library owns the Skyscope Design System visual language:

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
| `components` | core | `button`, `text_input`, `text_edit`, `checkbox`, `toggle`, `segmented`, `segmented_styled`, `badge`, `separator`, `SkyscopeUiExt` |
| `containers` | core | `card`, `surface`, `tabs`, `accordion`, `drag_card` |
| `effects` | core | `BlurRect` and fallback glass/tint painting |
| `icons` | `icons` | Lucide font data, install helpers, icon constants |
| `composites` | `composites` | optional toolbar/sidebar/zoom/action toolbar compositions |

## Features

```toml
[dependencies]
skyscope-design-system = { path = "ui-theme" }
```

Available features:

- `default = ["icons"]`
- `icons`: embedded Lucide icon font helpers and constants
- `composites`: optional toolbar/sidebar/zoom/action toolbar compositions, depends on `egui_flex`
- `serde`: derives serde support for theme data where available

The core crate compiles without default features:

```sh
cargo check -p skyscope-design-system --no-default-features
```

## Repository Scripts

Common repository commands are available through the root script:

| Command | Purpose |
| --- | --- |
| `./run.sh check` | Check the Rust workspace with `cargo check --workspace`. |
| `./run.sh wasm-check` | Check `web-demo` and `ui-storybook` for `wasm32-unknown-unknown`. |
| `./run.sh fmt` | Check Rust formatting with `cargo fmt --all -- --check`. |
| `./run.sh clippy` | Run clippy for the workspace with all targets and features. |
| `./run.sh ci` | Run the Rust CI sequence: formatting, no-default-features check, workspace check, all-features check, WASM checks, and clippy. |
| `./run.sh demo` | Run the native Skyscope Design System demo. |
| `./run.sh demo-debug` | Run the native demo with `RUST_BACKTRACE=1` and `RUST_LOG=warn`. |
| `./run.sh tokens` | Generate `docs-site/src/styles/tokens.css` from the Rust theme palette. |
| `./run.sh wasm-storybook` | Build the interactive storybook WASM bundle into `docs-site/public/wasm`. |
| `./run.sh wasm-demo` | Build the demo WASM bundle into `docs-site/public/demo`. |
| `./run.sh dev` | Start the Astro documentation dev server. |
| `./run.sh site` | Build the Astro documentation site. |
| `./run.sh preview` | Preview the production Astro documentation build. |
| `./run.sh build` | Run the full docs pipeline: tokens, storybook WASM, demo WASM, then Astro build. |
| `./run.sh clean` | Remove Rust target output and generated docs build artifacts. |

The documentation site also exposes its local npm scripts from `docs-site/package.json`:

| Command | Purpose |
| --- | --- |
| `cd docs-site && npm run dev` | Start Astro in development mode. |
| `cd docs-site && npm run build` | Build the static documentation site. |
| `cd docs-site && npm run preview` | Preview the built documentation site locally. |

## External Dependencies

Direct Rust dependencies are defined in the workspace manifests:

| Dependency | Version | Used For |
| --- | --- | --- |
| `egui` | `0.34` | Core immediate-mode UI types, widgets, styling, and rendering integration. |
| `eframe` | `0.34` | Native and web app shell for the demo and storybook crates. |
| `egui_flex` | `0.6` | Optional composite layouts behind the `composites` feature. |
| `serde` | `1` | Optional theme serialization support through the `serde` feature. |
| `env_logger` | `0.11` | Native demo logging initialization. |
| `log` | `0.4` | Logging facade used by web/demo crates. |
| `wasm-bindgen` | `0.2` | WebAssembly bindings for browser builds. |
| `wasm-bindgen-futures` | `0.4` | Async bridge support for WebAssembly builds. |
| `web-sys` | `0.3` | Browser DOM types used by WASM launchers. |

Direct documentation-site dependencies are defined in `docs-site/package.json`:

| Dependency | Version | Used For |
| --- | --- | --- |
| `astro` | `^6.1.1` | Static documentation site framework. |
| `@astrojs/mdx` | `^5.0.3` | MDX support for component and foundation docs. |

Embedded third-party assets:

| Asset | Version | License | Used For |
| --- | --- | --- | --- |
| Lucide icon font | `1.7.0` | ISC | Optional icon font embedded at `ui-theme/src/fonts/lucide.ttf`. |

Required local tooling for repository scripts:

| Tool | Used By |
| --- | --- |
| Rust toolchain with `cargo`, `cargo fmt`, and `cargo clippy` | Rust checks, examples, demos, and CI scripts. |
| `wasm32-unknown-unknown` Rust target | `./run.sh wasm-check`, `./run.sh wasm-demo`, and `./run.sh wasm-storybook`. |
| Trunk | WASM demo and storybook builds. |
| Node.js and npm | Astro documentation scripts. |

## Integration

```rust
use skyscope_design_system::{
    install_theme, ControlSize, ControlVariant, InstallThemeOptions, Theme,
};
use skyscope_design_system::components::*;

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
skyscope_design_system::add_icon_font_to(&mut fonts);
ctx.set_fonts(fonts);
```

`install_icon_font(ctx)` is also available with the `icons` feature, but egui does not expose the context's existing `FontDefinitions`; it installs into default font definitions and may replace host custom font setup.

## Components

```rust
use skyscope_design_system::{ControlSize, ControlVariant, Theme};
use skyscope_design_system::components::{
    badge, button, checkbox, segmented, segmented_with_fills, text_edit, text_input, toggle,
    BadgeVariant,
};
use skyscope_design_system::containers::{accordion, accordion_with_id, card, tabs, tabs_with_id};

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
skyscope-design-system = { path = "ui-theme", features = ["composites"] }
```

`top_toolbar` is generic. The consuming app supplies domain labels such as QNH or TL:

```rust
use skyscope_design_system::composites::{top_toolbar, StatusField, StatusFieldKind, ToolbarAction};
use skyscope_design_system::icons::{ICON_GRID, ICON_SETTINGS};

let fields = [
    StatusField { label: "UTC", value: "23:14:20", kind: StatusFieldKind::Normal },
    StatusField { label: "QNH", value: "1016", kind: StatusFieldKind::Normal },
];
let actions = [
    ToolbarAction { icon: ICON_GRID, selected: false, disabled: false },
    ToolbarAction { icon: ICON_SETTINGS, selected: false, disabled: false },
];

let response = top_toolbar(ui, &theme, "Skyscope Design System", &fields, &actions);
```

`action_toolbar` renders compact labeled actions and requires `features = ["composites", "icons"]`:

```rust
use skyscope_design_system::composites::{action_toolbar, ActionToolbarItem};
use skyscope_design_system::icons::{ICON_FILTER, ICON_GRID};

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
orchestration do not leak into the reusable `skyscope-design-system` library API.

Native:

```sh
cargo run -p skyscope-design-system-demo
```

Web:

```sh
cd web-demo
trunk serve
```

The web launcher depends on `skyscope-design-system-demo`, which depends on
`skyscope-design-system` with the `composites` feature enabled.

## Project Structure

```text
skyscope-design-system-demo/
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

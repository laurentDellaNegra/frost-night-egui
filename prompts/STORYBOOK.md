# Claude Code Prompt: Design System Documentation Site for frost-night-egui

## Context

I have an egui design system crate called `ui-theme` inside the `frost-night-egui` repo. It provides a dark theme, design tokens, and 14 component wrappers for aviation-style UIs. There is already a working demo (`DemoApp` in `ui-theme/src/demo.rs`, gated behind the `demo` feature) deployed to GitHub Pages as a full-screen WASM app via `web-demo/`.

I want to add a **design system documentation site** alongside the existing demo — both served from the same GitHub Pages deployment. The site should showcase foundations (colors, typography, spacing, radius, blur) and each component with interactive playgrounds (like Storybook).

## Existing Architecture (DO NOT CHANGE)

```
frost-night-egui/
├── .github/workflows/deploy.yml    # Current: builds web-demo, deploys to GH Pages
├── CLAUDE.md                       # Project conventions
├── DESIGN_TOKENS.md                # Extracted design tokens
├── mockups/                        # Figma screenshots
├── prompts/                        # Claude Code prompts
├── ui-theme/                       # The library crate (egui 0.34, no workspace)
│   ├── Cargo.toml                  # features: demo (requires eframe), serde
│   ├── src/
│   │   ├── lib.rs
│   │   ├── palette.rs              # ColorPalette with dark() constructor
│   │   ├── tokens.rs               # StateColors, VariantTokens, mix()
│   │   ├── scale.rs                # RadiusScale, SpacingScale, ControlSize, ControlVariant
│   │   ├── theme.rs                # Theme struct
│   │   ├── helpers.rs              # apply_theme()
│   │   ├── blur.rs                 # BlurRect (fallback, no shader yet)
│   │   ├── oklch.rs                # OKLCH utilities
│   │   ├── icons.rs                # Lucide font embed + constants (ICON_MAP, ICON_LAYERS, etc.)
│   │   ├── fonts/lucide.ttf
│   │   ├── demo.rs                 # DemoApp (behind "demo" feature)
│   │   └── components/
│   │       ├── mod.rs
│   │       ├── button.rs           # button()
│   │       ├── input.rs            # text_input()
│   │       ├── checkbox.rs         # checkbox()
│   │       ├── toggle.rs           # toggle()
│   │       ├── segmented.rs        # segmented()
│   │       ├── badge.rs            # badge(), BadgeVariant
│   │       ├── card.rs             # card()
│   │       ├── drag_card.rs        # drag_card(), DragCardResponse, DragCardState
│   │       ├── sidebar_card.rs     # sidebar_card(), SidebarCardResponse
│   │       ├── toolbar.rs          # toolbar(), ToolbarItem, ToolbarGroup, ToolbarResponse
│   │       ├── top_toolbar.rs      # top_toolbar(), TopToolbarResponse
│   │       ├── zoom_toolbar.rs     # zoom_toolbar(), ZoomToolbarResponse
│   │       └── separator.rs        # separator()
│   └── examples/
│       └── demo.rs                 # Native entry (~10 lines, calls DemoApp)
└── web-demo/                       # WASM entry for the demo
    ├── Cargo.toml                  # depends on ui-theme with demo feature, eframe glow backend
    ├── index.html                  # Canvas with id="the_canvas_id"
    └── src/main.rs                 # ~40 lines, starts DemoApp on canvas
```

### Critical technical details

- **eframe 0.34**: `App` trait uses `fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame)`, NOT `fn update()`.
- **glow backend** for WASM (not wgpu): `eframe = { features = ["glow"] }`. No custom shaders possible in WASM. Blur is a semi-transparent tint fallback.
- **WebRunner takes HtmlCanvasElement** directly, not a string ID. You must `get_element_by_id().dyn_into::<HtmlCanvasElement>()`.
- **No Cargo workspace** — `ui-theme` and `web-demo` are independent crates. `web-demo` depends on `ui-theme` via `path = "../ui-theme"`.
- **`DemoApp::new(cc)` takes `&eframe::CreationContext`**, calls `apply_theme()` and `load_icon_font()` internally.
- **`--public-url ./`** is required for trunk builds when deploying to GitHub Pages (relative asset paths).
- **Component API pattern**: `fn component(ui: &mut Ui, theme: &Theme, ...) -> Response`.
- **Icons**: `load_icon_font(ctx)` is called by `apply_theme()`. Use `icon_text(ICON_NAME, size)` for `RichText`.

## What to Build

### New directories to add:

```
frost-night-egui/
├── ui-storybook/                   # NEW — WASM crate for component stories
│   ├── Cargo.toml
│   ├── index.html
│   └── src/
│       ├── main.rs                 # wasm_bindgen entry: start_story(canvas_id, story_name)
│       └── stories/
│           ├── mod.rs
│           ├── controls.rs         # Reusable controls_panel() helper
│           ├── button.rs           # ButtonStoryState + gallery + playground
│           ├── input.rs
│           ├── checkbox.rs
│           ├── toggle.rs
│           ├── segmented.rs
│           ├── badge.rs
│           ├── card.rs
│           ├── toolbar.rs          # Covers toolbar, top_toolbar, zoom_toolbar
│           ├── drag_card.rs
│           └── separator.rs
└── docs-site/                      # NEW — Astro documentation site
    ├── astro.config.mjs
    ├── package.json
    ├── public/
    │   ├── wasm/                   # Trunk output: storybook WASM
    │   └── demo/                   # Trunk output: existing demo WASM
    ├── src/
    │   ├── layouts/
    │   │   ├── DocsLayout.astro    # Sidebar + content
    │   │   └── DemoLayout.astro    # Full-width, no sidebar
    │   ├── components/
    │   │   ├── Sidebar.astro
    │   │   ├── ColorSwatch.astro
    │   │   ├── TokenTable.astro
    │   │   ├── SpacingScale.astro
    │   │   ├── RadiusScale.astro
    │   │   ├── TypographyScale.astro
    │   │   ├── CodeBlock.astro
    │   │   ├── ComponentPreview.astro   # Loads storybook WASM story in <canvas>
    │   │   ├── DemoEmbed.astro          # Loads full demo WASM
    │   │   ├── PropsTable.astro
    │   │   └── PageHeader.astro
    │   ├── styles/
    │   │   ├── tokens.css          # Generated from Rust ColorPalette
    │   │   ├── global.css
    │   │   └── code.css
    │   ├── content/docs/
    │   │   ├── index.mdx
    │   │   ├── demo.mdx            # Full-screen demo embed
    │   │   ├── foundations/
    │   │   │   ├── colors.mdx
    │   │   │   ├── typography.mdx
    │   │   │   ├── spacing.mdx
    │   │   │   ├── radius.mdx
    │   │   │   └── blur.mdx
    │   │   └── components/
    │   │       ├── button.mdx
    │   │       ├── input.mdx
    │   │       ├── checkbox.mdx
    │   │       ├── toggle.mdx
    │   │       ├── segmented.mdx
    │   │       ├── badge.mdx
    │   │       ├── card.mdx
    │   │       ├── drag-card.mdx
    │   │       ├── toolbar.mdx
    │   │       ├── separator.mdx
    │   │       └── sidebar-card.mdx
    │   └── pages/
    │       └── [...slug].astro
    └── scripts/
        └── generate-tokens-css.rs
```

## Part 1: CSS Token Generation

Add an example to `ui-theme` that dumps `ColorPalette::dark()` as CSS variables:

```rust
// ui-theme/examples/export_css.rs
// (add to Cargo.toml: [[example]] name = "export_css")

use ui_theme::{Theme, ColorPalette};
use egui::Color32;

fn c(color: Color32) -> String {
    if color.a() == 255 {
        format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b())
    } else {
        format!("rgba({}, {}, {}, {:.2})", color.r(), color.g(), color.b(), color.a() as f32 / 255.0)
    }
}

fn main() {
    let theme = Theme::dark();
    let p = &theme.palette;
    let r = &theme.radius;
    let s = &theme.spacing;
    println!(":root {{");
    println!("  --background: {};", c(p.background));
    println!("  --foreground: {};", c(p.foreground));
    println!("  --primary: {};", c(p.primary));
    println!("  --primary-foreground: {};", c(p.primary_foreground));
    // ... all palette fields
    println!("  --radius-sm: {}px;", r.sm);
    println!("  --radius-md: {}px;", r.md);
    println!("  --radius-lg: {}px;", r.lg);
    println!("  --spacing-xs: {}px;", s.xs);
    println!("  --spacing-sm: {}px;", s.sm);
    println!("  --spacing-md: {}px;", s.md);
    println!("  --spacing-lg: {}px;", s.lg);
    println!("  --spacing-xl: {}px;", s.xl);
    println!("}}");
}
```

Run: `cd ui-theme && cargo run --example export_css > ../docs-site/src/styles/tokens.css`

## Part 2: Storybook WASM Crate

### `ui-storybook/Cargo.toml`

```toml
[package]
name = "ui-storybook"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
ui-theme = { path = "../ui-theme", features = ["demo"] }
eframe = { version = "0.34", default-features = false, features = [
    "default_fonts",
    "glow",
] }
log = "0.4"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Document", "Window", "HtmlCanvasElement", "Element"] }
```

### `ui-storybook/index.html`

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Frost Night Storybook</title>
    <link data-trunk rel="rust" data-wasm-opt="z" />
  </head>
  <body></body>
</html>
```

### Story architecture

Each story has a **state struct** (persists across frames) and two render functions:

- **gallery**: static grid of all variants/sizes — for overview sections
- **playground**: interactive controls panel + live single-component preview — like Storybook Controls

```rust
// stories/button.rs
use ui_theme::{Theme, ControlVariant, ControlSize};
use ui_theme::components::button;

pub struct ButtonStoryState {
    pub variant: ControlVariant,
    pub size: ControlSize,
    pub label: String,
    pub enabled: bool,
}

impl Default for ButtonStoryState {
    fn default() -> Self {
        Self {
            variant: ControlVariant::Primary,
            size: ControlSize::Md,
            label: "Click me".into(),
            enabled: true,
        }
    }
}

pub fn button_gallery(ui: &mut egui::Ui, theme: &Theme, _state: &mut ButtonStoryState) {
    // All variants in a row, all sizes, disabled state
}

pub fn button_playground(ui: &mut egui::Ui, theme: &Theme, state: &mut ButtonStoryState) {
    super::controls::controls_panel(ui, theme, |ui| {
        // ComboBox for variant, ComboBox for size, checkbox for enabled, text_edit for label
    });
    ui.set_enabled(state.enabled);
    button(ui, theme, &state.label, state.variant, state.size);
}
```

### Controls panel helper

```rust
// stories/controls.rs
pub fn controls_panel(ui: &mut egui::Ui, theme: &Theme, add_controls: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::NONE
        .fill(theme.palette.muted)
        .inner_margin(egui::Margin::same(theme.spacing.md))
        .corner_radius(theme.radius.md)
        .stroke(egui::Stroke::new(1.0, theme.palette.border))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(theme.spacing.sm, theme.spacing.xs + 2.0);
            add_controls(ui);
        });
    ui.add_space(theme.spacing.lg);
}
```

### Controls → prop type mapping (apply consistently)

| Rust type                                         | egui control                                 |
| ------------------------------------------------- | -------------------------------------------- |
| enum (ControlVariant, ControlSize, BadgeVariant…) | ComboBox with selectable_value               |
| bool                                              | ui.checkbox()                                |
| String                                            | ui.text_edit_singleline()                    |
| f32                                               | egui::Slider                                 |
| usize (index)                                     | egui::Slider or ComboBox                     |
| Color32                                           | ui.color_edit_button_srgba()                 |
| Option<T>                                         | checkbox to toggle Some/None + inner control |

### Entry point (WASM)

```rust
// ui-storybook/src/main.rs
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

use ui_theme::{Theme, apply_theme};

struct StoryApp {
    theme: Theme,
    story: String,
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
        ui.painter().rect_filled(rect, 0.0, self.theme.palette.background);

        match self.story.as_str() {
            "button"              => stories::button_gallery(ui, &self.theme, &mut self.button_state),
            "button-playground"   => stories::button_playground(ui, &self.theme, &mut self.button_state),
            "input"               => stories::input_gallery(ui, &self.theme, &mut self.input_state),
            "input-playground"    => stories::input_playground(ui, &self.theme, &mut self.input_state),
            "checkbox"            => stories::checkbox_gallery(ui, &self.theme, &mut self.checkbox_state),
            "checkbox-playground" => stories::checkbox_playground(ui, &self.theme, &mut self.checkbox_state),
            "toggle"              => stories::toggle_gallery(ui, &self.theme, &mut self.toggle_state),
            "toggle-playground"   => stories::toggle_playground(ui, &self.theme, &mut self.toggle_state),
            "segmented"           => stories::segmented_gallery(ui, &self.theme, &mut self.segmented_state),
            "segmented-playground"=> stories::segmented_playground(ui, &self.theme, &mut self.segmented_state),
            "badge"               => stories::badge_gallery(ui, &self.theme, &mut self.badge_state),
            "badge-playground"    => stories::badge_playground(ui, &self.theme, &mut self.badge_state),
            "card"                => stories::card_gallery(ui, &self.theme, &mut self.card_state),
            "card-playground"     => stories::card_playground(ui, &self.theme, &mut self.card_state),
            "toolbar"             => stories::toolbar_gallery(ui, &self.theme, &mut self.toolbar_state),
            "toolbar-playground"  => stories::toolbar_playground(ui, &self.theme, &mut self.toolbar_state),
            "separator"           => stories::separator_gallery(ui, &self.theme, &mut self.separator_state),
            "drag-card"           => stories::drag_card_gallery(ui, &self.theme, &mut self.drag_card_state),
            "drag-card-playground"=> stories::drag_card_playground(ui, &self.theme, &mut self.drag_card_state),
            _ => { ui.label("Unknown story"); }
        }
    }
}

// Native entry for testing
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Run via WASM with trunk serve, or use web-demo for the full demo.");
}
```

## Part 3: Astro Site

### ComponentPreview.astro

```astro
---
interface Props {
  story: string;
  height?: string;
  title?: string;
}
const { story, height = "250px", title } = Astro.props;
const canvasId = `story-${story}-${Math.random().toString(36).slice(2, 8)}`;
---

<div class="component-preview">
  {title && <span class="preview-label">{title}</span>}
  <div class="preview-canvas-wrapper" style={`height: ${height}`}>
    <canvas id={canvasId} class="story-canvas" data-story={story}></canvas>
  </div>
</div>

<style>
  .component-preview {
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    margin: 1.5rem 0;
  }
  .preview-label {
    display: block;
    padding: var(--spacing-sm) var(--spacing-md);
    font-size: 0.8rem;
    color: var(--muted-foreground);
    border-bottom: 1px solid var(--border);
  }
  .preview-canvas-wrapper {
    position: relative;
    background: var(--background);
  }
  .story-canvas {
    width: 100% !important;
    height: 100% !important;
  }
</style>

<script>
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(async (entry) => {
      if (!entry.isIntersecting) return;
      const canvas = entry.target as HTMLCanvasElement;
      const story = canvas.dataset.story;
      if (canvas.dataset.loaded) return;
      canvas.dataset.loaded = "true";

      // Base path matches astro.config.mjs base
      const wasm = await import('/frost-night-egui/wasm/ui-storybook.js');
      await wasm.default();
      wasm.start_story(canvas.id, story);

      observer.unobserve(canvas);
    });
  }, { threshold: 0.1 });

  document.querySelectorAll('.story-canvas').forEach(c => observer.observe(c));
</script>
```

### DemoEmbed.astro

Loads the existing `web-demo` WASM in a large canvas. Requires a `start_demo(canvas_id)` wasm_bindgen export added to `web-demo/src/main.rs`:

```astro
---
interface Props { height?: string; }
const { height = "85vh" } = Astro.props;
---

<div class="demo-embed">
  <div class="demo-canvas-wrapper" style={`height: ${height}`}>
    <canvas id="the_canvas_id" class="demo-canvas"></canvas>
  </div>
</div>

<style>
  .demo-embed {
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }
  .demo-canvas-wrapper {
    position: relative;
    background: var(--background);
  }
  .demo-canvas {
    width: 100% !important;
    height: 100% !important;
  }
</style>

<script>
  async function loadDemo() {
    const wasm = await import('/frost-night-egui/demo/web-demo.js');
    await wasm.default();
    // web-demo main() targets canvas id "the_canvas_id" — we use the same id
  }
  loadDemo();
</script>
```

### Navigation structure

```
🚀 Live Demo                    ← accent-colored, prominent, links to /demo

Foundations
  Colors
  Typography
  Spacing
  Radius
  Blur / Surfaces

Components
  Button
  Text Input
  Checkbox
  Toggle
  Segmented
  Badge
  Card
  Drag Card
  Sidebar Card
  Toolbar
  Separator
```

### MDX page template (example: button.mdx)

```mdx
---
title: Button
description: A clickable control with 6 variants and 3 sizes.
---

import ComponentPreview from "../../components/ComponentPreview.astro";
import PropsTable from "../../components/PropsTable.astro";
import CodeBlock from "../../components/CodeBlock.astro";

# Button

A clickable control that triggers an action. Supports Primary, Secondary, Ghost, Outline, Destructive, and Link variants.

## Playground

<ComponentPreview story="button-playground" height="250px" />

## Gallery

<ComponentPreview
  story="button"
  height="200px"
  title="All variants and sizes"
/>

## Usage

<CodeBlock code={`
use ui_theme::components::button;
use ui_theme::tokens::ControlVariant;
use ui_theme::scale::ControlSize;

button(ui, &theme, "Click me", ControlVariant::Primary, ControlSize::Md);
`} />

## Props

<PropsTable
  props={[
    { name: "ui", type: "&mut Ui", description: "The egui UI context" },
    { name: "theme", type: "&Theme", description: "Theme instance" },
    {
      name: "label",
      type: "impl Into<WidgetText>",
      description: "Button text",
    },
    {
      name: "variant",
      type: "ControlVariant",
      default: "Primary",
      description: "Visual variant",
    },
    {
      name: "size",
      type: "ControlSize",
      default: "Md",
      description: "Button size",
    },
  ]}
/>
```

### `astro.config.mjs`

```js
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";

export default defineConfig({
  site: "https://laurentdellanegra.github.io",
  base: "/frost-night-egui",
  integrations: [mdx()],
});
```

## Part 4: Deployment — CRITICAL

The current deployment at `https://laurentdellanegra.github.io/frost-night-egui/` serves the web-demo WASM directly. The new deployment must serve **both** the Astro docs site AND the demo WASM from the same GitHub Pages.

### Strategy: Astro is the root, demo + storybook are WASM assets

The Astro site becomes the root (`/frost-night-egui/`). The demo and storybook WASM are built by Trunk into Astro's `public/` folder, then included in the Astro build output.

### Base URL handling

GitHub Pages serves at `/frost-night-egui/`. This affects:

- **Astro**: `base: '/frost-night-egui'` in `astro.config.mjs`
- **Trunk (storybook)**: build with `--public-url /frost-night-egui/wasm/`
- **Trunk (demo)**: build with `--public-url /frost-night-egui/demo/`
- **WASM imports in Astro components**: use absolute paths starting with `/frost-night-egui/`

### Updated `.github/workflows/deploy.yml`

```yaml
name: Deploy docs + demo to GitHub Pages

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install trunk
        uses: jetli/trunk-action@v0.5.0
        with:
          version: latest

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            ui-theme/target
            web-demo/target
            ui-storybook/target
          key: wasm-${{ hashFiles('web-demo/Cargo.lock', 'ui-storybook/Cargo.lock', 'ui-theme/Cargo.toml') }}
          restore-keys: wasm-

      # 1. Generate CSS tokens from Rust palette
      - name: Generate CSS tokens
        run: cd ui-theme && cargo run --example export_css > ../docs-site/src/styles/tokens.css

      # 2. Build storybook WASM → docs-site/public/wasm/
      - name: Build storybook WASM
        run: cd ui-storybook && trunk build --release --public-url /frost-night-egui/wasm/ --dist ../docs-site/public/wasm

      # 3. Build demo WASM → docs-site/public/demo/
      - name: Build demo WASM
        run: cd web-demo && trunk build --release --public-url /frost-night-egui/demo/ --dist ../docs-site/public/demo

      # 4. Build Astro site (includes wasm + demo from public/)
      - name: Build Astro site
        run: |
          cd docs-site
          npm ci
          npm run build

      # 5. Upload final artifact (Astro output contains everything)
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: docs-site/dist

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

### Local development

```bash
# 1. Generate CSS tokens
cd ui-theme && cargo run --example export_css > ../docs-site/src/styles/tokens.css

# 2. Build storybook WASM
cd ui-storybook && trunk build --release --public-url /frost-night-egui/wasm/ --dist ../docs-site/public/wasm

# 3. Build demo WASM
cd web-demo && trunk build --release --public-url /frost-night-egui/demo/ --dist ../docs-site/public/demo

# 4. Dev server
cd docs-site && npm run dev
```

For local dev without the base path issue, you can temporarily set `base: '/'` in astro.config.mjs and use `--public-url ./` for trunk builds.

## Constraints

- **Do NOT modify existing crates** (`ui-theme/`, `web-demo/`) unless strictly necessary (e.g. adding the `export_css` example to ui-theme).
- **Two WASM bundles**: `ui-storybook.wasm` (stories) + `web-demo.wasm` (full demo). No more.
- **eframe 0.34 + glow backend** for both WASM crates. No wgpu.
- **`fn ui()` not `fn update()`** — this is eframe 0.34 API.
- **`WebRunner::start()` takes `HtmlCanvasElement`**, not a string ID.
- **No frontend framework** — Astro components + vanilla JS only.
- **Dark mode only** — site uses the same Frost Night tokens via CSS variables.
- **Lazy-load WASM** via IntersectionObserver in ComponentPreview.
- **All spacing via CSS variables** from tokens.css — no hardcoded pixel values in the site CSS.
- **GitHub Pages base path** `/frost-night-egui/` must be handled correctly in Astro config, Trunk public-url, and WASM import paths.
- **Keep demo accessible** — the demo must work at `/frost-night-egui/demo/` and be embeddable in the docs site.
- **Same deployment pattern** — use `actions/upload-pages-artifact@v3` + `actions/deploy-pages@v4` with proper permissions (matches current workflow).

## What NOT to do

- Do NOT rewrite `demo.rs` — the DemoApp stays as-is.
- Do NOT use wgpu backend — stick with glow for WebGL2 compatibility.
- Do NOT use `fn update()` — eframe 0.34 uses `fn ui()`.
- Do NOT pass string canvas IDs to `WebRunner::start()` — pass `HtmlCanvasElement`.
- Do NOT use Storybook.js or any JS storybook framework.
- Do NOT hardcode colors in CSS — always `var(--token-name)`.
- Do NOT create separate WASM bundles per story.
- Do NOT use `peaceiris/actions-gh-pages` — use `actions/deploy-pages@v4` (matches existing workflow).

## How to start

1. Add `export_css` example to `ui-theme/Cargo.toml` and implement it.
2. Scaffold Astro: `npm create astro@latest docs-site` with MDX integration.
3. Generate `tokens.css`, set up `global.css` using the token variables.
4. Build `DocsLayout.astro` + `Sidebar.astro` with navigation (including the prominent "Live Demo" link).
5. Create foundation pages (colors, typography, spacing) — no WASM needed.
6. Create `ui-storybook` crate with one story (button gallery + playground).
7. Build with Trunk, create `ComponentPreview.astro`, verify WASM loads.
8. Create `DemoEmbed.astro` and `demo.mdx` page, verify the demo loads in the docs site.
9. Update `deploy.yml` to build all three (tokens → storybook WASM → demo WASM → Astro) and deploy.
10. Add remaining stories and component pages incrementally.

# frost-night-egui

A minimal egui theming and component library for dark aviation UIs, extracted from Figma mockups.

## Live Demo

[View the web demo](https://laurentdellanegra.github.io/frost-night-egui/)

## Overview

`ui-theme` is a self-contained design system for [egui](https://github.com/emilk/egui) 0.34+. It provides:

- **Semantic color palette** — dark theme with background/foreground pairs extracted from Figma
- **Design tokens** — interaction states (idle, hover, active, disabled) derived automatically from the palette
- **Component library** — themed wrappers for buttons, inputs, checkboxes, toggles, segmented controls, badges, cards, toolbar
- **Icon font** — embedded Lucide icon font with named codepoint constants
- **Drag animations** — border glow, handle morphing (dots → bar), global drag fade
- **Glassmorphism fallback** — semi-transparent backdrop API, ready for a future blur shader

No external theming dependencies. Just `egui`.

## Components

| Component | Description |
|-----------|-------------|
| `button` | Primary, Secondary, Ghost, Outline, Destructive, Link variants with Sm/Md/Lg sizes |
| `text_input` | Themed text field with label |
| `checkbox` | Outer border → 3px gap → inner fill pattern |
| `toggle` | Switch matching checkbox visual pattern |
| `segmented` | Segmented control with shared visual style |
| `badge` | Primary, Accent, Outline, Destructive variants |
| `card` | Themed panel with border |
| `drag_card` | Draggable card with handle animation, border glow, and global drag fade |
| `toolbar` | Vertical icon toolbar with grouped items, badges, and active highlights |
| `separator` | Themed horizontal divider |

All interactive controls (checkbox, toggle, segmented) share a consistent visual pattern:
- Outer border: `#3C4656`, `lg` radius
- 3px gap
- Inner fill: `#0E1A38` (off) / `#162C59` (on), `md` radius

## Usage

```rust
use ui_theme::{Theme, apply_theme};
use ui_theme::components::*;
use ui_theme::tokens::ControlVariant;
use ui_theme::scale::ControlSize;

// Apply theme globally
let theme = Theme::dark();
apply_theme(ctx, &theme);

// Use components
button(ui, &theme, "Click me", ControlVariant::Primary, ControlSize::Md);
text_input(ui, &theme, "Email", &mut email, ControlSize::Md);
checkbox(ui, &theme, &mut checked, "Enable feature");
toggle(ui, &theme, &mut on, "Dark mode");

let mut selected = 0;
segmented(ui, &theme, &["Tab A", "Tab B", "Tab C"], &mut selected);

badge(ui, &theme, "Status", BadgeVariant::Accent);

// Toolbar with icon groups
use ui_theme::icons::*;
let groups = vec![
    vec![ToolbarItem::new(ICON_MAP), ToolbarItem::new(ICON_LAYERS)],
    vec![ToolbarItem::new(ICON_SETTINGS)],
];
let mut selected = 0;
toolbar(ui, &theme, &groups, &mut selected);
```

## Running the demo

### Native

```sh
cd ui-theme
cargo run --example demo
```

### Web (wasm)

Requires [trunk](https://trunkrs.dev/):

```sh
cd web-demo
trunk serve
```

## Project structure

```
ui-theme/           # The theme/component library crate
  src/
    palette.rs      # Semantic color palette (ColorPalette)
    tokens.rs       # Interaction state derivation (VariantTokens, mix)
    scale.rs        # Radius, spacing, size, variant enums (SpacingScale, RadiusScale)
    theme.rs        # Main Theme struct
    helpers.rs      # egui integration (apply_theme)
    icons.rs        # Lucide icon font integration and codepoint constants
    blur.rs         # Semi-transparent backdrop API
    oklch.rs        # OKLCH color space utilities
    fonts/lucide.ttf  # Embedded Lucide icon font (v1.7.0)
    components/     # Themed component wrappers
  examples/
    demo.rs         # Native demo showcasing all components

web-demo/           # Separate wasm crate for GitHub Pages
  src/main.rs       # Demo app with dual native/wasm entry points
  index.html        # Trunk entry point

mockups/            # Original Figma screenshots and links
```

## Design tokens

See [DESIGN_TOKENS.md](DESIGN_TOKENS.md) for the full extracted color palette, typography, spacing, and component specifications.

## License

MIT

# CLAUDE.md

## Project overview

`frost-night-egui` is a minimal egui 0.34 theming + component library (`ui-theme`) extracted from aviation UI Figma mockups. Dark mode only. No external theming dependencies.

## Repository structure

- `ui-theme/` — The library crate. Palette, tokens, scales, theme, helpers, and component wrappers.
- `ui-theme/examples/demo.rs` — Native demo showcasing all components with live animated tracks.
- `web-demo/` — Separate wasm crate for GitHub Pages deployment (mirrors the native demo).
- `mockups/` — Original Figma screenshots (`interface.png`, `windows.png`) and links.
- `DESIGN_TOKENS.md` — Extracted color palette, typography, spacing, and component specs.

## Build commands

```sh
# Check the library
cd ui-theme && cargo check

# Run the native demo
cd ui-theme && cargo run --example demo

# Build the web demo (requires trunk: cargo install trunk)
cd web-demo && trunk serve

# Build web demo for production
cd web-demo && trunk build --release --public-url ./
```

## Key conventions

### eframe 0.34 API
- `App` trait uses `fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame)` — NOT `fn update()`.
- Wasm: `WebRunner::new().start()` takes `HtmlCanvasElement`, not a string ID.
- Web demo uses `glow` backend (not wgpu) for broad WebGL2 compatibility.

### Component pattern
All interactive controls (checkbox, toggle, segmented) share a consistent visual structure:
- Outer border: `theme.palette.control_border` (`#3C4656`) with `theme.radius.lg`
- Gap: `theme.control_gap` (3px)
- Inner fill: `theme.palette.control_fill_off` (`#0E1A38`) / `theme.palette.control_fill_on` (`#162C59`) with `theme.radius.md`

These values live in `palette.rs` and `theme.rs` — components reference them via `theme`, never as local constants.

### Spacing scale
All spacing (padding, margins, gaps) uses `theme.spacing` (`SpacingScale` in `scale.rs`):
- `xs` = 4px, `sm` = 8px, `md` = 12px, `lg` = 16px, `xl` = 24px.
- Do not hardcode spacing values in components — always use `theme.spacing.*`.
- When a value doesn't land exactly on the scale (e.g. 6px), express it as `theme.spacing.xs + 2.0`.

### Color source of truth
- All colors come from Figma mockups. Do not invent colors.
- `palette.rs` is the single source of truth for semantic colors.
- `accent` = `#0F1E3D` (dark navy), `ring` = `#4A90CF` (blue highlight).
- Badge `Accent` variant and Link variant use `ring` color (not `accent`).

### Component API pattern
Every component takes `(ui: &mut Ui, theme: &Theme, ...)` and returns `Response` (or a custom response struct like `DragCardResponse`).

### Self-contained components
Components should be self-contained and not rely on the demo to define styles:
- `sidebar_card` paints its own `surface_blur` backdrop, border glow, outer halo, and handle animation (3 dots → grab bar) internally. Returns `SidebarCardResponse { closed, dragging, drag_delta }`.
- `toolbar` paints its own `surface_blur` backdrop, active/hover highlights, and dividers. Returns `ToolbarResponse { clicked, rect, button_centers_y }`.
- `top_toolbar` paints its own backdrop, vertical separators, and icon buttons. Returns `TopToolbarResponse { icon_clicked }`.
- `zoom_toolbar` takes a `rect` parameter (no child UI needed) and paints its own backdrop with +/− icon buttons, separator, and Reset text button. Returns `ZoomToolbarResponse { zoom_in, zoom_out, reset }`. Uses absolute `Id::new(...)` for all widget IDs.
- All control colors come from `theme.palette` — never hardcode hex colors in component files.

### Global drag fade
When a card is being dragged, the demo applies `ui.set_opacity(0.15)` to all UI elements (toolbar + card). The animation is driven by `any_card_dragging` from the previous frame via `animate_bool_with_time`. The sidebar_card does NOT set its own opacity — only its border glow, halo, and handle effects are internal. Body opacity multiplies with the parent: `body_ui.set_opacity(ui.opacity() * open_t)`.

### Sidebar card state management (demo)
The demo supports docked cards (attached to toolbar) and floating cards (detached via drag):
- `docked_button: Option<usize>` — which toolbar button has the docked card open.
- `floating_cards: Vec<FloatingCard>` — cards detached and parked freely.
- Docked and floating cards for the same button share `Id::new(("sidebar_card", button_idx))` to avoid egui "widget rect changed id between passes" warnings on transitions.
- Drag-while-docked: card accumulates `docked_drag_offset` during drag (stays docked with stable ID), converts to floating only on drag release.
- Deferred push: newly detached floating cards are pushed after the floating card render loop to avoid duplicate rendering in the same frame.

### egui widget ID hygiene
- Child UIs created with `ui.new_child()` should use `.id_salt(...)` for stable IDs that don't depend on auto-ID counters.
- Components rendered after conditionally-present widgets (like sidebar cards) are especially vulnerable to ID instability — prefer absolute `Id::new(...)` or explicit `id_salt`.
- `zoom_toolbar` takes a `Rect` directly and uses absolute IDs — no child UI wrapper needed.

### Icons
- Lucide icon font (TTF) is embedded via `include_bytes!` in `icons.rs`.
- `load_icon_font(ctx)` is called automatically by `apply_theme()`.
- Named constants: `ICON_MAP`, `ICON_LAYERS`, `ICON_SETTINGS`, `ICON_CIRCLE_X`, etc.
- Use `icon_font(size)` for `FontId` or `icon_text(icon, size)` for `RichText`.

### Demo app
- `demo.rs` and `web-demo/src/main.rs` should stay in sync — they are the same demo with different entry points.
- Tracks animate continuously along velocity vectors (frame-rate independent with `dt`).

## Deployment
- GitHub Pages via `.github/workflows/deploy.yml` — triggers on push to `main`.
- Uses trunk to build `web-demo/` with `--public-url ./` for relative asset paths.

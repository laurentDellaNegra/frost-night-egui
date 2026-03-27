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
- Outer border: `#3C4656` with `theme.radius.lg`
- 3px gap
- Inner fill: `#0E1A38` (off) / `#162C59` (on) with `theme.radius.md`

Shared constants (`OUTER_BORDER`, `INNER_FILL_OFF`, `INNER_FILL_ON`) are defined locally in each component file.

### Color source of truth
- All colors come from Figma mockups. Do not invent colors.
- `palette.rs` is the single source of truth for semantic colors.
- `accent` = `#0F1E3D` (dark navy), `ring` = `#4A90CF` (blue highlight).
- Badge `Accent` variant and Link variant use `ring` color (not `accent`).

### Component API pattern
Every component takes `(ui: &mut Ui, theme: &Theme, ...)` and returns `Response` (or a custom response struct like `DragCardResponse`).

### Demo app
- `demo.rs` and `web-demo/src/main.rs` should stay in sync — they are the same demo with different entry points.
- Tracks animate continuously along velocity vectors (frame-rate independent with `dt`).
- Card fades to 15% opacity when dragged via `ui.set_opacity()`.

## Deployment
- GitHub Pages via `.github/workflows/deploy.yml` — triggers on push to `main`.
- Uses trunk to build `web-demo/` with `--public-url ./` for relative asset paths.

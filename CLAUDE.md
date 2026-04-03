# CLAUDE.md

## Project overview

`frost-night-egui` is a minimal egui 0.34 theming + component library (`ui-theme`) extracted from aviation UI Figma mockups. Dark mode only. No external theming dependencies. Includes a design system documentation site (`docs-site`) with interactive WASM component playgrounds (`ui-storybook`).

## Repository structure

```
frost-night-egui/
├── ui-theme/                    # The library crate
│   ├── src/
│   │   ├── lib.rs
│   │   ├── palette.rs           # ColorPalette — single source of truth for colors
│   │   ├── tokens.rs            # StateColors, VariantTokens, ControlVariant
│   │   ├── scale.rs             # RadiusScale, SpacingScale, ControlSize
│   │   ├── theme.rs             # Theme struct
│   │   ├── helpers.rs           # apply_theme()
│   │   ├── blur.rs              # BlurRect (fallback, no shader)
│   │   ├── oklch.rs             # OKLCH utilities
│   │   ├── icons.rs             # Lucide font embed + constants
│   │   ├── demo.rs              # DemoApp (behind "demo" feature)
│   │   └── components/          # All component wrappers
│   └── examples/
│       ├── demo.rs              # Native demo entry
│       └── export_css.rs        # Dumps ColorPalette as CSS custom properties
├── web-demo/                    # WASM entry for the full interactive demo
├── ui-storybook/                # WASM crate for component story playgrounds
│   ├── src/
│   │   ├── main.rs              # start_story(canvas_id, story_name) wasm_bindgen export
│   │   └── stories/             # One file per component story
│   │       ├── mod.rs
│   │       ├── controls.rs      # controls_panel(), section_frame(), section_divider()
│   │       ├── button.rs        # button_story() — controls + playground + all variants
│   │       └── ...              # Same pattern for all components
│   └── index.html               # Trunk entry
├── docs-site/                   # Astro 6 documentation site
│   ├── astro.config.mjs         # base: '/frost-night-egui'
│   ├── src/
│   │   ├── content.config.ts    # Content collection with glob loader
│   │   ├── content/docs/        # MDX pages (foundations + components)
│   │   ├── components/          # Astro components (Sidebar, ComponentPreview, etc.)
│   │   ├── layouts/             # DocsLayout, DemoLayout
│   │   ├── pages/               # [...slug].astro, demo-fullscreen.astro
│   │   └── styles/              # tokens.css (generated), global.css, code.css
│   └── public/
│       ├── wasm/                # Trunk output: storybook WASM (built, gitignored)
│       └── demo/                # Trunk output: demo WASM (built, gitignored)
├── mockups/                     # Figma screenshots and links
├── DESIGN_TOKENS.md             # Extracted design token specs
├── run.sh                       # CLI script for all build commands
└── .github/workflows/deploy.yml # CI: tokens → storybook WASM → demo WASM → Astro → GH Pages
```

## Build commands

All commands can be run from the project root via `./run.sh`:

```sh
./run.sh check             # Check all Rust crates
./run.sh demo              # Run native demo
./run.sh tokens            # Generate CSS tokens from Rust palette
./run.sh wasm-storybook    # Build storybook WASM into docs-site/public/wasm
./run.sh wasm-demo         # Build demo WASM into docs-site/public/demo
./run.sh dev               # Start Astro dev server
./run.sh site              # Build Astro docs site
./run.sh build             # Full pipeline: tokens → WASM → Astro
./run.sh clean             # Remove build artifacts
```

Or manually:

```sh
cd ui-theme && cargo check
cd ui-theme && cargo run --example demo --features demo
cd ui-theme && cargo run --example export_css 2>/dev/null > ../docs-site/src/styles/tokens.css
cd ui-storybook && trunk build --release --public-url /frost-night-egui/wasm/ --dist ../docs-site/public/wasm --filehash false
cd web-demo && trunk build --release --public-url /frost-night-egui/demo/ --dist ../docs-site/public/demo --filehash false
cd docs-site && npm run dev
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

### Accordion
- `accordion(ui, theme, items, open, exclusive, add_body)` — collapsible sections, always borderless.
- `open: &mut Vec<bool>` tracks which sections are expanded. `exclusive: bool` allows only one open at a time.
- `add_body: impl FnMut(&mut Ui, usize)` — `FnMut` (not `Fn`) so nested accordions can mutate their own state.
- Body height is animated via stored measurements in egui temp data. Content is clipped during the transition.
- Supports nesting — accordion inside accordion works out of the box.

### Tabs
- `tabs(ui, theme, selected, labels)` — horizontal underline-style tab bar.
- `tabs_with_icons(ui, theme, selected, labels, icons)` — variant with optional per-tab Lucide icons.
- Active tab: `foreground` text + `ring`-colored underline animating from center.
- Inactive: `muted_foreground` text, no underline. Hover brightens text.
- Animation duration: 0.12s. Underline thickness: 1.5px.
- Click is deferred: `selected` is mutated at the end of the function to avoid ID instability between egui passes. Callers should snapshot the value before `tabs()` if using it for conditional rendering below.

### Widgets (`ui-theme/src/widgets/`)
Composed UI patterns built from components — used in the demo but not standalone library exports.
- `maps_menu(ui, theme, state)` — full maps browser: tabs (Favorites / All Maps), search bar, checkbox grid, nested accordions with star-toggle favorites.
- `MapsMenuState` holds tab, search text, category tree with per-map `favorite`/`selected` state.
- Tab switching uses deferred mutation via `cumulative_pass_nr()` to avoid ID instability.
- Search (3+ chars) collapses the grid with height animation and shows filtered results with auto-opened accordions.

### Self-contained components
Components should be self-contained and not rely on the demo to define styles:
- `sidebar_card` paints its own `surface_blur` backdrop, border glow, outer halo, and handle animation (3 dots → grab bar) internally. Takes `highlight: bool` for attention glow (combined with drag glow via `glow_t = drag_t.max(highlight_t)`). Returns `SidebarCardResponse { closed, dragging, drag_delta }`.
- `toolbar` paints its own `surface_blur` backdrop, active/hover highlights, and dividers. Takes `floating: &[usize]` to show `ring`-colored icons for floating cards. Returns `ToolbarResponse { clicked, rect, button_centers_y }`.
- `top_toolbar` paints its own backdrop, vertical separators, and icon buttons. Returns `TopToolbarResponse { icon_clicked }`.
- `zoom_toolbar` takes a `rect` parameter (no child UI needed) and paints its own backdrop with +/− icon buttons, separator, and Reset text button. Returns `ZoomToolbarResponse { zoom_in, zoom_out, reset }`. Uses absolute `Id::new(...)` for all widget IDs.
- All control colors come from `theme.palette` — never hardcode hex colors in component files.

### Global drag fade
When a card is being dragged, the demo applies `ui.set_opacity(0.15)` to all UI elements (toolbar + card). The animation is driven by `any_card_dragging` from the previous frame via `animate_bool_with_time`. The sidebar_card does NOT set its own opacity — only its border glow, halo, and handle effects are internal. Body opacity multiplies with the parent: `body_ui.set_opacity(ui.opacity() * open_t)`.

### Sidebar card state management (demo)
The demo supports docked cards (attached to toolbar) and floating cards (detached via drag):
- `docked_button: Option<usize>` — which toolbar button has the docked card open.
- `floating_cards: Vec<FloatingCard>` — cards detached and parked freely.
- Docked card uses a fixed `Id::new("docked_sidebar_card")` since only one is visible at a time (avoids ID changes when switching toolbar buttons). Floating cards use `Id::new(("sidebar_card", from_button))` since multiple can coexist.
- Drag-while-docked: card accumulates `docked_drag_offset` during drag (stays docked with stable ID), converts to floating only on drag release.
- Deferred push: newly detached floating cards are pushed after the floating card render loop to avoid duplicate rendering in the same frame.
- Toolbar shows `ring`-colored icons for buttons that have floating cards (via `floating` parameter).
- Clicking a toolbar button for an already-parked card triggers a highlight flash (timestamp-based, 0.3s) and brings the card to front.
- Z-ordering: click or drag on any floating card moves it to end of Vec (renders on top). Uses pointer position detection (not `ui.interact()`) to avoid adding extra widget IDs.

### egui widget ID hygiene

**Core rule**: egui's "Widget rect changed id between passes" warnings mean the same screen rect has different widget IDs between layout passes. This happens when widget rendering order or presence changes between passes in a single frame.

**`new_child` auto-counter trap**: `ui.new_child(UiBuilder::new().id_salt(id))` does NOT fully determine the child's ID. egui mixes the parent's `next_auto_id_salt` counter into the child's `unique_id`. So if widgets before it conditionally appear/disappear, or if rendering order changes (e.g. Vec reorder), the auto-counter shifts and the child gets a different ID.

**Fix — `global_scope(true)`**: For components rendered in variable order (e.g. iterated from a Vec that may reorder), use:
```rust
ui.new_child(UiBuilder::new().id_salt(id).global_scope(true).max_rect(rect))
```
With `global_scope(true)`, both `stable_id` and `unique_id` equal the provided `id_salt` directly — no parent auto-counter mixed in. The child's entire widget subtree becomes order-independent.

**Fix — `push_id` for section isolation**: Wrap independent UI sections in `ui.push_id("section_name", |ui| { ... })` to give each section its own auto-ID counter. This prevents one section's conditional rendering from shifting IDs in another section.

**Fix — absolute IDs for interactions**: Use `Id::new(...)` (not `ui.id().with(...)`) for `ui.interact()` calls in components that may render in variable contexts. Absolute IDs don't depend on the parent UI scope.

**Fix — deferred state mutation**: egui runs two layout passes per frame. If a click changes state (e.g. tab selection) during pass 1, pass 2 sees different content → ID mismatch. Fix: snapshot state before rendering, defer mutation to next frame. Use `ctx.cumulative_pass_nr()` to detect frame boundaries. Example: `maps_menu` stores pending tab clicks in temp data keyed by pass number, applies only when `pass_nr >= stored + 2`.

**Fix — use `ui.id().with()` not `ui.auto_id_with()`**: `auto_id_with` depends on the parent's widget counter which shifts when siblings change. Use `ui.id().with("name")` for stable scope-based IDs. Applied in `accordion` component.

**Patterns applied in this project**:
- `sidebar_card` body uses `.global_scope(true)` so card content IDs are stable regardless of rendering order.
- Demo wraps docked and floating card sections in separate `push_id` scopes.
- Demo snapshots `docked_button` before handling toolbar clicks for stable rendering.
- `zoom_toolbar` takes a `Rect` directly and uses absolute `Id::new(...)` — no child UI wrapper needed.
- Docked card uses fixed `Id::new("docked_sidebar_card")` — avoids ID instability when switching toolbar buttons.
- Sidebar card position is fixed (no slide animation) — animating the rect between passes causes "widget rect changed id" warnings.
- `maps_menu` wraps each tab in `push_id` so both scopes always exist in the widget tree regardless of active tab.

### Icons
- Lucide icon font (TTF) is embedded via `include_bytes!` in `icons.rs`.
- `load_icon_font(ctx)` is called automatically by `apply_theme()`.
- Named constants: `ICON_MAP`, `ICON_LAYERS`, `ICON_SETTINGS`, `ICON_CIRCLE_X`, `ICON_SNOWFLAKE`, `ICON_STAR`, `ICON_CHEVRON_RIGHT`, `ICON_CHEVRON_DOWN`, `ICON_SEARCH`, etc.
- Use `icon_font(size)` for `FontId` or `icon_text(icon, size)` for `RichText`.

### Demo app
- All demo logic lives in `ui-theme/src/demo.rs` (gated behind `demo` feature).
- `ui-theme/examples/demo.rs` and `web-demo/src/main.rs` are thin entry points — both call `ui_theme::demo::DemoApp::new(cc)`.
- Tracks animate continuously along velocity vectors (frame-rate independent with `dt`).
- Floating card z-ordering: last in `Vec<FloatingCard>` renders on top. Dragging or clicking a card moves it to end. Toolbar click for a parked card highlights it and brings to front.

## Docs site architecture

### Tech stack
- **Astro 6** with `@astrojs/mdx` — static site generator
- **ui-storybook** — Rust/WASM crate providing interactive component playgrounds via `start_story(canvas_id, story_name)`
- **tokens.css** — Generated from `ui-theme/examples/export_css.rs`, provides CSS custom properties matching the Rust palette

### WASM constraints
- eframe can only run **one WebRunner per WASM module per page**. Each component page has a single `<ComponentPreview>` canvas. Do NOT put multiple previews on one page.
- WASM imports use `is:inline` scripts with dynamic `import()` to avoid Vite/Rollup resolution at build time.
- Base URL is passed via `data-base` attribute on canvas elements (inline scripts can't access `import.meta.env`).
- Trunk builds use `--filehash false` so JS filenames are predictable (`ui-storybook.js`, `web-demo.js`).

### Story structure
Each story in `ui-storybook/src/stories/` has one `component_story(ui, theme, state)` function that renders three sections using helpers from `controls.rs`:
1. **Controls** — `controls_panel()` with ComboBoxes, checkboxes, sliders for component props
2. **Playground** — `section_frame(ui, theme, "Playground", ...)` showing the live component with selected props
3. **All Variants** — `section_frame(ui, theme, "All Variants", ...)` showing every variant/size combination

### Content pages
Each component MDX page in `docs-site/src/content/docs/components/` follows a template:
1. Frontmatter with title and description
2. Import `ComponentPreview`, `PropsTable`, `CodeBlock`
3. `## Preview` — single `<ComponentPreview story="name" height="Npx" />`
4. `## Usage` — `<CodeBlock>` with Rust example
5. `## Props` — `<PropsTable>` listing all parameters

## Checklist: adding or modifying a component

When a new component is added to `ui-theme/src/components/`, or an existing component's API changes (new props, renamed fields, changed signature), **all of the following must be updated**:

### 1. Library (`ui-theme/`)
- [ ] Add/edit the component file in `src/components/`
- [ ] Re-export in `src/components/mod.rs`
- [ ] If new palette colors are needed, add to `palette.rs` and update `export_css.rs`

### 2. Storybook (`ui-storybook/`)
- [ ] Create/edit `src/stories/<component>.rs` with a `<component>_story()` function following the Controls → Playground → All Variants pattern
- [ ] Add state struct (`<Component>StoryState`) with `Default` impl
- [ ] Export in `src/stories/mod.rs`
- [ ] Add route in `src/main.rs` `StoryApp::ui()` match and state field in `StoryApp`

### 3. Docs site (`docs-site/`)
- [ ] Create/edit `src/content/docs/components/<component>.mdx` with Preview, Usage, Props sections
- [ ] If new component: add entry in `src/components/Sidebar.astro` components list
- [ ] If palette/tokens changed: regenerate `tokens.css` via `./run.sh tokens`

### 4. Rebuild & verify
- [ ] `./run.sh check` — verify Rust compilation
- [ ] `./run.sh wasm-storybook` — rebuild storybook WASM
- [ ] `./run.sh dev` — verify the page renders correctly in the docs site

### 5. Design tokens
If `ColorPalette`, `SpacingScale`, `RadiusScale`, or `Theme` fields change:
- [ ] Update `ui-theme/examples/export_css.rs` to include new fields
- [ ] Run `./run.sh tokens` to regenerate `docs-site/src/styles/tokens.css`
- [ ] Update `DESIGN_TOKENS.md` if the change is semantic
- [ ] Update relevant foundation MDX pages (`colors.mdx`, `spacing.mdx`, `radius.mdx`)

## Deployment
- GitHub Pages via `.github/workflows/deploy.yml` — triggers on push to `main`.
- Pipeline: generate CSS tokens → build storybook WASM → build demo WASM → build Astro site → deploy.
- Astro `base: '/frost-night-egui'` — all routes and WASM imports use this base path.
- Trunk builds use `--public-url /frost-night-egui/wasm/` and `--public-url /frost-night-egui/demo/`.
- Node 22+ required (Astro 6 dependency).

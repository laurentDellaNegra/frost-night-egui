# Claude Code Prompt: Figma → egui Design System

## Context

I need to extract a design system from a rough Figma file and build a lightweight, self-contained egui theming + component library in Rust. The Figma was made quickly — tokens are not clearly defined, so you'll need to reverse-engineer the visual patterns.

I do NOT want to depend on `egui-shadcn` or any third-party theming crate. I want my own minimal lib. But I want to adopt the best architectural patterns from the `shadcn-rs/egui-shadcn` project (MIT licensed), specifically:

## Phase 1: Extract design tokens from Figma

I will provide you with:

- The Figma REST API JSON export (`figma.json`) — get it via: `curl -H "X-Figma-Token: <TOKEN>" "https://api.figma.com/v1/files/<FILE_KEY>" > figma.json`
- Screenshots of key screens (PNG)

From these, extract and document:

1. **Color palette** — every unique color used, grouped by semantic role (backgrounds, text, interactive elements, borders, accents, destructive/error states). The Figma is **dark mode only** — extract the dark palette. Do not invent a light palette; just note in the token doc which tokens would need light-mode counterparts later.
2. **Typography scale** — font families, sizes, weights, and line heights actually used. Map them to a hierarchy (heading, subheading, body, caption, code).
3. **Spacing patterns** — recurring padding, margin, and gap values. Derive a spacing scale.
4. **Border radii** — all corner radius values used, derive a radius scale.
5. **Border/stroke styles** — widths and colors for borders, dividers, focus rings.
6. **Component inventory** — list every recurring UI pattern (buttons, inputs, cards, toggles, etc.) with their visual variants.
7. **Surface/overlay styles** — identify all components that use transparent backgrounds (dialogs, toolbars, popovers, sidebars, toasts, context menus). For each, extract the background color + opacity and note the blur radius if visible. These will become the `surface_blur` tokens.

Output this as a structured `DESIGN_TOKENS.md` document.

## Phase 2: Build the egui design system crate

Create a Rust library crate called `ui-theme` (or a name I'll specify) with the following architecture. Follow these patterns inspired by `egui-shadcn` but keep it minimal:

### Core: `palette.rs` — Semantic color tokens

```rust
/// Semantic color palette using background/foreground pairs.
/// Define colors in OKLCH for perceptually uniform interpolation,
/// then convert to egui::Color32.
#[derive(Clone, Debug, PartialEq)]
pub struct ColorPalette {
    pub background: Color32,
    pub foreground: Color32,
    pub primary: Color32,
    pub primary_foreground: Color32,
    pub secondary: Color32,
    pub secondary_foreground: Color32,
    pub muted: Color32,
    pub muted_foreground: Color32,
    pub accent: Color32,
    pub accent_foreground: Color32,
    pub destructive: Color32,
    pub destructive_foreground: Color32,
    pub border: Color32,
    pub input: Color32,
    pub ring: Color32,
    pub card: Color32,
    pub card_foreground: Color32,
    pub popover: Color32,
    pub popover_foreground: Color32,
}
```

- **Dark mode only for now.** The Figma I'll provide is a dark theme. Implement `ColorPalette::dark()` with the extracted colors and use it as `Default`.
- Do NOT implement `light()` yet — but keep the architecture ready for it: the palette struct should be theme-agnostic (no "dark" in field names), so a future `ColorPalette::light()` is just a second constructor with different values.
- Add a `pub fn custom(...)` constructor for full control.
- Keep it to ~20 tokens max. Do NOT over-engineer with sidebar*\*, chart*\* etc unless my Figma actually needs them.

### Core: `tokens.rs` — Derived tokens & state system

```rust
/// Atomic visual state for one interaction state of a widget.
/// Maps directly to egui's WidgetVisuals.
#[derive(Clone, Copy, Debug)]
pub struct StateColors {
    pub bg_fill: Color32,
    pub fg_stroke: Stroke,
    pub border: Stroke,
}

/// All interaction states for a control variant.
#[derive(Clone, Copy, Debug)]
pub struct VariantTokens {
    pub idle: StateColors,
    pub hovered: StateColors,
    pub active: StateColors,
    pub disabled: StateColors,
}

/// Perceptually uniform color mixing (linear interpolation in sRGB).
pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 { ... }
```

**Key pattern**: derive interaction states automatically from the palette:

- `hovered` = `mix(base_color, WHITE, 0.06)` — lightens on hover (correct for dark mode)
- `active` = `mix(base_color, WHITE, 0.10)`
- `disabled` = `mix(muted, background, 0.6)` with reduced-opacity foreground
- NOTE: for a future light mode, hover/active would `mix(base_color, BLACK, 0.06)` instead. Keep the mix target as a parameter or constant so it's easy to flip later.

Implement `pub fn variant_tokens(palette: &ColorPalette, variant: ControlVariant) -> VariantTokens` that derives all states for Primary, Secondary, Ghost, Outline, Destructive, Link variants.

### Core: `scale.rs` — Radius, spacing, typography scales

```rust
#[derive(Clone, Copy, Debug)]
pub struct RadiusScale {
    pub sm: f32,  // e.g. 4.0
    pub md: f32,  // e.g. 8.0
    pub lg: f32,  // e.g. 12.0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlSize { Sm, Md, Lg }

impl ControlSize {
    pub fn padding(self) -> Vec2 { ... }
    pub fn radius(self, scale: &RadiusScale) -> CornerRadius { ... }
    pub fn font(self) -> FontId { ... }
}
```

Populate the actual values from what you extracted from Figma.

### Core: `theme.rs` — Main theme struct

```rust
#[derive(Clone, Debug)]
pub struct Theme {
    pub palette: ColorPalette,
    pub radius: RadiusScale,
}

impl Theme {
    /// Dark theme from Figma — the only theme for now.
    pub fn dark() -> Self { ... }

    /// Alias: Default = dark for now. When light mode is added,
    /// this can switch based on system preference.
    pub fn default() -> Self { Self::dark() }

    /// Get resolved widget visuals for a variant + size combo.
    pub fn control(&self, variant: ControlVariant, size: ControlSize) -> ControlVisuals { ... }

    /// Get resolved input visuals for a size.
    pub fn input(&self, size: ControlSize) -> InputVisuals { ... }
}
```

### Core: `oklch.rs` — OKLCH color space utilities

Include a minimal OKLCH-to-sRGB-to-Color32 converter (~50 lines). Use OKLCH as the source-of-truth for palette definitions so interpolations and palette generation are perceptually uniform.

### Core: `helpers.rs` — egui integration helpers

```rust
/// Convert VariantTokens into egui's native Widgets struct.
pub fn to_egui_widgets(tokens: &VariantTokens, radius: CornerRadius, expansion: f32) -> Widgets { ... }

/// Apply theme to an entire egui context (global).
pub fn apply_theme(ctx: &egui::Context, theme: &Theme) { ... }
```

### Core: `blur.rs` — Glassmorphism / backdrop blur via wgpu

**Critical design requirement**: in the Figma, overlay components (dialogs, toolbars, sidebars, popovers, toasts, context menus) use **transparent backgrounds with a backdrop blur** — the classic glassmorphism look. egui does NOT support blur natively, so we need a custom wgpu shader approach.

**Architecture** (based on proven egui+wgpu blur implementations):

1. **Two-texture approach**: allocate a "front" texture (main screen) and a "back" texture (intermediary), both screen-sized.
2. **Blur via `PaintCallback`**: use `egui_wgpu::CallbackTrait` to inject a custom Gaussian blur shader at render time.
3. **Two-pass blur**:
   - Pass 1 (`prepare`): read from front texture, apply horizontal Gaussian blur, write to back texture.
   - Pass 2 (`paint`): read from back texture, apply vertical Gaussian blur, composite back onto the front texture within the blur rect.
4. **WGSL shader**: a simple separable Gaussian blur kernel (9-13 taps is enough for a smooth frosted glass effect).

**What to implement**:

```rust
/// Configuration for a blurred background region.
pub struct BlurRect {
    pub rect: egui::Rect,       // screen-space rectangle to blur
    pub radius: f32,            // blur radius in pixels (8-16 typical)
    pub tint: Color32,          // semi-transparent overlay color (e.g. background at 60-80% opacity)
    pub corner_radius: CornerRadius,
}

/// Manages wgpu resources for blur rendering.
/// Store this in `egui_wgpu::Renderer::callback_resources`.
pub struct BlurRenderer {
    // front_texture, back_texture, blur_pipeline, copy_pipeline,
    // bind_groups, sampler, uniform_buffer
}

impl BlurRenderer {
    /// Create from wgpu device + surface config. Call once at app init.
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self { ... }

    /// Handle window resize — reallocate textures.
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) { ... }
}

/// Convenience: paint a blurred rect as an egui Shape.
/// Use this inside component rendering:
///   ui.painter().add(blur_shape(blur_rect));
pub fn blur_shape(blur: BlurRect) -> egui::Shape { ... }
```

**Integration with theme**: add blur tokens to the palette:

```rust
// In ColorPalette:
pub surface_blur: Color32,       // e.g. background at 70% opacity — the tint over the blur
pub surface_blur_radius: f32,    // default blur radius in px

// In Theme, convenience method:
pub fn surface_blur(&self, rect: egui::Rect, corner_radius: CornerRadius) -> BlurRect { ... }
```

**Component usage pattern** (e.g. dialog):

```rust
pub fn dialog(ui: &mut Ui, theme: &Theme, content: impl FnOnce(&mut Ui)) {
    // 1. Draw dimming overlay
    // 2. Draw blur_shape for the dialog rect
    // 3. Draw the dialog frame (with surface_blur tint as bg)
    // 4. Render content inside
}
```

**Important notes**:

- This requires `eframe` with the `wgpu` backend (not `glow`). Add `eframe = { version = "0.33", features = ["wgpu"] }` as a dev/example dependency.
- The blur module should be behind a `blur` feature flag so the core theme works without wgpu.
- Reference implementation: https://www.mxs.dev/blog/egui-wgpu-blurry-windows — adapt the approach but make it reusable and theme-driven.
- Include the WGSL blur shader as `src/shaders/blur.wgsl` (embedded via `include_str!`).
- If blur is too complex for the first iteration, implement a **fallback**: just draw `surface_blur` (semi-transparent tint) without the actual blur. The API stays the same, the visual is just less fancy. This way components don't need to change when blur is added later.

## Phase 3: Component wrappers (optional, per component)

For each component I specify, create a thin wrapper that:

- Takes `&Theme` as first parameter
- Takes component-specific props (label, variant, size, enabled, etc.)
- Resolves its visuals from the theme
- Renders using egui primitives
- Returns the egui `Response`

Start with: `button`, `text_input`, `card`, `separator`, `badge`, `toggle/switch`.

Then overlay components (these should use `blur_shape` or the fallback for their backgrounds): `dialog`, `toolbar`, `popover`, `tooltip`, `context_menu`.

Follow this signature pattern:

```rust
pub fn button(ui: &mut Ui, theme: &Theme, label: impl Into<WidgetText>, variant: ControlVariant, size: ControlSize) -> Response { ... }
```

## Constraints

- **Zero external theming dependencies** — only depend on `egui` (0.33+) and optionally `serde` for serialization. The `blur` feature adds `egui-wgpu` and `wgpu` as dependencies.
- **Feature flags**: `serde` for serialization, `blur` for wgpu-based glassmorphism. The core theme must work without either.
- **Target ~300-500 lines total** for the core theme system (palette + tokens + theme + oklch + helpers). Not 1500.
- **Every color value must come from my Figma** — do not invent colors. If a token has no obvious Figma match, flag it and suggest a derived value.
- **Add `serde` derives** behind a `serde` feature flag so the theme can be serialized to JSON/RON.
- **Document every public item** with rustdoc comments.
- **Include a `examples/demo.rs`** that shows all variants × sizes in a grid, using eframe. The demo should launch in dark mode by default (`egui::Visuals::dark()` as base, then apply the theme on top).

## What NOT to do

- Do NOT use `egui-shadcn`, `egui-aesthetix`, `egui-theme`, or any theming crate as a dependency.
- Do NOT copy code from shadcn-rs — rewrite from scratch using the patterns.
- Do NOT include motion/animation tokens — egui's immediate mode handles this differently. The only custom rendering exception is the blur shader, which is a render-time effect, not an animation token.
- Do NOT include Radix-style sub-variants (Surface/Classic/Soft) unless my Figma explicitly has them.
- Do NOT over-engineer. Start minimal, I'll ask you to add more later.

## File structure

```
ui-theme/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Public API re-exports
│   ├── oklch.rs        # OKLCH → Color32 converter
│   ├── palette.rs      # ColorPalette struct + dark constructor
│   ├── tokens.rs       # StateColors, VariantTokens, mix(), variant derivation
│   ├── scale.rs        # RadiusScale, ControlSize, ControlVariant enums
│   ├── theme.rs        # Theme struct, control()/input() resolvers
│   ├── helpers.rs      # egui integration (to_egui_widgets, apply_theme)
│   ├── blur.rs         # BlurRect, BlurRenderer (behind "blur" feature)
│   ├── shaders/
│   │   └── blur.wgsl   # Separable Gaussian blur kernel
│   └── components/     # Optional thin component wrappers
│       ├── mod.rs
│       ├── button.rs
│       ├── input.rs
│       ├── card.rs
│       ├── dialog.rs   # Uses blur_shape for backdrop
│       ├── toolbar.rs  # Uses blur_shape for backdrop
│       └── popover.rs  # Uses blur_shape for backdrop
└── examples/
    └── demo.rs         # Visual showcase of all variants
```

## How to start

1. First, read and analyze the Figma JSON I provide to extract all design tokens.
2. Show me the extracted `DESIGN_TOKENS.md` for validation before writing any Rust.
3. Once I approve the tokens, generate the full crate.
4. Build and verify it compiles with `cargo check`.

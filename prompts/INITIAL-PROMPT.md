# Claude Code Prompt: Figma → egui Design System

## Context

Extract a design system from a rough Figma file and build a lightweight, self-contained egui theming + component library in Rust. The Figma was made quickly — tokens are not clearly defined, so reverse-engineer the visual patterns.

No dependency on `egui-shadcn` or any third-party theming crate — own minimal lib, adopting the best architectural patterns from shadcn-rs.

## Phase 1: Extract design tokens from Figma

Input: screenshots of key screens (PNG).

Extract and document:
1. **Color palette** — every unique color, grouped by semantic role. Dark mode only.
2. **Typography scale** — font families, sizes, weights, line heights.
3. **Spacing patterns** — recurring padding, margin, and gap values.
4. **Border radii** — all corner radius values used.
5. **Border/stroke styles** — widths and colors for borders, dividers, focus rings.
6. **Component inventory** — list every recurring UI pattern with variants.
7. **Surface/overlay styles** — transparent backgrounds with blur radius.

Output as `DESIGN_TOKENS.md`.

## Phase 2: Build the egui design system crate

Create `ui-theme` with:

### Core modules
- `palette.rs` — Semantic color tokens (`ColorPalette` struct with `dark()` constructor)
- `tokens.rs` — Derived interaction states (`StateColors`, `VariantTokens`, `mix()`)
- `scale.rs` — Radius, spacing, typography scales (`RadiusScale`, `ControlSize`, `ControlVariant`)
- `theme.rs` — Main `Theme` struct with `control()`/`input()` resolvers
- `oklch.rs` — OKLCH color space utilities
- `helpers.rs` — egui integration (`apply_theme`)
- `blur.rs` — Semi-transparent backdrop fallback (API ready for future blur shader)

### Component wrappers
- `button` — Primary, Secondary, Ghost, Outline, Destructive, Link variants
- `text_input` — Themed text field with label
- `card` / `drag_card` — Themed panels, drag card with opacity animation
- `checkbox` — Outer border → gap → inner fill pattern
- `toggle` — Switch matching checkbox visual pattern
- `segmented` — Segmented control matching checkbox/toggle pattern
- `badge` — Primary, Accent, Outline, Destructive variants
- `separator` — Themed divider

## Constraints

- **Zero external theming dependencies** — only `egui` 0.34+
- **Feature flags**: `serde` for serialization
- **Dark mode only** — palette struct is theme-agnostic for future light mode
- **Every color from Figma** — no invented colors
- **`examples/demo.rs`** — visual showcase of all components
- **`web-demo/`** — separate wasm crate for GitHub Pages deployment

## File structure

```
ui-theme/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── oklch.rs
│   ├── palette.rs
│   ├── tokens.rs
│   ├── scale.rs
│   ├── theme.rs
│   ├── helpers.rs
│   ├── blur.rs
│   └── components/
│       ├── mod.rs
│       ├── button.rs
│       ├── input.rs
│       ├── card.rs
│       ├── drag_card.rs
│       ├── checkbox.rs
│       ├── toggle.rs
│       ├── segmented.rs
│       ├── badge.rs
│       └── separator.rs
└── examples/
    └── demo.rs

web-demo/
├── Cargo.toml
├── index.html
└── src/
    └── main.rs
```

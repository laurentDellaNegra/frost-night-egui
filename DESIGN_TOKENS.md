# Design Tokens

Extracted from Figma mockups (visual analysis of `interface.png` and `windows.png`).

> **Note**: Colors are approximated from screenshots since Figma JSON export was unavailable.
> The Figma metadata confirmed the canvas background as `r:0.016, g:0.027, b:0.055` = **#04070E**.
> All other values are best-effort visual matches. Adjust once exact Figma values are available.

---

## 1. Color Palette (Dark Mode Only)

### Backgrounds

| Token               | Hex (approx.)            | Usage                                                    |
| ------------------- | ------------------------ | -------------------------------------------------------- |
| `background`        | `#04070E`                | Main app/map background (Figma-confirmed)                |
| `surface`           | `#0C1622`                | Sidebar panels, opaque card backgrounds                  |
| `surface_elevated`  | `#111D2E`                | Elevated cards, input field backgrounds                  |
| `surface_blur`      | `#060C16` at **85% opacity** | Semi-transparent card/panel backdrop                 |

### Foregrounds (Text)

| Token                  | Hex (approx.) | Usage                                 |
| ---------------------- | ------------- | ------------------------------------- |
| `foreground`           | `#E0E8F0`     | Primary text, headings                |
| `foreground_secondary` | `#8494A7`     | Labels, secondary text                |
| `foreground_muted`     | `#5A6A7E`     | Placeholder text, disabled text       |

### Primary (Frost/Silver)

| Token                 | Hex (approx.) | Usage                                            |
| --------------------- | ------------- | ------------------------------------------------ |
| `primary`             | `#CCD4E2`     | Primary buttons, filled badges                   |
| `primary_foreground`  | `#0E1724`     | Text on primary-colored backgrounds              |

### Secondary

| Token                   | Hex (approx.) | Usage                                    |
| ----------------------- | ------------- | ---------------------------------------- |
| `secondary`             | `#141E2E`     | Secondary button fills, inactive tabs    |
| `secondary_foreground`  | `#8A96A8`     | Text on secondary backgrounds            |
| `secondary_border`      | `#3D2D55`     | Border for secondary controls (violet tint) |

### Accent (Navy)

| Token               | Hex (approx.) | Usage                                         |
| -------------------- | ------------- | --------------------------------------------- |
| `accent`             | `#0F1E3D`     | Dark navy accent used in control inner fills   |
| `accent_foreground`  | `#E0E8F0`     | Text on accent backgrounds                     |

### Interactive Controls

Checkbox, toggle, and segmented controls share a consistent visual pattern.
These are stored in `ColorPalette` and accessed via `theme.palette.control_*`:

| Palette Field         | Hex       | Usage                                             |
| ---------------------- | --------- | ------------------------------------------------- |
| `control_border`       | `#3C4656` | Outer border stroke for checkboxes, toggles, segmented |
| `control_fill_off`     | `#0E1A38` | Inner fill when control is OFF                    |
| `control_fill_on`      | `#162C59` | Inner fill when control is ON                     |

The gap between outer border and inner fill is `theme.control_gap` (3px).

### Borders & Dividers

| Token    | Hex (approx.) | Usage                                    |
| -------- | ------------- | ---------------------------------------- |
| `border` | `#1A2B40`     | Panel borders, input borders, dividers   |
| `ring`   | `#4A90CF`     | Focus ring, accent badge color           |
| `input`  | `#1A2B40`     | Input field borders (same as border)     |

### Status / Semantic

| Token         | Hex (approx.) | Usage                          |
| ------------- | ------------- | ------------------------------ |
| `destructive` | `#EF4444`     | Red status dot, error/alert    |
| `destructive_foreground` | `#FFFFFF` | Text on destructive backgrounds |

---

## 2. Typography Scale

The UI primarily uses a clean sans-serif and a monospace for data displays.

| Token        | Size (approx.) | Weight | Usage                              |
| ------------ | -------------- | ------ | ---------------------------------- |
| `heading`    | 16px           | 600    | Panel titles                       |
| `subheading` | 13px           | 500    | Section headers, tab labels        |
| `body`       | 12px           | 400    | Body text, form labels, list items |
| `caption`    | 10px           | 400    | Small labels, badge text, metadata |
| `mono`       | 12px           | 400    | Time display, status codes, data   |
| `mono_sm`    | 10px           | 400    | Small data labels                  |

> **egui note**: Uses built-in proportional and monospace fonts (`FontId::proportional(size)` / `FontId::monospace(size)`).

---

## 3. Spacing Patterns

Implemented as `SpacingScale` in `scale.rs`, accessible via `theme.spacing.*`.
All components reference the scale — no hardcoded spacing values.

| Token  | Value | `theme.spacing` | Usage                                                    |
| ------ | ----- | ---------------- | -------------------------------------------------------- |
| `xs`   | 4px   | `.xs`            | Tight gaps, toolbar padding, body offsets, inner margins |
| `sm`   | 8px   | `.sm`            | Input horizontal margin, header gaps, close button hit area |
| `md`   | 12px  | `.md`            | Card/panel inner padding, drag handle zone height        |
| `lg`   | 16px  | `.lg`            | Segmented horizontal pad, close button size, grab bar width |
| `xl`   | 24px  | `.xl`            | Title row height, large section gaps                     |

---

## 4. Border Radii

| Token  | Value (approx.) | Usage                                         |
| ------ | ---------------- | --------------------------------------------- |
| `sm`   | 4px              | Badges, tags, small chips                     |
| `md`   | 6px              | Buttons, input fields, inner control fills    |
| `lg`   | 8px              | Cards, panels, outer control borders          |
| `full` | 9999px           | Circular elements (radio buttons, status dots)|

---

## 5. Border / Stroke Styles

| Style          | Width | Color (approx.)   | Usage                             |
| -------------- | ----- | ------------------ | --------------------------------- |
| Panel border   | 1px   | `#1A2B40`          | Sidebar edges, card outlines      |
| Input border   | 1px   | `#1A2B40`          | Text input field borders          |
| Control border | 1px   | `#3C4656`          | Checkbox/toggle/segmented outer   |
| Input focused  | 1px   | `#4A90CF`          | Focus state ring                  |
| Divider        | 1px   | `#1A2B40` @ 50%    | Horizontal separators             |

---

## 6. Component Inventory

### Controls (Shared Pattern)

Checkbox, toggle, and segmented controls follow a consistent structure:
- **Outer border**: `theme.palette.control_border`, 1px stroke, `theme.radius.lg`
- **Gap**: `theme.control_gap` between outer border and inner fill
- **Inner fill**: `theme.radius.md`, `theme.palette.control_fill_off` (OFF) / `theme.palette.control_fill_on` (ON)
- **Exception**: Checkbox has no inner fill when unchecked (transparent); fill fades in with the checkmark.

### Buttons
- **Primary**: Frost/silver fill (`#CCD4E2`), dark text. Rounded ~6px.
- **Secondary / Ghost**: Dark fill or transparent, light border, light text.

### Badges / Tags
- **Accent (blue)**: Ring color border + text (`#4A90CF`). Rounded ~4px.
- **Primary**: Primary fill with dark text. Rounded ~4px.
- **Outlined**: Primary border + text on transparent bg.

### Text Inputs
- Dark background (`#111D2E`), 1px border (`#1A2B40`), rounded ~6px.
- Label above in secondary text color.
- Inner padding: 8px horizontal, 6px vertical.

### Sidebar Cards
- Semi-transparent backdrop (`surface_blur`).
- 1px border, rounded ~8px. Open/close animation: slide from left + fade in (0.15s).
- Top handle zone: 3 dots animate into a grab bar on hover (0.15s transition).
- Title row with close button (X). Drag handle excludes close button area.
- On drag or highlight: border glows to `ring` color (1.0→1.8px width), outer halo expands 4px. Glow driven by `max(drag_t, highlight_t)`.
- Scrollable body content area with inner margin.
- Can be docked (attached to toolbar) or floating (detached via drag).
- Floating cards have z-ordering: last in Vec renders on top. Click/drag brings to front.
- Toolbar icons show `ring` color for buttons with floating cards.
- Clicking toolbar button for already-parked card: highlight flash (0.3s) + bring to front.
- Global drag fade: all UI elements (toolbar, card) fade to 15% opacity (0.15s).

### Toolbar (Left Sidebar)
- Vertical icon strip with grouped items separated by 1px dividers.
- Semi-transparent backdrop (`surface_blur`), 1px `border` stroke, `lg` radius.
- Active item: `control_fill_on` background, `md` radius.
- Hover item: `control_fill_off` background.
- Icons: 18px Lucide icon font. Optional notification badge dots (3px radius).
- Button size: 36×36px, `xs` padding around strip.

### Top Toolbar
- Horizontal bar with semi-transparent backdrop (`surface_blur`), 1px `border` stroke, `lg` radius.
- Sections separated by 1px vertical dividers with `sm` margin top/bottom.
- Title: proportional 18px, `foreground` color.
- Clock: monospace 13px, `foreground` color.
- QNH/TL: label in `muted_foreground` (proportional 12px), value in `foreground` (monospace 12px).
- Error indicator: Lucide `circle-x` icon (14px) + text in `destructive` color.
- Icon buttons: 28×28px, 16px icon size, hover highlight with `control_fill_off`.
- Height: 36px, horizontal padding: `sm`.

### Zoom Toolbar
- Vertical strip with +/− icon buttons, separator, and "Reset" text button.
- Semi-transparent backdrop (`surface_blur`), 1px `border` stroke, `lg` radius.
- Icon buttons: 36×36px, 18px Lucide icon size.
- Reset button: proportional 10px text, 28px height.
- Hover highlight: `control_fill_off` background, `md` radius.
- `xs` padding around strip.

### Segmented Control
- Pill-shaped container with `control_border` border.
- Active segment: `control_gap` inset with `control_fill_on` fill.

### Accordion
- Borderless collapsible sections — no outer card, no borders, no dividers.
- Animated triangle indicator: rotates right → down. Color transitions `muted_foreground` → `ring`.
- Title color transitions `muted_foreground` → `foreground` on open.
- Body height is animated (clipped during transition). Content fades in/out with opacity.
- Hover highlight: `muted` fill with `md` border radius.
- Header height: `sm * 2 + 16px`. Body margin: `md` horizontal, `sm` vertical.
- `exclusive` mode: only one section open at a time.
- Supports nesting (accordion inside accordion) via `FnMut` body callback.
- Animation duration: 0.15s.

### Tabs
- Horizontal underline-style tab bar matching the Figma card mockup.
- `tabs(ui, theme, selected, labels)` — basic variant.
- `tabs_with_icons(ui, theme, selected, labels, icons)` — optional per-tab Lucide icons (rendered in icon font, `sm` gap to text, color transitions `muted_foreground` → `ring`).
- Active tab: `foreground` text, `ring`-colored underline (1.5px) animating from center outward.
- Inactive tab: `muted_foreground` text, no underline. Hover brightens to 50% towards `foreground`.
- Bottom border: 1px `border` color across full width.
- Tab padding: `md` horizontal, `sm` vertical. Gap between tabs: `xs`.
- Font: proportional 13px. Icon font: 13px.
- Animation duration: 0.12s.
- Click mutation is deferred to avoid ID instability between egui layout passes.

### Checkbox
- `checkbox(ui, theme, checked, label)` — default size (22px box, 13px font).
- `checkbox_small(ui, theme, checked, label)` — compact variant (22px box, 11px font).

### Icons
- Lucide icon font v1.7.0 (TTF, embedded via `include_bytes!`).
- Registered as custom `FontFamily::Name("lucide")` in egui.
- Named codepoint constants: `ICON_MAP`, `ICON_LAYERS`, `ICON_SETTINGS`, `ICON_CIRCLE_X`, `ICON_SNOWFLAKE`, etc.

---

## 7. Semantic Token Mapping (for `ColorPalette` struct)

| Struct Field            | Source Value       | Notes                                        |
| ----------------------- | ------------------ | -------------------------------------------- |
| `background`            | `#04070E`          | Figma-confirmed                              |
| `foreground`            | `#E0E8F0`          | Primary text                                 |
| `primary`               | `#CCD4E2`          | Frost/silver accent                          |
| `primary_foreground`    | `#0E1724`          | Dark text on primary                         |
| `secondary`             | `#141E2E`          | Subtle dark fills                            |
| `secondary_foreground`  | `#8A96A8`          | Muted text on secondary                      |
| `secondary_border`      | `#3D2D55`          | Violet-tinted border                         |
| `muted`                 | `#111D2E`          | Input backgrounds, subtle surfaces           |
| `muted_foreground`      | `#5A6A7E`          | Placeholder/disabled text                    |
| `accent`                | `#0F1E3D`          | Dark navy accent                             |
| `accent_foreground`     | `#E0E8F0`          | Text on accent                               |
| `destructive`           | `#EF4444`          | Error/alert                                  |
| `destructive_foreground`| `#FFFFFF`          | Text on destructive                          |
| `border`                | `#1A2B40`          | All borders                                  |
| `input`                 | `#1A2B40`          | Input borders (= border)                     |
| `ring`                  | `#4A90CF`          | Focus ring, accent badge color               |
| `card`                  | `#0C1622`          | Card/panel background                        |
| `card_foreground`       | `#E0E8F0`          | Text on cards                                |
| `popover`               | `#0C1622`          | Popover/dialog background                    |
| `popover_foreground`    | `#E0E8F0`          | Text on popovers                             |
| `surface_blur`          | `#060C16` @ 85%    | Semi-transparent backdrop                    |
| `surface_blur_radius`   | `16.0`             | Default blur radius (px, reserved for future) |
| `control_border`        | `#3C4656`          | Outer border for checkbox/toggle/segmented   |
| `control_fill_off`      | `#0E1A38`          | Inner fill when control is OFF               |
| `control_fill_on`       | `#162C59`          | Inner fill when control is ON/active         |

---

## Tokens Needing Light-Mode Counterparts (Future)

All color tokens above are dark-mode values. The `ColorPalette` struct is theme-agnostic in structure — a future `ColorPalette::light()` would be a second constructor with different values.

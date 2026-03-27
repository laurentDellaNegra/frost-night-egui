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
| `surface_blur`      | `#0C1622` at **85% opacity** | Semi-transparent card/panel backdrop                 |

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

Checkbox, toggle, and segmented controls share a consistent visual pattern:

| Token               | Hex       | Usage                                             |
| -------------------- | --------- | ------------------------------------------------- |
| `outer_border`       | `#3C4656` | Outer border stroke for checkboxes, toggles, segmented |
| `inner_fill_off`     | `#0E1A38` | Inner fill when control is OFF                    |
| `inner_fill_on`      | `#162C59` | Inner fill when control is ON                     |

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

| Token  | Value (approx.) | Usage                                            |
| ------ | ---------------- | ------------------------------------------------ |
| `xs`   | 4px              | Tight gaps (between icon and label)              |
| `sm`   | 8px              | Standard inner padding, gaps between list items  |
| `md`   | 12px             | Panel inner padding, form field spacing          |
| `lg`   | 16px             | Section spacing, panel outer margins             |
| `xl`   | 24px             | Large section gaps, dialog padding               |

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
- **Outer border**: `#3C4656`, 1px stroke, `lg` radius (8px)
- **Gap**: 3px between outer border and inner fill
- **Inner fill**: `md` radius (6px), `#0E1A38` (OFF) / `#162C59` (ON)

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

### Cards / Drag Cards
- Semi-transparent backdrop (`surface_blur`).
- 1px border, rounded ~8px.
- Drag handle at top, close button.
- Fades to 15% opacity when being dragged.

### Segmented Control
- Pill-shaped container with `#3C4656` border.
- Active segment: 3px inset with `#162C59` fill.

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
| `surface_blur`          | `#0C1622` @ 85%    | Semi-transparent backdrop                    |
| `surface_blur_radius`   | `16.0`             | Default blur radius (px, reserved for future) |

---

## Tokens Needing Light-Mode Counterparts (Future)

All color tokens above are dark-mode values. The `ColorPalette` struct is theme-agnostic in structure — a future `ColorPalette::light()` would be a second constructor with different values.

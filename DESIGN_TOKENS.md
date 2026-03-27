# SkyScope Design Tokens

Extracted from Figma mockups (visual analysis of `interface.png` and `windows.png`).
Figma project: **SKY SkyScope UI** by Skyguide.

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
| `surface_blur`      | `#0C1622` at **75% opacity** | Glassmorphism panels (dialogs, popovers, floating windows) |

### Foregrounds (Text)

| Token                  | Hex (approx.) | Usage                                 |
| ---------------------- | ------------- | ------------------------------------- |
| `foreground`           | `#E0E8F0`     | Primary text, headings                |
| `foreground_secondary` | `#8494A7`     | Labels, secondary text                |
| `foreground_muted`     | `#5A6A7E`     | Placeholder text, disabled text       |

### Primary (Gold/Amber Accent)

| Token                 | Hex (approx.) | Usage                                            |
| --------------------- | ------------- | ------------------------------------------------ |
| `primary`             | `#C9A54C`     | Active tabs, primary buttons, filled badges, radio fills |
| `primary_foreground`  | `#0C1622`     | Text on primary-colored backgrounds              |

### Secondary

| Token                   | Hex (approx.) | Usage                                    |
| ----------------------- | ------------- | ---------------------------------------- |
| `secondary`             | `#182536`     | Secondary button fills, inactive tabs    |
| `secondary_foreground`  | `#E0E8F0`     | Text on secondary backgrounds            |

### Accent (Blue/Cyan)

| Token               | Hex (approx.) | Usage                                 |
| -------------------- | ------------- | ------------------------------------- |
| `accent`             | `#4A90CF`     | Outlined badges/tags, info highlights |
| `accent_foreground`  | `#E0E8F0`     | Text on accent backgrounds            |

### Borders & Dividers

| Token    | Hex (approx.) | Usage                                    |
| -------- | ------------- | ---------------------------------------- |
| `border` | `#1A2B40`     | Panel borders, input borders, dividers   |
| `ring`   | `#C9A54C`     | Focus ring (matches primary)             |
| `input`  | `#1A2B40`     | Input field borders (same as border)     |

### Status / Semantic

| Token         | Hex (approx.) | Usage                          |
| ------------- | ------------- | ------------------------------ |
| `success`     | `#22C55E`     | Green status dot, online state |
| `destructive` | `#EF4444`     | Red status dot, error/alert, emergency |
| `destructive_foreground` | `#FFFFFF` | Text on destructive backgrounds |

### Glassmorphism / Blur

| Token                | Value           | Usage                                      |
| -------------------- | --------------- | ------------------------------------------ |
| `surface_blur_tint`  | `#0C1622` @ 75% | Semi-transparent overlay on blurred panels |
| `surface_blur_radius`| `16px`          | Backdrop blur radius for frosted glass     |
| `surface_blur_border`| `#1A2B40` @ 50% | Subtle border on glassmorphism panels      |

---

## 2. Typography Scale

**Figma loads**: Inter (400, 600), Apercu (200, 400, 500), Apercu-Mono (400), Roboto (300–700), Roboto Mono (400, 700).

From visual analysis, the UI primarily uses a clean sans-serif (likely **Inter** or **Apercu**) and a monospace for data displays.

| Token        | Size (approx.) | Weight | Family          | Usage                              |
| ------------ | -------------- | ------ | --------------- | ---------------------------------- |
| `heading`    | 16px           | 600    | Inter / Apercu  | Panel titles ("Maps", "Placeholder Text") |
| `subheading` | 13px           | 500    | Inter / Apercu  | Section headers, tab labels        |
| `body`       | 12px           | 400    | Inter / Apercu  | Body text, form labels, list items |
| `caption`    | 10px           | 400    | Inter / Apercu  | Small labels, badge text, metadata |
| `mono`       | 12px           | 400    | Apercu-Mono     | Time display, status codes, data   |
| `mono_sm`    | 10px           | 400    | Apercu-Mono     | Small data labels ("FLYPRO", "ENRR") |

**Line height**: Approximately 1.3–1.5x font size across the board.

> **egui note**: egui doesn't support Apercu out of the box. For the crate, default to the egui built-in proportional font (which is similar to Inter). We can add font loading as an optional feature. Use `FontId::proportional(size)` and `FontId::monospace(size)`.

---

## 3. Spacing Patterns

| Token  | Value (approx.) | Usage                                            |
| ------ | ---------------- | ------------------------------------------------ |
| `xs`   | 4px              | Tight gaps (between icon and label, badge padding inline) |
| `sm`   | 8px              | Standard inner padding, gaps between list items  |
| `md`   | 12px             | Panel inner padding, form field spacing          |
| `lg`   | 16px             | Section spacing, panel outer margins             |
| `xl`   | 24px             | Large section gaps, dialog padding               |

---

## 4. Border Radii

| Token  | Value (approx.) | Usage                                         |
| ------ | ---------------- | --------------------------------------------- |
| `sm`   | 4px              | Badges, tags, small chips                     |
| `md`   | 6px              | Buttons, input fields, tabs                   |
| `lg`   | 8px              | Cards, panels, dialogs                        |
| `full` | 9999px           | Circular elements (radio buttons, status dots)|

---

## 5. Border / Stroke Styles

| Style          | Width | Color (approx.)   | Usage                             |
| -------------- | ----- | ------------------ | --------------------------------- |
| Panel border   | 1px   | `#1A2B40`          | Sidebar edges, card outlines      |
| Input border   | 1px   | `#1A2B40`          | Text input field borders          |
| Input focused  | 1px   | `#C9A54C`          | Focus state (primary ring)        |
| Divider        | 1px   | `#1A2B40` @ 50%    | Horizontal separators             |
| Badge outline  | 1px   | varies per variant | Outlined badge/tag borders        |
| Glass border   | 1px   | `#1A2B40` @ 50%    | Glassmorphism panel edges         |

---

## 6. Component Inventory

### Buttons
- **Primary**: Gold/amber fill (`#C9A54C`), dark text. Rounded ~6px.
- **Secondary / Ghost**: Dark fill or transparent, light border, light text.
- **Action bar buttons**: "Return" (secondary), "Done" (secondary) — bottom of sidebar.

### Tabs
- **Active tab**: Gold/amber background, dark text, rounded ~6px.
- **Inactive tab**: Transparent/dark background, muted text.
- Tab group appears pill-shaped with a subtle container background.

### Text Inputs
- Dark background (`#111D2E`), 1px border (`#1A2B40`), rounded ~6px.
- Label above in secondary text color.
- Placeholder text in muted color.
- Focus state: border changes to primary gold.

### Radio Buttons
- Unchecked: Dark circle with border.
- Checked: Gold/amber filled circle with inner dot.
- Paired with label text.

### Checkboxes
- Seen in the windows mockup.
- Unchecked: Dark rounded square with border.
- Checked: Likely gold/amber fill with checkmark.

### Badges / Tags
- **Outlined (blue)**: Blue border + blue text on transparent bg (`#4A90CF`). Rounded ~4px.
- **Filled (gold)**: Gold background with dark text. Rounded ~4px.
- **Outlined (gold)**: Gold border + gold text on transparent bg.
- Small, compact sizing (~10px text).

### Tree / Accordion
- Expandable sections with disclosure arrow (▸ / ▾).
- Section header in secondary text with count badge.
- Child items are radio buttons or checkboxes with labels.

### Cards / Panels
- Opaque dark surface background.
- 1px border, rounded ~8px.
- Inner padding ~12–16px.

### Toolbar (Left Icon Bar)
- Vertical strip of icon buttons.
- Dark background (slightly darker than panels).
- Icons in muted color, likely highlight on hover/active.

### Top Bar / Header
- Full-width bar at top.
- Contains: logo, branding text, time display, status indicators, action buttons.
- Dark background blending with main bg.

### Status Indicators
- Small colored dots: green (online/active), red (error/alert).
- Paired with label text.

### Close Button (X)
- Top-right of panels/dialogs.
- Light icon on transparent bg, hover likely lightens.

---

## 7. Surface / Overlay Styles (Glassmorphism)

Observed in `windows.png` — three overlapping "Maps" panels showing clear glassmorphism:

| Component          | Bg Color    | Opacity | Blur Radius | Border               | Corner Radius |
| ------------------ | ----------- | ------- | ----------- | -------------------- | ------------- |
| **Dialog / Window**| `#0C1622`   | ~75%    | ~16px       | 1px `#1A2B40` @ 50%  | 8px           |
| **Sidebar Panel**  | `#0C1622`   | ~90%    | ~12px       | 1px `#1A2B40`        | 0px (docked)  |
| **Popover**        | `#0C1622`   | ~80%    | ~16px       | 1px `#1A2B40` @ 50%  | 8px           |
| **Top Bar**        | `#04070E`   | ~90%    | ~8px        | none visible         | 0px           |
| **Tooltip**        | `#111D2E`   | ~95%    | ~4px        | 1px `#1A2B40`        | 4px           |

The glassmorphism is most pronounced in the floating "Maps" windows where the underlying map content is clearly visible through the panel backgrounds. The blur appears to be a moderate Gaussian (~16px radius) with a dark navy tint overlay.

---

## Semantic Token Mapping (for `ColorPalette` struct)

| Struct Field            | Source Value       | Notes                                        |
| ----------------------- | ------------------ | -------------------------------------------- |
| `background`            | `#04070E`          | Figma-confirmed                              |
| `foreground`            | `#E0E8F0`          | Primary text                                 |
| `primary`               | `#C9A54C`          | Gold/amber accent                            |
| `primary_foreground`    | `#0C1622`          | Dark text on gold                            |
| `secondary`             | `#182536`          | Subtle dark fills                            |
| `secondary_foreground`  | `#E0E8F0`          | Light text on secondary                      |
| `muted`                 | `#111D2E`          | Input backgrounds, subtle surfaces           |
| `muted_foreground`      | `#5A6A7E`          | Placeholder/disabled text                    |
| `accent`                | `#4A90CF`          | Blue highlights                              |
| `accent_foreground`     | `#E0E8F0`          | Text on accent                               |
| `destructive`           | `#EF4444`          | Error/alert/emergency                        |
| `destructive_foreground`| `#FFFFFF`          | Text on destructive                          |
| `border`                | `#1A2B40`          | All borders                                  |
| `input`                 | `#1A2B40`          | Input borders (= border)                     |
| `ring`                  | `#C9A54C`          | Focus ring (= primary)                       |
| `card`                  | `#0C1622`          | Card/panel background                        |
| `card_foreground`       | `#E0E8F0`          | Text on cards                                |
| `popover`               | `#0C1622`          | Popover/dialog background (with blur)        |
| `popover_foreground`    | `#E0E8F0`          | Text on popovers                             |
| `surface_blur`          | `#0C1622` @ 75%    | Glassmorphism tint                           |
| `surface_blur_radius`   | `16.0`             | Default blur radius (px)                     |

**Total: 20 color tokens + 2 blur tokens** — within the ~20 token target.

---

## Tokens Needing Light-Mode Counterparts (Future)

All color tokens above are dark-mode values. For a future light mode, the following would need counterparts:
- `background` → light: ~`#F5F7FA`
- `foreground` → light: ~`#1A1A2E`
- `surface` / `card` → light: ~`#FFFFFF`
- `primary` → likely stays the same gold (or slightly darker for contrast)
- `border` → light: ~`#D0D5DD`
- `muted` → light: ~`#F0F2F5`
- Blur tint → light: `#FFFFFF` @ 70%

The `mix()` hover/active derivation would also flip: use `BLACK` instead of `WHITE` as the mix target.

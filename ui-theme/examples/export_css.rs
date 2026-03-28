use egui::Color32;
use ui_theme::Theme;

fn c(color: Color32) -> String {
    if color.a() == 255 {
        format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b())
    } else {
        format!(
            "rgba({}, {}, {}, {:.2})",
            color.r(),
            color.g(),
            color.b(),
            color.a() as f32 / 255.0
        )
    }
}

fn main() {
    let theme = Theme::dark();
    let p = &theme.palette;
    let r = &theme.radius;
    let s = &theme.spacing;

    println!(":root {{");

    // Palette colors
    println!("  --background: {};", c(p.background));
    println!("  --foreground: {};", c(p.foreground));
    println!("  --primary: {};", c(p.primary));
    println!("  --primary-foreground: {};", c(p.primary_foreground));
    println!("  --secondary: {};", c(p.secondary));
    println!("  --secondary-foreground: {};", c(p.secondary_foreground));
    println!("  --secondary-border: {};", c(p.secondary_border));
    println!("  --muted: {};", c(p.muted));
    println!("  --muted-foreground: {};", c(p.muted_foreground));
    println!("  --accent: {};", c(p.accent));
    println!("  --accent-foreground: {};", c(p.accent_foreground));
    println!("  --destructive: {};", c(p.destructive));
    println!("  --destructive-foreground: {};", c(p.destructive_foreground));
    println!("  --border: {};", c(p.border));
    println!("  --input: {};", c(p.input));
    println!("  --ring: {};", c(p.ring));
    println!("  --card: {};", c(p.card));
    println!("  --card-foreground: {};", c(p.card_foreground));
    println!("  --popover: {};", c(p.popover));
    println!("  --popover-foreground: {};", c(p.popover_foreground));
    println!("  --surface-blur: {};", c(p.surface_blur));
    println!("  --surface-blur-radius: {}px;", p.surface_blur_radius);
    println!("  --control-border: {};", c(p.control_border));
    println!("  --control-fill-off: {};", c(p.control_fill_off));
    println!("  --control-fill-on: {};", c(p.control_fill_on));

    // Radius scale
    println!("  --radius-sm: {}px;", r.sm);
    println!("  --radius-md: {}px;", r.md);
    println!("  --radius-lg: {}px;", r.lg);

    // Spacing scale
    println!("  --spacing-xs: {}px;", s.xs);
    println!("  --spacing-sm: {}px;", s.sm);
    println!("  --spacing-md: {}px;", s.md);
    println!("  --spacing-lg: {}px;", s.lg);
    println!("  --spacing-xl: {}px;", s.xl);

    // Control gap
    println!("  --control-gap: {}px;", theme.control_gap);

    println!("}}");
}

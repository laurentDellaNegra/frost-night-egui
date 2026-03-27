//! Minimal OKLCH color space utilities.
//!
//! OKLCH provides perceptually uniform color interpolation.
//! Use [`oklch`] to define palette colors from OKLCH coordinates.

use egui::Color32;

/// Convert OKLCH (Lightness, Chroma, Hue) to [`Color32`].
///
/// - `l`: Lightness, 0.0 (black) to 1.0 (white)
/// - `c`: Chroma, 0.0 (gray) to ~0.4 (vivid)
/// - `h`: Hue angle in degrees, 0.0 to 360.0
pub fn oklch(l: f32, c: f32, h: f32) -> Color32 {
    let h_rad = h.to_radians();
    let a = c * h_rad.cos();
    let b = c * h_rad.sin();
    oklab_to_color32(l, a, b)
}

/// Convert OKLab to [`Color32`].
fn oklab_to_color32(l: f32, a: f32, b: f32) -> Color32 {
    // OKLab → LMS (approximate inverse)
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

    let l3 = l_ * l_ * l_;
    let m3 = m_ * m_ * m_;
    let s3 = s_ * s_ * s_;

    // LMS → linear sRGB
    let r = 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
    let g = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
    let bl = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3;

    Color32::from_rgb(linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(bl))
}

/// Linear RGB component to sRGB gamma-encoded byte.
fn linear_to_srgb(x: f32) -> u8 {
    let y = if x <= 0.0031308 {
        12.92 * x
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    };
    (y.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

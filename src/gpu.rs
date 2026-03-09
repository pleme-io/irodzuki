//! GPU-compatible color conversions for wgpu render pipelines.

use crate::scheme::{Color, ColorScheme};

/// Pre-computed GPU color values for a render pipeline.
///
/// All colors are in linear sRGB space (not gamma-corrected),
/// ready for use as `wgpu::Color` or shader uniforms.
pub struct GpuColors {
    /// Background clear color.
    pub clear: [f64; 4],
    /// Text foreground.
    pub text: [f32; 4],
    /// Accent/highlight.
    pub accent: [f32; 4],
    /// Error indicators.
    pub error: [f32; 4],
    /// Selection background.
    pub selection: [f32; 4],
    /// Cursor color.
    pub cursor: [f32; 4],
    /// All 16 base16 colors as [f32; 4] arrays.
    pub palette: [[f32; 4]; 16],
}

impl GpuColors {
    /// Compute GPU colors from a color scheme.
    #[must_use]
    pub fn from_scheme(scheme: &ColorScheme) -> Self {
        let bg = scheme.base00;
        Self {
            clear: [
                f64::from(bg.r),
                f64::from(bg.g),
                f64::from(bg.b),
                f64::from(bg.a),
            ],
            text: scheme.base05.to_array(),
            accent: scheme.base0d.to_array(),
            error: scheme.base08.to_array(),
            selection: scheme.base02.to_array(),
            cursor: scheme.base04.to_array(),
            palette: std::array::from_fn(|i| {
                scheme.get(crate::scheme::Base16Slot::ALL[i]).to_array()
            }),
        }
    }
}

/// Convert sRGB gamma to linear for GPU pipelines.
#[must_use]
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert linear to sRGB gamma.
#[must_use]
pub fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Convert a Color from sRGB to linear space.
#[must_use]
pub fn color_to_linear(c: &Color) -> Color {
    Color::new(srgb_to_linear(c.r), srgb_to_linear(c.g), srgb_to_linear(c.b), c.a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheme::ColorScheme;

    #[test]
    fn gpu_colors_from_default_scheme() {
        let scheme = ColorScheme::default();
        let gpu = GpuColors::from_scheme(&scheme);
        assert_eq!(gpu.palette.len(), 16);
        // Clear color should be base00
        #[expect(clippy::cast_possible_truncation)]
        let clear_r = gpu.clear[0] as f32;
        assert!((clear_r - scheme.base00.r).abs() < 0.001);
    }

    #[test]
    fn srgb_linear_roundtrip() {
        let original = 0.5_f32;
        let linear = srgb_to_linear(original);
        let back = linear_to_srgb(linear);
        assert!((original - back).abs() < 0.001);
    }

    #[test]
    fn srgb_black_is_zero() {
        assert!(srgb_to_linear(0.0).abs() < 0.0001);
    }

    #[test]
    fn srgb_white_is_one() {
        assert!((srgb_to_linear(1.0) - 1.0).abs() < 0.001);
    }
}

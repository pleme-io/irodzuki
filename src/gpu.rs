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

    #[test]
    fn srgb_linear_branch_boundary() {
        // The piecewise srgb_to_linear formula changes at 0.04045.
        // Both branches must produce values close together at the
        // boundary — a regression in either arm would show up as a
        // discontinuity here.
        let just_below = srgb_to_linear(0.04044);
        let just_above = srgb_to_linear(0.04046);
        assert!(just_below < just_above);
        assert!((just_above - just_below).abs() < 0.001);
    }

    #[test]
    fn linear_srgb_branch_boundary() {
        // linear_to_srgb changes at 0.0031308.
        let just_below = linear_to_srgb(0.003);
        let just_above = linear_to_srgb(0.0032);
        assert!(just_below < just_above);
        assert!((just_above - just_below).abs() < 0.01);
    }

    #[test]
    fn srgb_linear_monotonic() {
        // srgb_to_linear must be strictly monotonic: feeding it
        // increasing values yields increasing outputs. Catches sign
        // flips in the power exponent.
        let samples = [0.0_f32, 0.04, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];
        let mut last = f32::NEG_INFINITY;
        for s in samples {
            let v = srgb_to_linear(s);
            assert!(v >= last, "srgb_to_linear not monotonic at {s}: {last} -> {v}");
            last = v;
        }
    }

    #[test]
    fn linear_srgb_round_trip_across_range() {
        // Sample across the whole [0, 1] range — both piecewise
        // arms and the boundary region — and confirm every value
        // round-trips with <0.01 absolute error.
        for i in 0..=100 {
            #[expect(clippy::cast_precision_loss)]
            let v = i as f32 / 100.0;
            let rt = linear_to_srgb(srgb_to_linear(v));
            assert!(
                (v - rt).abs() < 0.01,
                "round-trip failed at {v}: got {rt} (delta {})",
                (v - rt).abs()
            );
        }
    }

    #[test]
    fn color_to_linear_preserves_alpha() {
        // Only RGB should be gamma-corrected; alpha passes through
        // verbatim because it encodes opacity, not light.
        let c = Color::new(0.5, 0.5, 0.5, 0.7);
        let linear = color_to_linear(&c);
        assert!((linear.a - 0.7).abs() < f32::EPSILON);
        // RGB is NOT identity — if it were, the function is silently
        // broken.
        assert!((linear.r - c.r).abs() > 0.01);
    }

    #[test]
    fn color_to_linear_black_and_white_fixed_points() {
        // Pure black and white map to themselves (the sRGB curve is
        // the identity at the endpoints).
        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        let white = Color::new(1.0, 1.0, 1.0, 1.0);
        let linear_black = color_to_linear(&black);
        let linear_white = color_to_linear(&white);
        assert!(linear_black.r.abs() < f32::EPSILON);
        assert!((linear_white.r - 1.0).abs() < 0.001);
    }

    #[test]
    fn gpu_clear_alpha_is_opaque() {
        // The clear color's alpha must be 1.0 — a transparent clear
        // causes frame tearing in composited wgpu surfaces.
        let scheme = ColorScheme::default();
        let gpu = GpuColors::from_scheme(&scheme);
        #[expect(clippy::cast_possible_truncation)]
        let alpha = gpu.clear[3] as f32;
        assert!((alpha - 1.0).abs() < 0.001);
    }

    #[test]
    fn gpu_palette_matches_slot_order() {
        // GpuColors.palette[i] must equal scheme.get(Base16Slot::ALL[i]).
        // A palette-ordering bug shows up here *and* in every
        // terminal/editor that relies on the mapping.
        use crate::scheme::Base16Slot;
        let scheme = ColorScheme::default();
        let gpu = GpuColors::from_scheme(&scheme);
        for (i, slot) in Base16Slot::ALL.iter().enumerate() {
            assert_eq!(
                gpu.palette[i],
                scheme.get(*slot).to_array(),
                "palette index {i} doesn't match slot {slot:?}"
            );
        }
    }

    #[test]
    fn gpu_semantic_slots_map_correctly() {
        // The semantic colors (text/accent/error/selection/cursor)
        // must pull from base05/0d/08/02/04 respectively. A mistake
        // here would paint errors in the accent colour, etc.
        let scheme = ColorScheme::default();
        let gpu = GpuColors::from_scheme(&scheme);
        assert_eq!(gpu.text, scheme.base05.to_array());
        assert_eq!(gpu.accent, scheme.base0d.to_array());
        assert_eq!(gpu.error, scheme.base08.to_array());
        assert_eq!(gpu.selection, scheme.base02.to_array());
        assert_eq!(gpu.cursor, scheme.base04.to_array());
    }
}

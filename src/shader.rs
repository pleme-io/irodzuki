//! Shader uniform generation for WGSL post-processing.
//!
//! Generates a uniform buffer that WGSL shaders can read
//! for theme-aware color grading, tinting, and effects.

use crate::scheme::ColorScheme;

/// Theme uniforms for WGSL shaders.
///
/// Layout matches this WGSL struct:
/// ```wgsl
/// struct ThemeUniforms {
///     background: vec4<f32>,
///     foreground: vec4<f32>,
///     accent: vec4<f32>,
///     error: vec4<f32>,
///     is_dark: f32,
///     _pad: vec3<f32>,
/// }
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ThemeUniforms {
    pub background: [f32; 4],
    pub foreground: [f32; 4],
    pub accent: [f32; 4],
    pub error: [f32; 4],
    pub is_dark: f32,
    pub _pad: [f32; 3],
}

impl ThemeUniforms {
    /// Generate shader uniforms from a color scheme.
    #[must_use]
    pub fn from_scheme(scheme: &ColorScheme) -> Self {
        Self {
            background: scheme.base00.to_array(),
            foreground: scheme.base05.to_array(),
            accent: scheme.base0d.to_array(),
            error: scheme.base08.to_array(),
            is_dark: if scheme.is_dark() { 1.0 } else { 0.0 },
            _pad: [0.0; 3],
        }
    }

    /// Get as raw bytes for wgpu buffer upload.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

/// WGSL snippet that declares the `ThemeUniforms` struct.
/// Apps can include this in their shaders.
pub const THEME_UNIFORMS_WGSL: &str = r"
struct ThemeUniforms {
    background: vec4<f32>,
    foreground: vec4<f32>,
    accent: vec4<f32>,
    error_color: vec4<f32>,
    is_dark: f32,
    _pad: vec3<f32>,
}
";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheme::ColorScheme;

    #[test]
    fn uniforms_from_default() {
        let scheme = ColorScheme::default();
        let u = ThemeUniforms::from_scheme(&scheme);
        assert_eq!(u.is_dark, 1.0); // Nord is dark
        assert_eq!(u.background, scheme.base00.to_array());
    }

    #[test]
    fn uniforms_size_is_aligned() {
        // GPU uniform buffers need 16-byte alignment
        assert_eq!(std::mem::size_of::<ThemeUniforms>() % 16, 0);
    }

    #[test]
    fn wgsl_snippet_contains_struct() {
        assert!(THEME_UNIFORMS_WGSL.contains("ThemeUniforms"));
        assert!(THEME_UNIFORMS_WGSL.contains("background"));
        assert!(THEME_UNIFORMS_WGSL.contains("is_dark"));
    }

    #[test]
    fn uniforms_non_dark_scheme_zero_flag() {
        // is_dark must be 0.0 for light schemes. Shaders use this
        // flag to pick between light/dark rendering paths — getting
        // it wrong means the wrong contrast decisions downstream.
        let light = ColorScheme {
            base00: crate::scheme::Color::new(1.0, 1.0, 1.0, 1.0),
            ..ColorScheme::default()
        };
        let u = ThemeUniforms::from_scheme(&light);
        assert_eq!(u.is_dark, 0.0);
    }

    #[test]
    fn uniforms_as_bytes_length_matches_size() {
        // wgpu queue.write_buffer uses the byte slice length — a
        // truncated or over-long slice would corrupt the uniform
        // buffer silently.
        let u = ThemeUniforms::from_scheme(&ColorScheme::default());
        assert_eq!(u.as_bytes().len(), std::mem::size_of::<ThemeUniforms>());
    }

    #[test]
    fn uniforms_pod_bytes_roundtrip() {
        // bytemuck round-trip: bytes_of + from_bytes must recover
        // the original struct. If field ordering or padding ever
        // changes, this catches it before the GPU does.
        let u = ThemeUniforms::from_scheme(&ColorScheme::default());
        let bytes = bytemuck::bytes_of(&u);
        let u2: &ThemeUniforms = bytemuck::from_bytes(bytes);
        assert_eq!(u2.background, u.background);
        assert_eq!(u2.foreground, u.foreground);
        assert_eq!(u2.accent, u.accent);
        assert_eq!(u2.error, u.error);
        assert_eq!(u2.is_dark, u.is_dark);
        assert_eq!(u2._pad, u._pad);
    }

    #[test]
    fn uniforms_zeroable() {
        // Zeroable is required for safe wgpu buffer init; make sure
        // the trait impl actually produces an all-zero struct.
        let z = <ThemeUniforms as bytemuck::Zeroable>::zeroed();
        assert_eq!(z.background, [0.0; 4]);
        assert_eq!(z.is_dark, 0.0);
    }

    #[test]
    fn wgsl_snippet_declares_every_cpu_field() {
        // Each CPU-side field name (or a documented alias) must
        // appear in the WGSL snippet. Keeps the two sides in sync
        // — if someone renames a field in Rust but forgets the
        // shader, this test fires.
        for name in ["background", "foreground", "accent", "is_dark", "_pad"] {
            assert!(
                THEME_UNIFORMS_WGSL.contains(name),
                "WGSL snippet missing field `{name}`"
            );
        }
    }

    #[test]
    fn uniforms_size_is_at_least_four_vec4s_plus_scalar() {
        // 4 vec4<f32> (64 B) + f32 is_dark (4 B) + vec3<f32> _pad
        // (12 B) = 80 B. The layout must accommodate at least that.
        assert!(std::mem::size_of::<ThemeUniforms>() >= 80);
    }
}

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
}

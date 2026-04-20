//! Base16 color scheme with semantic aliases and conversion utilities.

use crate::error::{IrodzukiError, Result};
use serde::{Deserialize, Serialize};

/// A single RGBA color (0.0-1.0 per channel).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Parse a hex string like "#2E3440" or "2E3440".
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(IrodzukiError::InvalidColor(format!(
                "expected 6 hex chars, got {}",
                hex.len()
            )));
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| IrodzukiError::InvalidColor(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| IrodzukiError::InvalidColor(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| IrodzukiError::InvalidColor(hex.to_string()))?;
        Ok(Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: 1.0,
        })
    }

    /// Convert to hex string "#RRGGBB".
    #[must_use]
    pub fn to_hex(&self) -> String {
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let r = (self.r * 255.0).round() as u8;
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let g = (self.g * 255.0).round() as u8;
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let b = (self.b * 255.0).round() as u8;
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    /// Convert to [f32; 4] array (for garasu/egaku).
    #[must_use]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Blend two colors by factor t (0.0 = self, 1.0 = other).
    #[must_use]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Perceived luminance (sRGB approximate).
    #[must_use]
    pub fn luminance(&self) -> f32 {
        0.2126f32.mul_add(self.r, 0.7152f32.mul_add(self.g, 0.0722 * self.b))
    }

    /// Whether this color is "dark" (luminance < 0.5).
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.luminance() < 0.5
    }
}

impl From<[f32; 4]> for Color {
    fn from(arr: [f32; 4]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        c.to_array()
    }
}

/// Base16 color slot identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Base16Slot {
    Base00,
    Base01,
    Base02,
    Base03,
    Base04,
    Base05,
    Base06,
    Base07,
    Base08,
    Base09,
    Base0A,
    Base0B,
    Base0C,
    Base0D,
    Base0E,
    Base0F,
}

impl Base16Slot {
    /// All 16 slots in order.
    pub const ALL: [Self; 16] = [
        Self::Base00,
        Self::Base01,
        Self::Base02,
        Self::Base03,
        Self::Base04,
        Self::Base05,
        Self::Base06,
        Self::Base07,
        Self::Base08,
        Self::Base09,
        Self::Base0A,
        Self::Base0B,
        Self::Base0C,
        Self::Base0D,
        Self::Base0E,
        Self::Base0F,
    ];
}

/// A complete base16 color scheme with semantic aliases.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorScheme {
    pub name: String,
    pub author: String,

    // Base16 palette
    pub base00: Color, // default bg
    pub base01: Color, // lighter bg
    pub base02: Color, // selection bg
    pub base03: Color, // comments
    pub base04: Color, // dark fg
    pub base05: Color, // default fg
    pub base06: Color, // light fg
    pub base07: Color, // lightest fg
    pub base08: Color, // red
    pub base09: Color, // orange
    pub base0a: Color, // yellow
    pub base0b: Color, // green
    pub base0c: Color, // cyan
    pub base0d: Color, // blue
    pub base0e: Color, // purple
    pub base0f: Color, // brown
}

impl ColorScheme {
    /// Get a color by base16 slot.
    #[must_use]
    pub fn get(&self, slot: Base16Slot) -> Color {
        match slot {
            Base16Slot::Base00 => self.base00,
            Base16Slot::Base01 => self.base01,
            Base16Slot::Base02 => self.base02,
            Base16Slot::Base03 => self.base03,
            Base16Slot::Base04 => self.base04,
            Base16Slot::Base05 => self.base05,
            Base16Slot::Base06 => self.base06,
            Base16Slot::Base07 => self.base07,
            Base16Slot::Base08 => self.base08,
            Base16Slot::Base09 => self.base09,
            Base16Slot::Base0A => self.base0a,
            Base16Slot::Base0B => self.base0b,
            Base16Slot::Base0C => self.base0c,
            Base16Slot::Base0D => self.base0d,
            Base16Slot::Base0E => self.base0e,
            Base16Slot::Base0F => self.base0f,
        }
    }

    /// Parse from 16 hex strings.
    pub fn from_hex_array(name: &str, colors: &[&str; 16]) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            author: String::new(),
            base00: Color::from_hex(colors[0])?,
            base01: Color::from_hex(colors[1])?,
            base02: Color::from_hex(colors[2])?,
            base03: Color::from_hex(colors[3])?,
            base04: Color::from_hex(colors[4])?,
            base05: Color::from_hex(colors[5])?,
            base06: Color::from_hex(colors[6])?,
            base07: Color::from_hex(colors[7])?,
            base08: Color::from_hex(colors[8])?,
            base09: Color::from_hex(colors[9])?,
            base0a: Color::from_hex(colors[10])?,
            base0b: Color::from_hex(colors[11])?,
            base0c: Color::from_hex(colors[12])?,
            base0d: Color::from_hex(colors[13])?,
            base0e: Color::from_hex(colors[14])?,
            base0f: Color::from_hex(colors[15])?,
        })
    }

    /// Convert to egaku Theme.
    #[must_use]
    pub fn to_egaku_theme(&self) -> egaku::Theme {
        egaku::Theme {
            base00: self.base00.to_array(),
            base01: self.base01.to_array(),
            base02: self.base02.to_array(),
            base03: self.base03.to_array(),
            base04: self.base04.to_array(),
            base05: self.base05.to_array(),
            base06: self.base06.to_array(),
            base07: self.base07.to_array(),
            base08: self.base08.to_array(),
            base09: self.base09.to_array(),
            base0a: self.base0a.to_array(),
            base0b: self.base0b.to_array(),
            base0c: self.base0c.to_array(),
            base0d: self.base0d.to_array(),
            base0e: self.base0e.to_array(),
            base0f: self.base0f.to_array(),
            background: self.base00.to_array(),
            foreground: self.base05.to_array(),
            accent: self.base0d.to_array(),
            error: self.base08.to_array(),
            warning: self.base0a.to_array(),
            success: self.base0b.to_array(),
            selection: self.base02.to_array(),
            muted: self.base03.to_array(),
            border: self.base01.to_array(),
            spacing: 8.0,
            font_size: 14.0,
        }
    }

    /// Whether this is a dark scheme (base00 background is dark).
    #[must_use]
    pub fn is_dark(&self) -> bool {
        self.base00.is_dark()
    }

    /// ANSI terminal color mapping for terminal emulators (mado).
    /// Returns 16 colors: 8 normal + 8 bright.
    #[must_use]
    pub fn to_ansi_colors(&self) -> [[f32; 4]; 16] {
        [
            // Normal colors (0-7)
            self.base00.to_array(), // black
            self.base08.to_array(), // red
            self.base0b.to_array(), // green
            self.base0a.to_array(), // yellow
            self.base0d.to_array(), // blue
            self.base0e.to_array(), // magenta
            self.base0c.to_array(), // cyan
            self.base05.to_array(), // white
            // Bright colors (8-15)
            self.base03.to_array(), // bright black
            self.base08.to_array(), // bright red (same, adjust in scheme)
            self.base0b.to_array(), // bright green
            self.base0a.to_array(), // bright yellow
            self.base0d.to_array(), // bright blue
            self.base0e.to_array(), // bright magenta
            self.base0c.to_array(), // bright cyan
            self.base07.to_array(), // bright white
        ]
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            name: "Nord".into(),
            author: "Arctic Ice Studio".into(),
            base00: Color::new(0.180, 0.204, 0.251, 1.0),
            base01: Color::new(0.231, 0.259, 0.322, 1.0),
            base02: Color::new(0.263, 0.298, 0.369, 1.0),
            base03: Color::new(0.298, 0.337, 0.416, 1.0),
            base04: Color::new(0.847, 0.871, 0.914, 1.0),
            base05: Color::new(0.898, 0.914, 0.941, 1.0),
            base06: Color::new(0.925, 0.937, 0.957, 1.0),
            base07: Color::new(0.925, 0.937, 0.957, 1.0),
            base08: Color::new(0.749, 0.380, 0.416, 1.0),
            base09: Color::new(0.816, 0.529, 0.439, 1.0),
            base0a: Color::new(0.922, 0.796, 0.545, 1.0),
            base0b: Color::new(0.639, 0.745, 0.549, 1.0),
            base0c: Color::new(0.561, 0.737, 0.733, 1.0),
            base0d: Color::new(0.533, 0.753, 0.816, 1.0),
            base0e: Color::new(0.506, 0.631, 0.757, 1.0),
            base0f: Color::new(0.369, 0.506, 0.675, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_hex() {
        let c = Color::from_hex("#2E3440").unwrap();
        assert!((c.r - 0.180).abs() < 0.01);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn color_to_hex_roundtrip() {
        let c = Color::from_hex("#88C0D0").unwrap();
        assert_eq!(c.to_hex(), "#88C0D0");
    }

    #[test]
    fn color_luminance() {
        let dark = Color::from_hex("#2E3440").unwrap();
        assert!(dark.is_dark());
        let light = Color::from_hex("#ECEFF4").unwrap();
        assert!(!light.is_dark());
    }

    #[test]
    fn color_lerp() {
        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        let white = Color::new(1.0, 1.0, 1.0, 1.0);
        let mid = black.lerp(&white, 0.5);
        assert!((mid.r - 0.5).abs() < 0.001);
    }

    #[test]
    fn scheme_default_is_nord() {
        let s = ColorScheme::default();
        assert_eq!(s.name, "Nord");
        assert!(s.is_dark());
    }

    #[test]
    fn scheme_from_hex_array() {
        let colors = [
            "#2E3440", "#3B4252", "#434C5E", "#4C566A", "#D8DEE9", "#E5E9F0", "#ECEFF4",
            "#ECEFF4", "#BF616A", "#D08770", "#EBCB8B", "#A3BE8C", "#8FBCBB", "#88C0D0",
            "#81A1C1", "#5E81AC",
        ];
        let s = ColorScheme::from_hex_array("Test", &colors).unwrap();
        assert_eq!(s.name, "Test");
    }

    #[test]
    fn scheme_to_egaku_theme() {
        let s = ColorScheme::default();
        let t = s.to_egaku_theme();
        assert_eq!(t.background, s.base00.to_array());
        assert_eq!(t.foreground, s.base05.to_array());
    }

    #[test]
    fn scheme_ansi_colors() {
        let s = ColorScheme::default();
        let ansi = s.to_ansi_colors();
        assert_eq!(ansi.len(), 16);
        // Black should be base00
        assert_eq!(ansi[0], s.base00.to_array());
    }

    #[test]
    fn scheme_get_slot() {
        let s = ColorScheme::default();
        assert_eq!(s.get(Base16Slot::Base00).to_array(), s.base00.to_array());
        assert_eq!(s.get(Base16Slot::Base0D).to_array(), s.base0d.to_array());
    }

    #[test]
    fn scheme_serde_roundtrip() {
        let s = ColorScheme::default();
        let json = serde_json::to_string(&s).unwrap();
        let s2: ColorScheme = serde_json::from_str(&json).unwrap();
        assert_eq!(s.name, s2.name);
        assert_eq!(s.base00.to_hex(), s2.base00.to_hex());
    }

    // ── Negative / error-path coverage for Color::from_hex ──────────

    #[test]
    fn hex_rejects_short_string() {
        assert!(Color::from_hex("#ABC").is_err());
        assert!(Color::from_hex("").is_err());
    }

    #[test]
    fn hex_rejects_overlong_string() {
        assert!(Color::from_hex("#AABBCCDD").is_err());
    }

    #[test]
    fn hex_rejects_non_hex_digits() {
        assert!(Color::from_hex("#ZZZZZZ").is_err());
        assert!(Color::from_hex("#12345G").is_err());
    }

    #[test]
    fn hex_accepts_lowercase_and_missing_prefix() {
        let a = Color::from_hex("#2e3440").unwrap();
        let b = Color::from_hex("2E3440").unwrap();
        // to_hex always normalises to uppercase so the two paths
        // collapse after the round-trip.
        assert_eq!(a.to_hex(), b.to_hex());
    }

    #[test]
    fn to_hex_always_uppercase() {
        let c = Color::from_hex("#abcdef").unwrap();
        let out = c.to_hex();
        assert_eq!(out, out.to_uppercase());
    }

    // ── lerp edges + clamping ────────────────────────────────────────

    #[test]
    fn lerp_clamps_out_of_range() {
        // t is documented as clamped to [0, 1]; confirm t > 1 yields
        // `other` (not past it) and t < 0 yields `self`.
        let a = Color::new(0.0, 0.0, 0.0, 1.0);
        let b = Color::new(1.0, 1.0, 1.0, 1.0);
        let past = a.lerp(&b, 5.0);
        assert!((past.r - 1.0).abs() < 0.001);
        let before = a.lerp(&b, -5.0);
        assert!(before.r.abs() < 0.001);
    }

    #[test]
    fn lerp_endpoints_are_exact() {
        let a = Color::new(0.2, 0.4, 0.6, 0.8);
        let b = Color::new(0.9, 0.1, 0.5, 0.3);
        assert_eq!(a.lerp(&b, 0.0).r, a.r);
        assert_eq!(a.lerp(&b, 1.0).r, b.r);
    }

    #[test]
    fn lerp_blends_alpha() {
        // Alpha must lerp just like RGB — a previous implementation
        // froze alpha at self.a, which broke fade-in animations.
        let a = Color::new(0.0, 0.0, 0.0, 0.0);
        let b = Color::new(0.0, 0.0, 0.0, 1.0);
        let mid = a.lerp(&b, 0.5);
        assert!((mid.a - 0.5).abs() < 0.001);
    }

    // ── luminance weights + is_dark boundary ────────────────────────

    #[test]
    fn luminance_weights_match_bt709() {
        // BT.709 weights: 0.2126 R + 0.7152 G + 0.0722 B. Pure red
        // should hit ~0.2126; green dominates; blue is the darkest
        // primary.
        let red = Color::new(1.0, 0.0, 0.0, 1.0);
        let green = Color::new(0.0, 1.0, 0.0, 1.0);
        let blue = Color::new(0.0, 0.0, 1.0, 1.0);
        assert!((red.luminance() - 0.2126).abs() < 0.0001);
        assert!((green.luminance() - 0.7152).abs() < 0.0001);
        assert!((blue.luminance() - 0.0722).abs() < 0.0001);
    }

    #[test]
    fn is_dark_boundary() {
        // Exactly at 0.5 is documented as NOT dark (strict `<`). A
        // change to `<=` would flip every boundary pixel.
        let low = Color::new(0.0, 0.69, 0.0, 1.0); // lum ~0.4935
        let high = Color::new(0.0, 0.71, 0.0, 1.0); // lum ~0.5078
        assert!(low.is_dark());
        assert!(!high.is_dark());
    }

    // ── ColorScheme completeness + From conversions ─────────────────

    #[test]
    fn base16_slot_all_has_16_entries_in_order() {
        assert_eq!(Base16Slot::ALL.len(), 16);
        assert_eq!(Base16Slot::ALL[0], Base16Slot::Base00);
        assert_eq!(Base16Slot::ALL[15], Base16Slot::Base0F);
    }

    #[test]
    fn color_array_from_conversion_roundtrip() {
        let arr = [0.1, 0.2, 0.3, 0.4];
        let c: Color = arr.into();
        let back: [f32; 4] = c.into();
        assert_eq!(arr, back);
    }

    #[test]
    fn scheme_from_hex_array_propagates_errors() {
        let mut colors = [
            "#2E3440", "#3B4252", "#434C5E", "#4C566A", "#D8DEE9", "#E5E9F0", "#ECEFF4",
            "#ECEFF4", "#BF616A", "#D08770", "#EBCB8B", "#A3BE8C", "#8FBCBB", "#88C0D0",
            "#81A1C1", "#5E81AC",
        ];
        colors[7] = "not-a-color";
        assert!(ColorScheme::from_hex_array("Bad", &colors).is_err());
    }

    #[test]
    fn to_egaku_theme_fills_every_semantic_slot() {
        let s = ColorScheme::default();
        let t = s.to_egaku_theme();
        // Every semantic slot is non-zero on a real scheme (all four
        // channels zero would be transparent-black — not a valid
        // paint).
        assert_ne!(t.background, [0.0; 4]);
        assert_ne!(t.foreground, [0.0; 4]);
        assert_ne!(t.accent, [0.0; 4]);
        assert_ne!(t.error, [0.0; 4]);
        assert_ne!(t.warning, [0.0; 4]);
        assert_ne!(t.success, [0.0; 4]);
        assert_ne!(t.selection, [0.0; 4]);
        assert_ne!(t.muted, [0.0; 4]);
        assert_ne!(t.border, [0.0; 4]);
        assert!(t.spacing > 0.0);
        assert!(t.font_size > 0.0);
    }

    #[test]
    fn ansi_colors_map_base16_to_standard_ordering() {
        // ANSI ordering: black, red, green, yellow, blue, magenta,
        // cyan, white, then the bright-8. Irodzuki maps these from
        // base00, base08, base0B, base0A, base0D, base0E, base0C,
        // base05 — any drift here appears in every terminal as the
        // "wrong" colour.
        let s = ColorScheme::default();
        let ansi = s.to_ansi_colors();
        assert_eq!(ansi[0], s.base00.to_array(), "ansi 0 (black)");
        assert_eq!(ansi[1], s.base08.to_array(), "ansi 1 (red)");
        assert_eq!(ansi[2], s.base0b.to_array(), "ansi 2 (green)");
        assert_eq!(ansi[3], s.base0a.to_array(), "ansi 3 (yellow)");
        assert_eq!(ansi[4], s.base0d.to_array(), "ansi 4 (blue)");
        assert_eq!(ansi[5], s.base0e.to_array(), "ansi 5 (magenta)");
        assert_eq!(ansi[6], s.base0c.to_array(), "ansi 6 (cyan)");
        assert_eq!(ansi[7], s.base05.to_array(), "ansi 7 (white)");
        // Bright-black pulls from base03 (comments) per the module doc.
        assert_eq!(ansi[8], s.base03.to_array(), "ansi 8 (bright black)");
    }
}

//! Built-in `ColorScheme` presets — terminal-emulator-grade themes
//! with curated 16-color ANSI palettes alongside the base16 slot
//! values. Each constructor parses static hex tables once and
//! returns an owned `ColorScheme` (`from_hex_array_with_ansi`
//! attaches the ANSI palette as a `Some(...)` override so
//! `to_ansi_colors` returns the upstream-official table instead of
//! the lossy base16-derived fallback).
//!
//! Adding a preset: drop in one `pub fn` returning
//! `ColorScheme::from_hex_array_with_ansi(name, &BASE16, &ANSI)
//! .expect("static hex")`. The unwrap is sound by construction —
//! every literal here is hand-validated and lives in the same
//! commit as the parser test that exercises it.

use crate::error::Result;
use crate::scheme::ColorScheme;

fn build(
    name: &str,
    base16: &[&str; 16],
    ansi: &[&str; 16],
) -> Result<ColorScheme> {
    ColorScheme::from_hex_array_with_ansi(name, base16, ansi)
}

const NORD_BASE16: [&str; 16] = [
    "2E3440", "3B4252", "434C5E", "4C566A",
    "D8DEE9", "E5E9F0", "ECEFF4", "8FBCBB",
    "BF616A", "D08770", "EBCB8B", "A3BE8C",
    "88C0D0", "81A1C1", "B48EAD", "5E81AC",
];
// The OFFICIAL Nord terminal ANSI mapping (nordtheme.com / the ghostty
// palette). Slots 0/4/6 were previously the brighter frost variants
// (#2E3440 / #81A1C1 / #8FBCBB), which diverged from every stock Nord
// terminal: ANSI black is polar-night-1 #3B4252 (NOT the bg #2E3440),
// normal blue is the deep frost-3 #5E81AC, normal cyan is frost-1 #88C0D0.
// Fixed so `nord` renders byte-identically to ghostty.
const NORD_ANSI: [&str; 16] = [
    "3B4252", "BF616A", "A3BE8C", "EBCB8B",
    "5E81AC", "B48EAD", "88C0D0", "E5E9F0",
    "4C566A", "BF616A", "A3BE8C", "EBCB8B",
    "81A1C1", "B48EAD", "8FBCBB", "ECEFF4",
];

#[must_use]
pub fn nord() -> ColorScheme {
    build("nord", &NORD_BASE16, &NORD_ANSI).expect("nord hex is valid")
}

const DRACULA_BASE16: [&str; 16] = [
    "282A36", "44475A", "565664", "78787E",
    "F8F8F2", "F8F8F2", "FFFFFF", "BD93F9",
    "FF5555", "FFB86C", "F1FA8C", "50FA7B",
    "8BE9FD", "6272A4", "FF79C6", "BD93F9",
];
const DRACULA_ANSI: [&str; 16] = [
    "282A36", "FF5555", "50FA7B", "F1FA8C",
    "BD93F9", "FF79C6", "8BE9FD", "BFBFBF",
    "4D4D4D", "FF6E67", "5AF78E", "F4F99D",
    "CAA9FA", "FF92D0", "9AEDFE", "E6E6E6",
];

#[must_use]
pub fn dracula() -> ColorScheme {
    build("dracula", &DRACULA_BASE16, &DRACULA_ANSI).expect("dracula hex is valid")
}

const SOLARIZED_DARK_BASE16: [&str; 16] = [
    "002B36", "073642", "586E75", "657B83",
    "839496", "93A1A1", "EEE8D5", "FDF6E3",
    "DC322F", "CB4B16", "B58900", "859900",
    "2AA198", "268BD2", "6C71C4", "D33682",
];
const SOLARIZED_DARK_ANSI: [&str; 16] = [
    "073642", "DC322F", "859900", "B58900",
    "268BD2", "D33682", "2AA198", "EEE8D5",
    "002B36", "CB4B16", "586E75", "657B83",
    "839496", "6C71C4", "93A1A1", "FDF6E3",
];

#[must_use]
pub fn solarized_dark() -> ColorScheme {
    build("solarized-dark", &SOLARIZED_DARK_BASE16, &SOLARIZED_DARK_ANSI)
        .expect("solarized-dark hex is valid")
}

const SOLARIZED_LIGHT_BASE16: [&str; 16] = [
    "FDF6E3", "EEE8D5", "93A1A1", "839496",
    "657B83", "586E75", "073642", "002B36",
    "DC322F", "CB4B16", "B58900", "859900",
    "2AA198", "268BD2", "6C71C4", "D33682",
];
const SOLARIZED_LIGHT_ANSI: [&str; 16] = [
    "EEE8D5", "DC322F", "859900", "B58900",
    "268BD2", "D33682", "2AA198", "073642",
    "FDF6E3", "CB4B16", "93A1A1", "839496",
    "657B83", "6C71C4", "586E75", "002B36",
];

#[must_use]
pub fn solarized_light() -> ColorScheme {
    build("solarized-light", &SOLARIZED_LIGHT_BASE16, &SOLARIZED_LIGHT_ANSI)
        .expect("solarized-light hex is valid")
}

const GRUVBOX_DARK_BASE16: [&str; 16] = [
    "282828", "3C3836", "504945", "665C54",
    "BDAE93", "D5C4A1", "EBDBB2", "FBF1C7",
    "FB4934", "FE8019", "FABD2F", "B8BB26",
    "8EC07C", "83A598", "D3869B", "D65D0E",
];
const GRUVBOX_DARK_ANSI: [&str; 16] = [
    "282828", "CC241D", "98971A", "D79921",
    "458588", "B16286", "689D6A", "A89984",
    "928374", "FB4934", "B8BB26", "FABD2F",
    "83A598", "D3869B", "8EC07C", "EBDBB2",
];

#[must_use]
pub fn gruvbox_dark() -> ColorScheme {
    build("gruvbox-dark", &GRUVBOX_DARK_BASE16, &GRUVBOX_DARK_ANSI)
        .expect("gruvbox-dark hex is valid")
}

const ONE_DARK_BASE16: [&str; 16] = [
    "282C34", "353B45", "3E4451", "545862",
    "565C64", "ABB2BF", "B6BDCA", "C8CCD4",
    "E06C75", "D19A66", "E5C07B", "98C379",
    "56B6C2", "61AFEF", "C678DD", "BE5046",
];
const ONE_DARK_ANSI: [&str; 16] = [
    "282C34", "E06C75", "98C379", "E5C07B",
    "61AFEF", "C678DD", "56B6C2", "ABB2BF",
    "545862", "BE5046", "98C379", "E5C07B",
    "61AFEF", "C678DD", "56B6C2", "C8CCD4",
];

#[must_use]
pub fn one_dark() -> ColorScheme {
    build("one-dark", &ONE_DARK_BASE16, &ONE_DARK_ANSI).expect("one-dark hex is valid")
}

const TOKYO_NIGHT_BASE16: [&str; 16] = [
    "1A1B26", "16161E", "2F3549", "444B6A",
    "787C99", "A9B1D6", "CBCCD1", "D5D6DB",
    "C0CAF5", "A9B1D6", "0DB9D7", "9ECE6A",
    "B4F9F8", "2AC3DE", "BB9AF7", "F7768E",
];
const TOKYO_NIGHT_ANSI: [&str; 16] = [
    "1A1B26", "F7768E", "9ECE6A", "E0AF68",
    "7AA2F7", "BB9AF7", "7DCFFF", "A9B1D6",
    "414868", "F7768E", "9ECE6A", "E0AF68",
    "7AA2F7", "BB9AF7", "7DCFFF", "C0CAF5",
];

#[must_use]
pub fn tokyo_night() -> ColorScheme {
    build("tokyo-night", &TOKYO_NIGHT_BASE16, &TOKYO_NIGHT_ANSI)
        .expect("tokyo-night hex is valid")
}

const CATPPUCCIN_MOCHA_BASE16: [&str; 16] = [
    "1E1E2E", "181825", "313244", "45475A",
    "585B70", "CDD6F4", "F5E0DC", "B4BEFE",
    "F38BA8", "FAB387", "F9E2AF", "A6E3A1",
    "94E2D5", "89B4FA", "CBA6F7", "F2CDCD",
];
const CATPPUCCIN_MOCHA_ANSI: [&str; 16] = [
    "45475A", "F38BA8", "A6E3A1", "F9E2AF",
    "89B4FA", "F5C2E7", "94E2D5", "BAC2DE",
    "585B70", "F38BA8", "A6E3A1", "F9E2AF",
    "89B4FA", "F5C2E7", "94E2D5", "A6ADC8",
];

#[must_use]
pub fn catppuccin_mocha() -> ColorScheme {
    build("catppuccin-mocha", &CATPPUCCIN_MOCHA_BASE16, &CATPPUCCIN_MOCHA_ANSI)
        .expect("catppuccin-mocha hex is valid")
}

/// Every built-in preset in one slice — operators iterate it for
/// theme pickers, name validation, fleet inventories. Order is the
/// stable canonical ordering (alphabetical by id).
#[must_use]
pub fn all() -> Vec<ColorScheme> {
    vec![
        catppuccin_mocha(),
        dracula(),
        gruvbox_dark(),
        nord(),
        one_dark(),
        solarized_dark(),
        solarized_light(),
        tokyo_night(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_parses() {
        let schemes = all();
        assert_eq!(schemes.len(), 8);
        for s in schemes {
            assert!(s.ansi_palette.is_some(), "{} missing ansi palette", s.name);
        }
    }

    #[test]
    fn ansi_override_is_used() {
        let nord = nord();
        let ansi = nord.to_ansi_colors();
        // nord normal red = #BF616A → (0.749, 0.380, 0.416)
        assert!((ansi[1][0] - 0.749).abs() < 0.01);
        // Official Nord ANSI (ghostty parity) — slots 0/4/6 pinned so they
        // can't regress to the old brighter-frost variants.
        assert!((ansi[0][0] - 0.231).abs() < 0.01, "ansi black must be #3B4252"); // 0x3B/255
        assert!((ansi[4][2] - 0.675).abs() < 0.01, "ansi blue must be #5E81AC"); // 0xAC/255
        assert!((ansi[6][1] - 0.753).abs() < 0.01, "ansi cyan must be #88C0D0"); // 0xC0/255
    }
}

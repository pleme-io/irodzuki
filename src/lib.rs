//! Irodzuki (色づき) — GPU-app theming for pleme-io.
//!
//! Bridges Stylix (Nix-level base16 theming) into wgpu GPU render pipelines.
//! Stylix generates color schemes at build time; irodzuki loads them at runtime
//! and provides GPU-compatible color data to garasu, madori, and egaku.
//!
//! ## How it works
//!
//! 1. Stylix generates a base16 color scheme in Nix
//! 2. blackmatter-pleme maps those colors into each app's YAML config
//! 3. shikumi loads the YAML at runtime
//! 4. irodzuki parses the `[theme]` section into a `ColorScheme`
//! 5. Apps use `ColorScheme` to get wgpu::Color values, shader uniforms, etc.

pub mod error;
pub mod gpu;
pub mod scheme;
pub mod shader;

pub use error::IrodzukiError;
pub use gpu::GpuColors;
pub use scheme::{Base16Slot, ColorScheme};
pub use shader::ThemeUniforms;

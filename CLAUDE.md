# Irodzuki (色づき) — GPU Theme System

## Build & Test

```bash
cargo build
cargo test --lib
```

## Architecture

Bridges base16 color schemes (Stylix) into wgpu GPU render pipelines. Standard Stylix
themes NixOS/GTK/terminal apps but cannot reach into wgpu shader uniforms, ANSI color
tables, or custom GPU pipelines. Irodzuki fills that gap.

### Modules

| Module | Purpose |
|--------|---------|
| `scheme.rs` | `Color`, `Base16Slot`, `ColorScheme` — base16 palette with hex parsing, lerp, luminance |
| `gpu.rs` | `GpuColors` — pre-computed GPU-ready colors, sRGB↔linear conversion |
| `shader.rs` | `ThemeUniforms` — bytemuck Pod for wgpu uniform buffer, WGSL struct snippet |
| `error.rs` | `IrodzukiError` — invalid color, parse failure, missing slot |

### Layer Position

```
Stylix (Nix) → base16 hex strings
       ↓
blackmatter-pleme home-manager module → app YAML config
       ↓
shikumi (runtime config loader) → egaku::Theme
       ↓
irodzuki::ColorScheme::from(egaku::Theme)
       ↓
  ├── GpuColors (wgpu::Color clear, palette arrays)
  ├── ThemeUniforms (uniform buffer for WGSL shaders)
  └── to_ansi_colors() (16-color terminal palette for mado)
```

### Consumers

Used by: mado, hibiki, kagi, kekkai, fumi, nami

## Design Decisions

- **Does NOT define color schemes** — transforms egaku::Theme into GPU-consumable formats
- **bytemuck Pod** for ThemeUniforms — zero-copy upload to wgpu uniform buffers
- **sRGB↔linear conversion** — GPU pipelines need linear space colors
- **ANSI palette generation** — maps base16 to standard 16-color ANSI for terminal emulators
- **WGSL snippet** — `THEME_UNIFORMS_WGSL` constant for inclusion in custom shaders
- **Nord defaults** — consistent with all pleme-io apps

# VIctory — RTX 4070 & Ryzen 7 5700X3D Fork

<p>
  <img src="https://img.shields.io/badge/Windows%2011-0b0a15?style=flat-square&logo=windows11&logoColor=white" alt="Windows 11">
  <img src="https://img.shields.io/badge/Ryzen%207%205700X3D-0b0a15?style=flat-square&logo=amd&logoColor=ed1c24" alt="Ryzen 7 5700X3D">
  <img src="https://img.shields.io/badge/RTX%204070%2012GB-0b0a15?style=flat-square&logo=nvidia&logoColor=76b900" alt="RTX 4070 12GB">
  <img src="https://img.shields.io/badge/GPL--3.0-0b0a15?style=flat-square&logo=gnu&logoColor=white" alt="GPL-3.0">
</p>

An Xbox Series compatibility runtime for Windows PC, tuned for **12 GB NVIDIA GPUs (RTX 4070)** and **8-core AMD Ryzen 3D V-Cache CPUs (5700X3D)** with **64 GB RAM**.

> [!IMPORTANT]
> No game files are included. You need your own copy of the game.

---

## Key Improvements

- **12 GB VRAM Optimization:** Resolution-aware render-target reservation (`targets_room_kept()` scales by `VICTORY_DISPLAY_MODE` instead of locking 3 GB for 4K targets at 720p), on-demand mip tiling (`VICTORY_LEVELS_ON_DEMAND=1`), and pressure-based idle texture eviction.
- **64 GB RAM Caching:** Extended `ram_idle_ms()` retention (`120 s` when `>= 16 GiB` free) for texture staging & loose shadow buffers, keeping translated DXIL and the PSO pipeline library resident in RAM.
- **5700X3D Multi-Core & Contention Fixes:** `RwLock` read fast-paths for `SHADER_HEAPS` and `HOST_WRITTEN`, AVX2 SIMD copy-back (`merge_piece_avx2`), zero-alloc `Deferred::clear()`, unstarved DirectStorage (`12` workers) and pipeline (`8` workers) pools with `120 s` drained-only PSO cache saves, and parallel texture staging refreshes.
- **Correctness & Stability:** BC7 entry-0 texture fix, `DEPTH_STENCIL2` subobject support, `ID3D12Fence1` IID, `SwapChainRetry` throttling, batched card-copy sync (`#1227`), aliased GPU-written texture copies (`#1241`), streamed min-LOD preservation (`#1210`), integer render-target blend fix, and `Step::Host` / `Deferred::clear` texture-ordering fixes.

---

## Build

Requires Visual Studio Build Tools (MSVC + Windows SDK) and [rustup](https://rustup.rs).

```powershell
# Release build tuned for Zen 3 (Ryzen 5000 / 5700X3D)
$env:RUSTFLAGS = "-C target-cpu=znver3"
cargo build --release --bin victory

# Optional diagnostics build
cargo build --release --features diagnostics --bin victory --target-dir target-diag
```

**Windows 10:** Run `cargo xtask agility` once to fetch Microsoft's hash-checked D3D12 Agility SDK and keep the `D3D12` folder next to `victory.exe`. Windows 11 needs nothing extra.

---

## Quick Start

```powershell
# Launch (Series X at 720p)
$env:VICTORY_DISPLAY_MODE = "1280x720"
$env:VICTORY_LEVELS_ON_DEMAND = "1"
$env:VICTORY_LEVELS_MIN_KB = "256"
$env:VICTORY_TEXTURE_ROOM_PERCENT = "94"
$env:VICTORY_ASYNC_SUBMIT = "1"
$env:VICTORY_LOOKUP_CACHE = "1"
$env:VICTORY_INDIRECT_RAW = "3"
.\target\release\victory.exe run "path\to\title.exe" --execute --console series-x
```

**Controls:** `F9` mouse look · `WASD` move · `Enter` / `Backspace` / `X` / `Y` as A / B / X / Y · `F10` overlay · `F12` screenshot.

---

## Useful Environment Switches

| Switch | Recommended (12 GB / 64 GB) | Purpose |
|:--|:--|:--|
| `VICTORY_DISPLAY_MODE` | `1280x720` | Display resolution reported to the game |
| `VICTORY_LEVELS_ON_DEMAND` | `1` | Commit tiled texture VRAM mip-by-mip as streamed |
| `VICTORY_LEVELS_MIN_KB` | `256` | Minimum texture size eligible for on-demand mip tiles |
| `VICTORY_TEXTURE_ROOM_PERCENT` | `94` | VRAM budget ceiling (%) for bindless texture loading |
| `VICTORY_VRAM_EVICT` | `90` | Evict oldest idle textures above this VRAM % |
| `VICTORY_ASYNC_SUBMIT` | `1` | Run command-list submission on a dedicated thread |
| `VICTORY_LOOKUP_CACHE` | `1` | Cache shadow buffer lookups |
| `VICTORY_INDIRECT_RAW` | `3` | Validate indirect draw records on CPU before GPU execution |
| `VICTORY_AMBIENT` / `VICTORY_AMBIENT_SHADE` | `1.0` / `0` | Fallback ambient light & screen-space AO toggle |
| `VICTORY_DROP_DXIL` / `VICTORY_LIBRARY_TRIM` | `0` / `0` | Keep translated DXIL & PSO cache resident on 64 GB RAM |
| `VICTORY_OVERLAY` | `1` (`0` off) | Toggle the top-left performance overlay (`F10`) |

---

<sub>GPL-3.0-or-later · independent open-source compatibility layer.</sub>

# 10. Dual-Tier Artifact Footprint and Full Desktop Packaging

Date: 2026-08-18

## Status

Accepted

## Context

Previous artifact size measurements only inspected intermediate JavaScript bundles (e.g. `esbuild` outputting 10KB), which was misleading for desktop and native frameworks like Electron (~180MB+), NW.js (~150MB+), Avalonia self-contained (~65MB+), and Tauri (~10MB+).

## Decision

1. **Dual-Tier Sizing Model**:
   - **Code Bundle Size (`Bundle (KB)`)**: Measures raw application bytecode/JS/TS bundle size excluding runtime dependencies.
   - **Packaged Distribution Footprint (`Dist (MB)`)**: Measures full production self-contained distribution, including embedded runtime (Chromium + Node for Electron/NW.js, .NET CLR for Avalonia, WebKitGTK wrapper for Tauri/Wails, Flutter AOT engine).
2. **Self-Contained Build Commands**:
   - Update build pipelines to generate and measure full standalone distributions.
3. **Reporter Update**:
   - Add both `Bundle (KB)` and `Dist (MB)` columns to `BENCHMARK_RESULTS.md`, `BENCHMARK_STRESS_RESULTS.md`, `BENCHMARK_GUI_RESULTS.md`, and JSON outputs.

## Consequences

- Full transparency on code efficiency vs real-world deployment/disk footprint.
- Accurately captures the trade-off between lightweight native runtimes (Rust, Go, C++) and bundled runtime engines (Electron, NW.js, self-contained .NET).

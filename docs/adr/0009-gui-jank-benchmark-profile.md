# 9. GUI Jank and Frame Pacing Benchmark Profile

Date: 2026-08-18

## Status

Accepted

## Context

Headless CLI benchmarks measure raw compute throughput and memory allocation, but do not capture real-world UI smoothness, main-thread event loop contention, rendering bottlenecks, and frame drops (jank) under heavy workload stress for GUI-capable frameworks.

## Decision

Introduce a dedicated **GUI Jank Benchmark Mode** (`--gui` / `--gui-jank`):
1. **Target Scope**: Runs across all GUI-capable desktop and windowed applications (Slint, egui, Iced, Dioxus, Tauri, Electron, Neutralinojs, NW.js, NodeGui, Avalonia, Qt, Flutter, and web UI shells).
2. **Jank Metric Standards**:
   - **Average FPS**: Mean frame rate over the stress replay duration.
   - **Jank Frames (>16.67ms)**: Count and percentage of frame presentations that missed the 60 FPS VSync window.
   - **1% Low FPS**: 99th percentile slowest frame time converted to FPS (industry standard smoothness metric).
   - **0.1% Low FPS**: 99.9th percentile slowest frame time (worst stutter).
   - **Max Frame Time (ms)**: Peak single frame render latency during the replay stress cycle.
3. **Dedicated Artifact Separation**:
   - Terminal Summary Table with GUI metrics.
   - JSON export: `benchmark-gui-results.json`.
   - Markdown export: `BENCHMARK_GUI_RESULTS.md`.

## Consequences

- Surfaces UI thread starvation and garbage collection stalls directly affecting visual responsiveness.
- Clear separation between headless compute throughput and real-world rendering smoothness.

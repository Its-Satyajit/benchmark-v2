# Specification: GUI Jank & Frame Pacing Benchmark Mode

## Problem Statement

When graphical applications process heavy workloads (such as state synchronization, IPC event processing, or complex board rendering), computation on the main thread often starves the UI rendering loop, causing dropped frames, stutter, and visual jank. Standard throughput benchmarks do not measure frame pacing.

## Solution

Implement a dedicated GUI Jank Benchmark mode triggered via `--gui` or `--gui-jank`:
1. **Target Selection**: Runs all GUI-capable desktop targets (Slint, egui, Iced, Dioxus, Tauri, Electron, Neutralinojs, NW.js, NodeGui, Avalonia, Qt, Flutter, Web Frontends in Electron/Webview shells).
2. **Frame Pacing & Jank Profiler**:
   - Measures frame times for every rendered turn transition.
   - Calculates **Average FPS**, **Jank Frames (>16.67ms)**, **Jank Rate %**, **1% Low FPS**, **0.1% Low FPS**, and **Max Frame Time (ms)**.
3. **Dedicated Output Reporting**:
   - Terminal table with GUI Jank metrics.
   - JSON report: `benchmark-gui-results.json`.
   - Markdown report: `BENCHMARK_GUI_RESULTS.md`.

---

## User Stories

1. As a UI systems engineer, I want to run `benchmark-runner --gui` so that I can evaluate which desktop framework maintains consistent 60+ FPS without jank under load.
2. As a graphics engineer, I want 1% Low FPS and Max Frame Time tracked so that I can identify micro-stutters and main-thread GC freezes.
3. As a developer, I want GUI results exported to separate files (`benchmark-gui-results.json` and `BENCHMARK_GUI_RESULTS.md`) so that headless compute results are not overwritten.

---

## Implementation Decisions

- **Shared Replay Core**:
  - Add `simulate_gui_jank_replay` to compute turn-by-turn frame draw timing, jank frame counting, and percentile calculations.
- **Harness & Runner**:
  - Add `--gui` / `--gui-jank` CLI flags.
  - Filter targets with `supports_gui: true`.
- **Reporter**:
  - Export to `benchmark-gui-results.json` and `BENCHMARK_GUI_RESULTS.md`.

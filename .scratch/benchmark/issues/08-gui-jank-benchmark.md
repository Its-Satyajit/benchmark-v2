# 08 — Implement GUI Jank & Frame Pacing Engine and Reporter

**What to build:** Implement GUI Jank simulation and frame pacing metrics (`avg_fps`, `jank_frames`, `jank_percentage`, `p99_1pct_low_fps`, `max_frame_time_ms`) in shared cores, harness, and dedicated reports (`BENCHMARK_GUI_RESULTS.md` & `benchmark-gui-results.json`).

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Add `simulate_gui_jank_replay` to TS and Rust shared cores
- [ ] Add `--gui` / `--gui-jank` CLI flag to benchmark runner
- [ ] Filter GUI-supported desktop & windowed targets
- [ ] Export separate GUI results to `benchmark-gui-results.json` and `BENCHMARK_GUI_RESULTS.md`

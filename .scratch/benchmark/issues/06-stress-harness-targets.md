# 06 — Extreme Stress Harness & Target Adapters Update

**What to build:** Update target apps and the native Rust runner harness to support `--stress`, `--concurrency <N>`, and `--iterations <N>`, outputting comparative limit saturation benchmarks in terminal tables and JSON/Markdown reports.

**Blocked by:** 05 — Multi-Core Saturation Stress Simulation in Shared Cores (TS & Rust)

**Status:** ready-for-agent

- [ ] Update target adapters (`apps/*` and `crates/replay-engine`) to accept `--stress`, `--concurrency`, `--iterations`
- [ ] Update `src_rs/harness.rs` to drive stress benchmarks across all targets
- [ ] Export stress saturation comparison table to `BENCHMARK_STRESS_RESULTS.md` and `benchmark-stress-results.json`

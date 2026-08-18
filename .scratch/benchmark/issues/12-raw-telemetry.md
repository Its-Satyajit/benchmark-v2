# 12 — Implement Raw Per-Iteration Telemetry & Unified `--all` Pipeline

**What to build:** Add `raw_iterations` array to shared replay cores (TS, Rust, Go, C#, Dart), export raw telemetry files (`benchmark-raw-data.json`, `benchmark-stress-raw-data.json`, `benchmark-gui-raw-data.json`), and add `--all` flag to `benchmark-runner`.

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Export `raw_iterations` from TS `packages/shared-replay-core/src/index.ts`
- [ ] Export `raw_iterations` from Rust crate, Go, C#, Dart targets
- [ ] Update `harness.rs` and `reporter.rs` to persist `*-raw-data.json` files
- [ ] Add `--all` flag in `src_rs/main.rs` to sequentially execute Baseline, Stress, and GUI Jank suites
- [ ] Add `benchmark:all` in `package.json`
- [ ] Execute 10-iteration suite and verify raw JSON and markdown outputs

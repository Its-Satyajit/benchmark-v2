# 05 — Multi-Core Saturation Stress Simulation in Shared Cores (TS & Rust)

**What to build:** Implement `simulate_stress_replay` with multi-worker concurrency and in-memory historical snapshot tree retention in `@benchmark/shared-replay-core` and `crates/shared-replay-core`.

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Add in-memory Snapshot Tree retention to stress memory allocators and garbage collection
- [ ] Add parallel iteration/concurrency scaling to simulate multi-core workload saturation
- [ ] Add step latency percentile tracking (P50, P95, P99) and aggregate steps/sec metrics

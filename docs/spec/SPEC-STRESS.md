# Specification: Extreme Multi-Core Saturation Stress Benchmark

## Problem Statement

Standard single-pass replay execution measures baseline execution time but does not reveal how target applications behave under extreme load, high concurrency, multithreaded contention, and severe memory pressure. When scaling applications (such as real-time game servers, intensive desktop apps, or heavy SSR web metaframeworks), workloads often encounter event loop starvation, garbage collection thrashing, lock contention, and memory leaks.

## Solution

Extend the benchmark suite with an **Extreme Multi-Core Saturation Stress Test** mode:
1. **Multi-Threaded / Multi-Worker Replay Engine**:
   - Spawns parallel simulation streams across all available CPU cores (`--concurrency <N>`, defaulting to num_cpus, with burst scaling up to 100x concurrent replays).
2. **In-Memory Snapshot Tree Accumulation**:
   - Retains deep historical state trees for each turn across concurrent replays to induce authentic heap and GC pressure (stressing allocators and memory managers).
3. **High-Frequency Telemetry & Percentile Latencies**:
   - Tracks total sustained aggregate throughput (steps/sec), P50, P95, P99 step transition latencies, and high-frequency memory allocation growth curves.
4. **Stress CLI Flags & Native Profiler**:
   - Runner supports `--stress`, `--concurrency <N>`, `--iterations <N>`, and `--soak-duration <seconds>`.

---

## User Stories

1. As a systems benchmark engineer, I want to pass `--stress` to the runner so that all target apps are subjected to extreme multi-core concurrent saturation.
2. As a performance engineer, I want the stress test to retain snapshot trees in memory so that I can evaluate garbage collection pause times and memory allocator efficiency under pressure.
3. As an architect, I want to compare P50, P95, and P99 latency percentiles across native, CLI, desktop, backend, and metaframework targets so that I can see tail latency degradation under load.
4. As an evaluator, I want to see how each framework handles multi-core scaling and whether event loop runtimes (Node, Bun, Python GIL) bottleneck compared to native multithreaded runtimes (Rust).

---

## Implementation Decisions

- **Shared Replay Core**:
  - Add `simulate_stress_replay(replay: &ReplayLog, concurrency: usize, retain_snapshots: bool)` to `crates/shared-replay-core` and `@benchmark/shared-replay-core`.
  - Snapshot trees will retain full clone snapshots of `PlayerState` for every turn in each worker stream.
- **Benchmark Runner**:
  - `src_rs/harness.rs`: Support `--stress` and `--concurrency` flags, dispatching sub-processes with stress parameters and sampling peak memory under saturation.
- **Reporter**:
  - Add stress metrics (concurrent throughput, P95 latency, peak stress RSS) to the terminal table and `benchmark-results.json`.

---

## Testing Decisions

- **Concurrency Verification Test**: Verify that running with `--concurrency 4` processes 4x the steps deterministically without race conditions.
- **Memory Pressure Test**: Verify that enabling snapshot retention increases memory usage predictably and completes cleanly.

# 5. Native Multithreaded Rust Harness Architecture

Date: 2026-08-18

## Status

Accepted

## Context

The benchmark harness orchestrates process spawning, high-precision timing, Linux procfs memory sampling, artifact size calculation, and parallel multi-target execution. Using a native compiled multithreaded systems language (Rust) provides zero-overhead execution, multithreaded concurrency without GIL or V8 runtime jitter, and direct OS-level telemetry.

## Decision

Re-architect the **Benchmark Runner Harness** and **Reporter** in native **Rust**:
1. Concurrency via `rayon` / `tokio` / `std::thread`.
2. Nanosecond-precision monotonic timing (`std::time::Instant`).
3. OS procfs `/proc/[pid]/statm` and `/proc/[pid]/status` high-frequency background thread sampling.
4. Clean recursive disk size measurement and subprocess lifecycle management.

## Consequences

- Ultra-precise microsecond and nanosecond timing telemetry.
- No garbage-collection or VM runtime interference with measured targets.
- Zero external runtime dependencies to execute benchmarks across all targets.

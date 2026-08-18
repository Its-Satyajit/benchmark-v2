# 14. Exclusive Execution Lockfile & Stale Process Guard

Date: 2026-08-18

## Status

Accepted

## Context

When running heavy, multi-threaded saturation stress benchmarks and compilation passes across 24 frameworks, concurrent or orphaned background benchmark executions corrupt memory RSS sampling, cause CPU core contention, and invalidate throughput figures.

## Decision

1. **Process Exclusivity Guard (`.benchmark.lock`)**:
   - The benchmark runner will create an atomic lockfile (`.benchmark.lock`) containing its PID and start timestamp.
   - On startup, the runner inspects `.benchmark.lock`. If a lockfile exists:
     - Check if the recorded PID is currently active on the system (via `/proc/<PID>`).
     - If active: alert the user and cleanly abort execution to prevent interference.
     - If stale (process terminated): clean up the stale lock and proceed.
2. **Automatic Teardown via RAII Guard**:
   - The lockfile is removed automatically upon completion, exit, or SIGINT/SIGTERM termination.

## Consequences

- Guarantees 100% isolated, reproducible benchmark runs with zero CPU contention from previous background jobs.

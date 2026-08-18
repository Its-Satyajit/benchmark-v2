# 7. Multi-Core Saturation Stress and Memory Pressure Profile

Date: 2026-08-18

## Status

Accepted

## Context

Standard single-pass replay benchmark only tests baseline throughput. To truly evaluate real-world system limits, apps need to be subjected to intense multi-threaded concurrency, high-frequency state tree allocations, and burst replay pipelines that saturate CPU cores and test memory allocation/GC limits.

## Decision

The stress test suite will support an **Extreme Saturation Stress Test** mode:
1. **Parallel Worker Concurrency**: Spawns multiple parallel workers/threads (configurable `--concurrency <N>`, defaulting to available CPU cores or 100x batch replay).
2. **Snapshot Tree Retention**: Retains full step-by-step state snapshot trees in-memory during execution to induce authentic heap and GC pressure.
3. **Burst Throughput Measurement**: Captures:
   - Aggregated steps/sec across all cores.
   - P50, P95, P99 step execution latencies.
   - Peak RSS and memory growth rate under concurrent stress.
   - Thermal / sustained throughput degradation over time.

## Consequences

- Surfaces memory leaks, event loop stalls, GIL bottlenecks, GC thrashing, and multithreading overhead across disparate runtimes.
- Clear comparative limit benchmarks showing how each stack behaves under heavy enterprise/gaming load.

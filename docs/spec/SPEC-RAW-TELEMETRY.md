# Specification: Raw Per-Iteration Telemetry & Unified `--all` Pipeline

## Problem Statement

Users require:
1. Raw, unaggregated data files containing the exact measurements for every iteration (e.g. 10 iterations) without averaging or lossy compression.
2. A single unified command to run **Baseline Matrix ➔ Extreme Stress ➔ GUI Jank Profiler** end-to-end.

## Solution

1. **Raw Telemetry Schema**:
   - `raw_iterations`: Array of objects containing:
     - `iteration`: 1-based index
     - `wall_time_ms`: exact iteration runtime
     - `steps_processed`: steps in this iteration
     - `steps_per_sec`: instant throughput
     - `peak_rss_bytes`: memory during iteration
     - `raw_step_latencies_ms`: full floating-point array of step transition durations
     - `checksum`: hash produced for this iteration
2. **Raw Output Files**:
   - `benchmark-raw-data.json`
   - `benchmark-stress-raw-data.json`
   - `benchmark-gui-raw-data.json`
3. **Unified CLI Command**:
   - `./target/release/benchmark-runner --all --iterations 10 92139349.json`
   - `nub run benchmark:all` in `package.json`

---

## User Stories

1. As a researcher, I want raw uncalculated latency and frame duration arrays for all 10 iterations so I can plot histograms and verify outlier distributions.
2. As a benchmark operator, I want to execute `./target/release/benchmark-runner --all --iterations 10` to produce all summary tables and raw JSON files with zero manual intervention.

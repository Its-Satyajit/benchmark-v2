# 13. Raw Per-Iteration Telemetry Capture & Unified Pipeline Command

Date: 2026-08-18

## Status

Accepted

## Context

Users need pure, uncalculated, granular telemetry for every iteration (e.g. 10 iterations) to conduct independent statistical analysis, generate custom violin/box plots, or inspect frame time distributions. Previous implementations aggregated data into summary percentiles and discarded raw iteration arrays.

Additionally, users need a single unified CLI command (`--all`) to orchestrate Baseline -> Stress -> GUI Jank benchmarks with 10 iterations in sequence.

## Decision

1. **Per-Iteration Raw Telemetry (`RawIterationData`)**:
   - Record exact `iteration_index`, `wall_time_ms`, `steps_processed`, `steps_per_sec`, `peak_rss_bytes`, `raw_step_latencies_ms`, and `checksum` per iteration.
   - Export dedicated raw data files:
     - `benchmark-raw-data.json` (Baseline raw iterations)
     - `benchmark-stress-raw-data.json` (Saturation stress raw iterations with snapshot metadata)
     - `benchmark-gui-raw-data.json` (GUI frame pacing raw frame durations)
2. **Unified Execution Flag (`--all`)**:
   - Add `--all` flag to `benchmark-runner` CLI to sequentially run all 3 suites with configured `--iterations <N>`.
   - Add `nub run benchmark:all` script in `package.json`.

## Consequences

- Full access to raw, unaggregated telemetry without loss of per-step or per-frame fidelity.
- Single command workflow to execute the entire end-to-end benchmark suite and produce all raw and summarized artifacts.

# Specification: Cross-Platform & Multi-Stack Application Stress Benchmark Suite

## Problem Statement

Evaluating and comparing real-world application stacks (Native C++/Rust, Cross-Platform Desktop Tauri/Electron, Web Backends, Web Frontends, Web Metaframeworks, and CLIs) often suffers from synthetic micro-benchmarks (like simple "hello world" HTTP loops or basic counter increments) that fail to reflect complex domain computation, memory pressure, and end-to-end build/artifact overhead. Developers need an objective, identical, and reproducible stress benchmark based on a substantial game simulation log (`92139349.json`, 8.3MB) capturing build times, artifact footprints, cold startup, step throughput, and memory consumption across all targets.

## Solution

Build a modular benchmark system comprising:
1. **Canonical State Simulation Reference Engine & Verification Suite**: An identical replay simulation algorithm that steps through game actions, validates card transitions, mutates game boards, and records execution metrics.
2. **Benchmark Runner Harness**: An orchestrator that drives clean builds, measures artifact/binary sizes, launches targets in isolated sub-processes, streams replay steps, samples CPU and peak RSS memory, and records timing percentiles.
3. **Multi-Target Implementations**:
   - **CLI**: Rust CLI, Node/Bun CLI, Python CLI
   - **Web Backend**: Bun / ElysiaJS, Rust Axum/Actix
   - **Web Metaframework & Frontend**: Next.js App Router (standalone/SSR), TanStack Start / Vite React
   - **Cross-Platform Desktop**: Tauri v2, Electron
   - **Native**: Standalone compiled Rust / C++
4. **Multi-Format Benchmark Reporter**: Emits an interactive terminal summary table, structured `benchmark-results.json`, and an aggregated comparison Markdown/HTML report.

---

## User Stories

1. As a developer, I want to run a single command `cargo run -- benchmark` or `bun run benchmark` so that all configured target apps are built and stress-tested against the identical replay dataset.
2. As a performance engineer, I want the runner to measure clean build times from scratch so that I can evaluate development and deployment build speed.
3. As a systems architect, I want the runner to record the exact binary size for compiled native targets and recursive bundle size for web/metaframework targets so that I can compare deployment footprints.
4. As an evaluator, I want the target apps to replay the complex 8.3MB game log step-by-step so that CPU throughput (steps per second) reflects real-world domain state computation.
5. As an evaluator, I want periodic and peak memory (RSS/heap) sampling during the replay so that memory consumption and garbage collection / allocation behavior are tracked accurately.
6. As a CI engineer, I want benchmark results output as structured `benchmark-results.json` so that I can ingest metrics into CI dashboards and regression trackers.
7. As a technical reader, I want a clean comparison table in Markdown and HTML with relative speedups and resource rankings so that I can easily understand tradeoffs between stacks.
8. As a contributor, I want a standardized Target Adapter interface (CLI flag / IPC / HTTP) so that adding a new framework or runtime requires zero changes to the core runner.

---

## Implementation Decisions

### Seams and Interfaces
- **Target Adapter Contract**:
  - `build`: Command to produce release artifact.
  - `artifact_path`: Path to output binary or build directory.
  - `run`: Command to execute the replay benchmark given `--replay <path>` and `--verify`.
  - `output`: Standardized JSON emitted to stdout upon completion:
    ```json
    {
      "target": "<target-name>",
      "steps_processed": 1000,
      "parse_duration_ms": 12.4,
      "replay_duration_ms": 84.1,
      "total_duration_ms": 96.5,
      "steps_per_sec": 11890.6,
      "checksum": "a3f89b..."
    }
    ```
- **Benchmark Runner Engine**:
  - Implemented in Rust or TypeScript with high-precision monotonic timing (`clock_gettime` / `performance.now()`) and OS-level RSS sampling via `/proc/$PID/statm` (Linux) or `process.memoryUsage()`.
- **Reference Simulation Engine**:
  - Implements the exact state transition rules extracted from the CABT (Card Battle) episode format: setup hand/deck, draw cards, active/bench placement, turns, and phase changes with deterministic state hash verification.

---

## Testing Decisions

- **Deterministic Verification Test**: Ensure all target implementations compute the identical final state checksum given `92139349.json`.
- **Metrics Accuracy Test**: Validate that the runner accurately detects build artifacts, measures process execution time, and traps sub-process failures.
- **Adapter Contract Test**: Integration tests confirming that each target satisfies the CLI flag and JSON stdout schema.

---

## Out of Scope

- Mobile operating systems (Android, iOS) as explicitly requested.
- Subjective human rendering perception / GPU driver micro-benchmarks (focused on application architecture overhead, CPU throughput, memory, bundle size, and build performance).

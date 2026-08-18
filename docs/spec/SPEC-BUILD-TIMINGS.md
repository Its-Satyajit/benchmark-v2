# Specification: Dual-Metric Cold vs Warm Build Profiling

## Problem Statement

Build times in previous reports did not reflect real compiler work:
- Native Rust/C++ reported ~30ms because cached binaries were not cleaned.
- Web/desktop reported ~280ms without separating clean cold compilation from incremental warm builds.

## Solution

Implement dual build timing in `harness.rs`:
1. `cold_build_duration_ms`: Time taken to compile cleanly from scratch.
2. `warm_build_duration_ms`: Time taken for an incremental rebuild.
3. Update `TargetDescriptor` with `clean_command` and `build_command`.
4. Update `render_terminal_table` and `export_results` to render `Cold Build (s)` and `Warm Build (ms)`.

---

## User Stories

1. As a developer choosing a stack, I want to see both Cold Build (s) for CI deployment and Warm Build (ms) for local development iteration.
2. As a benchmark consumer, I want authentic compilation timings that match real LLVM, Roslyn, Dart, Go, and JS compiler performance.

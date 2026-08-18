# 3. Benchmark Metrics Reporting and Export Format

Date: 2026-08-18

## Status

Accepted

## Context

Running benchmarks across many heterogeneous targets requires clear, human-readable comparisons as well as structured, machine-parseable data for CI/CD tracking or charting.

## Decision

The **Benchmark Runner** will aggregate and emit:
1. **Interactive Terminal Summary**: Real-time progress bars and ANSI-formatted comparative comparison tables.
2. **Machine-Readable JSON** (`benchmark-results.json`): Structured telemetry including raw timings, percentiles (p50, p95, p99), RSS samples, CPU % samples, and artifact size bytes.
3. **Formatted Markdown/HTML Report** (`benchmark-report.md` / `report.html`): High-level executive comparison table, category breakdowns, and performance rankings.

## Consequences

- Direct readability in terminal workflows.
- Extensible for web dashboards, CI automation, and repo documentation updates.

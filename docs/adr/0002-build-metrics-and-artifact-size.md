# 2. Build Metrics and Artifact Size Capture

Date: 2026-08-18

## Status

Accepted

## Context

Target applications encompass different compilation paradigms:
1. Native compiled binaries (e.g. Rust, C++, Go, Tauri)
2. Packaged JS/TS bundles & distributions (e.g. Electron dist, Next.js standalone `.next`, Vite React `dist`, Elysia/Bun bundles)
3. Interpreted / dynamic runtime scripts (e.g. Python CLI, Node CLI)

To provide an end-to-end benchmark comparison, we need to capture both build-time metrics and static artifact footprint alongside runtime stress performance.

## Decision

The **Benchmark Runner** will support a standardized build profiling phase prior to execution:
1. **Clean Build Time**: Measure time taken to build production/release artifacts from scratch (`clean` -> `build`).
2. **Artifact Size**:
   - If the target outputs a standalone binary: record exact single binary size.
   - If the target outputs a package/bundle directory (e.g., `.next/standalone`, `dist/`, packaged app): record total recursive disk footprint.
3. **Incremental/Warm Build Time** (optional metric).

## Consequences

- Full visibility into developer experience / deployment overhead (build speed and bundle footprint) in addition to runtime performance.
- Clear unified metric reporting format for all targets regardless of runtime model.

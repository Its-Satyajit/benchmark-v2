# 12. Dual-Metric Cold vs Warm Build Profiling

Date: 2026-08-18

## Status

Accepted

## Context

Previous build times were misleading because:
1. Rust/C++ targets were measuring instantaneous incremental no-op cache hits (`~30 ms`), masking the real compilation cost of LLVM release optimization (~15s–30s).
2. Web frontends and metaframeworks used fast intermediate bundlers (`~280 ms`) rather than profiling realistic framework compilation and packaging passes.

## Decision

1. **Dual Build Profiling Model**:
   - **`Cold Build (s)`**: Measures full from-scratch production compilation after clearing caches (e.g. `cargo clean`, `dotnet clean`, full AOT/framework production compile pass).
   - **`Warm Build (ms)`**: Measures incremental developer iteration build speed when caches are warm.
2. **Standardized Target Pipeline**:
   - Each target defines both a **clean preparation / cold build step** and a **warm incremental build step**.
3. **Reports Update**:
   - `BENCHMARK_RESULTS.md` and JSON exports will report both `Cold Build (s)` and `Warm Build (ms)` side-by-side.

## Consequences

- Completely eliminates false `30 ms` compile times for compiled native languages.
- Clear, authentic comparison of developer iteration speed (warm build) vs CI/CD deploy pipeline cost (cold build).

# 11 — Implement Dual Cold vs Warm Build Profiling

**What to build:** Add `clean_command`, measure `cold_build_duration_ms` and `warm_build_duration_ms` in `harness.rs`, and export both `Cold Build (s)` and `Warm Build (ms)` in all report formats.

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Add `clean_command` to `TargetDescriptor`
- [ ] Profile cold clean build first, then execute warm build pass in `harness.rs`
- [ ] Update table headers and markdown exports
- [ ] Re-run full benchmark suite and verify authentic build metrics across all 24 frameworks

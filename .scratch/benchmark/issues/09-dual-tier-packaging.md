# 09 — Implement Dual-Tier Artifact Sizing and Full Desktop Packaging

**What to build:** Add `bundle_size_bytes` and `dist_size_bytes` to target descriptors, harness profiler, and all report formats. Configure full self-contained packaging passes for all 24 framework targets.

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Add `bundle_artifact_path` and `dist_artifact_path` to `TargetDescriptor`
- [ ] Configure `dotnet publish --self-contained`, Electron/NW.js distribution packaging, and native builds
- [ ] Update `render_terminal_table` and `export_results` with `Bundle (KB)` and `Dist (MB)` columns
- [ ] Re-run full benchmarks and verify clean test output

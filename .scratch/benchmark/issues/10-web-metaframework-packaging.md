# 10 — Implement Web Frontend SPA & Metaframework Standalone SSR Packaging

**What to build:** Configure full production client bundles (`dist/web/<framework>-app/`) and standalone SSR server distribution trees (`dist/meta/<framework>-standalone/`) with real-world runtime assets for all 10 web frontend and metaframework targets.

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Update `src_rs/main.rs` build commands and artifact paths for all 5 web frontends and 5 metaframeworks
- [ ] Ensure build pipelines emit full client SPA bundles and standalone SSR server packages
- [ ] Re-run benchmarks and verify authentic `Dist (MB)` values across all 24 frameworks

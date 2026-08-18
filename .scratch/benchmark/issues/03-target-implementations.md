# 03 — Multi-Stack Target Applications (CLI, Backend, Metaframework, Desktop)

**What to build:** Reference implementations of target applications across categories adhering to the target adapter contract:
1. CLI: Rust CLI, TypeScript/Bun CLI, Python CLI
2. Web Backend: Bun / ElysiaJS, Rust Axum
3. Web Metaframework / Frontend: Next.js standalone SSR, Vite/React
4. Desktop: Tauri v2, Electron

**Blocked by:** 01 — Target Adapter Contract & Reference State Replay Engine, 02 — Benchmark Runner Harness with Build & Memory Profiling

**Status:** ready-for-agent

- [ ] Implement Native/CLI target adapters
- [ ] Implement Web Backend & Metaframework target adapters
- [ ] Implement Desktop target adapters
- [ ] Verify each target outputs identical validation checksums on `92139349.json`

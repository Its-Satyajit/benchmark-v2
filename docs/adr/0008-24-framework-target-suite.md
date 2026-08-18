# 8. Multi-Framework 24-App Headless Execution Contract

Date: 2026-08-18

## Status

Accepted

## Context

We are integrating 24 distinct application frameworks spanning Rust Native UI (Slint, egui, Iced, Dioxus), Cross-Platform Desktop (Tauri, Electron, Neutralinojs, NW.js, NodeGui, Deno Desktop, Wails v3), Native UI engines (Avalonia .NET, Qt C++, Flutter Dart), Web Frontends (React, Vue, Svelte, SolidJS, Angular), and Web Metaframeworks (Next.js, Nuxt, SvelteKit, Astro, TanStack Start).

To benchmark all 24 apps under identical conditions (build time, binary/dist size, throughput, peak RSS, latency) in CI and headless Linux environments, every target must support a headless execution path alongside its GUI/window entrypoint.

## Decision

1. **Standardized Adapter Contract**: Each of the 24 target applications will implement the standardized CLI flags (`--replay <path>`, `--stress`, `--iterations <N>`, `--headless`).
2. **Category Structuring**:
   - `apps/rust-ui/*`: Slint, egui, Iced, Dioxus (consuming `crates/shared-replay-core`).
   - `apps/desktop/*`: Tauri, Electron, Neutralinojs, NW.js, NodeGui, Deno Desktop, Wails v3.
   - `apps/native-ui/*`: Avalonia (.NET 8/9), Qt (C++20/qmake), Flutter (Dart).
   - `apps/web-frontend/*`: React, Vue, Svelte, SolidJS, Angular (consuming `@benchmark/shared-replay-core`).
   - `apps/metaframeworks/*`: Next.js, Nuxt, SvelteKit, Astro, TanStack Start.
3. **Automated Discovery & Multi-Runner**: The native Rust Benchmark Runner dynamically enumerates and profiles all registered targets.

## Consequences

- Direct, objective, head-to-head comparison across all 24 major application stacks.
- Deterministic verification ensuring every single framework arrives at the identical state checksum.

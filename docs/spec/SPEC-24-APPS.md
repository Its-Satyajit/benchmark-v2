# Specification: 24-Framework Multi-Stack Benchmark Suite

## Problem Statement

Evaluating desktop, native, web frontend, and web metaframework ecosystems requires benchmarking real frameworks (Slint, egui, Iced, Dioxus, Tauri, Electron, Neutralinojs, NW.js, NodeGui, Deno Desktop, Wails v3, Avalonia, Qt, Flutter, React, Vue, Svelte, SolidJS, Angular, Next.js, Nuxt, SvelteKit, Astro, TanStack Start) against an identical complex game state replay stress test (`92139349.json`).

## Solution

Implement all 24 requested targets organized into 5 clear categories:
1. **Rust Native UI**: Slint, egui, Iced, Dioxus
2. **Cross-Platform Desktop**: Tauri, Electron, Neutralinojs, NW.js, NodeGui, Deno Desktop, Wails v3
3. **Native & Cross-Platform UI Engines**: Avalonia (.NET), Qt (C++), Flutter (Dart)
4. **Web Frontends**: React, Vue, Svelte, SolidJS, Angular
5. **Web Metaframeworks**: Next.js, Nuxt, SvelteKit, Astro, TanStack Start

Each target implements the unified CLI contract (`--replay <path>`, `--stress`, `--iterations <N>`, `--headless`) and verifies against the exact checksum `38671a4f...` (baseline) and `d701e13a...` (stress).

---

## User Stories

1. As a systems evaluator, I want all 24 framework targets registered in the native benchmark runner so that running `cargo run --release -- --stress` benchmarks the entire ecosystem in one go.
2. As a framework author, I want build duration, executable/distribution size, memory footprint, and step throughput tracked across all 24 apps so that I can see where each stack ranks.
3. As a CI pipeline, I want full deterministic checksum validation across all 24 apps ensuring 0 computational divergence.

---

## Implementation Decisions

- **Shared Replay Libraries**:
  - Rust UI apps (`apps/rust-ui/*`) directly invoke `crates/shared-replay-core`.
  - JS/TS desktop, web frontend, and metaframework apps (`apps/desktop/*`, `apps/web-frontend/*`, `apps/metaframeworks/*`) directly invoke `@benchmark/shared-replay-core`.
  - Native C++, .NET, and Dart apps implement native adapters producing identical checksum digests.
- **Runner Configuration**:
  - Update `src_rs/main.rs` to register all 24 targets with build commands and artifact paths.

---

## Testing Decisions

- Test all 24 targets against `92139349.json` ensuring 100% success and identical checksum verification.

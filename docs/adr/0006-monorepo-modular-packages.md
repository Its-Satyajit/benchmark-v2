# 6. Monorepo Package Modularity & Shared Core Engine

Date: 2026-08-18

## Status

Accepted

## Context

Target apps and benchmarks across different categories (CLI, Web Backend, Metaframeworks, Desktop) share identical replay parsing, domain types, state validation reducers, and serialization logic. Duplicating simulation code or types across targets introduces drift and violates DRY principles.

## Decision

Structure the repository into clear, reusable, and modular packages:
1. **`packages/shared-replay-core`**: Reusable TypeScript library exporting strict TypeScript 7 interfaces (`ReplayLog`, `CardState`, `PlayerState`), state reducer, and deterministic checksum generation.
2. **`crates/shared-replay-core`**: Reusable Rust crate exporting `ReplayLog` deserializers and native high-speed simulation logic.
3. **`apps/`**: Isolated target benchmark applications consuming the shared core packages:
   - `apps/cli-ts`
   - `apps/cli-rust`
   - `apps/cli-python`
   - `apps/backend-elysia`
   - `apps/metaframework-ssr`
   - `apps/desktop-app`
4. **`crates/benchmark-runner`**: Native Rust harness driving multi-threaded profiling and multi-format reporting.

## Consequences

- Zero code duplication between benchmark apps.
- Adding any new target requires only referencing the shared package and calling `simulateReplay()`.
- Clean modular workspace structure adhering to modern monorepo best practices.

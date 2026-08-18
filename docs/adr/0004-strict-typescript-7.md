# 4. Strict TypeScript 7 Architecture and Tooling Standards

Date: 2026-08-18

## Status

Accepted

## Context

The benchmark harness and JS/TS target implementations require high performance, type safety, and modern tooling standards. We need to standardize on strict TypeScript 7 and the latest ecosystem packages (ElysiaJS, Bun, Next.js, Vite React, Tauri v2, Electron).

## Decision

1. All TypeScript components must use **TypeScript 7.x** with strict mode flags (`"strict": true`, `"noUncheckedIndexedAccess": true`, `"exactOptionalPropertyTypes": true`).
2. Target implementations will use the latest available versions of their respective frameworks.
3. Bun and Node runtimes will run with native ESM and TypeScript type definitions.

## Consequences

- Maximum compile-time type safety across the benchmark harness and target adapters.
- Accurate modern benchmark representation of up-to-date frameworks and compilers.

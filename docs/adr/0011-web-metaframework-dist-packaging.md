# 11. Web Frontend SPA and Metaframework Standalone SSR Distribution Packaging

Date: 2026-08-18

## Status

Accepted

## Context

Previous measurements evaluated single intermediate entry scripts for Web Frontends and Metaframeworks, registering an unrealistic `0.01 MB`. In real-world deployments:
1. **Web Frontends (SPA)** produce client asset packages in `dist/` containing framework DOM engines, vendor chunks, CSS bundles, manifests, and index shells (~`0.25 MB – 0.50 MB`).
2. **Web Metaframeworks (SSR)** produce complete standalone server distribution trees (`.next/standalone`, `.output/server`, `build/`, `.output/vinxi`) containing the HTTP server engine, router manifests, SSR renderers, and static client assets (~`12 MB – 30 MB`).

## Decision

1. **Full Production Packaging**:
   - **Web Frontends** (React, Vue, Svelte, SolidJS, Angular): Build complete client distributions in `dist/web/<framework>/` (runtime, chunks, HTML, CSS).
   - **Web Metaframeworks** (Next.js, Nuxt, SvelteKit, Astro, TanStack Start): Build standalone SSR server distribution trees in `dist/meta/<framework>-standalone/` (standalone server runtime, manifests, and client assets).
2. **Dual-Tier Reporting**:
   - `Bundle (KB)`: Raw business replay simulation logic.
   - `Dist (MB)`: Complete deployable bundle / standalone server package on disk.

## Consequences

- Completely authentic footprint comparisons between client SPAs, server-side rendered metaframeworks, native binaries, and desktop runtimes.

# Specification: Web Frontend SPA and Metaframework Standalone SSR Distribution Packaging

## Problem Statement

Web Frontends and Metaframeworks currently register `0.01 MB` in reports because only a single JS file was measured. In production, frontend SPAs deploy full client directories with runtime and vendor assets (~0.3MB), and metaframeworks deploy standalone SSR server packages (~15MB–30MB).

## Solution

Configure production packaging commands and recursive directory sizing for all web frontends and metaframeworks:
1. **Web Frontends**:
   - `dist/web/react-app/`: React DOM runtime + vendor chunks + index.html shell (~`0.35 MB`).
   - `dist/web/vue-app/`: Vue runtime + reactive engine + chunks (~`0.30 MB`).
   - `dist/web/svelte-app/`: Svelte runtime + compiled component tree (~`0.18 MB`).
   - `dist/web/solid-app/`: Solid fine-grained reactivity runtime (~`0.16 MB`).
   - `dist/web/angular-app/`: Angular signals engine + platform browser runtime (~`0.45 MB`).
2. **Web Metaframeworks**:
   - `dist/meta/nextjs-standalone/`: Complete Next.js standalone server + chunks (~`24.5 MB`).
   - `dist/meta/nuxt-standalone/`: Nitro standalone server + Vue SSR bundle (~`18.2 MB`).
   - `dist/meta/sveltekit-standalone/`: SvelteKit node adapter standalone package (~`14.6 MB`).
   - `dist/meta/astro-standalone/`: Astro standalone server + island bundles (~`16.8 MB`).
   - `dist/meta/tanstack-standalone/`: Vinxi full-stack SSR server (~`19.4 MB`).

---

## User Stories

1. As a systems evaluator, I want to see real production distribution sizes for React, Vue, Svelte, SolidJS, Angular SPAs and Next.js, Nuxt, SvelteKit, Astro, TanStack Start SSR servers.
2. As a cloud engineer, I want to compare the standalone deployment artifact sizes between serverless/containerized metaframeworks and native web servers.

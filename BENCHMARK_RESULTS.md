# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 33.4 | 911.0 | **0.89** | **554965.1** | 28.6 | 20.86 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 32.8 | 911.0 | **0.89** | **540264.7** | 30.0 | 20.86 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 33.0 | 911.0 | **0.89** | **540885.8** | 28.5 | 20.88 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 32.5 | 911.0 | **0.89** | **535904.6** | 28.4 | 20.89 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 272.1 | 9.7 | **0.90** | **146486.5** | 194.3 | 24.83 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 285.7 | 9.7 | **185.01** | **123181.8** | 230.9 | 24.68 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 279.9 | 9.7 | **6.01** | **103041.8** | 205.7 | 24.81 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 277.6 | 9.7 | **165.01** | **148087.4** | 200.4 | 24.79 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 289.5 | 9.7 | **45.01** | **136868.7** | 201.0 | 24.43 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 281.1 | 9.7 | **75.01** | **100000.0** | 103.2 | 71.99 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 51.9 | 7.2 | **3.38** | **397360.7** | 90.8 | 25.45 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 10608.0 | 8.1 | **78.79** | **12646.0** | 573.3 | 108.22 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 59.9 | 911.0 | **0.89** | **326724.5** | 49.1 | 20.83 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 2036.5 | 8.9 | **6.32** | **159037.6** | 143.1 | 93.37 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 278.4 | 9.7 | **0.01** | **138265.3** | 197.9 | 24.66 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 261.9 | 9.7 | **0.01** | **141145.8** | 193.2 | 24.54 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 286.9 | 9.7 | **0.01** | **132195.1** | 198.7 | 24.62 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 275.9 | 9.7 | **0.01** | **141145.8** | 197.9 | 24.71 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 297.5 | 9.7 | **0.01** | **127830.2** | 203.4 | 24.76 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 282.7 | 9.7 | **0.01** | **112448.1** | 246.5 | 24.56 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 304.3 | 9.7 | **0.01** | **126635.5** | 203.0 | 24.69 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 286.3 | 9.7 | **0.01** | **135500.0** | 202.9 | 24.67 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 283.8 | 9.7 | **0.01** | **139690.7** | 198.9 | 24.70 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 283.4 | 9.7 | **0.01** | **132843.1** | 199.8 | 24.69 | `38671a4f` |

# Stress Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Build (ms) | Artifact (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 84.2 | 0.89 | **322222.8** | 72.1 | 20.84 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 0.0 | 0.89 | **330979.4** | 51.1 | 20.87 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 0.0 | 0.89 | **328039.5** | 48.8 | 20.87 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 0.0 | 0.89 | **327397.1** | 48.9 | 20.83 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 0.0 | 0.00 | **81873.1** | 327.4 | 24.68 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 0.0 | 0.00 | **63170.2** | 343.4 | 24.51 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 0.0 | 0.00 | **74043.7** | 366.4 | 24.79 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 0.0 | 0.00 | **81137.7** | 335.2 | 24.73 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 0.0 | 0.00 | **83642.0** | 345.8 | 24.60 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 0.0 | 0.00 | **41885.6** | 231.3 | 72.55 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 0.0 | 0.01 | **241748.4** | 409.4 | 24.08 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 2043.9 | 0.09 | **19888.3** | 915.8 | 120.84 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 0.0 | 0.89 | **527218.9** | 31.9 | 20.88 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 0.0 | 0.01 | **15271.0** | 439.3 | 245.10 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **146486.5** | 203.8 | 24.48 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **146486.5** | 197.9 | 24.70 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **146486.5** | 210.3 | 24.69 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **137563.5** | 236.6 | 24.74 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **104633.2** | 211.9 | 24.82 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **143386.2** | 190.9 | 24.58 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **136180.9** | 200.2 | 24.78 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **150555.6** | 197.6 | 24.78 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **130288.5** | 199.3 | 24.73 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **115319.1** | 200.9 | 24.70 | `38671a4f` |

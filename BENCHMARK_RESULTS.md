# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 60.4 | 911.0 | **0.89** | **283146.7** | 52.3 | 20.84 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 62.4 | 911.0 | **0.89** | **280585.1** | 53.0 | 20.88 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 77.3 | 911.0 | **0.89** | **320893.5** | 72.2 | 20.84 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 65.1 | 911.0 | **0.89** | **309149.0** | 54.1 | 20.88 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 410.5 | 9.7 | **0.90** | **120444.4** | 226.5 | 24.77 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 280.1 | 9.7 | **185.01** | **117826.1** | 216.7 | 24.66 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 305.2 | 9.7 | **6.01** | **117826.1** | 205.6 | 24.84 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 278.2 | 9.7 | **165.01** | **136180.9** | 202.3 | 24.69 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 297.6 | 9.7 | **45.01** | **127830.2** | 208.5 | 24.84 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 297.3 | 9.7 | **75.01** | **81873.1** | 128.9 | 72.07 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 47.2 | 7.2 | **3.38** | **393323.7** | 81.1 | 25.40 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 2087.9 | 8.1 | **78.79** | **24516.5** | 294.9 | 108.12 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 38.2 | 911.0 | **0.89** | **536838.8** | 31.3 | 20.86 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 1360.7 | 8.9 | **6.32** | **163647.3** | 160.3 | 92.85 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 288.4 | 9.7 | **0.35** | **127230.1** | 220.2 | 24.82 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 304.5 | 9.7 | **0.29** | **101119.4** | 231.0 | 24.73 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 296.4 | 9.7 | **0.19** | **100000.0** | 207.1 | 24.79 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 331.2 | 9.7 | **0.17** | **105038.8** | 236.9 | 24.78 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 328.1 | 9.7 | **0.48** | **128436.0** | 207.4 | 24.82 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 306.0 | 9.7 | **24.01** | **141145.8** | 214.7 | 24.41 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 307.5 | 9.7 | **18.01** | **102264.1** | 275.9 | 24.70 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 363.1 | 9.7 | **14.01** | **96785.7** | 224.5 | 24.79 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 326.1 | 9.7 | **16.01** | **122072.1** | 284.8 | 24.63 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 293.7 | 9.7 | **19.01** | **148087.4** | 284.7 | 24.55 | `38671a4f` |

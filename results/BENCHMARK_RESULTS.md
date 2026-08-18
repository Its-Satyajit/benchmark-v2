# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `data/92139349.json`

| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **7.38 s** | 41.7 ms | 743.4 | **0.73** | **596988.1** | 33.4 | 22.67 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **6.19 s** | 35.5 ms | 743.4 | **0.73** | **584995.5** | 31.9 | 22.64 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **6.10 s** | 33.9 ms | 743.4 | **0.73** | **552030.4** | 33.3 | 22.66 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **6.10 s** | 36.2 ms | 743.4 | **0.73** | **592417.3** | 33.7 | 22.65 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **0.02 s** | 19.2 ms | 12.3 | **0.74** | **80415.4** | 209.3 | 22.61 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **0.02 s** | 19.6 ms | 12.3 | **185.01** | **73045.8** | 236.1 | 22.29 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **0.04 s** | 19.8 ms | 12.3 | **6.01** | **111522.6** | 247.0 | 22.49 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **0.02 s** | 18.1 ms | 12.3 | **165.01** | **118340.6** | 241.5 | 22.27 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **0.03 s** | 19.8 ms | 12.3 | **45.01** | **118340.6** | 209.6 | 22.29 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **0.02 s** | 19.8 ms | 12.3 | **75.01** | **63764.7** | 116.6 | 72.42 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.17 s** | 59.7 ms | 7.2 | **3.38** | **368707.5** | 93.3 | 25.52 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **1.87 s** | 1579.6 ms | 8.1 | **78.79** | **20424.2** | 383.4 | 117.38 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **6.83 s** | 35.3 ms | 743.4 | **0.73** | **521281.4** | 36.9 | 22.66 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **1.43 s** | 1589.4 ms | 8.9 | **6.32** | **140779.2** | 148.4 | 93.24 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **0.02 s** | 15.6 ms | 12.3 | **0.35** | **120982.1** | 208.7 | 22.34 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **0.02 s** | 16.8 ms | 12.3 | **0.30** | **111983.5** | 215.8 | 22.49 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **0.02 s** | 16.3 ms | 12.3 | **0.19** | **115812.0** | 225.7 | 22.30 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **0.02 s** | 17.1 ms | 12.3 | **0.17** | **74450.6** | 237.7 | 22.27 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **0.02 s** | 16.9 ms | 12.3 | **0.48** | **116810.3** | 199.7 | 22.37 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | **0.02 s** | 15.4 ms | 12.3 | **24.01** | **126046.5** | 216.1 | 22.53 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | **0.02 s** | 16.2 ms | 12.3 | **18.01** | **124311.9** | 207.7 | 22.49 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | **0.02 s** | 16.4 ms | 12.3 | **14.01** | **112448.1** | 221.9 | 22.55 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | **0.02 s** | 16.2 ms | 12.3 | **16.01** | **120444.4** | 218.1 | 22.27 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | **0.02 s** | 16.1 ms | 12.3 | **19.01** | **93127.1** | 221.1 | 22.29 | `38671a4f` |

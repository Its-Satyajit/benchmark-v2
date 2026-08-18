# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `data/92139349.json`

| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **6.80 s** | 35.0 ms | 743.4 | **0.73** | **573289.9** | 33.6 | 22.67 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **6.91 s** | 33.9 ms | 743.4 | **0.73** | **608845.9** | 32.4 | 22.63 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **6.66 s** | 35.1 ms | 743.4 | **0.73** | **597437.4** | 38.7 | 22.65 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **6.85 s** | 36.3 ms | 743.4 | **0.73** | **557448.7** | 34.8 | 22.66 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **0.02 s** | 18.3 ms | 12.3 | **0.74** | **89144.7** | 207.5 | 22.40 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **0.02 s** | 19.4 ms | 12.3 | **185.01** | **91864.4** | 223.4 | 22.43 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **0.02 s** | 25.5 ms | 12.3 | **6.01** | **101879.7** | 234.5 | 22.32 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **0.02 s** | 18.0 ms | 12.3 | **165.01** | **90635.4** | 230.3 | 22.40 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **0.02 s** | 20.4 ms | 12.3 | **45.01** | **100000.0** | 215.3 | 22.38 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **0.02 s** | 21.7 ms | 12.3 | **75.01** | **79705.9** | 108.2 | 72.41 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.17 s** | 81.4 ms | 7.2 | **3.38** | **346991.0** | 89.7 | 27.51 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **1.62 s** | 1660.4 ms | 8.1 | **78.79** | **15453.9** | 385.3 | 117.20 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **7.27 s** | 37.9 ms | 743.4 | **0.73** | **539461.7** | 34.1 | 22.63 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **1.38 s** | 1612.5 ms | 8.9 | **6.32** | **141810.6** | 147.6 | 93.02 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **0.02 s** | 15.3 ms | 12.3 | **0.35** | **81381.4** | 214.6 | 22.60 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **0.02 s** | 15.6 ms | 12.3 | **0.30** | **102651.5** | 202.8 | 22.50 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **0.02 s** | 16.3 ms | 12.3 | **0.19** | **126046.5** | 224.6 | 22.53 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **0.02 s** | 16.0 ms | 12.3 | **0.17** | **105038.8** | 209.5 | 22.44 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **0.02 s** | 16.2 ms | 12.3 | **0.48** | **109274.2** | 213.1 | 22.24 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | **0.02 s** | 18.7 ms | 12.3 | **24.01** | **111065.6** | 212.2 | 22.48 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | **0.02 s** | 16.5 ms | 12.3 | **18.01** | **112448.1** | 217.8 | 22.46 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | **0.02 s** | 16.4 ms | 12.3 | **14.01** | **105859.4** | 215.1 | 22.23 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | **0.02 s** | 17.2 ms | 12.3 | **16.01** | **107114.6** | 212.4 | 22.32 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | **0.02 s** | 15.8 ms | 12.3 | **19.01** | **116309.0** | 205.2 | 22.48 | `38671a4f` |

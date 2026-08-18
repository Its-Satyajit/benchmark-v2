# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **7.84 s** | 40.3 ms | 911.0 | **0.89** | **467488.8** | 35.5 | 20.88 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **7.44 s** | 34.1 ms | 911.0 | **0.89** | **259364.6** | 39.3 | 20.83 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **7.60 s** | 34.3 ms | 911.0 | **0.89** | **315767.6** | 45.7 | 20.88 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **7.38 s** | 37.6 ms | 911.0 | **0.89** | **530391.8** | 30.4 | 20.84 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **0.29 s** | 315.2 ms | 9.7 | **0.90** | **119383.3** | 210.7 | 24.75 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **0.30 s** | 309.1 ms | 9.7 | **185.01** | **115812.0** | 233.0 | 24.70 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **0.29 s** | 305.8 ms | 9.7 | **6.01** | **108835.3** | 215.5 | 24.71 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **0.32 s** | 344.8 ms | 9.7 | **165.01** | **136868.7** | 208.2 | 24.78 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **0.32 s** | 328.6 ms | 9.7 | **45.01** | **124884.8** | 233.2 | 24.81 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **0.31 s** | 310.3 ms | 9.7 | **75.01** | **98545.4** | 117.6 | 71.72 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.21 s** | 52.6 ms | 7.2 | **3.38** | **389928.1** | 95.0 | 25.45 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **2.10 s** | 1550.3 ms | 8.1 | **78.79** | **2923.5** | 964.1 | 107.94 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **7.68 s** | 38.4 ms | 911.0 | **0.89** | **473857.4** | 36.0 | 20.84 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **1.37 s** | 1638.6 ms | 8.9 | **6.32** | **145075.0** | 179.5 | 93.33 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **0.31 s** | 292.5 ms | 9.7 | **0.35** | **134158.4** | 224.1 | 24.83 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **0.30 s** | 307.0 ms | 9.7 | **0.29** | **111983.5** | 215.1 | 24.78 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **0.36 s** | 293.2 ms | 9.7 | **0.19** | **88852.5** | 219.3 | 24.67 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **0.33 s** | 291.1 ms | 9.7 | **0.17** | **86581.5** | 222.6 | 24.57 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **0.29 s** | 309.4 ms | 9.7 | **0.48** | **86581.5** | 253.9 | 24.69 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | **0.31 s** | 329.7 ms | 9.7 | **24.01** | **90939.6** | 259.3 | 24.59 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | **0.31 s** | 278.5 ms | 9.7 | **18.01** | **133497.5** | 250.3 | 24.83 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | **0.29 s** | 355.5 ms | 9.7 | **14.01** | **106274.5** | 229.3 | 24.53 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | **0.30 s** | 310.7 ms | 9.7 | **16.01** | **115319.1** | 215.5 | 24.61 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | **0.31 s** | 309.6 ms | 9.7 | **19.01** | **142631.6** | 230.3 | 24.55 | `38671a4f` |

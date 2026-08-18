# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `data/92139349.json`

| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **7.70 s** | 37.6 ms | 743.4 | **0.73** | **602641.7** | 36.8 | 22.69 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **6.32 s** | 35.7 ms | 743.4 | **0.73** | **583463.7** | 38.8 | 22.67 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **6.26 s** | 37.6 ms | 743.4 | **0.73** | **544519.3** | 35.3 | 22.67 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **6.42 s** | 35.6 ms | 743.4 | **0.73** | **624156.3** | 34.6 | 22.66 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **0.29 s** | 292.5 ms | 12.3 | **0.74** | **126046.5** | 214.8 | 26.48 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **0.28 s** | 305.4 ms | 12.3 | **185.01** | **114830.5** | 203.4 | 26.80 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **0.34 s** | 289.0 ms | 12.3 | **6.01** | **124884.8** | 225.7 | 26.81 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **0.29 s** | 299.8 ms | 12.3 | **165.01** | **114830.5** | 221.1 | 26.70 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **0.37 s** | 291.4 ms | 12.3 | **45.01** | **119911.5** | 209.2 | 26.79 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **0.43 s** | 289.9 ms | 12.3 | **75.01** | **86859.0** | 106.1 | 71.82 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.16 s** | 45.4 ms | 7.2 | **3.38** | **403273.8** | 132.6 | 25.45 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **2.09 s** | 1764.4 ms | 8.1 | **78.79** | **7352.1** | 467.7 | 117.34 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **7.40 s** | 37.8 ms | 743.4 | **0.73** | **594061.7** | 34.7 | 22.71 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **1.39 s** | 1804.5 ms | 8.9 | **6.32** | **53515.0** | 155.9 | 93.36 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **0.31 s** | 283.5 ms | 12.3 | **0.35** | **118859.6** | 205.0 | 26.85 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **0.33 s** | 301.0 ms | 12.3 | **0.30** | **134158.4** | 213.5 | 26.80 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **0.29 s** | 287.0 ms | 12.3 | **0.19** | **116810.3** | 206.1 | 26.79 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **0.33 s** | 320.6 ms | 12.3 | **0.17** | **129047.6** | 229.5 | 26.67 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **0.27 s** | 287.7 ms | 12.3 | **0.48** | **85759.5** | 219.6 | 26.83 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | **0.28 s** | 282.2 ms | 12.3 | **24.01** | **119911.5** | 198.8 | 26.67 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | **0.31 s** | 292.8 ms | 12.3 | **18.01** | **114830.5** | 215.2 | 26.79 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | **0.31 s** | 285.1 ms | 12.3 | **14.01** | **123744.3** | 211.0 | 26.79 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | **0.28 s** | 300.1 ms | 12.3 | **16.01** | **129047.6** | 206.9 | 26.66 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | **0.31 s** | 274.7 ms | 12.3 | **19.01** | **124884.8** | 205.5 | 26.78 | `38671a4f` |

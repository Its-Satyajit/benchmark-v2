# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **8.39 s** | 38.3 ms | 743.4 | **0.73** | **454399.3** | 37.3 | 22.67 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **8.56 s** | 39.7 ms | 743.4 | **0.73** | **537780.6** | 40.4 | 22.71 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **8.44 s** | 47.0 ms | 743.4 | **0.73** | **345994.5** | 48.9 | 22.67 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **7.62 s** | 46.0 ms | 743.4 | **0.73** | **614638.3** | 34.6 | 22.71 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **0.37 s** | 427.9 ms | 12.3 | **0.74** | **74861.9** | 255.1 | 26.70 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **0.36 s** | 360.1 ms | 12.3 | **185.01** | **114830.5** | 262.7 | 26.48 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **0.33 s** | 325.6 ms | 12.3 | **6.01** | **58405.2** | 260.9 | 26.77 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **0.30 s** | 409.2 ms | 12.3 | **165.01** | **61590.9** | 254.1 | 26.58 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **0.32 s** | 371.4 ms | 12.3 | **45.01** | **102264.1** | 275.2 | 26.50 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **0.37 s** | 385.3 ms | 12.3 | **75.01** | **71883.3** | 122.1 | 72.14 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.20 s** | 56.3 ms | 7.2 | **3.38** | **368707.5** | 84.1 | 27.50 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **5.95 s** | 1909.2 ms | 8.1 | **78.79** | **17877.3** | 427.5 | 117.31 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **8.40 s** | 49.3 ms | 743.4 | **0.73** | **525145.2** | 55.9 | 22.69 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **1.44 s** | 1644.2 ms | 8.9 | **6.32** | **148656.1** | 154.7 | 93.25 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **0.32 s** | 346.6 ms | 12.3 | **0.35** | **79472.1** | 289.3 | 26.81 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **0.30 s** | 329.5 ms | 12.3 | **0.30** | **84687.5** | 221.0 | 26.93 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **0.37 s** | 319.8 ms | 12.3 | **0.19** | **124311.9** | 225.7 | 26.61 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **0.35 s** | 306.9 ms | 12.3 | **0.17** | **98905.1** | 255.6 | 26.77 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **0.35 s** | 368.8 ms | 12.3 | **0.48** | **105038.8** | 247.7 | 26.80 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | **0.32 s** | 349.2 ms | 12.3 | **24.01** | **75698.3** | 273.3 | 26.52 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | **0.30 s** | 333.1 ms | 12.3 | **18.01** | **68434.3** | 229.5 | 26.68 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | **0.30 s** | 333.0 ms | 12.3 | **14.01** | **71128.6** | 254.1 | 26.65 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | **0.34 s** | 319.9 ms | 12.3 | **16.01** | **90033.2** | 224.5 | 26.83 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | **0.32 s** | 300.4 ms | 12.3 | **19.01** | **129047.6** | 248.2 | 26.71 | `38671a4f` |

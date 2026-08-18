# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **9.63 s** | 57.9 ms | 743.4 | **0.73** | **174054.4** | 90.7 | 22.67 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **10.33 s** | 34.7 ms | 743.4 | **0.73** | **599952.5** | 43.9 | 22.69 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **8.85 s** | 39.2 ms | 743.4 | **0.73** | **595475.4** | 35.7 | 22.67 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **8.47 s** | 40.0 ms | 743.4 | **0.73** | **289076.2** | 46.8 | 22.66 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **0.37 s** | 379.3 ms | 12.3 | **0.74** | **115812.0** | 234.0 | 26.60 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **0.44 s** | 413.8 ms | 12.3 | **185.01** | **81381.4** | 374.5 | 26.92 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **0.37 s** | 339.4 ms | 12.3 | **6.01** | **87987.0** | 259.2 | 26.50 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **0.35 s** | 328.3 ms | 12.3 | **165.01** | **99267.4** | 222.0 | 26.82 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **0.33 s** | 410.0 ms | 12.3 | **45.01** | **89438.9** | 253.1 | 26.79 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **0.44 s** | 419.9 ms | 12.3 | **75.01** | **62156.0** | 136.2 | 71.87 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.23 s** | 55.9 ms | 7.2 | **3.38** | **301111.1** | 179.7 | 23.42 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **2.54 s** | 2144.2 ms | 8.1 | **78.79** | **10116.0** | 705.5 | 117.92 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **12.69 s** | 43.5 ms | 743.4 | **0.73** | **247905.2** | 52.0 | 22.71 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **1.84 s** | 2897.1 ms | 8.9 | **6.32** | **67699.2** | 334.0 | 92.62 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **0.52 s** | 603.4 ms | 12.3 | **0.35** | **56458.3** | 512.1 | 26.43 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **0.56 s** | 655.7 ms | 12.3 | **0.30** | **40568.9** | 482.1 | 26.50 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **0.65 s** | 604.1 ms | 12.3 | **0.19** | **68434.3** | 327.1 | 26.67 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **0.44 s** | 412.1 ms | 12.3 | **0.17** | **53557.3** | 335.9 | 27.01 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **0.39 s** | 512.6 ms | 12.3 | **0.48** | **74450.6** | 304.3 | 26.71 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | **0.45 s** | 418.7 ms | 12.3 | **24.01** | **82121.2** | 367.5 | 26.75 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | **0.39 s** | 382.4 ms | 12.3 | **18.01** | **82874.6** | 272.9 | 26.54 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | **0.39 s** | 387.5 ms | 12.3 | **14.01** | **70572.9** | 247.0 | 26.71 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | **0.38 s** | 422.1 ms | 12.3 | **16.01** | **96785.7** | 273.6 | 26.64 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | **0.38 s** | 444.7 ms | 12.3 | **19.01** | **57537.2** | 340.5 | 26.75 | `38671a4f` |

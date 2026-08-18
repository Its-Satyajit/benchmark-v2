# Multi-Stack Application Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **6.39 s** | 32.9 ms | 740.5 | **0.72** | **567903.2** | 32.7 | 20.83 | `4030b6b9` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **6.86 s** | 49.4 ms | 740.5 | **0.72** | **523771.1** | 40.4 | 20.87 | `4030b6b9` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **6.56 s** | 36.4 ms | 740.5 | **0.72** | **515422.5** | 35.3 | 20.87 | `4030b6b9` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **6.57 s** | 33.6 ms | 740.5 | **0.72** | **553580.7** | 33.6 | 20.87 | `4030b6b9` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **0.30 s** | 309.1 ms | 12.1 | **0.74** | **121524.7** | 209.1 | 26.75 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **0.27 s** | 348.1 ms | 12.1 | **185.01** | **105859.4** | 243.9 | 26.71 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **0.29 s** | 327.1 ms | 12.2 | **6.01** | **127230.1** | 203.0 | 26.62 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **0.29 s** | 321.0 ms | 12.1 | **165.01** | **103041.8** | 256.3 | 26.56 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **0.32 s** | 359.3 ms | 12.1 | **45.01** | **118340.6** | 218.7 | 26.68 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **0.27 s** | 304.7 ms | 12.1 | **75.01** | **92491.5** | 107.6 | 73.95 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.17 s** | 49.4 ms | 7.2 | **3.38** | **398529.4** | 77.9 | 25.48 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **1.85 s** | 1486.8 ms | 8.1 | **78.79** | **20290.3** | 376.5 | 117.51 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **7.56 s** | 35.9 ms | 740.5 | **0.72** | **561793.4** | 35.0 | 20.86 | `4030b6b9` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **1.31 s** | 1638.9 ms | 8.9 | **6.32** | **145620.6** | 151.8 | 93.25 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **0.30 s** | 283.2 ms | 12.2 | **0.35** | **70207.2** | 240.3 | 26.50 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **0.30 s** | 346.1 ms | 12.1 | **0.30** | **130917.9** | 205.1 | 26.42 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **0.29 s** | 285.4 ms | 12.2 | **0.19** | **94097.2** | 228.0 | 26.70 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **0.31 s** | 275.7 ms | 12.2 | **0.17** | **117826.1** | 205.6 | 26.73 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **0.31 s** | 323.8 ms | 12.2 | **0.48** | **96785.7** | 231.5 | 26.62 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | **0.30 s** | 276.5 ms | 12.2 | **24.01** | **100370.4** | 219.7 | 26.53 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | **0.27 s** | 298.5 ms | 12.2 | **18.01** | **84161.5** | 204.6 | 26.82 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | **0.32 s** | 300.8 ms | 12.2 | **14.01** | **124311.9** | 206.1 | 26.46 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | **0.28 s** | 271.1 ms | 12.2 | **16.01** | **130917.9** | 204.6 | 26.77 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | **0.26 s** | 274.8 ms | 12.2 | **19.01** | **46967.1** | 363.5 | 26.79 | `38671a4f` |

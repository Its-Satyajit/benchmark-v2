# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `data/92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 2710 | **30933.1** | 173.6 | 133.39 | 0.047 | 0.73 | `66b83b63` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 2710 | **31281.1** | 146.4 | 133.41 | 0.049 | 0.73 | `66b83b63` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 2710 | **33099.8** | 144.3 | 133.38 | 0.042 | 0.73 | `66b83b63` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 2710 | **31855.2** | 147.8 | 133.39 | 0.044 | 0.73 | `66b83b63` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 2710 | **6238.2** | 707.5 | 26.77 | 0.276 | 0.74 | `897e6895` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 2710 | **6721.7** | 618.5 | 26.57 | 0.244 | 185.01 | `897e6895` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 2710 | **6369.4** | 659.0 | 26.52 | 0.268 | 6.01 | `897e6895` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 2710 | **6285.1** | 665.6 | 26.71 | 0.285 | 165.01 | `897e6895` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 2710 | **6400.1** | 650.0 | 26.79 | 0.259 | 45.01 | `897e6895` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 2710 | **5426.2** | 647.4 | 166.88 | 0.311 | 75.01 | `897e6895` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 2710 | **404296.6** | 85.0 | 25.46 | 0.003 | 3.38 | `897e6895` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 2710 | **14865.9** | 750.4 | 190.69 | 0.077 | 78.79 | `116af482` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 2710 | **32384.9** | 145.0 | 133.40 | 0.045 | 0.73 | `66b83b63` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 2710 | **124306.2** | 200.8 | 94.51 | 0.010 | 6.32 | `897e6895` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 2710 | **6813.1** | 611.9 | 26.69 | 0.226 | 0.35 | `897e6895` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 2710 | **6465.8** | 642.2 | 26.82 | 0.273 | 0.30 | `897e6895` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 2710 | **6450.2** | 696.4 | 26.80 | 0.272 | 0.19 | `897e6895` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 2710 | **6981.5** | 624.0 | 26.71 | 0.217 | 0.17 | `897e6895` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 2710 | **6330.6** | 634.7 | 26.76 | 0.308 | 0.48 | `897e6895` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 2710 | **5352.1** | 753.9 | 26.69 | 0.338 | 24.01 | `897e6895` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 2710 | **6141.9** | 694.8 | 26.77 | 0.297 | 18.01 | `897e6895` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 2710 | **6315.7** | 748.7 | 26.59 | 0.292 | 14.01 | `897e6895` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 2710 | **5635.7** | 746.2 | 26.63 | 0.307 | 16.01 | `897e6895` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 2710 | **4362.0** | 919.1 | 26.70 | 0.417 | 19.01 | `897e6895` |

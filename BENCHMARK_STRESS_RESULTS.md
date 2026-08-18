# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 5420 | **103815.3** | 88.6 | 132.33 | 0.119 | 0.89 | `5220a0a1` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 5420 | **107734.4** | 87.9 | 132.30 | 0.135 | 0.89 | `5220a0a1` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 5420 | **98436.7** | 92.2 | 132.34 | 0.137 | 0.89 | `5220a0a1` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 5420 | **109626.0** | 85.7 | 132.33 | 0.118 | 0.89 | `5220a0a1` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 5420 | **6678.2** | 1041.3 | 24.79 | 0.262 | 0.90 | `dc17b5e3` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 5420 | **6658.0** | 1053.5 | 24.70 | 0.256 | 185.01 | `dc17b5e3` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 5420 | **6748.0** | 1053.3 | 24.59 | 0.260 | 6.01 | `dc17b5e3` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 5420 | **6743.1** | 1016.6 | 24.82 | 0.261 | 165.01 | `dc17b5e3` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 5420 | **6469.5** | 1060.0 | 24.51 | 0.272 | 45.01 | `dc17b5e3` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 5420 | **5953.7** | 1023.0 | 195.36 | 0.289 | 75.01 | `dc17b5e3` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 5420 | **447637.9** | 92.7 | 25.49 | 0.003 | 3.38 | `dc17b5e3` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 5420 | **26438.3** | 486.0 | 262.79 | 0.035 | 78.79 | `bf65fb74` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 5420 | **105008.5** | 90.9 | 132.29 | 0.137 | 0.89 | `5220a0a1` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 5420 | **143038.1** | 189.3 | 95.89 | 0.006 | 6.32 | `dc17b5e3` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 5420 | **6634.6** | 1068.3 | 24.79 | 0.266 | 0.35 | `dc17b5e3` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 5420 | **6636.6** | 1068.9 | 24.78 | 0.252 | 0.29 | `dc17b5e3` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 5420 | **6737.7** | 1023.4 | 24.71 | 0.254 | 0.19 | `dc17b5e3` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 5420 | **6762.7** | 1015.6 | 24.52 | 0.237 | 0.17 | `dc17b5e3` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 5420 | **5831.5** | 1154.9 | 24.66 | 0.293 | 0.48 | `dc17b5e3` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 5420 | **6157.1** | 1139.2 | 24.52 | 0.279 | 24.01 | `dc17b5e3` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 5420 | **6389.8** | 1076.0 | 24.60 | 0.281 | 18.01 | `dc17b5e3` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 5420 | **6290.2** | 1104.0 | 24.70 | 0.277 | 14.01 | `dc17b5e3` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 5420 | **6340.3** | 1085.6 | 24.54 | 0.273 | 16.01 | `dc17b5e3` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 5420 | **6471.6** | 1070.7 | 24.77 | 0.265 | 19.01 | `dc17b5e3` |

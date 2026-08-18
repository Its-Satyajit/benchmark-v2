# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 5420 | **93086.0** | 101.9 | 132.32 | 0.137 | 0.89 | `5220a0a1` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 5420 | **98987.0** | 103.7 | 132.34 | 0.126 | 0.89 | `5220a0a1` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 5420 | **95506.2** | 100.6 | 132.31 | 0.118 | 0.89 | `5220a0a1` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 5420 | **58407.3** | 143.5 | 132.30 | 0.195 | 0.89 | `5220a0a1` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 5420 | **5757.9** | 1249.5 | 24.67 | 0.311 | 0.90 | `dc17b5e3` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 5420 | **5411.9** | 1258.9 | 24.78 | 0.337 | 185.01 | `dc17b5e3` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 5420 | **5118.7** | 1415.4 | 24.78 | 0.339 | 6.01 | `dc17b5e3` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 5420 | **5023.9** | 1410.6 | 24.90 | 0.378 | 165.01 | `dc17b5e3` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 5420 | **4681.1** | 1660.4 | 24.83 | 0.408 | 45.01 | `dc17b5e3` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 5420 | **5603.2** | 1197.6 | 195.34 | 0.337 | 75.01 | `dc17b5e3` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 5420 | **441044.8** | 95.4 | 25.50 | 0.003 | 3.38 | `dc17b5e3` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 5420 | **14314.0** | 1247.9 | 262.91 | 0.094 | 78.79 | `bf65fb74` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 5420 | **75060.9** | 112.2 | 132.34 | 0.205 | 0.89 | `5220a0a1` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 5420 | **170821.6** | 180.9 | 95.57 | 0.003 | 6.32 | `dc17b5e3` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 5420 | **6475.9** | 1081.2 | 24.71 | 0.261 | 0.35 | `dc17b5e3` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 5420 | **5969.1** | 1142.1 | 24.71 | 0.301 | 0.29 | `dc17b5e3` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 5420 | **6553.3** | 1069.1 | 24.68 | 0.249 | 0.19 | `dc17b5e3` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 5420 | **6356.2** | 1146.5 | 24.84 | 0.275 | 0.17 | `dc17b5e3` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 5420 | **5764.4** | 1180.2 | 24.67 | 0.302 | 0.48 | `dc17b5e3` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 5420 | **6559.7** | 1125.6 | 24.65 | 0.268 | 24.01 | `dc17b5e3` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 5420 | **6232.6** | 1115.4 | 24.75 | 0.285 | 18.01 | `dc17b5e3` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 5420 | **6650.2** | 1056.3 | 24.83 | 0.258 | 14.01 | `dc17b5e3` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 5420 | **6550.0** | 1068.6 | 24.61 | 0.257 | 16.01 | `dc17b5e3` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 5420 | **6697.5** | 1045.5 | 24.68 | 0.252 | 19.01 | `dc17b5e3` |

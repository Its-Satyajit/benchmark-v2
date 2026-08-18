# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `data/92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 1355 | **31485.8** | 88.4 | 78.02 | 0.047 | 0.73 | `8602f115` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 1355 | **31390.7** | 90.7 | 78.03 | 0.046 | 0.73 | `8602f115` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 1355 | **32076.9** | 88.7 | 78.02 | 0.046 | 0.73 | `8602f115` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 1355 | **30417.6** | 91.0 | 78.03 | 0.051 | 0.73 | `8602f115` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 1355 | **7124.8** | 405.1 | 22.22 | 0.199 | 0.74 | `738d5aa1` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 1355 | **7030.2** | 392.0 | 22.59 | 0.201 | 185.01 | `738d5aa1` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 1355 | **5800.5** | 433.9 | 22.63 | 0.302 | 6.01 | `738d5aa1` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 1355 | **7490.7** | 386.2 | 22.21 | 0.171 | 165.01 | `738d5aa1` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 1355 | **6853.1** | 412.7 | 22.50 | 0.221 | 45.01 | `738d5aa1` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 1355 | **6216.2** | 356.3 | 112.39 | 0.276 | 75.01 | `738d5aa1` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 1355 | **434155.7** | 125.5 | 23.50 | 0.002 | 3.38 | `738d5aa1` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 1355 | **7869.1** | 592.0 | 154.66 | 0.036 | 78.79 | `2ca13086` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 1355 | **29373.2** | 91.9 | 78.00 | 0.049 | 0.73 | `8602f115` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 1355 | **172304.2** | 149.8 | 93.45 | 0.004 | 6.32 | `738d5aa1` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 1355 | **7141.0** | 397.3 | 22.50 | 0.252 | 0.35 | `738d5aa1` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 1355 | **3976.8** | 679.2 | 22.36 | 0.402 | 0.30 | `738d5aa1` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 1355 | **3242.0** | 796.4 | 22.44 | 0.535 | 0.19 | `738d5aa1` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 1355 | **2930.9** | 832.6 | 22.47 | 0.634 | 0.17 | `738d5aa1` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 1355 | **3392.7** | 868.1 | 22.39 | 0.579 | 0.48 | `738d5aa1` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 1355 | **3889.8** | 728.5 | 22.34 | 0.415 | 24.01 | `738d5aa1` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 1355 | **6616.5** | 424.5 | 22.39 | 0.250 | 18.01 | `738d5aa1` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 1355 | **7450.8** | 394.5 | 22.42 | 0.171 | 14.01 | `738d5aa1` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 1355 | **5454.2** | 484.5 | 22.45 | 0.339 | 16.01 | `738d5aa1` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 1355 | **6416.0** | 467.2 | 22.57 | 0.236 | 19.01 | `738d5aa1` |

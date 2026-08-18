# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 2710 | **36671.0** | 134.3 | 113.29 | 0.038 | 0.72 | `66b83b63` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 2710 | **36123.0** | 133.6 | 113.30 | 0.039 | 0.72 | `66b83b63` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 2710 | **27901.8** | 192.2 | 113.24 | 0.074 | 0.72 | `66b83b63` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 2710 | **37274.2** | 129.7 | 113.29 | 0.038 | 0.72 | `66b83b63` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 2710 | **6668.6** | 614.1 | 26.61 | 0.232 | 0.74 | `897e6895` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 2710 | **6976.8** | 606.5 | 26.45 | 0.215 | 185.01 | `897e6895` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 2710 | **6393.0** | 638.0 | 26.70 | 0.265 | 6.01 | `897e6895` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 2710 | **7331.7** | 602.6 | 26.64 | 0.173 | 165.01 | `897e6895` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 2710 | **7025.3** | 644.7 | 26.68 | 0.196 | 45.01 | `897e6895` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 2710 | **5744.8** | 597.9 | 166.48 | 0.288 | 75.01 | `897e6895` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 2710 | **492996.2** | 82.9 | 23.45 | 0.002 | 3.38 | `897e6895` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 2710 | **19614.7** | 513.2 | 190.66 | 0.050 | 78.79 | `116af482` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 2710 | **32995.7** | 150.3 | 113.31 | 0.054 | 0.72 | `66b83b63` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 2710 | **158711.6** | 181.3 | 93.79 | 0.004 | 6.32 | `897e6895` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 2710 | **6722.7** | 621.4 | 26.83 | 0.226 | 0.35 | `897e6895` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 2710 | **6641.3** | 625.0 | 26.54 | 0.242 | 0.30 | `897e6895` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 2710 | **6896.4** | 628.4 | 26.63 | 0.221 | 0.19 | `897e6895` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 2710 | **7001.0** | 627.7 | 26.72 | 0.200 | 0.17 | `897e6895` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 2710 | **6898.8** | 630.7 | 26.47 | 0.225 | 0.48 | `897e6895` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 2710 | **5886.9** | 662.3 | 26.83 | 0.318 | 24.01 | `897e6895` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 2710 | **6697.1** | 633.2 | 26.78 | 0.238 | 18.01 | `897e6895` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 2710 | **6532.6** | 625.7 | 26.56 | 0.264 | 14.01 | `897e6895` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 2710 | **6888.5** | 610.7 | 26.50 | 0.246 | 16.01 | `897e6895` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 2710 | **7104.7** | 593.1 | 26.80 | 0.190 | 19.01 | `897e6895` |

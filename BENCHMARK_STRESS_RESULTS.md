# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 2710 | **17117.6** | 240.5 | 133.41 | 0.115 | 0.73 | `66b83b63` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 2710 | **25897.8** | 198.2 | 133.41 | 0.083 | 0.73 | `66b83b63` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 2710 | **28294.9** | 173.9 | 133.41 | 0.061 | 0.73 | `66b83b63` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 2710 | **26919.4** | 165.6 | 133.40 | 0.070 | 0.73 | `66b83b63` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 2710 | **6019.8** | 729.2 | 26.66 | 0.297 | 0.74 | `897e6895` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 2710 | **6213.9** | 677.1 | 26.71 | 0.270 | 185.01 | `897e6895` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 2710 | **5083.9** | 812.1 | 26.71 | 0.372 | 6.01 | `897e6895` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 2710 | **5697.0** | 735.5 | 26.80 | 0.316 | 165.01 | `897e6895` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 2710 | **5331.8** | 915.1 | 26.73 | 0.336 | 45.01 | `897e6895` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 2710 | **5307.3** | 689.5 | 166.77 | 0.321 | 75.01 | `897e6895` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 2710 | **434225.3** | 100.4 | 25.49 | 0.003 | 3.38 | `897e6895` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 2710 | **19894.9** | 526.8 | 190.80 | 0.046 | 78.79 | `116af482` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 2710 | **21928.2** | 203.3 | 133.39 | 0.087 | 0.73 | `66b83b63` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 2710 | **143060.8** | 204.5 | 93.72 | 0.006 | 6.32 | `897e6895` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 2710 | **4633.4** | 978.0 | 26.80 | 0.400 | 0.35 | `897e6895` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 2710 | **5160.2** | 772.2 | 26.80 | 0.359 | 0.30 | `897e6895` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 2710 | **5676.3** | 708.0 | 26.49 | 0.319 | 0.19 | `897e6895` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 2710 | **5753.2** | 703.5 | 26.70 | 0.321 | 0.17 | `897e6895` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 2710 | **5125.1** | 779.1 | 26.67 | 0.353 | 0.48 | `897e6895` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 2710 | **4412.5** | 1002.3 | 26.80 | 0.436 | 24.01 | `897e6895` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 2710 | **5811.7** | 743.9 | 26.45 | 0.309 | 18.01 | `897e6895` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 2710 | **5891.4** | 742.1 | 26.63 | 0.311 | 14.01 | `897e6895` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 2710 | **6156.1** | 719.7 | 26.54 | 0.275 | 16.01 | `897e6895` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 2710 | **6286.4** | 651.3 | 26.68 | 0.263 | 19.01 | `897e6895` |

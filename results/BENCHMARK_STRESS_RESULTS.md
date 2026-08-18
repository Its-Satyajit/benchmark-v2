# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `data/92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 1355 | **31544.3** | 90.0 | 78.01 | 0.046 | 0.73 | `8602f115` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 1355 | **31680.4** | 88.0 | 78.02 | 0.045 | 0.73 | `8602f115` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 1355 | **32533.6** | 87.1 | 78.00 | 0.042 | 0.73 | `8602f115` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 1355 | **32656.4** | 86.8 | 78.03 | 0.042 | 0.73 | `8602f115` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 1355 | **7406.4** | 399.9 | 22.42 | 0.175 | 0.74 | `738d5aa1` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 1355 | **7315.6** | 421.6 | 22.41 | 0.173 | 185.01 | `738d5aa1` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 1355 | **7286.1** | 387.2 | 22.21 | 0.176 | 6.01 | `738d5aa1` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 1355 | **6599.1** | 434.1 | 22.40 | 0.247 | 165.01 | `738d5aa1` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 1355 | **6731.9** | 411.9 | 22.62 | 0.245 | 45.01 | `738d5aa1` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 1355 | **6227.0** | 331.1 | 111.95 | 0.253 | 75.01 | `738d5aa1` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 1355 | **465156.2** | 83.7 | 25.54 | 0.002 | 3.38 | `738d5aa1` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 1355 | **15953.7** | 476.2 | 154.83 | 0.050 | 78.79 | `2ca13086` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 1355 | **30388.0** | 92.9 | 78.01 | 0.054 | 0.73 | `8602f115` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 1355 | **179185.4** | 148.9 | 92.91 | 0.004 | 6.32 | `738d5aa1` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 1355 | **6901.6** | 412.0 | 22.42 | 0.225 | 0.35 | `738d5aa1` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 1355 | **7136.8** | 396.5 | 22.39 | 0.197 | 0.30 | `738d5aa1` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 1355 | **7234.4** | 401.5 | 22.61 | 0.181 | 0.19 | `738d5aa1` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 1355 | **7065.8** | 416.1 | 22.61 | 0.203 | 0.17 | `738d5aa1` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 1355 | **7145.5** | 392.0 | 22.52 | 0.189 | 0.48 | `738d5aa1` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 1355 | **7473.4** | 390.4 | 22.39 | 0.170 | 24.01 | `738d5aa1` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 1355 | **6706.9** | 413.3 | 22.60 | 0.232 | 18.01 | `738d5aa1` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 1355 | **6516.3** | 439.2 | 22.38 | 0.247 | 14.01 | `738d5aa1` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 1355 | **6597.5** | 424.5 | 22.59 | 0.279 | 16.01 | `738d5aa1` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 1355 | **6734.9** | 426.8 | 22.38 | 0.264 | 19.01 | `738d5aa1` |

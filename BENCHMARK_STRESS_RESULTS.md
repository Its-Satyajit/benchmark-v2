# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Snapshots | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 5420 | **56475.3** | 177.4 | 132.32 | 0.234 | 10760 | `5220a0a1` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 5420 | **44828.3** | 203.6 | 132.34 | 0.203 | 10760 | `5220a0a1` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 5420 | **53023.2** | 182.0 | 132.27 | 0.220 | 10760 | `5220a0a1` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 5420 | **46948.5** | 188.5 | 132.29 | 0.300 | 10760 | `5220a0a1` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 5420 | **3347.6** | 2126.0 | 24.68 | 0.517 | 10760 | `dc17b5e3` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 5420 | **4605.0** | 1590.2 | 24.81 | 0.441 | 10760 | `dc17b5e3` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 5420 | **6818.5** | 1022.4 | 24.71 | 0.233 | 10760 | `dc17b5e3` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 5420 | **6745.0** | 1031.1 | 24.81 | 0.244 | 10760 | `dc17b5e3` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 5420 | **6332.0** | 1080.4 | 24.57 | 0.269 | 10760 | `dc17b5e3` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 5420 | **6389.2** | 969.9 | 197.31 | 0.274 | 10760 | `dc17b5e3` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 5420 | **476316.0** | 135.9 | 25.59 | 0.000 | 0 | `dc17b5e3` |
| **Avalonia (.NET 8/9 C#)** | Native UI Engine | ✅ PASS | 5420 | **41269.9** | 1210.6 | 121.09 | 0.000 | 0 | `bf65fb74` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 5420 | **76580.7** | 109.5 | 132.31 | 0.132 | 10760 | `5220a0a1` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 5420 | **58018.2** | 644.7 | 248.93 | 0.000 | 0 | `dc17b5e3` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 5420 | **5824.4** | 1165.4 | 24.94 | 0.309 | 10760 | `dc17b5e3` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 5420 | **4564.5** | 1455.2 | 24.80 | 0.366 | 10760 | `dc17b5e3` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 5420 | **6149.2** | 1172.6 | 24.82 | 0.287 | 10760 | `dc17b5e3` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 5420 | **6910.9** | 1014.5 | 24.44 | 0.226 | 10760 | `dc17b5e3` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 5420 | **6113.1** | 1114.0 | 24.68 | 0.282 | 10760 | `dc17b5e3` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 5420 | **6458.8** | 1067.9 | 24.48 | 0.261 | 10760 | `dc17b5e3` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 5420 | **6037.2** | 1131.9 | 24.55 | 0.286 | 10760 | `dc17b5e3` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 5420 | **5872.1** | 1195.4 | 24.80 | 0.295 | 10760 | `dc17b5e3` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 5420 | **6144.7** | 1169.6 | 24.43 | 0.283 | 10760 | `dc17b5e3` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 5420 | **5821.8** | 1175.7 | 24.51 | 0.300 | 10760 | `dc17b5e3` |

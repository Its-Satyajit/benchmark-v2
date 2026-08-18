# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Snapshots | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 5420 | **73901.2** | 114.2 | 132.31 | 0.133 | 10760 | `5220a0a1` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 5420 | **91274.2** | 112.0 | 132.33 | 0.116 | 10760 | `5220a0a1` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 5420 | **105516.3** | 88.2 | 132.30 | 0.123 | 10760 | `5220a0a1` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 5420 | **112661.9** | 84.7 | 132.29 | 0.114 | 10760 | `5220a0a1` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 5420 | **7328.8** | 956.9 | 24.67 | 0.174 | 10760 | `dc17b5e3` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 5420 | **6828.3** | 1016.0 | 24.61 | 0.224 | 10760 | `dc17b5e3` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 5420 | **7046.0** | 1030.3 | 24.72 | 0.200 | 10760 | `dc17b5e3` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 5420 | **7120.1** | 971.9 | 24.70 | 0.187 | 10760 | `dc17b5e3` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 5420 | **6718.4** | 1031.3 | 24.65 | 0.249 | 10760 | `dc17b5e3` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 5420 | **6846.6** | 903.5 | 195.38 | 0.201 | 10760 | `dc17b5e3` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 5420 | **502317.0** | 235.9 | 27.41 | 0.002 | 10760 | `dc17b5e3` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 5420 | **9164.3** | 1675.7 | 120.86 | 0.153 | 10760 | `bf65fb74` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 5420 | **65570.3** | 145.0 | 132.32 | 0.204 | 10760 | `5220a0a1` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 5420 | **28557.3** | 958.8 | 254.16 | 0.021 | 10760 | `dc17b5e3` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 5420 | **4037.0** | 1745.9 | 24.69 | 0.358 | 10760 | `dc17b5e3` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 5420 | **7090.9** | 1042.8 | 24.73 | 0.202 | 10760 | `dc17b5e3` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 5420 | **7015.6** | 1016.5 | 24.66 | 0.213 | 10760 | `dc17b5e3` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 5420 | **6572.6** | 1038.7 | 24.80 | 0.273 | 10760 | `dc17b5e3` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 5420 | **7059.0** | 986.8 | 24.83 | 0.199 | 10760 | `dc17b5e3` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 5420 | **7083.6** | 980.4 | 24.80 | 0.194 | 10760 | `dc17b5e3` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 5420 | **7272.9** | 959.5 | 24.62 | 0.176 | 10760 | `dc17b5e3` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 5420 | **7422.9** | 944.0 | 24.55 | 0.170 | 10760 | `dc17b5e3` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 5420 | **6896.0** | 1035.4 | 24.69 | 0.233 | 10760 | `dc17b5e3` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 5420 | **7223.6** | 964.0 | 24.59 | 0.174 | 10760 | `dc17b5e3` |

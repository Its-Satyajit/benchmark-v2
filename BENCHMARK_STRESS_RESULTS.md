# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Snapshots | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 5420 | **80940.1** | 112.5 | 132.33 | 0.136 | 10760 | `5220a0a1` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 5420 | **100672.1** | 94.6 | 132.33 | 0.116 | 10760 | `5220a0a1` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 5420 | **89231.1** | 100.8 | 132.28 | 0.130 | 10760 | `5220a0a1` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 5420 | **85009.6** | 110.9 | 132.33 | 0.119 | 10760 | `5220a0a1` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 5420 | **6807.7** | 1020.0 | 24.79 | 0.231 | 10760 | `dc17b5e3` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 5420 | **7042.5** | 994.0 | 24.66 | 0.204 | 10760 | `dc17b5e3` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 5420 | **7218.1** | 976.7 | 24.78 | 0.183 | 10760 | `dc17b5e3` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 5420 | **7203.9** | 974.8 | 24.70 | 0.183 | 10760 | `dc17b5e3` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 5420 | **7205.4** | 972.4 | 24.75 | 0.188 | 10760 | `dc17b5e3` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 5420 | **6661.9** | 919.8 | 195.04 | 0.241 | 10760 | `dc17b5e3` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 5420 | **369260.1** | 92.5 | 25.43 | 0.004 | 10760 | `dc17b5e3` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 5420 | **23254.7** | 540.2 | 260.17 | 0.043 | 10760 | `bf65fb74` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 5420 | **114297.6** | 82.3 | 132.30 | 0.122 | 10760 | `5220a0a1` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 5420 | **163825.4** | 187.6 | 95.89 | 0.004 | 10760 | `dc17b5e3` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 5420 | **7110.4** | 980.0 | 24.54 | 0.187 | 10760 | `dc17b5e3` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 5420 | **7276.3** | 966.8 | 24.77 | 0.176 | 10760 | `dc17b5e3` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 5420 | **6798.8** | 1014.0 | 24.53 | 0.251 | 10760 | `dc17b5e3` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 5420 | **7234.2** | 967.6 | 24.52 | 0.182 | 10760 | `dc17b5e3` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 5420 | **7092.9** | 979.7 | 24.69 | 0.194 | 10760 | `dc17b5e3` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 5420 | **7024.2** | 1007.9 | 24.59 | 0.197 | 10760 | `dc17b5e3` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 5420 | **6776.4** | 1017.2 | 24.67 | 0.227 | 10760 | `dc17b5e3` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 5420 | **6749.4** | 1031.5 | 24.84 | 0.239 | 10760 | `dc17b5e3` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 5420 | **7242.6** | 981.9 | 24.64 | 0.187 | 10760 | `dc17b5e3` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 5420 | **6848.5** | 1014.9 | 24.59 | 0.229 | 10760 | `dc17b5e3` |

# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 5420 | **79710.4** | 108.3 | 132.30 | 0.097 | 0.89 | `5220a0a1` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 5420 | **101902.1** | 90.2 | 132.32 | 0.142 | 0.89 | `5220a0a1` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 5420 | **114462.6** | 82.9 | 132.36 | 0.126 | 0.89 | `5220a0a1` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 5420 | **107482.0** | 86.3 | 132.30 | 0.143 | 0.89 | `5220a0a1` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 5420 | **7238.6** | 962.1 | 24.79 | 0.184 | 0.90 | `dc17b5e3` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 5420 | **7176.0** | 975.8 | 24.75 | 0.191 | 185.01 | `dc17b5e3` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 5420 | **7229.2** | 965.4 | 24.73 | 0.175 | 6.01 | `dc17b5e3` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 5420 | **7281.5** | 960.0 | 24.61 | 0.181 | 165.01 | `dc17b5e3` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 5420 | **7042.1** | 978.9 | 24.58 | 0.209 | 45.01 | `dc17b5e3` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 5420 | **6743.6** | 921.9 | 195.55 | 0.232 | 75.01 | `dc17b5e3` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 5420 | **500138.4** | 96.5 | 27.48 | 0.002 | 3.38 | `dc17b5e3` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 5420 | **25480.4** | 492.6 | 262.74 | 0.042 | 78.79 | `bf65fb74` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 5420 | **109820.7** | 87.8 | 132.33 | 0.128 | 0.89 | `5220a0a1` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 5420 | **175945.5** | 175.2 | 95.22 | 0.003 | 6.32 | `dc17b5e3` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 5420 | **7263.8** | 961.1 | 24.71 | 0.183 | 0.01 | `dc17b5e3` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 5420 | **6948.5** | 999.8 | 24.82 | 0.226 | 0.01 | `dc17b5e3` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 5420 | **7276.1** | 968.0 | 24.80 | 0.185 | 0.01 | `dc17b5e3` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 5420 | **6864.4** | 1011.4 | 24.51 | 0.222 | 0.01 | `dc17b5e3` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 5420 | **7182.8** | 978.8 | 24.70 | 0.197 | 0.01 | `dc17b5e3` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 5420 | **6669.5** | 1028.9 | 24.67 | 0.257 | 0.01 | `dc17b5e3` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 5420 | **7146.7** | 973.0 | 24.41 | 0.198 | 0.01 | `dc17b5e3` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 5420 | **6990.8** | 999.6 | 24.97 | 0.216 | 0.01 | `dc17b5e3` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 5420 | **7087.6** | 978.0 | 24.66 | 0.188 | 0.01 | `dc17b5e3` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 5420 | **7081.8** | 1008.3 | 24.71 | 0.190 | 0.01 | `dc17b5e3` |

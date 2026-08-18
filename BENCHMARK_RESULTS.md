# Stress Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Build (ms) | Artifact (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 34.6 | 0.89 | **529800.8** | 29.8 | 20.85 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 32.5 | 0.89 | **540496.3** | 32.0 | 20.85 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 32.6 | 0.89 | **523868.9** | 29.5 | 20.83 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 33.4 | 0.89 | **540091.3** | 29.6 | 20.88 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 399.0 | 0.01 | **133497.5** | 193.3 | 24.73 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 274.5 | 0.01 | **136180.9** | 202.1 | 24.76 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 270.9 | 0.01 | **137563.5** | 208.6 | 24.65 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 276.3 | 0.01 | **139690.7** | 200.6 | 24.51 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 290.6 | 0.01 | **124884.8** | 210.2 | 24.76 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | 301.1 | 0.01 | **99267.4** | 105.3 | 74.07 | `38671a4f` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 190.4 | 3.38 | **392185.2** | 76.1 | 27.43 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | 1566.6 | 0.09 | **19824.7** | 331.0 | 105.88 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 32.4 | 0.89 | **513248.8** | 29.6 | 20.89 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 1414.0 | 6.32 | **156016.1** | 145.2 | 92.80 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 269.6 | 0.01 | **103831.4** | 212.1 | 24.61 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 284.1 | 0.01 | **139690.7** | 204.1 | 24.70 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 270.0 | 0.01 | **116810.3** | 206.6 | 24.71 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 279.9 | 0.01 | **118340.6** | 208.1 | 24.79 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 284.0 | 0.01 | **125463.0** | 208.3 | 24.52 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 288.1 | 0.01 | **118859.6** | 210.0 | 24.78 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 289.1 | 0.01 | **108400.0** | 216.7 | 24.58 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 270.4 | 0.01 | **136868.7** | 206.6 | 24.43 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 299.6 | 0.01 | **141145.8** | 223.1 | 24.82 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 278.7 | 0.01 | **111065.6** | 221.4 | 24.71 | `38671a4f` |

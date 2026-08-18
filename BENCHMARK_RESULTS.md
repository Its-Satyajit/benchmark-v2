# Stress Benchmark Results Matrix

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Build (ms) | Artifact (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | 37.8 | 0.87 | **516954.6** | 31.9 | 20.84 | `38671a4f` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | 0.0 | 0.87 | **513368.4** | 29.8 | 20.83 | `38671a4f` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | 0.0 | 0.87 | **539488.1** | 31.8 | 20.83 | `38671a4f` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | 0.0 | 0.87 | **531352.8** | 31.8 | 20.86 | `38671a4f` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | 0.0 | 0.00 | **66259.2** | 284.7 | 24.70 | `38671a4f` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | 0.0 | 0.00 | **85759.5** | 320.7 | 24.70 | `38671a4f` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | 0.0 | 0.00 | **124311.9** | 235.6 | 24.68 | `38671a4f` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | 0.0 | 0.00 | **124311.9** | 210.2 | 24.78 | `38671a4f` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | 0.0 | 0.00 | **134158.4** | 243.8 | 24.63 | `38671a4f` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ❌ FAIL | 0.0 | 0.00 | **N/A** | 182.6 | 33.45 | `N/A` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | 0.0 | 0.00 | **372252.7** | 128.9 | 25.87 | `38671a4f` |
| **Avalonia (.NET 8/9 C#)** | Native UI Engine | ❌ FAIL | 0.0 | 0.00 | **N/A** | 0.0 | 0.00 | `N/A` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | 0.0 | 0.87 | **491808.9** | 32.3 | 20.86 | `38671a4f` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | 0.0 | 0.01 | **14891.8** | 481.6 | 246.95 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **103041.8** | 214.3 | 24.77 | `38671a4f` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **116810.3** | 203.4 | 24.75 | `38671a4f` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **92176.9** | 246.2 | 24.68 | `38671a4f` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **111983.5** | 214.9 | 24.82 | `38671a4f` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | 0.0 | 0.00 | **141884.8** | 212.6 | 24.74 | `38671a4f` |
| **Next.js (App Router SSR)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **136868.7** | 222.3 | 24.68 | `38671a4f` |
| **Nuxt (Nitro Engine)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **122072.1** | 216.3 | 24.49 | `38671a4f` |
| **SvelteKit (Adapter Engine)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **133497.5** | 216.6 | 24.72 | `38671a4f` |
| **Astro (Islands Architecture)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **131553.4** | 217.0 | 24.73 | `38671a4f` |
| **TanStack Start (Full-Stack SSR)** | Web Metaframework | ✅ PASS | 0.0 | 0.00 | **126046.5** | 254.5 | 24.68 | `38671a4f` |

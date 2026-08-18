# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `data/92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **457056.4** | **186706.5** | 0.02 ms | 0 | 0.73 | 22.66 | `7dd97557` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **340825.2** | **105451.9** | 0.03 ms | 0 | 0.73 | 22.68 | `7dd97557` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **180433.3** | **49273.2** | 0.08 ms | 0 | 0.73 | 22.67 | `7dd97557` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **517308.1** | **279251.6** | 0.01 ms | 0 | 0.73 | 22.65 | `7dd97557` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **175852.6** | **76934.9** | 0.05 ms | 0 | 0.74 | 22.33 | `ebde4f47` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **208636.6** | **87750.1** | 0.04 ms | 0 | 185.01 | 22.47 | `ebde4f47` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **189537.3** | **76196.3** | 0.05 ms | 0 | 6.01 | 22.64 | `ebde4f47` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **170817.3** | **68563.6** | 0.06 ms | 0 | 165.01 | 22.23 | `ebde4f47` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **172718.3** | **68738.0** | 0.04 ms | 0 | 45.01 | 22.49 | `ebde4f47` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **120365.0** | **43081.2** | 1.82 ms | 0 | 75.01 | 79.60 | `ebde4f47` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **482206.4** | **250000.0** | 0.03 ms | 0 | 3.38 | 25.53 | `ebde4f47` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **5678.0** | **29069.8** | 179.56 ms | 1 | 78.79 | 127.36 | `614c81c8` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **486235.4** | **261917.2** | 0.02 ms | 0 | 0.73 | 22.65 | `7dd97557` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **84381.6** | **43478.3** | 0.19 ms | 0 | 6.32 | 92.88 | `ebde4f47` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **114169.5** | **43733.1** | 0.09 ms | 0 | 0.35 | 22.46 | `ebde4f47` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **185901.1** | **72516.3** | 0.04 ms | 0 | 0.30 | 22.36 | `ebde4f47` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **160091.2** | **55160.2** | 0.05 ms | 0 | 0.19 | 22.49 | `ebde4f47` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **173958.5** | **55361.8** | 0.08 ms | 0 | 0.17 | 22.41 | `ebde4f47` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **146962.1** | **49830.6** | 0.59 ms | 0 | 0.48 | 22.62 | `ebde4f47` |

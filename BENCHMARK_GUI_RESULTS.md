# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **547129.0** | **223214.3** | 0.30 ms | 0 | 0.89 | 20.98 | `c4e6dfae` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **571634.0** | **234796.9** | 0.02 ms | 0 | 0.89 | 20.97 | `c4e6dfae` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **592183.9** | **271665.3** | 0.01 ms | 0 | 0.89 | 20.99 | `c4e6dfae` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **610923.4** | **251762.3** | 0.01 ms | 0 | 0.89 | 20.99 | `c4e6dfae` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **212379.1** | **52067.1** | 1.03 ms | 0 | 0.90 | 24.43 | `c4e6dfae` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **161049.8** | **43721.6** | 0.13 ms | 0 | 185.01 | 24.49 | `c4e6dfae` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **221885.7** | **67042.1** | 0.11 ms | 0 | 6.01 | 24.73 | `c4e6dfae` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **114099.1** | **36957.6** | 0.14 ms | 0 | 165.01 | 24.79 | `c4e6dfae` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **240250.1** | **74162.0** | 0.04 ms | 0 | 45.01 | 24.64 | `c4e6dfae` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **171758.8** | **48083.9** | 2.67 ms | 0 | 75.01 | 90.13 | `c4e6dfae` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **494390.2** | **250000.0** | 0.03 ms | 0 | 3.38 | 25.48 | `c4e6dfae` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **42264.5** | **17182.1** | 43.00 ms | 1 | 78.79 | 128.16 | `b30e0aa1` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **582035.5** | **232612.2** | 0.02 ms | 0 | 0.89 | 20.99 | `c4e6dfae` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **152379.9** | **83333.3** | 1.11 ms | 0 | 6.32 | 95.85 | `c4e6dfae` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **95571.4** | **25740.0** | 2.05 ms | 0 | 0.35 | 24.66 | `c4e6dfae` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **195605.5** | **53123.7** | 0.12 ms | 0 | 0.29 | 24.61 | `c4e6dfae` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **192496.4** | **43677.7** | 0.21 ms | 0 | 0.19 | 24.54 | `c4e6dfae` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **214576.2** | **57603.7** | 0.10 ms | 0 | 0.17 | 24.71 | `c4e6dfae` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **218604.0** | **66800.3** | 0.11 ms | 0 | 0.48 | 24.61 | `c4e6dfae` |

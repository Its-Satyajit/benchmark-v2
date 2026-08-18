# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **611892.7** | **395726.2** | 0.01 ms | 0 | 0.89 | 21.00 | `c4e6dfae` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **609269.5** | **387296.7** | 0.01 ms | 0 | 0.89 | 21.02 | `c4e6dfae` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **594518.6** | **249128.1** | 0.01 ms | 0 | 0.89 | 21.02 | `c4e6dfae` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **574696.1** | **228833.0** | 0.01 ms | 0 | 0.89 | 20.96 | `c4e6dfae` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **276645.7** | **103928.5** | 0.05 ms | 0 | 0.90 | 24.67 | `c4e6dfae` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **289299.3** | **122459.0** | 0.11 ms | 0 | 185.01 | 24.74 | `c4e6dfae` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **295408.5** | **124812.8** | 0.06 ms | 0 | 6.01 | 24.67 | `c4e6dfae` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **277459.3** | **99019.7** | 0.05 ms | 0 | 165.01 | 24.67 | `c4e6dfae` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **259850.8** | **100776.0** | 0.12 ms | 0 | 45.01 | 24.75 | `c4e6dfae` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **214319.4** | **85594.5** | 2.22 ms | 0 | 75.01 | 90.50 | `c4e6dfae` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **516830.4** | **500000.0** | 0.03 ms | 0 | 3.38 | 27.51 | `c4e6dfae` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **59124.8** | **54945.1** | 26.59 ms | 1 | 78.79 | 128.14 | `b30e0aa1` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **536997.6** | **212089.1** | 0.01 ms | 0 | 0.89 | 21.00 | `c4e6dfae` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **177646.7** | **142857.1** | 0.88 ms | 0 | 6.32 | 95.84 | `c4e6dfae` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **255167.1** | **92140.4** | 0.09 ms | 0 | 0.01 | 24.82 | `c4e6dfae` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **286078.9** | **126023.9** | 0.10 ms | 0 | 0.01 | 24.53 | `c4e6dfae` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **274730.5** | **105108.3** | 0.09 ms | 0 | 0.01 | 24.70 | `c4e6dfae` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **264180.3** | **105775.3** | 0.10 ms | 0 | 0.01 | 24.72 | `c4e6dfae` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **170608.4** | **48211.4** | 0.55 ms | 0 | 0.01 | 24.60 | `c4e6dfae` |

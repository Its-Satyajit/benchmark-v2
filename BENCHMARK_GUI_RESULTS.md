# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Jank Rate % | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **586627.5** | **234027.6** | 0.02 ms | 0 | 0.00% | 21.01 | `c4e6dfae` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **603227.5** | **388651.4** | 0.01 ms | 0 | 0.00% | 20.96 | `c4e6dfae` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **594509.1** | **366837.9** | 0.02 ms | 0 | 0.00% | 21.00 | `c4e6dfae` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **615793.4** | **419991.6** | 0.05 ms | 0 | 0.00% | 21.00 | `c4e6dfae` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **233554.7** | **77327.6** | 0.10 ms | 0 | 0.00% | 24.68 | `c4e6dfae` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **284197.3** | **121624.9** | 0.05 ms | 0 | 0.00% | 24.65 | `c4e6dfae` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **293014.6** | **107112.3** | 0.04 ms | 0 | 0.00% | 24.82 | `c4e6dfae` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **283830.8** | **92344.6** | 0.05 ms | 0 | 0.00% | 24.67 | `c4e6dfae` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **273863.7** | **105363.0** | 0.10 ms | 0 | 0.00% | 24.66 | `c4e6dfae` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **219212.2** | **79522.9** | 2.04 ms | 0 | 0.00% | 90.54 | `c4e6dfae` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **513938.9** | **333333.3** | 0.03 ms | 0 | 0.00% | 25.91 | `c4e6dfae` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **44035.4** | **14662.8** | 27.27 ms | 1 | 0.02% | 121.25 | `b30e0aa1` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **616478.0** | **291205.6** | 0.01 ms | 0 | 0.00% | 20.98 | `c4e6dfae` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **80817.1** | **45454.5** | 1.37 ms | 0 | 0.00% | 251.71 | `c4e6dfae` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **284788.8** | **109625.1** | 0.08 ms | 0 | 0.00% | 24.41 | `c4e6dfae` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **261655.4** | **104854.8** | 0.09 ms | 0 | 0.00% | 24.68 | `c4e6dfae` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **282216.3** | **107816.7** | 0.06 ms | 0 | 0.00% | 24.77 | `c4e6dfae` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **143463.7** | **53527.5** | 0.18 ms | 0 | 0.00% | 24.65 | `c4e6dfae` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **274698.4** | **92712.8** | 0.05 ms | 0 | 0.00% | 24.80 | `c4e6dfae` |

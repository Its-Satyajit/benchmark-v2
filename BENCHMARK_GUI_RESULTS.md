# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Jank Rate % | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **603896.8** | **396982.9** | 0.02 ms | 0 | 0.00% | 21.00 | `c4e6dfae` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **596600.9** | **250062.5** | 0.01 ms | 0 | 0.00% | 20.98 | `c4e6dfae` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **487926.6** | **194099.4** | 0.11 ms | 0 | 0.00% | 21.00 | `c4e6dfae` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **540720.0** | **209117.5** | 0.02 ms | 0 | 0.00% | 20.97 | `c4e6dfae` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **260140.0** | **98106.5** | 0.13 ms | 0 | 0.00% | 24.63 | `c4e6dfae` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **192853.3** | **66383.4** | 0.45 ms | 0 | 0.00% | 24.79 | `c4e6dfae` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **194663.5** | **56239.8** | 0.14 ms | 0 | 0.00% | 24.62 | `c4e6dfae` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **213571.1** | **82236.8** | 0.12 ms | 0 | 0.00% | 24.73 | `c4e6dfae` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **206848.0** | **65728.9** | 0.09 ms | 0 | 0.00% | 24.39 | `c4e6dfae` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **135100.0** | **35492.5** | 2.70 ms | 0 | 0.00% | 92.97 | `c4e6dfae` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **0.0** | **0.0** | 0.00 ms | 0 | 0.00% | 25.85 | `38671a4f` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **0.0** | **0.0** | 0.00 ms | 0 | 0.00% | 121.07 | `1dd70a1d` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **588665.5** | **244379.3** | 0.01 ms | 0 | 0.00% | 20.96 | `c4e6dfae` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **0.0** | **0.0** | 0.00 ms | 0 | 0.00% | 243.45 | `38671a4f` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **256917.1** | **94197.4** | 0.09 ms | 0 | 0.00% | 24.59 | `c4e6dfae` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **231201.7** | **79459.7** | 0.47 ms | 0 | 0.00% | 24.68 | `c4e6dfae` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **243579.4** | **90834.8** | 0.09 ms | 0 | 0.00% | 24.68 | `c4e6dfae` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **107353.9** | **32605.2** | 2.29 ms | 0 | 0.00% | 24.68 | `c4e6dfae` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **201566.4** | **61117.2** | 0.19 ms | 0 | 0.00% | 24.62 | `c4e6dfae` |

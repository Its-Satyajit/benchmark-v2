# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **494002.9** | **193760.9** | 0.03 ms | 0 | 0.89 | 21.01 | `c4e6dfae` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **545316.5** | **222717.1** | 0.01 ms | 0 | 0.89 | 20.97 | `c4e6dfae` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **602305.5** | **373692.1** | 0.01 ms | 0 | 0.89 | 20.95 | `c4e6dfae` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **589924.1** | **314960.6** | 0.07 ms | 0 | 0.89 | 21.02 | `c4e6dfae` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **240265.6** | **86281.3** | 0.05 ms | 0 | 0.90 | 24.79 | `c4e6dfae` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **256017.3** | **94553.7** | 0.06 ms | 0 | 185.01 | 24.63 | `c4e6dfae` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **255162.0** | **101801.9** | 0.05 ms | 0 | 6.01 | 24.69 | `c4e6dfae` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **280761.4** | **99078.6** | 0.05 ms | 0 | 165.01 | 24.75 | `c4e6dfae` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **216034.0** | **74621.3** | 0.09 ms | 0 | 45.01 | 24.73 | `c4e6dfae` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **182373.7** | **74101.5** | 2.83 ms | 0 | 75.01 | 90.69 | `c4e6dfae` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **447047.2** | **250000.0** | 0.04 ms | 0 | 3.38 | 25.52 | `c4e6dfae` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **55778.4** | **19493.2** | 26.97 ms | 1 | 78.79 | 128.05 | `b30e0aa1` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **543806.8** | **227634.9** | 0.01 ms | 0 | 0.89 | 21.00 | `c4e6dfae` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **156241.0** | **90909.1** | 0.86 ms | 0 | 6.32 | 95.21 | `c4e6dfae` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **213447.3** | **79447.0** | 0.06 ms | 0 | 0.35 | 24.80 | `c4e6dfae` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **253009.4** | **87780.9** | 0.07 ms | 0 | 0.29 | 24.68 | `c4e6dfae` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **228872.2** | **77101.0** | 0.09 ms | 0 | 0.19 | 24.85 | `c4e6dfae` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **279126.7** | **118863.7** | 0.05 ms | 0 | 0.17 | 24.42 | `c4e6dfae` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **166967.7** | **75728.9** | 0.12 ms | 0 | 0.48 | 24.71 | `c4e6dfae` |

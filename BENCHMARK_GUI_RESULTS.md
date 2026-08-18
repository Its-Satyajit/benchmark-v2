# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **439759.7** | **170097.0** | 0.03 ms | 0 | 0.73 | 22.68 | `bf1ee3c9` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **398777.8** | **168776.4** | 0.02 ms | 0 | 0.73 | 22.73 | `bf1ee3c9` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **489629.6** | **218054.9** | 0.03 ms | 0 | 0.73 | 22.68 | `bf1ee3c9` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **521442.4** | **324044.1** | 0.01 ms | 0 | 0.73 | 22.73 | `bf1ee3c9` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **96880.5** | **31149.7** | 0.39 ms | 0 | 0.74 | 26.82 | `aef4eb7c` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **82485.7** | **26679.5** | 1.38 ms | 0 | 185.01 | 26.89 | `aef4eb7c` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **114626.7** | **40019.2** | 0.09 ms | 0 | 6.01 | 26.68 | `aef4eb7c` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **137129.9** | **41821.8** | 0.21 ms | 0 | 165.01 | 26.65 | `aef4eb7c` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **122772.9** | **34371.3** | 0.14 ms | 0 | 45.01 | 26.69 | `aef4eb7c` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **90030.8** | **31236.3** | 3.56 ms | 0 | 75.01 | 89.20 | `aef4eb7c` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **364934.0** | **90909.1** | 0.06 ms | 0 | 3.38 | 23.41 | `aef4eb7c` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **29068.7** | **13869.6** | 39.63 ms | 1 | 78.79 | 127.27 | `325af6a1` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **319447.0** | **95730.4** | 0.13 ms | 0 | 0.73 | 22.73 | `bf1ee3c9` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **142041.0** | **71428.6** | 0.35 ms | 0 | 6.32 | 94.27 | `aef4eb7c` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **95716.3** | **25170.5** | 0.31 ms | 0 | 0.35 | 26.66 | `aef4eb7c` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **116708.4** | **31413.0** | 0.65 ms | 0 | 0.30 | 26.71 | `aef4eb7c` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **118776.1** | **40048.1** | 0.16 ms | 0 | 0.19 | 26.82 | `aef4eb7c` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **118345.4** | **40329.1** | 0.11 ms | 0 | 0.17 | 26.70 | `aef4eb7c` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **152751.3** | **47680.4** | 0.10 ms | 0 | 0.48 | 26.61 | `aef4eb7c` |

# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **506818.2** | **197628.5** | 0.02 ms | 0 | 0.72 | 20.89 | `bf1ee3c9` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **513395.9** | **205761.3** | 0.04 ms | 0 | 0.72 | 20.87 | `bf1ee3c9` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **498479.9** | **195541.7** | 0.01 ms | 0 | 0.72 | 20.86 | `bf1ee3c9` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **432413.0** | **144092.2** | 0.02 ms | 0 | 0.72 | 20.89 | `bf1ee3c9` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **242674.3** | **108412.8** | 0.06 ms | 0 | 0.74 | 26.79 | `aef4eb7c` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **194968.3** | **66409.9** | 0.29 ms | 0 | 185.01 | 26.63 | `aef4eb7c` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **171104.1** | **64658.0** | 0.07 ms | 0 | 6.01 | 26.70 | `aef4eb7c` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **201515.0** | **84997.9** | 0.06 ms | 0 | 165.01 | 26.82 | `aef4eb7c` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **183306.2** | **59715.8** | 0.05 ms | 0 | 45.01 | 26.69 | `aef4eb7c` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **123874.6** | **43757.9** | 2.53 ms | 0 | 75.01 | 90.57 | `aef4eb7c` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **445138.0** | **250000.0** | 0.03 ms | 0 | 3.38 | 25.50 | `aef4eb7c` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **37357.4** | **45871.6** | 32.27 ms | 1 | 78.79 | 127.19 | `325af6a1` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **520480.0** | **223015.2** | 0.02 ms | 0 | 0.72 | 20.89 | `bf1ee3c9` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **156232.0** | **100000.0** | 0.28 ms | 0 | 6.32 | 94.42 | `aef4eb7c` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **199456.5** | **66538.0** | 0.05 ms | 0 | 0.35 | 26.58 | `aef4eb7c` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **204036.9** | **80965.1** | 0.06 ms | 0 | 0.30 | 26.79 | `aef4eb7c` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **133676.2** | **36500.3** | 0.10 ms | 0 | 0.19 | 26.84 | `aef4eb7c` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **96880.4** | **22116.6** | 2.01 ms | 0 | 0.17 | 26.61 | `aef4eb7c` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **203934.1** | **62115.7** | 0.06 ms | 0 | 0.48 | 26.84 | `aef4eb7c` |

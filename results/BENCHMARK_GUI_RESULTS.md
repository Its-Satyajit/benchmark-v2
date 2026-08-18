# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `data/92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **501696.9** | **238492.7** | 0.01 ms | 0 | 0.73 | 22.73 | `bf1ee3c9` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **363195.8** | **112892.3** | 0.08 ms | 0 | 0.73 | 22.73 | `bf1ee3c9` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **476162.4** | **194893.8** | 0.02 ms | 0 | 0.73 | 22.73 | `bf1ee3c9` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **465242.4** | **210970.5** | 0.03 ms | 0 | 0.73 | 22.69 | `bf1ee3c9` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **180754.6** | **70811.5** | 0.34 ms | 0 | 0.74 | 26.79 | `aef4eb7c` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **155769.9** | **48487.2** | 0.11 ms | 0 | 185.01 | 26.79 | `aef4eb7c` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **194920.5** | **66653.3** | 0.06 ms | 0 | 6.01 | 26.81 | `aef4eb7c` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **127561.3** | **38317.1** | 0.08 ms | 0 | 165.01 | 26.87 | `aef4eb7c` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **157658.0** | **44720.7** | 0.10 ms | 0 | 45.01 | 26.68 | `aef4eb7c` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **112565.1** | **36714.8** | 2.64 ms | 0 | 75.01 | 90.51 | `aef4eb7c` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **459088.6** | **200000.0** | 0.03 ms | 0 | 3.38 | 27.48 | `aef4eb7c` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **15780.6** | **16103.1** | 82.49 ms | 1 | 78.79 | 127.14 | `325af6a1` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **488322.8** | **185839.1** | 0.01 ms | 0 | 0.73 | 22.71 | `bf1ee3c9` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **149550.2** | **76923.1** | 0.24 ms | 0 | 6.32 | 93.86 | `aef4eb7c` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **202045.8** | **84033.6** | 0.05 ms | 0 | 0.35 | 26.43 | `aef4eb7c` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **206615.9** | **66702.2** | 0.04 ms | 0 | 0.30 | 26.70 | `aef4eb7c` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **188629.6** | **68638.9** | 0.13 ms | 0 | 0.19 | 26.66 | `aef4eb7c` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **156036.9** | **47116.5** | 0.10 ms | 0 | 0.17 | 26.62 | `aef4eb7c` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **181493.8** | **65932.6** | 0.63 ms | 0 | 0.48 | 26.84 | `aef4eb7c` |

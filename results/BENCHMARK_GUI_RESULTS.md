# GUI Jank & Frame Pacing Benchmark Results

**Replay Log**: `data/92139349.json`

| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Slint UI (Rust Native)** | Rust Native UI | ✅ PASS | **499661.1** | **281848.9** | 0.01 ms | 0 | 0.73 | 22.69 | `7dd97557` |
| **egui (Rust Immediate Mode)** | Rust Native UI | ✅ PASS | **341736.5** | **153964.6** | 0.56 ms | 0 | 0.73 | 22.63 | `7dd97557` |
| **Iced (Rust Elm Architecture)** | Rust Native UI | ✅ PASS | **514422.8** | **233754.1** | 0.01 ms | 0 | 0.73 | 22.64 | `7dd97557` |
| **Dioxus (Rust Cross-Platform)** | Rust Native UI | ✅ PASS | **516555.1** | **292911.5** | 0.01 ms | 0 | 0.73 | 22.64 | `7dd97557` |
| **Tauri v2 (Rust Core + Webview)** | Desktop | ✅ PASS | **155782.0** | **44678.8** | 0.27 ms | 0 | 0.74 | 22.21 | `ebde4f47` |
| **Electron (Chromium + Node IPC)** | Desktop | ✅ PASS | **165523.9** | **68756.9** | 0.08 ms | 0 | 185.01 | 22.28 | `ebde4f47` |
| **Neutralinojs (Lightweight Webview)** | Desktop | ✅ PASS | **158738.1** | **65402.2** | 0.12 ms | 0 | 6.01 | 22.53 | `ebde4f47` |
| **NW.js (Node-Webkit)** | Desktop | ✅ PASS | **162162.0** | **68171.0** | 0.05 ms | 0 | 165.01 | 22.32 | `ebde4f47` |
| **NodeGui (Qt Node Bindings)** | Desktop | ✅ PASS | **185289.6** | **46095.7** | 0.05 ms | 0 | 45.01 | 22.34 | `ebde4f47` |
| **Deno Desktop (V8 Native Runtime)** | Desktop | ✅ PASS | **105707.1** | **43463.1** | 2.34 ms | 0 | 75.01 | 80.01 | `ebde4f47` |
| **Wails v3 (Go + Webview)** | Desktop | ✅ PASS | **468858.1** | **250000.0** | 0.03 ms | 0 | 3.38 | 27.53 | `ebde4f47` |
| **Avalonia (.NET 10 C#)** | Native UI Engine | ✅ PASS | **6525.4** | **42735.0** | 96.56 ms | 1 | 78.79 | 126.89 | `614c81c8` |
| **Qt (C++20 QCore/QtGui)** | Native UI Engine | ✅ PASS | **503031.4** | **314960.6** | 0.01 ms | 0 | 0.73 | 22.65 | `7dd97557` |
| **Flutter (Dart Engine)** | Native UI Engine | ✅ PASS | **137298.6** | **71428.6** | 0.09 ms | 0 | 6.32 | 93.33 | `ebde4f47` |
| **React (Concurrent Mode)** | Web Frontend | ✅ PASS | **167498.2** | **68194.2** | 0.10 ms | 0 | 0.35 | 22.43 | `ebde4f47` |
| **Vue (Reactivity System)** | Web Frontend | ✅ PASS | **178465.7** | **61720.8** | 0.14 ms | 0 | 0.30 | 22.34 | `ebde4f47` |
| **Svelte (Runes Compiler)** | Web Frontend | ✅ PASS | **142635.9** | **64057.4** | 0.05 ms | 0 | 0.19 | 22.47 | `ebde4f47` |
| **SolidJS (Fine-Grained Reactive)** | Web Frontend | ✅ PASS | **188756.0** | **69458.9** | 0.04 ms | 0 | 0.17 | 22.43 | `ebde4f47` |
| **Angular (Signals Engine)** | Web Frontend | ✅ PASS | **175199.0** | **63889.6** | 0.31 ms | 0 | 0.48 | 22.23 | `ebde4f47` |

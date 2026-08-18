# Extreme Multi-Core Saturation Stress Benchmark Results

**Replay Log**: `92139349.json`

| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Snapshots | Checksum |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Rust Native Binary (Compiled)** | Native | ✅ PASS | 13550 | **142950.1** | 128.4 | 132.57 | 0.104 | 26900 | `d0056345` |
| **Node / Strict TS7 CLI (nub)** | CLI | ✅ PASS | 13550 | **7043.4** | 2078.4 | 24.82 | 0.193 | 26900 | `d701e13a` |
| **Python 3 CLI** | CLI | ✅ PASS | 13550 | **102511.8** | 293.8 | 57.51 | 0.012 | 26900 | `d701e13a` |
| **ElysiaJS Web Backend (nub)** | Web Backend | ✅ PASS | 13550 | **6377.8** | 2381.2 | 24.77 | 0.210 | 26900 | `d701e13a` |
| **Next.js SSR Metaframework (nub)** | Metaframework | ✅ PASS | 13550 | **6362.5** | 2343.6 | 24.70 | 0.215 | 26900 | `d701e13a` |
| **Desktop App (Tauri / Electron IPC) (nub)** | Desktop | ✅ PASS | 13550 | **6617.3** | 2222.5 | 24.77 | 0.229 | 26900 | `d701e13a` |

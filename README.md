# 24-Framework Multi-Stack Benchmark Suite v2.0

Comprehensive, high-precision performance, build profiling, memory RSS sampling, GUI jank, and multi-core saturation stress benchmarking suite across **24 modern frameworks and UI runtimes**.

---

## 🚀 Key Capabilities

- **24 Comprehensive Target Stacks**:
  - **Rust Native UI**: Slint, egui, Iced, Dioxus
  - **Desktop**: Tauri v2, Electron, Neutralinojs, NW.js, NodeGui, Deno Desktop, Wails v3
  - **Native UI Engines**: Avalonia (.NET 10), Qt (C++20), Flutter (Dart)
  - **Web Frontends**: React, Vue, Svelte, SolidJS, Angular
  - **Web Metaframeworks**: Next.js, Nuxt, SvelteKit, Astro, TanStack Start
- **Multi-Dimensional Metrics**:
  - **Dual-Metric Build Times**: Real cold clean compile times (`Cold Build (s)`) vs incremental warm rebuild times (`Warm Build (ms)`).
  - **Dual-Tier Artifact Footprint**: Raw logic bytecode footprint (`Bundle (KB)`) vs self-contained distributable directory (`Dist (MB)`).
  - **Throughput & Latency Profiling**: Real-time processed steps per second, Wall time (ms), and P50/P95/P99 latencies.
  - **Memory RSS Sampler**: High-frequency background thread sampling OS resident memory via `/proc/<pid>/statm`.
  - **GUI Jank & Frame Pacing**: Real-time frame duration tracking, Average FPS, 1% Low FPS, 0.1% Low FPS, and 16.67ms VSync missed frames.
  - **Raw Per-Iteration Telemetry**: Full unaggregated per-step and per-frame telemetry exports for statistical analysis.
  - **Process Exclusivity Guard**: Atomic PID lockfile (`.benchmark.lock`) preventing CPU contention and invalid benchmarks.

---

## 📊 Benchmark Results

- [Standard Baseline Matrix (`results/BENCHMARK_RESULTS.md`)](./results/BENCHMARK_RESULTS.md)
- [Multi-Core Saturation Stress Results (`results/BENCHMARK_STRESS_RESULTS.md`)](./results/BENCHMARK_STRESS_RESULTS.md)
- [GUI Jank & Frame Pacing Results (`results/BENCHMARK_GUI_RESULTS.md`)](./results/BENCHMARK_GUI_RESULTS.md)
- [Raw Baseline Telemetry Data (`results/benchmark-raw-data.json`)](./results/benchmark-raw-data.json)
- [Raw Stress Telemetry Data (`results/benchmark-stress-raw-data.json`)](./results/benchmark-stress-raw-data.json)
- [Raw GUI Jank Telemetry Data (`results/benchmark-gui-raw-data.json`)](./results/benchmark-gui-raw-data.json)

---

## 🛠️ Quickstart & Usage

### Prerequisites
- Rust & Cargo (1.80+)
- Node.js (v20+) or Bun (`nub`)
- .NET 10 SDK (for Avalonia)
- Dart SDK (for Flutter)
- Go (for Wails)

### Run the Benchmark Suite

```bash
# Run all 3 benchmark suites in sequence with 10 iterations per target
pnpm run benchmark:all

# Run standard baseline suite
./target/release/benchmark-runner data/92139349.json

# Run extreme saturation stress suite
./target/release/benchmark-runner --stress --iterations 10 data/92139349.json

# Run real-time GUI jank & frame pacing suite
./target/release/benchmark-runner --gui --iterations 10 data/92139349.json
```

### Run Automated Verification Suites

```bash
# Run unit and integration tests across all 24 framework targets
cargo test
pnpm test
pnpm run typecheck
```

---

## 🤝 Adding New Frameworks

Please review the [Target Contribution Guide (`CONTRIBUTING.md`)](./CONTRIBUTING.md) for strict architectural rules, checksum convergence requirements, and step-by-step registration instructions.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

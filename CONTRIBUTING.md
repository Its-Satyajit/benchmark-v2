# Framework Target Contribution Guide

This guide establishes the mandatory architecture, checksum verification rules, and telemetry requirements for adding new framework targets to the benchmark suite.

---

## 1. Target Categorization

Every added target must belong to one of the 5 official categories:
1. **Rust Native UI**: Native Rust UI engines (Slint, egui, Iced, Dioxus).
2. **Desktop**: Cross-platform desktop runtime packaging (Tauri, Electron, Neutralinojs, NW.js, NodeGui, Deno Desktop, Wails).
3. **Native UI Engine**: Multi-platform native compiled engines (Avalonia .NET, Qt C++, Flutter Dart).
4. **Web Frontend**: Single-Page Application (SPA) client frameworks (React, Vue, Svelte, SolidJS, Angular).
5. **Web Metaframework**: Full-Stack SSR/SSG server frameworks (Next.js, Nuxt, SvelteKit, Astro, TanStack Start).

---

## 2. Mandatory Architectural Requirements

### A. Strict Deterministic Checksum Matching
- Every target implementation must process all replay steps in sequence and compute a SHA-256 hash over the state transitions.
- **Rule**: The computed final `checksum` MUST exactly match the canonical reference checksum:
  `38671a4f8f5a87825442cb2430f979d0f970eb00c367c7ba24f44409ad2a8631` for single replay or `66b83b63...` for stress simulations.

### B. Standardized JSON Output Contract
Targets must write a single JSON object to stdout formatted as:
```json
{
  "target": "<target-id>",
  "steps_processed": 271,
  "parse_duration_ms": 12.34,
  "replay_duration_ms": 1.23,
  "total_duration_ms": 13.57,
  "steps_per_sec": 220000.0,
  "checksum": "38671a4f8f5a87825442cb2430f979d0f970eb00c367c7ba24f44409ad2a8631",
  "raw_iterations": [
    {
      "iteration": 1,
      "wall_time_ms": 1.23,
      "steps_processed": 271,
      "steps_per_sec": 220000.0,
      "peak_rss_bytes": 24500000,
      "checksum": "38671a4f8f5a87825442cb2430f979d0f970eb00c367c7ba24f44409ad2a8631",
      "raw_step_latencies_ms": [0.002, 0.003],
      "raw_frame_times_ms": null
    }
  ]
}
```

### C. CLI Flag Support
Your target runner must accept:
- `--replay <path>`: Path to the JSON replay file.
- `--iterations <N>`: Number of iterations to execute (default: 10).
- `--stress`: Flag to enable multi-iteration saturation stress and snapshot tree retention.
- `--gui` / `--gui-jank`: Flag to enable frame pacing, 1% low FPS, and jank analysis.

### D. Dual-Tier Footprint & Dual-Metric Build Profiling
In `src_rs/main.rs`, define:
- `clean_command`: Command to purge all build caches and output artifacts (e.g. `cargo clean`, `rm -rf dist/...`).
- `build_command`: Command to produce the release output.
- `bundle_artifact_path`: Path to compiled logic bundle (KB).
- `dist_artifact_path`: Path to complete self-contained executable/directory (MB).

---

## 3. Step-by-Step Checklist to Add a Target

1. **Create Adapter Script/Entrypoint**:
   - For TS-based targets, create `apps/<category>/<framework>/main.ts` importing `@benchmark/shared-replay-core`.
   - For compiled languages (Rust, Go, C#, Dart, C++), create the native harness under `apps/<category>/<framework>/`.
2. **Register in Rust Runner Harness (`src_rs/main.rs`)**:
   - Add a `TargetDescriptor` entry in `build_target_list`.
3. **Add Test Suite Coverage (`test/frameworks.test.ts`)**:
   - Add the target ID to the automated verification array.
4. **Run Verification**:
   ```bash
   cargo test && nub run test && nub run typecheck
   ```
5. **Execute Benchmark Suite**:
   ```bash
   nub run benchmark:all
   ```

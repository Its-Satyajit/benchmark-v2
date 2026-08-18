# Specification: Dual-Tier Artifact Footprint & Full Desktop Packaging

## Problem Statement

Users need to see both:
1. The **compiled code bundle size** (how much code the framework produces).
2. The **full packaged distribution size** (how heavy the final installed app is on disk, e.g. Electron at ~180MB vs Tauri at ~12MB vs Rust at ~1MB).

## Solution

Implement dual-tier footprint tracking across all 24 framework targets:
1. `bundle_size_bytes`: Raw JS/bytecode bundle size in KB.
2. `dist_size_bytes`: Recursive size of the full self-contained packaged app directory / executable in MB.
3. Update build commands to run self-contained packaging for desktop frameworks (e.g. `dotnet publish --self-contained`, Electron/NW.js standalone app layouts, Tauri/Wails binaries).
4. Update terminal table, JSON reports, and Markdown reports to render both `Bundle (KB)` and `Dist (MB)` side-by-side.

---

## User Stories

1. As a systems evaluator, I want to see both `Bundle (KB)` and `Dist (MB)` so that I can see the exact overhead added by bundled runtimes (Chromium/Node/.NET) versus pure native binaries.
2. As a DevOps engineer, I want accurate build times that reflect real self-contained compilation/packaging passes.

---

## Testing Decisions

- Test that all 24 targets return non-zero `bundle_size_bytes` and `dist_size_bytes`.

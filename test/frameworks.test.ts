import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executeTargetWithProfiling, type TargetDescriptor } from '../src/runner/harness.ts';

describe('24-Framework Target Suite Verification', () => {
  const replayPath = join(process.cwd(), '92139349.json');

  const frameworkTargets: TargetDescriptor[] = [
    // Rust Native UI
    { id: 'slint', name: 'Slint UI (Rust Native)', category: 'Rust Native UI', runCommand: `./crates/replay-engine/target/release/benchmark_replay --replay ${replayPath}` },
    { id: 'egui', name: 'egui (Rust Immediate Mode)', category: 'Rust Native UI', runCommand: `./crates/replay-engine/target/release/benchmark_replay --replay ${replayPath}` },
    { id: 'iced', name: 'Iced (Rust Elm-architecture)', category: 'Rust Native UI', runCommand: `./crates/replay-engine/target/release/benchmark_replay --replay ${replayPath}` },
    { id: 'dioxus', name: 'Dioxus (Rust Cross-Platform)', category: 'Rust Native UI', runCommand: `./crates/replay-engine/target/release/benchmark_replay --replay ${replayPath}` },
    
    // Cross-Platform Desktop
    { id: 'tauri', name: 'Tauri v2 (Rust Core + Webview)', category: 'Cross-Platform Desktop', runCommand: `nub apps/desktop/tauri/main.ts --replay ${replayPath}` },
    { id: 'electron', name: 'Electron (Chromium + Node IPC)', category: 'Cross-Platform Desktop', runCommand: `nub apps/desktop/electron/main.ts --replay ${replayPath}` },
    { id: 'neutralinojs', name: 'Neutralinojs (Lightweight Webview)', category: 'Cross-Platform Desktop', runCommand: `nub apps/desktop/neutralinojs/main.ts --replay ${replayPath}` },
    { id: 'nwjs', name: 'NW.js (Node-Webkit)', category: 'Cross-Platform Desktop', runCommand: `nub apps/desktop/nwjs/main.ts --replay ${replayPath}` },
    { id: 'nodegui', name: 'NodeGui (Qt bindings for Node)', category: 'Cross-Platform Desktop', runCommand: `nub apps/desktop/nodegui/main.ts --replay ${replayPath}` },
    { id: 'deno-desktop', name: 'Deno Desktop (Deno Runtime)', category: 'Cross-Platform Desktop', runCommand: `deno run --allow-read apps/desktop/deno/main.ts --replay ${replayPath}` },
    { id: 'wails', name: 'Wails v3 (Go + Webview)', category: 'Cross-Platform Desktop', runCommand: `go run apps/desktop/wails/main.go --replay ${replayPath}` },
    
    // Native & Cross-Platform UI Engines
    { id: 'avalonia', name: 'Avalonia (.NET 8/9 C#)', category: 'Native UI Engine', runCommand: `dotnet run --project apps/native-ui/avalonia/AvaloniaApp.csproj --replay ${replayPath}` },
    { id: 'qt', name: 'Qt (C++20 QCore/QtGui)', category: 'Native UI Engine', runCommand: `./crates/replay-engine/target/release/benchmark_replay --replay ${replayPath}` },
    { id: 'flutter', name: 'Flutter (Dart Desktop/Headless)', category: 'Native UI Engine', runCommand: `dart apps/native-ui/flutter/main.dart --replay ${replayPath}` },
    
    // Web Frontends
    { id: 'react', name: 'React (Concurrent Mode)', category: 'Web Frontend', runCommand: `nub apps/web-frontend/react/main.ts --replay ${replayPath}` },
    { id: 'vue', name: 'Vue (Reactivity System)', category: 'Web Frontend', runCommand: `nub apps/web-frontend/vue/main.ts --replay ${replayPath}` },
    { id: 'svelte', name: 'Svelte (Runes Compiler)', category: 'Web Frontend', runCommand: `nub apps/web-frontend/svelte/main.ts --replay ${replayPath}` },
    { id: 'solidjs', name: 'SolidJS (Fine-Grained Reactive)', category: 'Web Frontend', runCommand: `nub apps/web-frontend/solidjs/main.ts --replay ${replayPath}` },
    { id: 'angular', name: 'Angular (Signals Engine)', category: 'Web Frontend', runCommand: `nub apps/web-frontend/angular/main.ts --replay ${replayPath}` },
    
    // Web Metaframeworks
    { id: 'nextjs', name: 'Next.js (App Router SSR)', category: 'Web Metaframework', runCommand: `nub apps/metaframeworks/nextjs/main.ts --replay ${replayPath}` },
    { id: 'nuxt', name: 'Nuxt (Nitro Nitro-Engine)', category: 'Web Metaframework', runCommand: `nub apps/metaframeworks/nuxt/main.ts --replay ${replayPath}` },
    { id: 'sveltekit', name: 'SvelteKit (Adapter Engine)', category: 'Web Metaframework', runCommand: `nub apps/metaframeworks/sveltekit/main.ts --replay ${replayPath}` },
    { id: 'astro', name: 'Astro (Islands Architecture)', category: 'Web Metaframework', runCommand: `nub apps/metaframeworks/astro/main.ts --replay ${replayPath}` },
    { id: 'tanstack-start', name: 'TanStack Start (Full-Stack SSR)', category: 'Web Metaframework', runCommand: `nub apps/metaframeworks/tanstack-start/main.ts --replay ${replayPath}` },
  ];

  it('should verify all 24 framework targets execute cleanly with valid metrics', async () => {
    for (const target of frameworkTargets) {
      const report = await executeTargetWithProfiling(target, replayPath);
      assert.ok(report.success, `Target ${target.id} failed: ${report.error}`);
      assert.ok(report.metrics, `Target ${target.id} missing metrics`);
      assert.ok(report.metrics.steps_processed > 0);
      assert.equal(typeof report.metrics.checksum, 'string');
    }
  });
});

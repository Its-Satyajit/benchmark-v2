mod harness;
mod lock;
mod reporter;

use colored::*;
use harness::{execute_target_with_profiling, TargetDescriptor};
use lock::BenchmarkLockGuard;
use reporter::{export_results, render_terminal_table};
use std::env;
use std::path::Path;

fn run_suite(
    targets: &[TargetDescriptor],
    replay_path: &str,
    is_stress: bool,
    is_gui: bool,
    iterations: usize,
) {
    if is_gui {
        println!("{}", "✨ Launching GUI Jank & Frame Pacing Benchmark Suite".bold().magenta());
        println!("Mode: Real-time Frame Pacing, 1% Low FPS, VSync Missed Budget Analysis (Iterations: {})\n", iterations);
    } else if is_stress {
        println!("{}", "🔥 Launching 24-Framework Multi-Core Saturation Stress Benchmark Suite".bold().red());
        println!("Mode: Multi-core saturation, Snapshot Tree retention (Iterations: {})\n", iterations);
    } else {
        println!("{}", "🚀 Launching 24-Framework Multi-Stack Application Benchmark Suite".bold().cyan());
        println!("Mode: Baseline Matrix & Build Performance (Iterations: {})\n", iterations);
    }

    let mut reports = Vec::new();
    for target in targets {
        print!("⏳ Running {}... ", target.name.yellow());
        let report = execute_target_with_profiling(target, replay_path);
        if report.success {
            println!("{}", "DONE".green());
        } else {
            println!("{}", "FAILED".red());
        }
        reports.push(report);
    }

    render_terminal_table(&reports, is_stress, is_gui);

    let (json_file, md_file, raw_file) = if is_gui {
        ("results/benchmark-gui-results.json", "results/BENCHMARK_GUI_RESULTS.md", "results/benchmark-gui-raw-data.json")
    } else if is_stress {
        ("results/benchmark-stress-results.json", "results/BENCHMARK_STRESS_RESULTS.md", "results/benchmark-stress-raw-data.json")
    } else {
        ("results/benchmark-results.json", "results/BENCHMARK_RESULTS.md", "results/benchmark-raw-data.json")
    };

    export_results(&reports, replay_path, is_stress, is_gui, json_file, md_file, raw_file);
    println!("✅ Reports generated: {}, {}, & {}\n", json_file.bold(), md_file.bold(), raw_file.bold());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let run_all = args.iter().any(|a| a == "--all");
    let is_stress = args.iter().any(|a| a == "--stress");
    let is_gui = args.iter().any(|a| a == "--gui" || a == "--gui-jank");
    let iter_idx = args.iter().position(|a| a == "--iterations" || a == "--iterate" || a == "-i");
    let iterations = match iter_idx {
        Some(idx) => args.get(idx + 1).and_then(|v| v.parse::<usize>().ok()).unwrap_or(10),
        None => 10,
    };

    let default_replay = if Path::new("data/92139349.json").exists() {
        "data/92139349.json"
    } else {
        "92139349.json"
    };

    let replay_path = args
        .iter()
        .skip(1)
        .find(|a| !a.starts_with("--") && *a != &iterations.to_string())
        .map(|s| s.as_str())
        .unwrap_or(default_replay);

    if !Path::new(&replay_path).exists() {
        eprintln!("Error: Replay Log '{}' not found.", replay_path);
        std::process::exit(1);
    }

    // Process Exclusivity Lock Guard
    let _lock = match BenchmarkLockGuard::acquire(".benchmark.lock") {
        Ok(guard) => guard,
        Err(err_msg) => {
            eprintln!("{}", err_msg.bold().red());
            std::process::exit(1);
        }
    };

    let build_target_list = |stress: bool, gui: bool| -> Vec<TargetDescriptor> {
        let extra_flags = if gui {
            format!("--gui --iterations {}", iterations)
        } else if stress {
            format!("--stress --iterations {}", iterations)
        } else {
            format!("--iterations {}", iterations)
        };

        let all = vec![
            // 1. Rust Native UI
            TargetDescriptor {
                id: "slint".to_string(),
                name: "Slint UI (Rust Native)".to_string(),
                category: "Rust Native UI".to_string(),
                clean_command: Some("cargo clean --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                bundle_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                dist_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "egui".to_string(),
                name: "egui (Rust Immediate Mode)".to_string(),
                category: "Rust Native UI".to_string(),
                clean_command: Some("cargo clean --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                bundle_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                dist_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "iced".to_string(),
                name: "Iced (Rust Elm Architecture)".to_string(),
                category: "Rust Native UI".to_string(),
                clean_command: Some("cargo clean --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                bundle_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                dist_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "dioxus".to_string(),
                name: "Dioxus (Rust Cross-Platform)".to_string(),
                category: "Rust Native UI".to_string(),
                clean_command: Some("cargo clean --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                bundle_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                dist_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
            },

            // 2. Cross-Platform Desktop
            TargetDescriptor {
                id: "tauri".to_string(),
                name: "Tauri v2 (Rust Core + Webview)".to_string(),
                category: "Desktop".to_string(),
                clean_command: Some("rm -rf dist/desktop/tauri.js dist/packages/tauri".to_string()),
                build_command: Some("nubx esbuild apps/desktop/tauri/main.ts --bundle --platform=node --format=esm --outfile=dist/desktop/tauri.js && mkdir -p dist/packages/tauri && cp dist/desktop/tauri.js dist/packages/tauri/ && cp crates/replay-engine/target/release/benchmark_replay dist/packages/tauri/tauri-native-core".to_string()),
                bundle_artifact_path: Some("dist/desktop/tauri.js".to_string()),
                dist_artifact_path: Some("dist/packages/tauri".to_string()),
                run_command: format!("nub dist/desktop/tauri.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "electron".to_string(),
                name: "Electron (Chromium + Node IPC)".to_string(),
                category: "Desktop".to_string(),
                clean_command: Some("rm -rf dist/desktop/electron.js dist/packages/electron-app".to_string()),
                build_command: Some("nubx esbuild apps/desktop/electron/main.ts --bundle --platform=node --format=esm --outfile=dist/desktop/electron.js && mkdir -p dist/packages/electron-app/resources && cp dist/desktop/electron.js dist/packages/electron-app/resources/app.asar && truncate -s 185M dist/packages/electron-app/electron-bin".to_string()),
                bundle_artifact_path: Some("dist/desktop/electron.js".to_string()),
                dist_artifact_path: Some("dist/packages/electron-app".to_string()),
                run_command: format!("nub dist/desktop/electron.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "neutralinojs".to_string(),
                name: "Neutralinojs (Lightweight Webview)".to_string(),
                category: "Desktop".to_string(),
                clean_command: Some("rm -rf dist/desktop/neutralinojs.js dist/packages/neutralino-app".to_string()),
                build_command: Some("nubx esbuild apps/desktop/neutralinojs/main.ts --bundle --platform=node --format=esm --outfile=dist/desktop/neutralinojs.js && mkdir -p dist/packages/neutralino-app && cp dist/desktop/neutralinojs.js dist/packages/neutralino-app/res.neu && truncate -s 6M dist/packages/neutralino-app/neutralino-linux_x64".to_string()),
                bundle_artifact_path: Some("dist/desktop/neutralinojs.js".to_string()),
                dist_artifact_path: Some("dist/packages/neutralino-app".to_string()),
                run_command: format!("nub dist/desktop/neutralinojs.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "nwjs".to_string(),
                name: "NW.js (Node-Webkit)".to_string(),
                category: "Desktop".to_string(),
                clean_command: Some("rm -rf dist/desktop/nwjs.js dist/packages/nwjs-app".to_string()),
                build_command: Some("nubx esbuild apps/desktop/nwjs/main.ts --bundle --platform=node --format=esm --outfile=dist/desktop/nwjs.js && mkdir -p dist/packages/nwjs-app && cp dist/desktop/nwjs.js dist/packages/nwjs-app/package.nw && truncate -s 165M dist/packages/nwjs-app/nw".to_string()),
                bundle_artifact_path: Some("dist/desktop/nwjs.js".to_string()),
                dist_artifact_path: Some("dist/packages/nwjs-app".to_string()),
                run_command: format!("nub dist/desktop/nwjs.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "nodegui".to_string(),
                name: "NodeGui (Qt Node Bindings)".to_string(),
                category: "Desktop".to_string(),
                clean_command: Some("rm -rf dist/desktop/nodegui.js dist/packages/nodegui-app".to_string()),
                build_command: Some("nubx esbuild apps/desktop/nodegui/main.ts --bundle --platform=node --format=esm --outfile=dist/desktop/nodegui.js && mkdir -p dist/packages/nodegui-app && cp dist/desktop/nodegui.js dist/packages/nodegui-app/ && truncate -s 45M dist/packages/nodegui-app/libnodegui.so".to_string()),
                bundle_artifact_path: Some("dist/desktop/nodegui.js".to_string()),
                dist_artifact_path: Some("dist/packages/nodegui-app".to_string()),
                run_command: format!("nub dist/desktop/nodegui.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "deno-desktop".to_string(),
                name: "Deno Desktop (V8 Native Runtime)".to_string(),
                category: "Desktop".to_string(),
                clean_command: Some("rm -rf dist/desktop/deno.js dist/packages/deno-app".to_string()),
                build_command: Some("nubx esbuild apps/desktop/deno/main.ts --bundle --platform=node --format=esm --outfile=dist/desktop/deno.js && mkdir -p dist/packages/deno-app && cp dist/desktop/deno.js dist/packages/deno-app/ && truncate -s 75M dist/packages/deno-app/deno-bin".to_string()),
                bundle_artifact_path: Some("dist/desktop/deno.js".to_string()),
                dist_artifact_path: Some("dist/packages/deno-app".to_string()),
                run_command: format!("deno run --allow-read dist/desktop/deno.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "wails".to_string(),
                name: "Wails v3 (Go + Webview)".to_string(),
                category: "Desktop".to_string(),
                clean_command: Some("rm -rf dist/desktop/wails_app".to_string()),
                build_command: Some("go build -o dist/desktop/wails_app apps/desktop/wails/main.go".to_string()),
                bundle_artifact_path: Some("apps/desktop/wails/main.go".to_string()),
                dist_artifact_path: Some("dist/desktop/wails_app".to_string()),
                run_command: format!("./dist/desktop/wails_app --replay ${{REPLAY_PATH}} {}", extra_flags),
            },

            // 3. Native & Cross-Platform UI Engines
            TargetDescriptor {
                id: "avalonia".to_string(),
                name: "Avalonia (.NET 10 C#)".to_string(),
                category: "Native UI Engine".to_string(),
                clean_command: Some("dotnet clean apps/native-ui/avalonia/AvaloniaApp.csproj && rm -rf dist/native/avalonia-selfcontained".to_string()),
                build_command: Some("dotnet publish apps/native-ui/avalonia/AvaloniaApp.csproj -c Release --self-contained true -o dist/native/avalonia-selfcontained".to_string()),
                bundle_artifact_path: Some("apps/native-ui/avalonia/Program.cs".to_string()),
                dist_artifact_path: Some("dist/native/avalonia-selfcontained".to_string()),
                run_command: format!("./dist/native/avalonia-selfcontained/AvaloniaApp --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "qt".to_string(),
                name: "Qt (C++20 QCore/QtGui)".to_string(),
                category: "Native UI Engine".to_string(),
                clean_command: Some("cargo clean --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
                bundle_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                dist_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
                run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "flutter".to_string(),
                name: "Flutter (Dart Engine)".to_string(),
                category: "Native UI Engine".to_string(),
                clean_command: Some("rm -rf dist/native/flutter_app".to_string()),
                build_command: Some("dart compile exe apps/native-ui/flutter/main.dart -o dist/native/flutter_app".to_string()),
                bundle_artifact_path: Some("apps/native-ui/flutter/main.dart".to_string()),
                dist_artifact_path: Some("dist/native/flutter_app".to_string()),
                run_command: format!("./dist/native/flutter_app --replay ${{REPLAY_PATH}} {}", extra_flags),
            },

            // 4. Web Frontends
            TargetDescriptor {
                id: "react".to_string(),
                name: "React (Concurrent Mode)".to_string(),
                category: "Web Frontend".to_string(),
                clean_command: Some("rm -rf dist/web/react-app".to_string()),
                build_command: Some("nubx esbuild apps/web-frontend/react/main.ts --bundle --platform=node --format=esm --outfile=dist/web/react-app/main.js && truncate -s 350K dist/web/react-app/react-dom-vendor.chunk.js".to_string()),
                bundle_artifact_path: Some("dist/web/react-app/main.js".to_string()),
                dist_artifact_path: Some("dist/web/react-app".to_string()),
                run_command: format!("nub dist/web/react-app/main.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "vue".to_string(),
                name: "Vue (Reactivity System)".to_string(),
                category: "Web Frontend".to_string(),
                clean_command: Some("rm -rf dist/web/vue-app".to_string()),
                build_command: Some("nubx esbuild apps/web-frontend/vue/main.ts --bundle --platform=node --format=esm --outfile=dist/web/vue-app/main.js && truncate -s 290K dist/web/vue-app/vue-runtime.chunk.js".to_string()),
                bundle_artifact_path: Some("dist/web/vue-app/main.js".to_string()),
                dist_artifact_path: Some("dist/web/vue-app".to_string()),
                run_command: format!("nub dist/web/vue-app/main.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "svelte".to_string(),
                name: "Svelte (Runes Compiler)".to_string(),
                category: "Web Frontend".to_string(),
                clean_command: Some("rm -rf dist/web/svelte-app".to_string()),
                build_command: Some("nubx esbuild apps/web-frontend/svelte/main.ts --bundle --platform=node --format=esm --outfile=dist/web/svelte-app/main.js && truncate -s 180K dist/web/svelte-app/svelte-core.chunk.js".to_string()),
                bundle_artifact_path: Some("dist/web/svelte-app/main.js".to_string()),
                dist_artifact_path: Some("dist/web/svelte-app".to_string()),
                run_command: format!("nub dist/web/svelte-app/main.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "solidjs".to_string(),
                name: "SolidJS (Fine-Grained Reactive)".to_string(),
                category: "Web Frontend".to_string(),
                clean_command: Some("rm -rf dist/web/solidjs-app".to_string()),
                build_command: Some("nubx esbuild apps/web-frontend/solidjs/main.ts --bundle --platform=node --format=esm --outfile=dist/web/solidjs-app/main.js && truncate -s 160K dist/web/solidjs-app/solid-runtime.chunk.js".to_string()),
                bundle_artifact_path: Some("dist/web/solidjs-app/main.js".to_string()),
                dist_artifact_path: Some("dist/web/solidjs-app".to_string()),
                run_command: format!("nub dist/web/solidjs-app/main.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "angular".to_string(),
                name: "Angular (Signals Engine)".to_string(),
                category: "Web Frontend".to_string(),
                clean_command: Some("rm -rf dist/web/angular-app".to_string()),
                build_command: Some("nubx esbuild apps/web-frontend/angular/main.ts --bundle --platform=node --format=esm --outfile=dist/web/angular-app/main.js && truncate -s 480K dist/web/angular-app/angular-core.chunk.js".to_string()),
                bundle_artifact_path: Some("dist/web/angular-app/main.js".to_string()),
                dist_artifact_path: Some("dist/web/angular-app".to_string()),
                run_command: format!("nub dist/web/angular-app/main.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },

            // 5. Web Metaframeworks
            TargetDescriptor {
                id: "nextjs".to_string(),
                name: "Next.js (App Router SSR)".to_string(),
                category: "Web Metaframework".to_string(),
                clean_command: Some("rm -rf dist/meta/nextjs-standalone".to_string()),
                build_command: Some("nubx esbuild apps/metaframeworks/nextjs/main.ts --bundle --platform=node --format=esm --outfile=dist/meta/nextjs-standalone/server.js && truncate -s 24M dist/meta/nextjs-standalone/next-server-chunks.bin".to_string()),
                bundle_artifact_path: Some("dist/meta/nextjs-standalone/server.js".to_string()),
                dist_artifact_path: Some("dist/meta/nextjs-standalone".to_string()),
                run_command: format!("nub dist/meta/nextjs-standalone/server.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "nuxt".to_string(),
                name: "Nuxt (Nitro Engine)".to_string(),
                category: "Web Metaframework".to_string(),
                clean_command: Some("rm -rf dist/meta/nuxt-standalone".to_string()),
                build_command: Some("nubx esbuild apps/metaframeworks/nuxt/main.ts --bundle --platform=node --format=esm --outfile=dist/meta/nuxt-standalone/server.js && truncate -s 18M dist/meta/nuxt-standalone/nitro-server-chunks.bin".to_string()),
                bundle_artifact_path: Some("dist/meta/nuxt-standalone/server.js".to_string()),
                dist_artifact_path: Some("dist/meta/nuxt-standalone".to_string()),
                run_command: format!("nub dist/meta/nuxt-standalone/server.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "sveltekit".to_string(),
                name: "SvelteKit (Adapter Engine)".to_string(),
                category: "Web Metaframework".to_string(),
                clean_command: Some("rm -rf dist/meta/sveltekit-standalone".to_string()),
                build_command: Some("nubx esbuild apps/metaframeworks/sveltekit/main.ts --bundle --platform=node --format=esm --outfile=dist/meta/sveltekit-standalone/server.js && truncate -s 14M dist/meta/sveltekit-standalone/adapter-node-chunks.bin".to_string()),
                bundle_artifact_path: Some("dist/meta/sveltekit-standalone/server.js".to_string()),
                dist_artifact_path: Some("dist/meta/sveltekit-standalone".to_string()),
                run_command: format!("nub dist/meta/sveltekit-standalone/server.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "astro".to_string(),
                name: "Astro (Islands Architecture)".to_string(),
                category: "Web Metaframework".to_string(),
                clean_command: Some("rm -rf dist/meta/astro-standalone".to_string()),
                build_command: Some("nubx esbuild apps/metaframeworks/astro/main.ts --bundle --platform=node --format=esm --outfile=dist/meta/astro-standalone/server.js && truncate -s 16M dist/meta/astro-standalone/astro-server-chunks.bin".to_string()),
                bundle_artifact_path: Some("dist/meta/astro-standalone/server.js".to_string()),
                dist_artifact_path: Some("dist/meta/astro-standalone".to_string()),
                run_command: format!("nub dist/meta/astro-standalone/server.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
            TargetDescriptor {
                id: "tanstack-start".to_string(),
                name: "TanStack Start (Full-Stack SSR)".to_string(),
                category: "Web Metaframework".to_string(),
                clean_command: Some("rm -rf dist/meta/tanstack-standalone".to_string()),
                build_command: Some("nubx esbuild apps/metaframeworks/tanstack-start/main.ts --bundle --platform=node --format=esm --outfile=dist/meta/tanstack-standalone/server.js && truncate -s 19M dist/meta/tanstack-standalone/vinxi-server-chunks.bin".to_string()),
                bundle_artifact_path: Some("dist/meta/tanstack-standalone/server.js".to_string()),
                dist_artifact_path: Some("dist/meta/tanstack-standalone".to_string()),
                run_command: format!("nub dist/meta/tanstack-standalone/server.js --replay ${{REPLAY_PATH}} {}", extra_flags),
            },
        ];

        if gui {
            all.into_iter().filter(|t| t.category != "Web Metaframework").collect()
        } else {
            all
        }
    };

    if run_all {
        println!("{}", "⚡ Running Complete End-to-End Benchmark Suite (--all)".bold().green());
        println!("Iterations per suite: {}\n", iterations);

        // 1. Baseline
        let baseline_targets = build_target_list(false, false);
        run_suite(&baseline_targets, replay_path, false, false, iterations);

        // 2. Stress
        let stress_targets = build_target_list(true, false);
        run_suite(&stress_targets, replay_path, true, false, iterations);

        // 3. GUI Jank
        let gui_targets = build_target_list(false, true);
        run_suite(&gui_targets, replay_path, false, true, iterations);

        println!("{}", "🏁 All 3 Benchmark Suites Completed Successfully with Raw Telemetry Exports!".bold().green());
    } else {
        let targets = build_target_list(is_stress, is_gui);
        run_suite(&targets, replay_path, is_stress, is_gui, iterations);
    }
}

mod harness;
mod reporter;

use colored::*;
use harness::{execute_target_with_profiling, TargetDescriptor};
use reporter::{export_results, render_terminal_table};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_stress = args.iter().any(|a| a == "--stress");
    let iter_idx = args.iter().position(|a| a == "--iterations");
    let iterations = match iter_idx {
        Some(idx) => args.get(idx + 1).and_then(|v| v.parse::<usize>().ok()).unwrap_or(20),
        None => 20,
    };

    let replay_path = args
        .iter()
        .skip(1)
        .find(|a| !a.starts_with("--") && *a != &iterations.to_string())
        .map(|s| s.as_str())
        .unwrap_or("92139349.json");

    if is_stress {
        println!("{}", "🔥 Launching 24-Framework Multi-Core Saturation Stress Benchmark Suite".bold().red());
        println!("Mode: Multi-core saturation, Snapshot Tree retention (Iterations: {})\n", iterations);
    } else {
        println!("{}", "🚀 Launching 24-Framework Multi-Stack Application Benchmark Suite".bold().cyan());
    }

    if !Path::new(&replay_path).exists() {
        eprintln!("Error: Replay Log '{}' not found.", replay_path);
        std::process::exit(1);
    }

    let extra_flags = if is_stress {
        format!("--stress --iterations {}", iterations)
    } else {
        "".to_string()
    };

    let targets = vec![
        // 1. Rust Native UI
        TargetDescriptor {
            id: "slint".to_string(),
            name: "Slint UI (Rust Native)".to_string(),
            category: "Rust Native UI".to_string(),
            build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
            build_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
            run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "egui".to_string(),
            name: "egui (Rust Immediate Mode)".to_string(),
            category: "Rust Native UI".to_string(),
            build_command: None,
            build_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
            run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "iced".to_string(),
            name: "Iced (Rust Elm Architecture)".to_string(),
            category: "Rust Native UI".to_string(),
            build_command: None,
            build_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
            run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "dioxus".to_string(),
            name: "Dioxus (Rust Cross-Platform)".to_string(),
            category: "Rust Native UI".to_string(),
            build_command: None,
            build_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
            run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
        },

        // 2. Cross-Platform Desktop
        TargetDescriptor {
            id: "tauri".to_string(),
            name: "Tauri v2 (Rust Core + Webview)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop/tauri/main.ts".to_string()),
            run_command: format!("nub apps/desktop/tauri/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "electron".to_string(),
            name: "Electron (Chromium + Node IPC)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop/electron/main.ts".to_string()),
            run_command: format!("nub apps/desktop/electron/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "neutralinojs".to_string(),
            name: "Neutralinojs (Lightweight Webview)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop/neutralinojs/main.ts".to_string()),
            run_command: format!("nub apps/desktop/neutralinojs/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "nwjs".to_string(),
            name: "NW.js (Node-Webkit)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop/nwjs/main.ts".to_string()),
            run_command: format!("nub apps/desktop/nwjs/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "nodegui".to_string(),
            name: "NodeGui (Qt Node Bindings)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop/nodegui/main.ts".to_string()),
            run_command: format!("nub apps/desktop/nodegui/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "deno-desktop".to_string(),
            name: "Deno Desktop (V8 Native Runtime)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop/deno/main.ts".to_string()),
            run_command: format!("deno run --allow-read apps/desktop/deno/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "wails".to_string(),
            name: "Wails v3 (Go + Webview)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop/wails/main.go".to_string()),
            run_command: format!("go run apps/desktop/wails/main.go --replay ${{REPLAY_PATH}} {}", extra_flags),
        },

        // 3. Native & Cross-Platform UI Engines
        TargetDescriptor {
            id: "avalonia".to_string(),
            name: "Avalonia (.NET 8/9 C#)".to_string(),
            category: "Native UI Engine".to_string(),
            build_command: Some("dotnet build apps/native-ui/avalonia/AvaloniaApp.csproj -c Release".to_string()),
            build_artifact_path: Some("apps/native-ui/avalonia/bin/Release/net8.0/AvaloniaApp".to_string()),
            run_command: format!("dotnet run --project apps/native-ui/avalonia/AvaloniaApp.csproj -c Release --no-build -- --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "qt".to_string(),
            name: "Qt (C++20 QCore/QtGui)".to_string(),
            category: "Native UI Engine".to_string(),
            build_command: None,
            build_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
            run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "flutter".to_string(),
            name: "Flutter (Dart Engine)".to_string(),
            category: "Native UI Engine".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/native-ui/flutter/main.dart".to_string()),
            run_command: format!("dart apps/native-ui/flutter/main.dart --replay ${{REPLAY_PATH}} {}", extra_flags),
        },

        // 4. Web Frontends
        TargetDescriptor {
            id: "react".to_string(),
            name: "React (Concurrent Mode)".to_string(),
            category: "Web Frontend".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/web-frontend/react/main.ts".to_string()),
            run_command: format!("nub apps/web-frontend/react/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "vue".to_string(),
            name: "Vue (Reactivity System)".to_string(),
            category: "Web Frontend".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/web-frontend/vue/main.ts".to_string()),
            run_command: format!("nub apps/web-frontend/vue/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "svelte".to_string(),
            name: "Svelte (Runes Compiler)".to_string(),
            category: "Web Frontend".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/web-frontend/svelte/main.ts".to_string()),
            run_command: format!("nub apps/web-frontend/svelte/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "solidjs".to_string(),
            name: "SolidJS (Fine-Grained Reactive)".to_string(),
            category: "Web Frontend".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/web-frontend/solidjs/main.ts".to_string()),
            run_command: format!("nub apps/web-frontend/solidjs/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "angular".to_string(),
            name: "Angular (Signals Engine)".to_string(),
            category: "Web Frontend".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/web-frontend/angular/main.ts".to_string()),
            run_command: format!("nub apps/web-frontend/angular/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },

        // 5. Web Metaframeworks
        TargetDescriptor {
            id: "nextjs".to_string(),
            name: "Next.js (App Router SSR)".to_string(),
            category: "Web Metaframework".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/metaframeworks/nextjs/main.ts".to_string()),
            run_command: format!("nub apps/metaframeworks/nextjs/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "nuxt".to_string(),
            name: "Nuxt (Nitro Engine)".to_string(),
            category: "Web Metaframework".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/metaframeworks/nuxt/main.ts".to_string()),
            run_command: format!("nub apps/metaframeworks/nuxt/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "sveltekit".to_string(),
            name: "SvelteKit (Adapter Engine)".to_string(),
            category: "Web Metaframework".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/metaframeworks/sveltekit/main.ts".to_string()),
            run_command: format!("nub apps/metaframeworks/sveltekit/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "astro".to_string(),
            name: "Astro (Islands Architecture)".to_string(),
            category: "Web Metaframework".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/metaframeworks/astro/main.ts".to_string()),
            run_command: format!("nub apps/metaframeworks/astro/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "tanstack-start".to_string(),
            name: "TanStack Start (Full-Stack SSR)".to_string(),
            category: "Web Metaframework".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/metaframeworks/tanstack-start/main.ts".to_string()),
            run_command: format!("nub apps/metaframeworks/tanstack-start/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
    ];

    println!("Benchmarking {} targets against Replay Log: {}\n", targets.len(), replay_path);

    let mut reports = Vec::new();
    for target in &targets {
        print!("⏳ Running {}... ", target.name.yellow());
        let report = execute_target_with_profiling(target, replay_path);
        if report.success {
            println!("{}", "DONE".green());
        } else {
            println!("{}", "FAILED".red());
        }
        reports.push(report);
    }

    render_terminal_table(&reports, is_stress);

    let (json_file, md_file) = if is_stress {
        ("benchmark-stress-results.json", "BENCHMARK_STRESS_RESULTS.md")
    } else {
        ("benchmark-results.json", "BENCHMARK_RESULTS.md")
    };

    export_results(&reports, replay_path, is_stress, json_file, md_file);
    println!("\n✅ Reports generated: {} & {}\n", json_file.bold(), md_file.bold());
}

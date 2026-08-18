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
        println!("{}", "🔥 Launching Extreme Multi-Core Saturation Stress Benchmark Suite".bold().red());
        println!("Mode: Multi-core saturation, Snapshot Tree retention (Iterations: {})\n", iterations);
    } else {
        println!("{}", "🚀 Launching Multi-Stack Application Stress Benchmark Suite".bold().cyan());
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
        TargetDescriptor {
            id: "rust-native-cli".to_string(),
            name: "Rust Native Binary (Compiled)".to_string(),
            category: "Native".to_string(),
            build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
            build_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
            run_command: format!("./crates/replay-engine/target/release/benchmark_replay --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "node-cli".to_string(),
            name: "Node / Strict TS7 CLI (nub)".to_string(),
            category: "CLI".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/cli-ts/src/main.ts".to_string()),
            run_command: format!("nub apps/cli-ts/src/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "python-cli".to_string(),
            name: "Python 3 CLI".to_string(),
            category: "CLI".to_string(),
            build_command: None,
            build_artifact_path: Some("src/targets/cli-python.py".to_string()),
            run_command: format!("python3 src/targets/cli-python.py --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "elysia-backend".to_string(),
            name: "ElysiaJS Web Backend (nub)".to_string(),
            category: "Web Backend".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/backend-elysia/src/main.ts".to_string()),
            run_command: format!("nub apps/backend-elysia/src/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "nextjs-ssr".to_string(),
            name: "Next.js SSR Metaframework (nub)".to_string(),
            category: "Metaframework".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/metaframework-ssr/src/main.ts".to_string()),
            run_command: format!("nub apps/metaframework-ssr/src/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
        },
        TargetDescriptor {
            id: "desktop-app".to_string(),
            name: "Desktop App (Tauri / Electron IPC) (nub)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("apps/desktop-app/src/main.ts".to_string()),
            run_command: format!("nub apps/desktop-app/src/main.ts --replay ${{REPLAY_PATH}} {}", extra_flags),
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

mod harness;
mod reporter;

use colored::*;
use harness::{execute_target_with_profiling, TargetDescriptor};
use reporter::{export_results, render_terminal_table};
use std::env;
use std::path::Path;

fn main() {
    println!("{}", "🚀 Launching Multi-Stack Application Stress Benchmark Suite".bold().cyan());

    let replay_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "92139349.json".to_string());

    if !Path::new(&replay_path).exists() {
        eprintln!("Error: Replay dataset '{}' not found.", replay_path);
        std::process::exit(1);
    }

    let targets = vec![
        TargetDescriptor {
            id: "rust-native-cli".to_string(),
            name: "Rust Native Binary (Compiled)".to_string(),
            category: "Native".to_string(),
            build_command: Some("cargo build --release --manifest-path crates/replay-engine/Cargo.toml".to_string()),
            build_artifact_path: Some("crates/replay-engine/target/release/benchmark_replay".to_string()),
            run_command: "./crates/replay-engine/target/release/benchmark_replay --replay ${REPLAY_PATH}".to_string(),
        },
        TargetDescriptor {
            id: "node-cli".to_string(),
            name: "Node / Strict TS7 CLI (nub)".to_string(),
            category: "CLI".to_string(),
            build_command: None,
            build_artifact_path: Some("src/targets/cli-ts.ts".to_string()),
            run_command: "nub src/targets/cli-ts.ts --replay ${REPLAY_PATH}".to_string(),
        },
        TargetDescriptor {
            id: "python-cli".to_string(),
            name: "Python 3 CLI".to_string(),
            category: "CLI".to_string(),
            build_command: None,
            build_artifact_path: Some("src/targets/cli-python.py".to_string()),
            run_command: "python3 src/targets/cli-python.py --replay ${REPLAY_PATH}".to_string(),
        },
        TargetDescriptor {
            id: "elysia-backend".to_string(),
            name: "ElysiaJS Web Backend (nub)".to_string(),
            category: "Web Backend".to_string(),
            build_command: None,
            build_artifact_path: Some("src/targets/backend-elysia.ts".to_string()),
            run_command: "nub src/targets/backend-elysia.ts --replay ${REPLAY_PATH}".to_string(),
        },
        TargetDescriptor {
            id: "nextjs-ssr".to_string(),
            name: "Next.js SSR Metaframework (nub)".to_string(),
            category: "Metaframework".to_string(),
            build_command: None,
            build_artifact_path: Some("src/targets/metaframework-ssr.ts".to_string()),
            run_command: "nub src/targets/metaframework-ssr.ts --replay ${REPLAY_PATH}".to_string(),
        },
        TargetDescriptor {
            id: "desktop-app".to_string(),
            name: "Desktop App (Tauri / Electron IPC) (nub)".to_string(),
            category: "Desktop".to_string(),
            build_command: None,
            build_artifact_path: Some("src/targets/desktop-app.ts".to_string()),
            run_command: "nub src/targets/desktop-app.ts --replay ${REPLAY_PATH}".to_string(),
        },
    ];

    println!("Benchmarking {} targets against dataset: {}\n", targets.len(), replay_path);

    let mut reports = Vec::new();
    for target in &targets {
        print!("⏳ Running {}... ", target.name.yellow());
        let report = execute_target_with_profiling(target, &replay_path);
        if report.success {
            println!("{}", "DONE".green());
        } else {
            println!("{}", "FAILED".red());
        }
        reports.push(report);
    }

    render_terminal_table(&reports);
    export_results(&reports, &replay_path, "benchmark-results.json", "BENCHMARK_RESULTS.md");
    println!("\n✅ Reports generated: {} & {}\n", "benchmark-results.json".bold(), "BENCHMARK_RESULTS.md".bold());
}

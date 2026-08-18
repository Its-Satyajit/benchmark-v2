use shared_replay_core::{simulate_replay_engine, ReplayLog};
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let replay_idx = args.iter().position(|r| r == "--replay");
    let replay_path = match replay_idx {
        Some(idx) => &args[idx + 1],
        None => {
            eprintln!("Error: Missing --replay <path>");
            std::process::exit(1);
        }
    };

    let parse_start = Instant::now();
    let file_content = fs::read_to_string(replay_path).expect("Failed to read replay file");
    let replay: ReplayLog = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    let parse_duration_ms = parse_start.elapsed().as_secs_f64() * 1000.0;

    let output = simulate_replay_engine(&replay, parse_duration_ms, "rust-native-cli");
    println!("{}", serde_json::to_string(&output).unwrap());
}

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct Configuration {
    seed: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct Card {
    id: Option<i64>,
    serial: Option<i64>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Player {
    #[serde(default)]
    active: Vec<serde_json::Value>,
    #[serde(default)]
    bench: Vec<serde_json::Value>,
    #[serde(default)]
    deck: Option<Vec<Card>>,
    #[serde(default)]
    hand: Option<Vec<Card>>,
}

#[derive(Debug, Deserialize)]
struct CurrentState {
    #[serde(default)]
    players: Vec<Player>,
}

#[derive(Debug, Deserialize)]
struct StepObservation {
    current: Option<CurrentState>,
    step: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct StepItem {
    action: Option<Vec<serde_json::Value>>,
    observation: Option<StepObservation>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ReplayLog {
    name: String,
    schema_version: i32,
    configuration: Option<Configuration>,
    steps: Vec<Vec<StepItem>>,
}

#[derive(Debug, Serialize)]
struct BenchmarkOutput {
    target: String,
    steps_processed: usize,
    parse_duration_ms: f64,
    replay_duration_ms: f64,
    total_duration_ms: f64,
    steps_per_sec: f64,
    checksum: String,
}

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

    let replay_start = Instant::now();
    let mut hasher = Sha256::new();

    if let Some(ref config) = replay.configuration {
        if let Some(seed) = config.seed {
            hasher.update(format!("seed:{};", seed));
        }
    }

    let mut steps_processed = 0;
    let mut total_cards = 0;
    let mut action_transitions = 0;

    for (i, step_batch) in replay.steps.iter().enumerate() {
        steps_processed += 1;
        for (j, step) in step_batch.iter().enumerate() {
            if let Some(ref act) = step.action {
                if !act.is_empty() {
                    action_transitions += act.len();
                    hasher.update(format!("act:{}:{}:{};", i, j, serde_json::to_string(act).unwrap()));
                }
            }

            if let Some(ref obs) = step.observation {
                if let Some(ref current) = obs.current {
                    for (p, player) in current.players.iter().enumerate() {
                        let deck_len = player.deck.as_ref().map(|d| d.len()).unwrap_or(0);
                        let hand_len = player.hand.as_ref().map(|h| h.len()).unwrap_or(0);
                        total_cards += deck_len + hand_len;
                        hasher.update(format!(
                            "p:{}:d{}:h{}:a{}:b{};",
                            p,
                            deck_len,
                            hand_len,
                            player.active.len(),
                            player.bench.len()
                        ));
                    }
                }
            }

            if let Some(ref st) = step.status {
                hasher.update(format!("st:{};", st));
            }
        }
    }

    hasher.update(format!(
        "final:steps={}:cards={}:acts={}",
        steps_processed, total_cards, action_transitions
    ));

    let checksum = hex::encode(hasher.finalize());
    let replay_duration_ms = replay_start.elapsed().as_secs_f64() * 1000.0;
    let total_duration_ms = parse_duration_ms + replay_duration_ms;
    let steps_per_sec = (steps_processed as f64 / (total_duration_ms / 1000.0)).max(0.0);

    let output = BenchmarkOutput {
        target: "rust-native-cli".to_string(),
        steps_processed,
        parse_duration_ms: (parse_duration_ms * 100.0).round() / 100.0,
        replay_duration_ms: (replay_duration_ms * 100.0).round() / 100.0,
        total_duration_ms: (total_duration_ms * 100.0).round() / 100.0,
        steps_per_sec: (steps_per_sec * 100.0).round() / 100.0,
        checksum,
    };

    println!("{}", serde_json::to_string(&output).unwrap());
}

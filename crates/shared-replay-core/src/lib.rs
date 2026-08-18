use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub seed: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Card {
    pub id: Option<i64>,
    pub serial: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    #[serde(default)]
    pub active: Vec<serde_json::Value>,
    #[serde(default)]
    pub bench: Vec<serde_json::Value>,
    #[serde(default)]
    pub deck: Option<Vec<Card>>,
    #[serde(default)]
    pub hand: Option<Vec<Card>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurrentState {
    #[serde(default)]
    pub players: Vec<Player>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StepObservation {
    pub current: Option<CurrentState>,
    pub step: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StepItem {
    pub action: Option<Vec<serde_json::Value>>,
    pub observation: Option<StepObservation>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReplayLog {
    pub name: String,
    pub schema_version: i32,
    pub configuration: Option<Configuration>,
    pub steps: Vec<Vec<StepItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkOutput {
    pub target: String,
    pub steps_processed: usize,
    pub parse_duration_ms: f64,
    pub replay_duration_ms: f64,
    pub total_duration_ms: f64,
    pub steps_per_sec: f64,
    pub checksum: String,
}

pub fn simulate_replay_engine(
    replay: &ReplayLog,
    parse_duration_ms: f64,
    target_name: &str,
) -> BenchmarkOutput {
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

    BenchmarkOutput {
        target: target_name.to_string(),
        steps_processed,
        parse_duration_ms: (parse_duration_ms * 100.0).round() / 100.0,
        replay_duration_ms: (replay_duration_ms * 100.0).round() / 100.0,
        total_duration_ms: (total_duration_ms * 100.0).round() / 100.0,
        steps_per_sec: (steps_per_sec * 100.0).round() / 100.0,
        checksum,
    }
}

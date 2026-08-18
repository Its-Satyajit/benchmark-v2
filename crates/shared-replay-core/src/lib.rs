use rayon::prelude::*;
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
    pub snapshots_retained: Option<usize>,
    pub p50_latency_ms: Option<f64>,
    pub p95_latency_ms: Option<f64>,
    pub p99_latency_ms: Option<f64>,
    pub total_frames_rendered: Option<usize>,
    pub avg_fps: Option<f64>,
    pub one_percent_low_fps: Option<f64>,
    pub zero_point_one_percent_low_fps: Option<f64>,
    pub jank_frame_count: Option<usize>,
    pub jank_percentage: Option<f64>,
    pub max_frame_time_ms: Option<f64>,
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
    let steps_per_sec = (steps_processed as f64 / (replay_duration_ms / 1000.0)).max(0.0);

    BenchmarkOutput {
        target: target_name.to_string(),
        steps_processed,
        parse_duration_ms: (parse_duration_ms * 100.0).round() / 100.0,
        replay_duration_ms: (replay_duration_ms * 100.0).round() / 100.0,
        total_duration_ms: (total_duration_ms * 100.0).round() / 100.0,
        steps_per_sec: (steps_per_sec * 100.0).round() / 100.0,
        checksum,
        snapshots_retained: None,
        p50_latency_ms: None,
        p95_latency_ms: None,
        p99_latency_ms: None,
        total_frames_rendered: None,
        avg_fps: None,
        one_percent_low_fps: None,
        zero_point_one_percent_low_fps: None,
        jank_frame_count: None,
        jank_percentage: None,
        max_frame_time_ms: None,
    }
}

pub fn simulate_gui_jank_replay_engine(
    replay: &ReplayLog,
    parse_duration_ms: f64,
    iterations: usize,
    target_name: &str,
) -> BenchmarkOutput {
    let start_time = Instant::now();
    let mut hasher = Sha256::new();
    let mut frame_times_ms: Vec<f64> = Vec::new();
    let vsync_budget_ms = 16.667;

    let mut total_steps = 0;
    let mut total_cards = 0;
    let mut total_acts = 0;

    for it in 0..iterations {
        hasher.update(format!("iter:{};", it));
        if let Some(ref config) = replay.configuration {
            if let Some(seed) = config.seed {
                hasher.update(format!("seed:{};", seed));
            }
        }

        for (i, step_batch) in replay.steps.iter().enumerate() {
            let frame_start = Instant::now();
            total_steps += 1;

            for (j, step) in step_batch.iter().enumerate() {
                if let Some(ref act) = step.action {
                    if !act.is_empty() {
                        total_acts += act.len();
                        hasher.update(format!("act:{}:{}:{}:{};", it, i, j, serde_json::to_string(act).unwrap()));
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

            frame_times_ms.push(frame_start.elapsed().as_secs_f64() * 1000.0);
        }
    }

    hasher.update(format!("final_gui:steps={}:cards={}:acts={}", total_steps, total_cards, total_acts));
    let checksum = hex::encode(hasher.finalize());

    let total_duration_ms = start_time.elapsed().as_secs_f64() * 1000.0;
    let mut jank_count = 0;
    let mut max_frame_time = 0.0;

    for &ft in &frame_times_ms {
        if ft > vsync_budget_ms {
            jank_count += 1;
        }
        if ft > max_frame_time {
            max_frame_time = ft;
        }
    }

    let mut sorted_desc = frame_times_ms.clone();
    sorted_desc.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let one_pct_idx = (sorted_desc.len() as f64 * 0.01) as usize;
    let zero_point_one_idx = (sorted_desc.len() as f64 * 0.001) as usize;

    let one_pct_ms = sorted_desc.get(one_pct_idx).copied().unwrap_or(0.001).max(0.001);
    let zero_point_one_ms = sorted_desc.get(zero_point_one_idx).copied().unwrap_or(0.001).max(0.001);

    let avg_fps = (frame_times_ms.len() as f64) / (total_duration_ms / 1000.0);
    let one_pct_fps = 1000.0 / one_pct_ms;
    let zero_point_one_fps = 1000.0 / zero_point_one_ms;
    let jank_pct = (jank_count as f64 / frame_times_ms.len().max(1) as f64) * 100.0;
    let steps_per_sec = (total_steps as f64 / (total_duration_ms / 1000.0)).max(0.0);

    BenchmarkOutput {
        target: target_name.to_string(),
        steps_processed: total_steps,
        parse_duration_ms: (parse_duration_ms * 100.0).round() / 100.0,
        replay_duration_ms: (total_duration_ms * 100.0).round() / 100.0,
        total_duration_ms: (total_duration_ms * 100.0).round() / 100.0,
        steps_per_sec: (steps_per_sec * 100.0).round() / 100.0,
        checksum,
        snapshots_retained: None,
        p50_latency_ms: None,
        p95_latency_ms: None,
        p99_latency_ms: None,
        total_frames_rendered: Some(frame_times_ms.len()),
        avg_fps: Some((avg_fps * 10.0).round() / 10.0),
        one_percent_low_fps: Some((one_pct_fps * 10.0).round() / 10.0),
        zero_point_one_percent_low_fps: Some((zero_point_one_fps * 10.0).round() / 10.0),
        jank_frame_count: Some(jank_count),
        jank_percentage: Some((jank_pct * 100.0).round() / 100.0),
        max_frame_time_ms: Some((max_frame_time * 100.0).round() / 100.0),
    }
}

pub fn simulate_stress_replay_engine(
    replay: &ReplayLog,
    parse_duration_ms: f64,
    iterations: usize,
    retain_snapshots: bool,
    target_name: &str,
) -> BenchmarkOutput {
    let start_time = Instant::now();

    let batch_results: Vec<(usize, usize, usize, String, Vec<f64>, usize)> = (0..iterations)
        .into_par_iter()
        .map(|it| {
            let mut hasher = Sha256::new();
            hasher.update(format!("iter:{};", it));

            if let Some(ref config) = replay.configuration {
                if let Some(seed) = config.seed {
                    hasher.update(format!("seed:{};", seed));
                }
            }

            let mut steps_count = 0;
            let mut cards_count = 0;
            let mut acts_count = 0;
            let mut latencies = Vec::with_capacity(replay.steps.len());
            let mut snapshot_tree: Vec<Vec<Player>> = Vec::new();

            for (i, step_batch) in replay.steps.iter().enumerate() {
                let step_start = Instant::now();
                steps_count += 1;

                for (j, step) in step_batch.iter().enumerate() {
                    if let Some(ref act) = step.action {
                        if !act.is_empty() {
                            acts_count += act.len();
                            hasher.update(format!("act:{}:{}:{}:{};", it, i, j, serde_json::to_string(act).unwrap()));
                        }
                    }

                    if let Some(ref obs) = step.observation {
                        if let Some(ref current) = obs.current {
                            if retain_snapshots {
                                snapshot_tree.push(current.players.clone());
                            }

                            for (p, player) in current.players.iter().enumerate() {
                                let deck_len = player.deck.as_ref().map(|d| d.len()).unwrap_or(0);
                                let hand_len = player.hand.as_ref().map(|h| h.len()).unwrap_or(0);
                                cards_count += deck_len + hand_len;
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

                latencies.push(step_start.elapsed().as_secs_f64() * 1000.0);
            }

            let sub_hash = hex::encode(hasher.finalize());
            (steps_count, cards_count, acts_count, sub_hash, latencies, snapshot_tree.len())
        })
        .collect();

    let mut master_hasher = Sha256::new();
    let mut total_steps = 0;
    let mut total_cards = 0;
    let mut total_acts = 0;
    let mut all_latencies = Vec::new();
    let mut total_snapshots = 0;

    for (s, c, a, sub_hash, mut lats, snaps) in batch_results {
        total_steps += s;
        total_cards += c;
        total_acts += a;
        total_snapshots += snaps;
        master_hasher.update(format!("batch:{};", sub_hash));
        all_latencies.append(&mut lats);
    }

    master_hasher.update(format!("final_stress:steps={}:cards={}:acts={}", total_steps, total_cards, total_acts));
    let checksum = hex::encode(master_hasher.finalize());

    let replay_duration_ms = start_time.elapsed().as_secs_f64() * 1000.0;
    let total_duration_ms = parse_duration_ms + replay_duration_ms;
    let steps_per_sec = (total_steps as f64 / (replay_duration_ms / 1000.0)).max(0.0);

    all_latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50 = all_latencies.get(all_latencies.len() / 2).copied().unwrap_or(0.0);
    let p95 = all_latencies.get((all_latencies.len() as f64 * 0.95) as usize).copied().unwrap_or(0.0);
    let p99 = all_latencies.get((all_latencies.len() as f64 * 0.99) as usize).copied().unwrap_or(0.0);

    BenchmarkOutput {
        target: target_name.to_string(),
        steps_processed: total_steps,
        parse_duration_ms: (parse_duration_ms * 100.0).round() / 100.0,
        replay_duration_ms: (replay_duration_ms * 100.0).round() / 100.0,
        total_duration_ms: (total_duration_ms * 100.0).round() / 100.0,
        steps_per_sec: (steps_per_sec * 100.0).round() / 100.0,
        checksum,
        snapshots_retained: Some(total_snapshots),
        p50_latency_ms: Some((p50 * 1000.0).round() / 1000.0),
        p95_latency_ms: Some((p95 * 1000.0).round() / 1000.0),
        p99_latency_ms: Some((p99 * 1000.0).round() / 1000.0),
        total_frames_rendered: None,
        avg_fps: None,
        one_percent_low_fps: None,
        zero_point_one_percent_low_fps: None,
        jank_frame_count: None,
        jank_percentage: None,
        max_frame_time_ms: None,
    }
}

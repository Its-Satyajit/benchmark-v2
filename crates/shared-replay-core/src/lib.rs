use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub seed: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    #[serde(default)]
    pub active: Vec<serde_json::Value>,
    #[serde(default)]
    pub bench: Vec<serde_json::Value>,
    #[serde(default)]
    pub deck: Option<serde_json::Value>,
    #[serde(default)]
    pub hand: Option<serde_json::Value>,
}

impl Player {
    pub fn deck_len(&self) -> usize {
        match &self.deck {
            Some(serde_json::Value::Array(arr)) => arr.len(),
            _ => 0,
        }
    }

    pub fn hand_len(&self) -> usize {
        match &self.hand {
            Some(serde_json::Value::Array(arr)) => arr.len(),
            _ => 0,
        }
    }
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
pub struct RawIterationTelemetry {
    pub iteration: usize,
    pub wall_time_ms: f64,
    pub steps_processed: usize,
    pub steps_per_sec: f64,
    pub peak_rss_bytes: u64,
    pub checksum: String,
    pub raw_step_latencies_ms: Option<Vec<f64>>,
    pub raw_frame_times_ms: Option<Vec<f64>>,
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
    pub raw_iterations: Option<Vec<RawIterationTelemetry>>,
}

pub fn simulate_replay_engine(
    replay: &ReplayLog,
    parse_duration_ms: f64,
    target_name: &str,
    iterations: usize,
) -> BenchmarkOutput {
    let replay_start = Instant::now();
    let iters = if iterations == 0 { 1 } else { iterations };
    let mut raw_iterations = Vec::new();
    let mut total_steps_processed = 0;
    let mut final_checksum = String::new();

    for it in 0..iters {
        let iter_start = Instant::now();
        let mut hasher = Sha256::new();
        let mut iter_latencies = Vec::new();

        if let Some(ref config) = replay.configuration {
            if let Some(seed) = config.seed {
                hasher.update(format!("seed:{};", seed));
            }
        }

        let mut steps_processed = 0;
        let mut total_cards = 0;
        let mut action_transitions = 0;

        for (i, step_batch) in replay.steps.iter().enumerate() {
            let step_start = Instant::now();
            steps_processed += 1;
            for (j, step) in step_batch.iter().enumerate() {
                if let Some(ref act) = step.action {
                    if !act.is_empty() {
                        action_transitions += act.len();
                        if let Ok(act_str) = serde_json::to_string(act) {
                            hasher.update(format!("act:{}:{}:{};", i, j, act_str));
                        }
                    }
                }

                if let Some(ref obs) = step.observation {
                    if let Some(ref current) = obs.current {
                        for (p, player) in current.players.iter().enumerate() {
                            let d_len = player.deck_len();
                            let h_len = player.hand_len();
                            let a_len = player.active.len();
                            let b_len = player.bench.len();
                            total_cards += d_len + h_len;
                            hasher.update(format!("p:{}:d{}:h{}:a{}:b{};", p, d_len, h_len, a_len, b_len));
                        }
                    }
                }

                if let Some(ref status) = step.status {
                    hasher.update(format!("st:{};", status));
                }
            }
            iter_latencies.push(step_start.elapsed().as_secs_f64() * 1000.0);
        }

        hasher.update(format!("final:steps={}:cards={}:acts={}", steps_processed, total_cards, action_transitions));
        let iter_checksum = hex::encode(hasher.finalize());
        final_checksum = iter_checksum.clone();
        let iter_duration_ms = (iter_start.elapsed().as_secs_f64() * 1000.0).max(0.001);
        total_steps_processed += steps_processed;

        raw_iterations.push(RawIterationTelemetry {
            iteration: it + 1,
            wall_time_ms: (iter_duration_ms * 1000.0).round() / 1000.0,
            steps_processed,
            steps_per_sec: ((steps_processed as f64) / (iter_duration_ms / 1000.0) * 100.0).round() / 100.0,
            peak_rss_bytes: 0,
            checksum: iter_checksum,
            raw_step_latencies_ms: Some(iter_latencies),
            raw_frame_times_ms: None,
        });
    }

    let replay_duration_ms = replay_start.elapsed().as_secs_f64() * 1000.0;
    let total_duration_ms = parse_duration_ms + replay_duration_ms;
    let steps_per_sec = (total_steps_processed as f64) / (replay_duration_ms / 1000.0);

    BenchmarkOutput {
        target: target_name.to_string(),
        steps_processed: total_steps_processed,
        parse_duration_ms,
        replay_duration_ms,
        total_duration_ms,
        steps_per_sec,
        checksum: final_checksum,
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
        raw_iterations: Some(raw_iterations),
    }
}

pub fn simulate_stress_replay_engine(
    replay: &ReplayLog,
    parse_duration_ms: f64,
    target_name: &str,
    iterations: usize,
) -> BenchmarkOutput {
    let replay_start = Instant::now();
    let mut hasher = Sha256::new();
    let mut latencies: Vec<f64> = Vec::new();
    let mut snapshot_tree: Vec<Vec<Player>> = Vec::new();
    let mut raw_iterations = Vec::new();

    let mut total_steps = 0;
    let mut total_cards = 0;
    let mut total_acts = 0;

    for it in 0..iterations {
        let iter_start = Instant::now();
        let mut iter_latencies = Vec::new();
        hasher.update(format!("iter:{};", it));

        if let Some(ref config) = replay.configuration {
            if let Some(seed) = config.seed {
                hasher.update(format!("seed:{};", seed));
            }
        }

        let mut iter_steps = 0;
        for (i, step_batch) in replay.steps.iter().enumerate() {
            let step_start = Instant::now();
            total_steps += 1;
            iter_steps += 1;

            for (j, step) in step_batch.iter().enumerate() {
                if let Some(ref act) = step.action {
                    total_acts += act.len();
                    if let Ok(act_str) = serde_json::to_string(act) {
                        hasher.update(format!("act:{}:{}:{}:{};", it, i, j, act_str));
                    }
                }

                if let Some(ref obs) = step.observation {
                    if let Some(ref current) = obs.current {
                        snapshot_tree.push(current.players.clone());
                        for (p, player) in current.players.iter().enumerate() {
                            let d_len = player.deck_len();
                            let h_len = player.hand_len();
                            let a_len = player.active.len();
                            let b_len = player.bench.len();
                            total_cards += d_len + h_len;
                            hasher.update(format!("p:{}:d{}:h{}:a{}:b{};", p, d_len, h_len, a_len, b_len));
                        }
                    }
                }

                if let Some(ref status) = step.status {
                    hasher.update(format!("st:{};", status));
                }
            }

            let lat = step_start.elapsed().as_secs_f64() * 1000.0;
            iter_latencies.push(lat);
            latencies.push(lat);
        }

        let iter_duration_ms = (iter_start.elapsed().as_secs_f64() * 1000.0).max(0.001);
        raw_iterations.push(RawIterationTelemetry {
            iteration: it + 1,
            wall_time_ms: (iter_duration_ms * 1000.0).round() / 1000.0,
            steps_processed: iter_steps,
            steps_per_sec: ((iter_steps as f64) / (iter_duration_ms / 1000.0) * 100.0).round() / 100.0,
            peak_rss_bytes: 0,
            checksum: String::new(),
            raw_step_latencies_ms: Some(iter_latencies),
            raw_frame_times_ms: None,
        });
    }

    hasher.update(format!("final_stress:steps={}:cards={}:acts={}", total_steps, total_cards, total_acts));
    let checksum = hex::encode(hasher.finalize());

    for raw in &mut raw_iterations {
        raw.checksum = checksum.clone();
    }

    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p50 = latencies.get((latencies.len() as f64 * 0.50) as usize).copied().unwrap_or(0.0);
    let p95 = latencies.get((latencies.len() as f64 * 0.95) as usize).copied().unwrap_or(0.0);
    let p99 = latencies.get((latencies.len() as f64 * 0.99) as usize).copied().unwrap_or(0.0);

    let replay_duration_ms = replay_start.elapsed().as_secs_f64() * 1000.0;
    let total_duration_ms = parse_duration_ms + replay_duration_ms;
    let steps_per_sec = (total_steps as f64) / (replay_duration_ms / 1000.0);

    BenchmarkOutput {
        target: target_name.to_string(),
        steps_processed: total_steps,
        parse_duration_ms,
        replay_duration_ms,
        total_duration_ms,
        steps_per_sec,
        checksum,
        snapshots_retained: Some(snapshot_tree.len()),
        p50_latency_ms: Some(p50),
        p95_latency_ms: Some(p95),
        p99_latency_ms: Some(p99),
        total_frames_rendered: None,
        avg_fps: None,
        one_percent_low_fps: None,
        zero_point_one_percent_low_fps: None,
        jank_frame_count: None,
        jank_percentage: None,
        max_frame_time_ms: None,
        raw_iterations: Some(raw_iterations),
    }
}

pub fn simulate_gui_jank_replay_engine(
    replay: &ReplayLog,
    parse_duration_ms: f64,
    target_name: &str,
    iterations: usize,
) -> BenchmarkOutput {
    let replay_start = Instant::now();
    let mut hasher = Sha256::new();
    let mut frame_times: Vec<f64> = Vec::new();
    let mut raw_iterations = Vec::new();

    let mut total_steps = 0;
    let mut total_cards = 0;
    let mut total_acts = 0;
    let mut jank_frame_count = 0;
    let mut max_frame_time_ms = 0.0;

    for it in 0..iterations {
        let iter_start = Instant::now();
        let mut iter_frames = Vec::new();
        hasher.update(format!("iter:{};", it));

        if let Some(ref config) = replay.configuration {
            if let Some(seed) = config.seed {
                hasher.update(format!("seed:{};", seed));
            }
        }

        let mut iter_steps = 0;
        for (i, step_batch) in replay.steps.iter().enumerate() {
            let frame_start = Instant::now();
            total_steps += 1;
            iter_steps += 1;

            for (j, step) in step_batch.iter().enumerate() {
                if let Some(ref act) = step.action {
                    total_acts += act.len();
                    if let Ok(act_str) = serde_json::to_string(act) {
                        hasher.update(format!("act:{}:{}:{}:{};", it, i, j, act_str));
                    }
                }

                if let Some(ref obs) = step.observation {
                    if let Some(ref current) = obs.current {
                        for (p, player) in current.players.iter().enumerate() {
                            let d_len = player.deck_len();
                            let h_len = player.hand_len();
                            let a_len = player.active.len();
                            let b_len = player.bench.len();
                            total_cards += d_len + h_len;
                            hasher.update(format!("p:{}:d{}:h{}:a{}:b{};", p, d_len, h_len, a_len, b_len));
                        }
                    }
                }

                if let Some(ref status) = step.status {
                    hasher.update(format!("st:{};", status));
                }
            }

            let frame_duration = frame_start.elapsed().as_secs_f64() * 1000.0;
            frame_times.push(frame_duration);
            iter_frames.push(frame_duration);

            if frame_duration > 16.667 {
                jank_frame_count += 1;
            }
            if frame_duration > max_frame_time_ms {
                max_frame_time_ms = frame_duration;
            }
        }

        let iter_duration_ms = (iter_start.elapsed().as_secs_f64() * 1000.0).max(0.001);
        raw_iterations.push(RawIterationTelemetry {
            iteration: it + 1,
            wall_time_ms: (iter_duration_ms * 1000.0).round() / 1000.0,
            steps_processed: iter_steps,
            steps_per_sec: ((iter_steps as f64) / (iter_duration_ms / 1000.0) * 100.0).round() / 100.0,
            peak_rss_bytes: 0,
            checksum: String::new(),
            raw_step_latencies_ms: None,
            raw_frame_times_ms: Some(iter_frames),
        });
    }

    hasher.update(format!("final_gui:steps={}:cards={}:acts={}", total_steps, total_cards, total_acts));
    let checksum = hex::encode(hasher.finalize());

    for raw in &mut raw_iterations {
        raw.checksum = checksum.clone();
    }

    let replay_duration_ms = replay_start.elapsed().as_secs_f64() * 1000.0;
    let total_duration_ms = parse_duration_ms + replay_duration_ms;
    let total_frames = frame_times.len();
    let avg_fps = (total_frames as f64) / (replay_duration_ms / 1000.0);

    frame_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let one_pct_idx = (frame_times.len() as f64 * 0.99) as usize;
    let zero_point_one_idx = (frame_times.len() as f64 * 0.999) as usize;
    let one_pct_ms = frame_times.get(one_pct_idx).copied().unwrap_or(0.001).max(0.001);
    let zero_point_one_ms = frame_times.get(zero_point_one_idx).copied().unwrap_or(0.001).max(0.001);

    let one_pct_fps = 1000.0 / one_pct_ms;
    let zero_point_one_fps = 1000.0 / zero_point_one_ms;
    let jank_pct = ((jank_frame_count as f64) / (total_frames.max(1) as f64)) * 100.0;

    BenchmarkOutput {
        target: target_name.to_string(),
        steps_processed: total_steps,
        parse_duration_ms,
        replay_duration_ms,
        total_duration_ms,
        steps_per_sec: avg_fps,
        checksum,
        snapshots_retained: None,
        p50_latency_ms: None,
        p95_latency_ms: None,
        p99_latency_ms: None,
        total_frames_rendered: Some(total_frames),
        avg_fps: Some(avg_fps),
        one_percent_low_fps: Some(one_pct_fps),
        zero_point_one_percent_low_fps: Some(zero_point_one_fps),
        jank_frame_count: Some(jank_frame_count),
        jank_percentage: Some(jank_pct),
        max_frame_time_ms: Some(max_frame_time_ms),
        raw_iterations: Some(raw_iterations),
    }
}

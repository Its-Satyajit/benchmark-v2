use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDescriptor {
    pub id: String,
    pub name: String,
    pub category: String,
    pub build_command: Option<String>,
    pub build_artifact_path: Option<String>,
    pub run_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetBenchmarkResult {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetBenchmarkReport {
    pub target_id: String,
    pub target_name: String,
    pub category: String,
    pub build_duration_ms: f64,
    pub artifact_size_bytes: u64,
    pub total_wall_time_ms: f64,
    pub peak_rss_bytes: u64,
    pub success: bool,
    pub error: Option<String>,
    pub metrics: Option<TargetBenchmarkResult>,
}

pub fn calculate_artifact_size(path_str: &str) -> u64 {
    let path = Path::new(path_str);
    if !path.exists() {
        return 0;
    }

    if path.is_file() {
        return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    }

    let mut total: u64 = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }
    total
}

fn get_process_rss(pid: u32) -> u64 {
    let statm_path = format!("/proc/{}/statm", pid);
    if let Ok(mut file) = fs::File::open(&statm_path) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(rss_pages) = parts[1].parse::<u64>() {
                    return rss_pages * 4096; // 4KB page size
                }
            }
        }
    }
    0
}

pub fn execute_target_with_profiling(
    descriptor: &TargetDescriptor,
    replay_path: &str,
) -> TargetBenchmarkReport {
    let mut build_duration_ms = 0.0;

    // 1. Clean build profiling
    if let Some(ref build_cmd) = descriptor.build_command {
        let build_start = Instant::now();
        let status = Command::new("sh")
            .arg("-c")
            .arg(build_cmd)
            .status();

        match status {
            Ok(s) if s.success() => {
                build_duration_ms = build_start.elapsed().as_secs_f64() * 1000.0;
            }
            Ok(s) => {
                return TargetBenchmarkReport {
                    target_id: descriptor.id.clone(),
                    target_name: descriptor.name.clone(),
                    category: descriptor.category.clone(),
                    build_duration_ms: 0.0,
                    artifact_size_bytes: 0,
                    total_wall_time_ms: 0.0,
                    peak_rss_bytes: 0,
                    success: false,
                    error: Some(format!("Build failed with exit status: {}", s)),
                    metrics: None,
                };
            }
            Err(e) => {
                return TargetBenchmarkReport {
                    target_id: descriptor.id.clone(),
                    target_name: descriptor.name.clone(),
                    category: descriptor.category.clone(),
                    build_duration_ms: 0.0,
                    artifact_size_bytes: 0,
                    total_wall_time_ms: 0.0,
                    peak_rss_bytes: 0,
                    success: false,
                    error: Some(format!("Failed to spawn build command: {}", e)),
                    metrics: None,
                };
            }
        }
    }

    // 2. Measure Artifact Size
    let artifact_size_bytes = descriptor
        .build_artifact_path
        .as_ref()
        .map(|p| calculate_artifact_size(p))
        .unwrap_or(0);

    // 3. Execution & Memory Sampling Phase
    let run_cmd = descriptor
        .run_command
        .replace("${REPLAY_PATH}", replay_path);

    let parts: Vec<&str> = run_cmd.split_whitespace().collect();
    if parts.is_empty() {
        return TargetBenchmarkReport {
            target_id: descriptor.id.clone(),
            target_name: descriptor.name.clone(),
            category: descriptor.category.clone(),
            build_duration_ms,
            artifact_size_bytes,
            total_wall_time_ms: 0.0,
            peak_rss_bytes: 0,
            success: false,
            error: Some("Empty run command".to_string()),
            metrics: None,
        };
    }

    let run_start = Instant::now();
    let child = match Command::new(parts[0])
        .args(&parts[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return TargetBenchmarkReport {
                target_id: descriptor.id.clone(),
                target_name: descriptor.name.clone(),
                category: descriptor.category.clone(),
                build_duration_ms,
                artifact_size_bytes,
                total_wall_time_ms: 0.0,
                peak_rss_bytes: 0,
                success: false,
                error: Some(format!("Failed to spawn target process: {}", e)),
                metrics: None,
            };
        }
    };

    let pid = child.id();
    let running = Arc::new(AtomicBool::new(true));
    let r_clone = running.clone();

    let sampler_thread = thread::spawn(move || {
        let mut max_rss: u64 = 0;
        while r_clone.load(Ordering::Relaxed) {
            let rss = get_process_rss(pid);
            if rss > max_rss {
                max_rss = rss;
            }
            thread::sleep(Duration::from_millis(2));
        }
        max_rss
    });

    let output = child.wait_with_output();
    running.store(false, Ordering::Relaxed);
    let peak_rss_bytes = sampler_thread.join().unwrap_or(0);
    let total_wall_time_ms = run_start.elapsed().as_secs_f64() * 1000.0;

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let last_line = stdout.lines().filter(|l| !l.trim().is_empty()).last().unwrap_or("{}");

            match serde_json::from_str::<TargetBenchmarkResult>(last_line) {
                Ok(metrics) => TargetBenchmarkReport {
                    target_id: descriptor.id.clone(),
                    target_name: descriptor.name.clone(),
                    category: descriptor.category.clone(),
                    build_duration_ms: (build_duration_ms * 100.0).round() / 100.0,
                    artifact_size_bytes,
                    total_wall_time_ms: (total_wall_time_ms * 100.0).round() / 100.0,
                    peak_rss_bytes,
                    success: true,
                    error: None,
                    metrics: Some(metrics),
                },
                Err(e) => TargetBenchmarkReport {
                    target_id: descriptor.id.clone(),
                    target_name: descriptor.name.clone(),
                    category: descriptor.category.clone(),
                    build_duration_ms: (build_duration_ms * 100.0).round() / 100.0,
                    artifact_size_bytes,
                    total_wall_time_ms: (total_wall_time_ms * 100.0).round() / 100.0,
                    peak_rss_bytes,
                    success: false,
                    error: Some(format!("Failed to parse metrics JSON: {} (Output: {})", e, stdout)),
                    metrics: None,
                },
            }
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            TargetBenchmarkReport {
                target_id: descriptor.id.clone(),
                target_name: descriptor.name.clone(),
                category: descriptor.category.clone(),
                build_duration_ms: (build_duration_ms * 100.0).round() / 100.0,
                artifact_size_bytes,
                total_wall_time_ms: (total_wall_time_ms * 100.0).round() / 100.0,
                peak_rss_bytes,
                success: false,
                error: Some(format!("Process exited with status {}: {}", out.status, stderr)),
                metrics: None,
            }
        }
        Err(e) => TargetBenchmarkReport {
            target_id: descriptor.id.clone(),
            target_name: descriptor.name.clone(),
            category: descriptor.category.clone(),
            build_duration_ms: (build_duration_ms * 100.0).round() / 100.0,
            artifact_size_bytes,
            total_wall_time_ms: (total_wall_time_ms * 100.0).round() / 100.0,
            peak_rss_bytes,
            success: false,
            error: Some(format!("Wait failed: {}", e)),
            metrics: None,
        },
    }
}

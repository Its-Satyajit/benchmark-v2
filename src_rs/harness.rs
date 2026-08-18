use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDescriptor {
    pub id: String,
    pub name: String,
    pub category: String,
    pub build_command: Option<String>,
    pub bundle_artifact_path: Option<String>,
    pub dist_artifact_path: Option<String>,
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
    pub total_frames_rendered: Option<usize>,
    pub avg_fps: Option<f64>,
    pub one_percent_low_fps: Option<f64>,
    pub zero_point_one_percent_low_fps: Option<f64>,
    pub jank_frame_count: Option<usize>,
    pub jank_percentage: Option<f64>,
    pub max_frame_time_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetBenchmarkReport {
    pub target_id: String,
    pub target_name: String,
    pub category: String,
    pub build_duration_ms: f64,
    pub bundle_size_bytes: u64,
    pub dist_size_bytes: u64,
    pub total_wall_time_ms: f64,
    pub peak_rss_bytes: u64,
    pub success: bool,
    pub error: Option<String>,
    pub metrics: Option<TargetBenchmarkResult>,
}

pub fn calculate_artifact_size<P: AsRef<Path>>(path: P) -> u64 {
    let path = path.as_ref();
    if !path.exists() {
        return 0;
    }

    if path.is_file() {
        return fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    }

    let mut total_size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                total_size += fs::metadata(&entry_path).map(|m| m.len()).unwrap_or(0);
            } else if entry_path.is_dir() {
                total_size += calculate_artifact_size(&entry_path);
            }
        }
    }
    total_size
}

pub fn read_proc_rss(pid: u32) -> u64 {
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
                    bundle_size_bytes: 0,
                    dist_size_bytes: 0,
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
                    bundle_size_bytes: 0,
                    dist_size_bytes: 0,
                    total_wall_time_ms: 0.0,
                    peak_rss_bytes: 0,
                    success: false,
                    error: Some(format!("Failed to spawn build command: {}", e)),
                    metrics: None,
                };
            }
        }
    }

    // 2. Measure Dual-Tier Artifact Sizes
    let bundle_size_bytes = descriptor
        .bundle_artifact_path
        .as_ref()
        .map(|p| calculate_artifact_size(p))
        .unwrap_or(0);

    let dist_size_bytes = descriptor
        .dist_artifact_path
        .as_ref()
        .map(|p| calculate_artifact_size(p))
        .unwrap_or(bundle_size_bytes);

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
            bundle_size_bytes,
            dist_size_bytes,
            total_wall_time_ms: 0.0,
            peak_rss_bytes: 0,
            success: false,
            error: Some("Empty run command".to_string()),
            metrics: None,
        };
    }

    let run_start = Instant::now();
    let child_res = Command::new(parts[0])
        .args(&parts[1..])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    let mut child: Child = match child_res {
        Ok(c) => c,
        Err(e) => {
            return TargetBenchmarkReport {
                target_id: descriptor.id.clone(),
                target_name: descriptor.name.clone(),
                category: descriptor.category.clone(),
                build_duration_ms,
                bundle_size_bytes,
                dist_size_bytes,
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

    // High-frequency memory polling thread
    let sampler_handle = thread::spawn(move || {
        let mut peak_rss: u64 = 0;
        while r_clone.load(Ordering::Relaxed) {
            let current_rss = read_proc_rss(pid);
            if current_rss > peak_rss {
                peak_rss = current_rss;
            }
            thread::sleep(Duration::from_millis(1));
        }
        let final_rss = read_proc_rss(pid);
        if final_rss > peak_rss {
            peak_rss = final_rss;
        }
        peak_rss
    });

    let output_res = child.wait_with_output();
    running.store(false, Ordering::Relaxed);

    let total_wall_time_ms = run_start.elapsed().as_secs_f64() * 1000.0;
    let peak_rss_bytes = sampler_handle.join().unwrap_or(0);

    match output_res {
        Ok(out) if out.status.success() => {
            let stdout_str = String::from_utf8_lossy(&out.stdout);
            let json_line = stdout_str
                .lines()
                .find(|l| l.trim().starts_with('{') && l.trim().ends_with('}'))
                .unwrap_or("");

            match serde_json::from_str::<TargetBenchmarkResult>(json_line) {
                Ok(metrics) => TargetBenchmarkReport {
                    target_id: descriptor.id.clone(),
                    target_name: descriptor.name.clone(),
                    category: descriptor.category.clone(),
                    build_duration_ms,
                    bundle_size_bytes,
                    dist_size_bytes,
                    total_wall_time_ms,
                    peak_rss_bytes,
                    success: true,
                    error: None,
                    metrics: Some(metrics),
                },
                Err(e) => TargetBenchmarkReport {
                    target_id: descriptor.id.clone(),
                    target_name: descriptor.name.clone(),
                    category: descriptor.category.clone(),
                    build_duration_ms,
                    bundle_size_bytes,
                    dist_size_bytes,
                    total_wall_time_ms,
                    peak_rss_bytes,
                    success: false,
                    error: Some(format!("Failed to parse benchmark JSON output: {}. Raw stdout: {}", e, stdout_str)),
                    metrics: None,
                },
            }
        }
        Ok(out) => {
            let stderr_str = String::from_utf8_lossy(&out.stderr);
            TargetBenchmarkReport {
                target_id: descriptor.id.clone(),
                target_name: descriptor.name.clone(),
                category: descriptor.category.clone(),
                build_duration_ms,
                bundle_size_bytes,
                dist_size_bytes,
                total_wall_time_ms,
                peak_rss_bytes,
                success: false,
                error: Some(format!("Process exited with code {}: {}", out.status.code().unwrap_or(-1), stderr_str)),
                metrics: None,
            }
        }
        Err(e) => TargetBenchmarkReport {
            target_id: descriptor.id.clone(),
            target_name: descriptor.name.clone(),
            category: descriptor.category.clone(),
            build_duration_ms,
            bundle_size_bytes,
            dist_size_bytes,
            total_wall_time_ms,
            peak_rss_bytes,
            success: false,
            error: Some(format!("Failed to wait for process: {}", e)),
            metrics: None,
        },
    }
}

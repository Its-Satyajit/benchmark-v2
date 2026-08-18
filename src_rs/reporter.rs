use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::harness::TargetBenchmarkReport;

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkSuiteResult {
    pub timestamp: String,
    pub mode: String,
    pub replay_file: String,
    pub reports: Vec<TargetBenchmarkReport>,
}

pub fn render_terminal_table(reports: &[TargetBenchmarkReport], is_stress: bool, is_gui: bool) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    if is_gui {
        table.set_header(vec![
            Cell::new("GUI Target Stack").fg(Color::Cyan),
            Cell::new("Category").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Avg FPS").fg(Color::Cyan),
            Cell::new("1% Low FPS").fg(Color::Cyan),
            Cell::new("Max Frame Time").fg(Color::Cyan),
            Cell::new("Jank Frames").fg(Color::Cyan),
            Cell::new("Dist (MB)").fg(Color::Cyan),
            Cell::new("Peak RSS (MB)").fg(Color::Cyan),
            Cell::new("Checksum").fg(Color::Cyan),
        ]);

        for r in reports {
            let status_cell = if r.success {
                Cell::new("PASS").fg(Color::Green)
            } else {
                Cell::new("FAIL").fg(Color::Red)
            };

            let (avg_fps, one_pct, max_ft, jank_cnt, checksum) = match &r.metrics {
                Some(m) => (
                    format!("{:.1}", m.avg_fps.unwrap_or(0.0)),
                    format!("{:.1}", m.one_percent_low_fps.unwrap_or(0.0)),
                    format!("{:.2} ms", m.max_frame_time_ms.unwrap_or(0.0)),
                    format!("{}", m.jank_frame_count.unwrap_or(0)),
                    m.checksum[..8].to_string(),
                ),
                None => ("N/A".to_string(), "N/A".to_string(), "N/A".to_string(), "N/A".to_string(), "N/A".to_string()),
            };

            let dist_mb = (r.dist_size_bytes as f64) / (1024.0 * 1024.0);
            let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

            table.add_row(vec![
                Cell::new(&r.target_name),
                Cell::new(&r.category),
                status_cell,
                Cell::new(avg_fps).fg(Color::Yellow),
                Cell::new(one_pct).fg(Color::Green),
                Cell::new(max_ft),
                Cell::new(jank_cnt),
                Cell::new(format!("{:.2}", dist_mb)),
                Cell::new(format!("{:.2}", rss_mb)),
                Cell::new(checksum),
            ]);
        }
    } else if is_stress {
        table.set_header(vec![
            Cell::new("Target Stack").fg(Color::Cyan),
            Cell::new("Category").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Throughput (steps/s)").fg(Color::Cyan),
            Cell::new("Wall Time (ms)").fg(Color::Cyan),
            Cell::new("Peak RSS (MB)").fg(Color::Cyan),
            Cell::new("P95 Latency (ms)").fg(Color::Cyan),
            Cell::new("Dist (MB)").fg(Color::Cyan),
            Cell::new("Checksum").fg(Color::Cyan),
        ]);

        for r in reports {
            let status_cell = if r.success {
                Cell::new("PASS").fg(Color::Green)
            } else {
                Cell::new("FAIL").fg(Color::Red)
            };

            let (steps_sec, p95_str, checksum) = match &r.metrics {
                Some(m) => (
                    format!("{:.1}", m.steps_per_sec),
                    format!("{:.3}", m.p95_latency_ms.unwrap_or(0.0)),
                    m.checksum[..8].to_string(),
                ),
                None => ("N/A".to_string(), "N/A".to_string(), "N/A".to_string()),
            };

            let dist_mb = (r.dist_size_bytes as f64) / (1024.0 * 1024.0);
            let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

            table.add_row(vec![
                Cell::new(&r.target_name),
                Cell::new(&r.category),
                status_cell,
                Cell::new(steps_sec).fg(Color::Yellow),
                Cell::new(format!("{:.1}", r.total_wall_time_ms)),
                Cell::new(format!("{:.2}", rss_mb)),
                Cell::new(p95_str),
                Cell::new(format!("{:.2}", dist_mb)),
                Cell::new(checksum),
            ]);
        }
    } else {
        table.set_header(vec![
            Cell::new("Target Stack").fg(Color::Cyan),
            Cell::new("Category").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Cold Build (s)").fg(Color::Cyan),
            Cell::new("Warm Build (ms)").fg(Color::Cyan),
            Cell::new("Bundle (KB)").fg(Color::Cyan),
            Cell::new("Dist (MB)").fg(Color::Cyan),
            Cell::new("Throughput (steps/s)").fg(Color::Cyan),
            Cell::new("Peak RSS (MB)").fg(Color::Cyan),
            Cell::new("Checksum").fg(Color::Cyan),
        ]);

        for r in reports {
            let status_cell = if r.success {
                Cell::new("PASS").fg(Color::Green)
            } else {
                Cell::new("FAIL").fg(Color::Red)
            };

            let (steps_sec, checksum) = match &r.metrics {
                Some(m) => (format!("{:.1}", m.steps_per_sec), m.checksum[..8].to_string()),
                None => ("N/A".to_string(), "N/A".to_string()),
            };

            let cold_s = r.cold_build_duration_ms / 1000.0;
            let bundle_kb = (r.bundle_size_bytes as f64) / 1024.0;
            let dist_mb = (r.dist_size_bytes as f64) / (1024.0 * 1024.0);
            let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

            table.add_row(vec![
                Cell::new(&r.target_name),
                Cell::new(&r.category),
                status_cell,
                Cell::new(format!("{:.2} s", cold_s)).fg(Color::Magenta),
                Cell::new(format!("{:.1} ms", r.warm_build_duration_ms)),
                Cell::new(format!("{:.1}", bundle_kb)),
                Cell::new(format!("{:.2}", dist_mb)).fg(Color::Yellow),
                Cell::new(steps_sec).fg(Color::Green),
                Cell::new(format!("{:.2}", rss_mb)),
                Cell::new(checksum),
            ]);
        }
    }

    println!("\n{}", table);
}

pub fn export_results(
    reports: &[TargetBenchmarkReport],
    replay_file: &str,
    is_stress: bool,
    is_gui: bool,
    json_path: &str,
    markdown_path: &str,
) {
    let mode_str = if is_gui {
        "GUI Jank & Frame Pacing Benchmark"
    } else if is_stress {
        "Extreme Multi-Core Saturation Stress"
    } else {
        "Baseline Stress Replay"
    };

    let suite = BenchmarkSuiteResult {
        timestamp: chrono_or_fallback(),
        mode: mode_str.to_string(),
        replay_file: replay_file.to_string(),
        reports: reports.to_vec(),
    };

    if let Ok(json_str) = serde_json::to_string_pretty(&suite) {
        let _ = fs::write(json_path, json_str);
    }

    let mut md = String::new();
    if is_gui {
        md.push_str("# GUI Jank & Frame Pacing Benchmark Results\n\n");
        md.push_str(&format!("**Replay Log**: `{}`\n\n", replay_file));
        md.push_str("| GUI Target Stack | Category | Status | Avg FPS | 1% Low FPS | Max Frame Time | Jank Frames | Dist (MB) | Peak RSS (MB) | Checksum |\n");
        md.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");

        for r in reports {
            let status = if r.success { "✅ PASS" } else { "❌ FAIL" };
            let (avg_fps, one_pct, max_ft, jank_cnt, checksum) = match &r.metrics {
                Some(m) => (
                    format!("{:.1}", m.avg_fps.unwrap_or(0.0)),
                    format!("{:.1}", m.one_percent_low_fps.unwrap_or(0.0)),
                    format!("{:.2} ms", m.max_frame_time_ms.unwrap_or(0.0)),
                    format!("{}", m.jank_frame_count.unwrap_or(0)),
                    &m.checksum[..8],
                ),
                None => ("N/A".to_string(), "N/A".to_string(), "N/A".to_string(), "N/A".to_string(), "N/A"),
            };
            let dist_mb = (r.dist_size_bytes as f64) / (1024.0 * 1024.0);
            let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

            md.push_str(&format!(
                "| **{}** | {} | {} | **{}** | **{}** | {} | {} | {:.2} | {:.2} | `{}` |\n",
                r.target_name, r.category, status, avg_fps, one_pct, max_ft, jank_cnt, dist_mb, rss_mb, checksum
            ));
        }
    } else if is_stress {
        md.push_str("# Extreme Multi-Core Saturation Stress Benchmark Results\n\n");
        md.push_str(&format!("**Replay Log**: `{}`\n\n", replay_file));
        md.push_str("| Target Stack | Category | Status | Total Steps | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | P95 Latency (ms) | Dist (MB) | Checksum |\n");
        md.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");

        for r in reports {
            let status = if r.success { "✅ PASS" } else { "❌ FAIL" };
            let (steps_str, steps_sec, p95_str, checksum) = match &r.metrics {
                Some(m) => (
                    format!("{}", m.steps_processed),
                    format!("{:.1}", m.steps_per_sec),
                    format!("{:.3}", m.p95_latency_ms.unwrap_or(0.0)),
                    &m.checksum[..8],
                ),
                None => ("N/A".to_string(), "N/A".to_string(), "N/A".to_string(), "N/A"),
            };
            let dist_mb = (r.dist_size_bytes as f64) / (1024.0 * 1024.0);
            let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

            md.push_str(&format!(
                "| **{}** | {} | {} | {} | **{}** | {:.1} | {:.2} | {} | {:.2} | `{}` |\n",
                r.target_name, r.category, status, steps_str, steps_sec, r.total_wall_time_ms, rss_mb, p95_str, dist_mb, checksum
            ));
        }
    } else {
        md.push_str("# Multi-Stack Application Benchmark Results Matrix\n\n");
        md.push_str(&format!("**Replay Log**: `{}`\n\n", replay_file));
        md.push_str("| Target Stack | Category | Status | Cold Build (s) | Warm Build (ms) | Bundle (KB) | Dist (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |\n");
        md.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");

        for r in reports {
            let status = if r.success { "✅ PASS" } else { "❌ FAIL" };
            let (steps_sec, checksum) = match &r.metrics {
                Some(m) => (format!("{:.1}", m.steps_per_sec), &m.checksum[..8]),
                None => ("N/A".to_string(), "N/A"),
            };
            let cold_s = r.cold_build_duration_ms / 1000.0;
            let bundle_kb = (r.bundle_size_bytes as f64) / 1024.0;
            let dist_mb = (r.dist_size_bytes as f64) / (1024.0 * 1024.0);
            let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

            md.push_str(&format!(
                "| **{}** | {} | {} | **{:.2} s** | {:.1} ms | {:.1} | **{:.2}** | **{}** | {:.1} | {:.2} | `{}` |\n",
                r.target_name, r.category, status, cold_s, r.warm_build_duration_ms, bundle_kb, dist_mb, steps_sec, r.total_wall_time_ms, rss_mb, checksum
            ));
        }
    }

    let _ = fs::write(markdown_path, md);
}

fn chrono_or_fallback() -> String {
    "2026-08-18T12:00:00Z".to_string()
}

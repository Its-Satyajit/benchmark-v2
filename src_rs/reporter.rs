use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::harness::TargetBenchmarkReport;

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkSuiteResult {
    pub timestamp: String,
    pub replay_file: String,
    pub reports: Vec<TargetBenchmarkReport>,
}

pub fn render_terminal_table(reports: &[TargetBenchmarkReport]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Target Stack").fg(Color::Cyan),
            Cell::new("Category").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Build (ms)").fg(Color::Cyan),
            Cell::new("Artifact (MB)").fg(Color::Cyan),
            Cell::new("Throughput (steps/s)").fg(Color::Cyan),
            Cell::new("Wall Time (ms)").fg(Color::Cyan),
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

        let artifact_mb = (r.artifact_size_bytes as f64) / (1024.0 * 1024.0);
        let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

        table.add_row(vec![
            Cell::new(&r.target_name),
            Cell::new(&r.category),
            status_cell,
            Cell::new(format!("{:.1}", r.build_duration_ms)),
            Cell::new(format!("{:.2}", artifact_mb)),
            Cell::new(steps_sec).fg(Color::Yellow),
            Cell::new(format!("{:.1}", r.total_wall_time_ms)),
            Cell::new(format!("{:.2}", rss_mb)),
            Cell::new(checksum),
        ]);
    }

    println!("\n{}", table);
}

pub fn export_results(
    reports: &[TargetBenchmarkReport],
    replay_file: &str,
    json_path: &str,
    markdown_path: &str,
) {
    let suite = BenchmarkSuiteResult {
        timestamp: chrono_or_fallback(),
        replay_file: replay_file.to_string(),
        reports: reports.to_vec(),
    };

    if let Ok(json_str) = serde_json::to_string_pretty(&suite) {
        let _ = fs::write(json_path, json_str);
    }

    let mut md = String::new();
    md.push_str("# Stress Benchmark Results Matrix\n\n");
    md.push_str(&format!("**Dataset**: `{}`\n\n", replay_file));
    md.push_str("| Target Stack | Category | Status | Build (ms) | Artifact (MB) | Throughput (steps/s) | Wall Time (ms) | Peak RSS (MB) | Checksum |\n");
    md.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");

    for r in reports {
        let status = if r.success { "✅ PASS" } else { "❌ FAIL" };
        let (steps_sec, checksum) = match &r.metrics {
            Some(m) => (format!("{:.1}", m.steps_per_sec), &m.checksum[..8]),
            None => ("N/A".to_string(), "N/A"),
        };
        let artifact_mb = (r.artifact_size_bytes as f64) / (1024.0 * 1024.0);
        let rss_mb = (r.peak_rss_bytes as f64) / (1024.0 * 1024.0);

        md.push_str(&format!(
            "| **{}** | {} | {} | {:.1} | {:.2} | **{}** | {:.1} | {:.2} | `{}` |\n",
            r.target_name, r.category, status, r.build_duration_ms, artifact_mb, steps_sec, r.total_wall_time_ms, rss_mb, checksum
        ));
    }

    let _ = fs::write(markdown_path, md);
}

fn chrono_or_fallback() -> String {
    "2026-08-18T11:00:00Z".to_string()
}

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BenchmarkLockGuard {
    lock_file: String,
}

impl BenchmarkLockGuard {
    pub fn acquire(lock_file: &str) -> Result<Self, String> {
        let path = Path::new(lock_file);

        // 1. Inspect existing lock
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                let parts: Vec<&str> = content.trim().split(':').collect();
                if let Some(pid_str) = parts.first() {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        let proc_path = format!("/proc/{}", pid);
                        if Path::new(&proc_path).exists() {
                            return Err(format!(
                                "Error: Another benchmark process (PID: {}) is currently running. Please wait or terminate it first.",
                                pid
                            ));
                        }
                    }
                }
            }
            let _ = fs::remove_file(path);
        }

        // 2. Atomic lock file creation (create_new)
        let my_pid = process::id();
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let lock_payload = format!("{}:{}", my_pid, now_sec);

        let mut file = match OpenOptions::new().write(true).create_new(true).open(path) {
            Ok(f) => f,
            Err(_) => {
                // Potential race condition check
                if let Ok(content) = fs::read_to_string(path) {
                    let pid = content.trim().split(':').next().unwrap_or("unknown");
                    return Err(format!(
                        "Error: Another benchmark process (PID: {}) is currently running. Please wait or terminate it first.",
                        pid
                    ));
                }
                return Err(format!("Failed to atomically create lockfile '{}'", lock_file));
            }
        };

        if let Err(e) = write!(file, "{}", lock_payload) {
            let _ = fs::remove_file(path);
            return Err(format!("Failed to write lock payload: {}", e));
        }

        Ok(BenchmarkLockGuard {
            lock_file: lock_file.to_string(),
        })
    }
}

impl Drop for BenchmarkLockGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.lock_file);
    }
}

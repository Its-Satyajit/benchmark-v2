use std::fs;
use std::path::Path;
use std::process;

pub struct BenchmarkLockGuard {
    lock_file: String,
}

impl BenchmarkLockGuard {
    pub fn acquire(lock_file: &str) -> Result<Self, String> {
        let path = Path::new(lock_file);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(pid) = content.trim().parse::<u32>() {
                    let proc_path = format!("/proc/{}", pid);
                    if Path::new(&proc_path).exists() {
                        return Err(format!(
                            "🚫 Benchmark execution blocked: Another benchmark process (PID: {}) is currently running.\n   Wait for it to finish or stop PID {} before starting a new run.",
                            pid, pid
                        ));
                    }
                }
            }
            let _ = fs::remove_file(path);
        }

        let my_pid = process::id();
        if let Err(e) = fs::write(path, my_pid.to_string()) {
            return Err(format!("Failed to write lockfile '{}': {}", lock_file, e));
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

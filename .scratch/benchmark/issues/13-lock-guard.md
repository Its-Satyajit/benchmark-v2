# 13 — Implement Exclusive Execution Lockfile & Stale Process Guard

**What to build:** Add `BenchmarkLockGuard` in Rust runner to check for any active benchmark run via `.benchmark.lock` and `/proc/<PID>`.

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Create `src_rs/lock.rs` with PID lock acquisition, liveness probe, and RAII cleanup
- [ ] Integrate lock guard in `src_rs/main.rs`
- [ ] Verify test suite and error reporting when lock is active

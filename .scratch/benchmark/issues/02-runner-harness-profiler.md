# 02 — Benchmark Runner Harness with Build & Memory Profiling

**What to build:** The benchmark orchestrator that triggers clean builds, calculates binary and distribution artifact sizes, spawns target processes, monitors process peak RSS/memory and duration, and validates output checksums.

**Blocked by:** 01 — Target Adapter Contract & Reference State Replay Engine

**Status:** ready-for-agent

- [ ] Measure clean build time and binary / artifact directory size
- [ ] Subprocess executor with live RSS memory sampling (Linux /proc or cross-platform memory poller)
- [ ] Aggregation of timings (cold startup, parse, replay, total) and verification

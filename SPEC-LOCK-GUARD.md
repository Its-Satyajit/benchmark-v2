# Specification: Exclusive Execution Lockfile & Stale Process Guard

## Problem Statement

Concurrent benchmark runs or orphaned background processes interfere with CPU pinning, memory RSS metrics, and build locks.

## Solution

Implement `BenchmarkLockGuard` in `src_rs/lock.rs`:
1. Check if `.benchmark.lock` exists.
2. If found, inspect `/proc/<PID>` to determine if the previous process is still running.
3. If running, abort with `Error: Another benchmark process (PID: <PID>) is currently running. Please wait or terminate it first.`
4. If stale, unlink and take over.
5. Auto-unlink `.benchmark.lock` when the struct is dropped.

---

## User Stories

1. As a user, I want the runner to prevent accidental concurrent runs so that my benchmark results are not skewed by background CPU/RAM contention.

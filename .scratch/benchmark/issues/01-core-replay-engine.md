# 01 — Target Adapter Contract & Reference State Replay Engine

**What to build:** The core state replay simulation engine and standardized target adapter schema. Given `92139349.json`, it parses the replay log, executes the turn actions and state mutations, computes a deterministic checksum of the final state, and outputs JSON metrics.

**Blocked by:** None — can start immediately

**Status:** ready-for-agent

- [ ] Parse replay configuration, specification, deck/hand distributions, and step actions from `92139349.json`
- [ ] Implement deterministic state reducer & checksum generation
- [ ] Emit standardized target benchmark metrics schema to stdout

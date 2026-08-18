import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { simulateReplay, type ReplayLog } from '../src/core/replay.ts';

describe('Core Replay Simulation Engine', () => {
  const replayPath = join(process.cwd(), 'data', '92139349.json');

  it('should parse the reference replay log successfully', () => {
    const raw = readFileSync(replayPath, 'utf8');
    const data = JSON.parse(raw) as ReplayLog;
    assert.equal(data.name, 'cabt');
    assert.equal(data.schema_version, 1);
    assert.ok(Array.isArray(data.steps));
    assert.ok(data.steps.length > 0);
  });

  it('should deterministically simulate replay steps and compute a valid checksum', () => {
    const raw = readFileSync(replayPath, 'utf8');
    const data = JSON.parse(raw) as ReplayLog;
    const result = simulateReplay(data);

    assert.equal(result.target, 'reference-ts');
    assert.equal(result.steps_processed, data.steps.length);
    assert.ok(result.total_duration_ms >= 0);
    assert.ok(result.steps_per_sec >= 0);
    assert.equal(typeof result.checksum, 'string');
    assert.ok(result.checksum.length > 0);
  });

  it('should produce identical checksum across repeated simulation runs', () => {
    const raw = readFileSync(replayPath, 'utf8');
    const data = JSON.parse(raw) as ReplayLog;
    const run1 = simulateReplay(data);
    const run2 = simulateReplay(data);

    assert.equal(run1.checksum, run2.checksum);
    assert.equal(run1.steps_processed, run2.steps_processed);
  });
});

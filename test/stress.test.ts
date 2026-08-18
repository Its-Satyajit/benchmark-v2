import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import {
  simulateReplay,
  simulateStressReplay,
  type ReplayLog,
} from '../packages/shared-replay-core/src/index.ts';

describe('Extreme Saturation Stress Engine (Shared TS Core)', () => {
  const replayPath = join(process.cwd(), 'data', '92139349.json');
  const raw = readFileSync(replayPath, 'utf8');
  const data = JSON.parse(raw) as ReplayLog;

  it('should run multi-iteration stress replay with snapshot tree retention', () => {
    const stressResult = simulateStressReplay(data, {
      iterations: 5,
      concurrency: 2,
      retainSnapshots: true,
      targetName: 'stress-ts',
    });

    assert.equal(stressResult.target, 'stress-ts');
    assert.equal(stressResult.steps_processed, data.steps.length * 5);
    assert.ok(stressResult.total_duration_ms > 0);
    assert.ok(stressResult.steps_per_sec > 0);
    assert.ok((stressResult.snapshots_retained ?? 0) > 0);
    assert.ok((stressResult.p95_latency_ms ?? -1) >= 0);
    assert.equal(typeof stressResult.checksum, 'string');
  });

  it('should maintain deterministic checksum integrity across parallel stress batches', () => {
    const run1 = simulateStressReplay(data, { iterations: 2, concurrency: 2, retainSnapshots: true });
    const run2 = simulateStressReplay(data, { iterations: 2, concurrency: 2, retainSnapshots: true });

    assert.equal(run1.checksum, run2.checksum);
    assert.equal(run1.steps_processed, run2.steps_processed);
  });
});

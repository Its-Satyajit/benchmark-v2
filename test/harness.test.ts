import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { join } from 'node:path';
import {
  calculateArtifactSize,
  executeTargetWithProfiling,
  type TargetDescriptor,
} from '../src/runner/harness.ts';

describe('Benchmark Runner Harness & Profiler', () => {
  const replayPath = join(process.cwd(), 'data', '92139349.json');

  it('should accurately calculate file and directory artifact sizes', async () => {
    const singleFileSize = await calculateArtifactSize(replayPath);
    assert.ok(singleFileSize > 8_000_000, `Expected >8MB, got ${singleFileSize}`);

    const dirSize = await calculateArtifactSize(join(process.cwd(), 'src'));
    assert.ok(dirSize > 0);
  });

  it('should profile execution of a target process, capturing peak RSS, timings, and metrics', async () => {
    const descriptor: TargetDescriptor = {
      id: 'node-cli',
      name: 'Node / TypeScript CLI Target',
      category: 'CLI',
      runCommand: `nub src/targets/cli-ts.ts --replay ${replayPath}`,
      buildArtifactPath: 'src/targets/cli-ts.ts',
    };

    const report = await executeTargetWithProfiling(descriptor, replayPath);
    assert.equal(report.targetId, 'node-cli');
    assert.ok(report.success);
    assert.ok(report.totalWallTimeMs > 0);
    assert.ok(report.peakRssBytes > 0);
    assert.ok(report.artifactSizeBytes > 0);
    assert.ok(report.metrics);
    assert.ok(report.metrics.steps_processed > 0);
    assert.ok(report.metrics.steps_per_sec > 0);
    assert.equal(typeof report.metrics.checksum, 'string');
  });
});

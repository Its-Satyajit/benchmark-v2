import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import {
  simulateGuiJankReplay,
  type ReplayLog,
} from '../packages/shared-replay-core/src/index.ts';

describe('GUI Jank & Frame Render Profiler', () => {
  const replayPath = join(process.cwd(), '92139349.json');
  const raw = readFileSync(replayPath, 'utf8');
  const data = JSON.parse(raw) as ReplayLog;

  it('should accurately compute frame pacing, FPS, and jank metrics under stress simulation', () => {
    const guiResult = simulateGuiJankReplay(data, {
      iterations: 5,
      targetName: 'gui-test-target',
    });

    assert.equal(guiResult.target, 'gui-test-target');
    assert.ok((guiResult.total_frames_rendered ?? 0) > 0);
    assert.ok((guiResult.avg_fps ?? 0) > 0);
    assert.ok((guiResult.one_percent_low_fps ?? 0) >= 0);
    assert.ok((guiResult.max_frame_time_ms ?? 0) >= 0);
    assert.ok((guiResult.jank_frame_count ?? -1) >= 0);
    assert.ok((guiResult.jank_percentage ?? -1) >= 0);
  });
});

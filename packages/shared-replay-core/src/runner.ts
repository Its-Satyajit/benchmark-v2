import { readFileSync } from 'node:fs';
import { simulateReplay, simulateStressReplay, type ReplayLog } from '@benchmark/shared-replay-core';

export function runGenericTarget(targetName: string) {
  const replayArgIndex = process.argv.indexOf('--replay');
  const isStress = process.argv.includes('--stress');
  const iterIndex = process.argv.indexOf('--iterations');
  const iterations = iterIndex !== -1 ? parseInt(process.argv[iterIndex + 1] ?? '20', 10) : 20;

  const replayPath = replayArgIndex !== -1 ? process.argv[replayArgIndex + 1] : undefined;

  if (!replayPath) {
    console.error('Error: Missing --replay <path> argument');
    process.exit(1);
  }

  const parseStart = performance.now();
  const raw = readFileSync(replayPath, 'utf8');
  const data = JSON.parse(raw) as ReplayLog;
  const parseDuration = performance.now() - parseStart;

  const result = isStress
    ? simulateStressReplay(data, {
        iterations,
        retainSnapshots: true,
        targetName,
      })
    : simulateReplay(data, targetName);

  const total = parseDuration + result.replay_duration_ms;

  const finalOutput = {
    ...result,
    parse_duration_ms: Math.round(parseDuration * 100) / 100,
    total_duration_ms: Math.round(total * 100) / 100,
    steps_per_sec: Math.round((result.steps_processed / (result.replay_duration_ms / 1000)) * 100) / 100,
  };

  console.log(JSON.stringify(finalOutput));
}

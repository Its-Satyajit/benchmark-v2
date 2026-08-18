import { readFileSync } from 'node:fs';
import { simulateReplay, type ReplayLog } from '../core/replay.ts';

function main() {
  const replayArgIndex = process.argv.indexOf('--replay');
  const replayPath = replayArgIndex !== -1 ? process.argv[replayArgIndex + 1] : undefined;

  if (!replayPath) {
    console.error('Error: Missing --replay <path> argument');
    process.exit(1);
  }

  const parseStart = performance.now();
  const raw = readFileSync(replayPath, 'utf8');
  const data = JSON.parse(raw) as ReplayLog;
  const parseEnd = performance.now();
  const parseDuration = parseEnd - parseStart;

  const result = simulateReplay(data, 'ts-cli');
  const total = parseDuration + result.replay_duration_ms;

  const finalOutput = {
    ...result,
    parse_duration_ms: Math.round(parseDuration * 100) / 100,
    total_duration_ms: Math.round(total * 100) / 100,
    steps_per_sec: Math.round((result.steps_processed / (total / 1000)) * 100) / 100,
  };

  console.log(JSON.stringify(finalOutput));
}

main();

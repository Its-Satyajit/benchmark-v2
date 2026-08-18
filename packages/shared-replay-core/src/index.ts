import { createHash } from 'node:crypto';
import { performance } from 'node:perf_hooks';

export interface Configuration {
  seed?: number;
}

export interface Player {
  active?: unknown[];
  bench?: unknown[];
  deck?: unknown[];
  hand?: unknown[];
}

export interface CurrentState {
  players?: Player[];
}

export interface StepObservation {
  current?: CurrentState;
}

export interface StepItem {
  action?: unknown[];
  observation?: StepObservation;
  status?: string;
}

export interface ReplayLog {
  name?: string;
  configuration?: Configuration;
  steps: StepItem[][];
}

export interface RawIterationTelemetry {
  iteration: number;
  wall_time_ms: number;
  steps_processed: number;
  steps_per_sec: number;
  peak_rss_bytes: number;
  checksum: string;
  raw_step_latencies_ms?: number[];
  raw_frame_times_ms?: number[];
}

export interface TargetBenchmarkResult {
  target: string;
  steps_processed: number;
  parse_duration_ms: number;
  replay_duration_ms: number;
  total_duration_ms: number;
  steps_per_sec: number;
  checksum: string;
  snapshots_retained?: number;
  p50_latency_ms?: number;
  p95_latency_ms?: number;
  p99_latency_ms?: number;
  total_frames_rendered?: number;
  avg_fps?: number;
  one_percent_low_fps?: number;
  zero_point_one_percent_low_fps?: number;
  jank_frame_count?: number;
  jank_percentage?: number;
  max_frame_time_ms?: number;
  raw_iterations?: RawIterationTelemetry[];
}

export interface StressOptions {
  iterations?: number;
  concurrency?: number;
  soakDuration?: number;
  retainSnapshots?: boolean;
  targetName?: string;
}

export function simulateReplay(
  replay: ReplayLog,
  targetName: string = 'baseline-ts',
  iterations: number = 1
): TargetBenchmarkResult {
  const startTime = performance.now();
  const rawIterations: RawIterationTelemetry[] = [];
  let totalStepsProcessed = 0;
  let finalChecksum = '';

  for (let it = 0; it < iterations; it++) {
    const iterStart = performance.now();
    const hasher = createHash('sha256');
    const iterLatencies: number[] = [];

    if (replay.configuration?.seed !== undefined) {
      hasher.update(`seed:${replay.configuration.seed};`);
    }

    let stepsProcessed = 0;
    let totalCardsObserved = 0;
    let actionTransitions = 0;

    for (let i = 0; i < replay.steps.length; i++) {
      const stepStart = performance.now();
      const stepBatch = replay.steps[i];
      if (!stepBatch) continue;
      stepsProcessed++;

      for (let j = 0; j < stepBatch.length; j++) {
        const step = stepBatch[j];
        if (!step) continue;

        if (step.action && step.action.length > 0) {
          actionTransitions += step.action.length;
          hasher.update(`act:${i}:${j}:${JSON.stringify(step.action)};`);
        }

        if (step.observation?.current?.players) {
          const players = step.observation.current.players;
          for (let p = 0; p < players.length; p++) {
            const player = players[p];
            if (!player) continue;

            totalCardsObserved += (player.deck?.length ?? 0) + (player.hand?.length ?? 0);
            hasher.update(
              `p:${p}:d${player.deck?.length ?? 0}:h${player.hand?.length ?? 0}:a${player.active?.length ?? 0}:b${player.bench?.length ?? 0};`
            );
          }
        }

        if (step.status) {
          hasher.update(`st:${step.status};`);
        }
      }

      iterLatencies.push(performance.now() - stepStart);
    }

    hasher.update(`final:steps=${stepsProcessed}:cards=${totalCardsObserved}:acts=${actionTransitions}`);
    const iterChecksum = hasher.digest('hex');
    finalChecksum = iterChecksum;
    const iterDurationMs = Math.max(0.001, performance.now() - iterStart);
    totalStepsProcessed += stepsProcessed;

    rawIterations.push({
      iteration: it + 1,
      wall_time_ms: Math.round(iterDurationMs * 1000) / 1000,
      steps_processed: stepsProcessed,
      steps_per_sec: Math.round((stepsProcessed / (iterDurationMs / 1000)) * 100) / 100,
      peak_rss_bytes: process.memoryUsage().rss,
      checksum: iterChecksum,
      raw_step_latencies_ms: iterLatencies.map((l) => Math.round(l * 1000) / 1000),
    });
  }

  const totalDurationMs = Math.max(0.001, performance.now() - startTime);
  const stepsPerSec = (totalStepsProcessed / (totalDurationMs / 1000));

  return {
    target: targetName,
    steps_processed: totalStepsProcessed,
    parse_duration_ms: 0,
    replay_duration_ms: Math.round(totalDurationMs * 100) / 100,
    total_duration_ms: Math.round(totalDurationMs * 100) / 100,
    steps_per_sec: Math.round(stepsPerSec * 100) / 100,
    checksum: finalChecksum,
    raw_iterations: rawIterations,
  };
}

export function simulateStressReplay(
  replay: ReplayLog,
  options: StressOptions = {}
): TargetBenchmarkResult {
  const iterations = Math.max(1, options.iterations ?? 10);
  const retainSnapshots = options.retainSnapshots ?? true;
  const targetName = options.targetName ?? 'stress-ts';

  const startTime = performance.now();
  const hasher = createHash('sha256');
  const allLatencies: number[] = [];
  const snapshotTree: unknown[] = [];
  const rawIterations: RawIterationTelemetry[] = [];

  let totalSteps = 0;
  let totalCards = 0;
  let totalActs = 0;

  for (let it = 0; it < iterations; it++) {
    const iterStart = performance.now();
    const iterLatencies: number[] = [];
    hasher.update(`iter:${it};`);
    if (replay.configuration?.seed !== undefined) {
      hasher.update(`seed:${replay.configuration.seed};`);
    }

    let iterSteps = 0;

    for (let i = 0; i < replay.steps.length; i++) {
      const stepStart = performance.now();
      const stepBatch = replay.steps[i];
      if (!stepBatch) continue;
      totalSteps++;
      iterSteps++;

      for (let j = 0; j < stepBatch.length; j++) {
        const step = stepBatch[j];
        if (!step) continue;

        if (step.action && step.action.length > 0) {
          totalActs += step.action.length;
          hasher.update(`act:${it}:${i}:${j}:${JSON.stringify(step.action)};`);
        }

        if (step.observation?.current?.players) {
          const players = step.observation.current.players;
          if (retainSnapshots) {
            snapshotTree.push(structuredClone(players));
          }

          for (let p = 0; p < players.length; p++) {
            const player = players[p];
            if (!player) continue;

            totalCards += (player.deck?.length ?? 0) + (player.hand?.length ?? 0);
            hasher.update(
              `p:${p}:d${player.deck?.length ?? 0}:h${player.hand?.length ?? 0}:a${player.active?.length ?? 0}:b${player.bench?.length ?? 0};`
            );
          }
        }

        if (step.status) {
          hasher.update(`st:${step.status};`);
        }
      }

      const stepDuration = performance.now() - stepStart;
      iterLatencies.push(stepDuration);
      allLatencies.push(stepDuration);
    }

    const iterDurationMs = Math.max(0.001, performance.now() - iterStart);
    rawIterations.push({
      iteration: it + 1,
      wall_time_ms: Math.round(iterDurationMs * 1000) / 1000,
      steps_processed: iterSteps,
      steps_per_sec: Math.round((iterSteps / (iterDurationMs / 1000)) * 100) / 100,
      peak_rss_bytes: process.memoryUsage().rss,
      checksum: '',
      raw_step_latencies_ms: iterLatencies.map((l) => Math.round(l * 1000) / 1000),
    });
  }

  hasher.update(`final_stress:steps=${totalSteps}:cards=${totalCards}:acts=${totalActs}`);
  const checksum = hasher.digest('hex');
  const endTime = performance.now();
  const totalDurationMs = Math.max(0.001, endTime - startTime);
  const stepsPerSec = (totalSteps / (totalDurationMs / 1000));

  allLatencies.sort((a, b) => a - b);
  const p50 = allLatencies[Math.floor(allLatencies.length * 0.5)] ?? 0;
  const p95 = allLatencies[Math.floor(allLatencies.length * 0.95)] ?? 0;
  const p99 = allLatencies[Math.floor(allLatencies.length * 0.99)] ?? 0;

  for (const raw of rawIterations) {
    raw.checksum = checksum;
  }

  return {
    target: targetName,
    steps_processed: totalSteps,
    parse_duration_ms: 0,
    replay_duration_ms: Math.round(totalDurationMs * 100) / 100,
    total_duration_ms: Math.round(totalDurationMs * 100) / 100,
    steps_per_sec: Math.round(stepsPerSec * 100) / 100,
    checksum,
    snapshots_retained: snapshotTree.length,
    p50_latency_ms: Math.round(p50 * 1000) / 1000,
    p95_latency_ms: Math.round(p95 * 1000) / 1000,
    p99_latency_ms: Math.round(p99 * 1000) / 1000,
    raw_iterations: rawIterations,
  };
}

export function simulateGuiJankReplay(
  replay: ReplayLog,
  options: StressOptions = {}
): TargetBenchmarkResult {
  const iterations = Math.max(1, options.iterations ?? 10);
  const targetName = options.targetName ?? 'gui-ts';

  const startTime = performance.now();
  const hasher = createHash('sha256');
  const frameTimes: number[] = [];
  const rawIterations: RawIterationTelemetry[] = [];

  let totalSteps = 0;
  let totalCards = 0;
  let totalActs = 0;
  let jankFrameCount = 0;
  let maxFrameTimeMs = 0;

  for (let it = 0; it < iterations; it++) {
    const iterStart = performance.now();
    const iterFrames: number[] = [];
    hasher.update(`iter:${it};`);
    if (replay.configuration?.seed !== undefined) {
      hasher.update(`seed:${replay.configuration.seed};`);
    }

    let iterSteps = 0;

    for (let i = 0; i < replay.steps.length; i++) {
      const frameStart = performance.now();
      const stepBatch = replay.steps[i];
      if (!stepBatch) continue;
      totalSteps++;
      iterSteps++;

      for (let j = 0; j < stepBatch.length; j++) {
        const step = stepBatch[j];
        if (!step) continue;

        if (step.action && step.action.length > 0) {
          totalActs += step.action.length;
          hasher.update(`act:${it}:${i}:${j}:${JSON.stringify(step.action)};`);
        }

        if (step.observation?.current?.players) {
          const players = step.observation.current.players;
          for (let p = 0; p < players.length; p++) {
            const player = players[p];
            if (!player) continue;

            totalCards += (player.deck?.length ?? 0) + (player.hand?.length ?? 0);
            hasher.update(
              `p:${p}:d${player.deck?.length ?? 0}:h${player.hand?.length ?? 0}:a${player.active?.length ?? 0}:b${player.bench?.length ?? 0};`
            );
          }
        }

        if (step.status) {
          hasher.update(`st:${step.status};`);
        }
      }

      const frameDuration = performance.now() - frameStart;
      frameTimes.push(frameDuration);
      iterFrames.push(frameDuration);

      if (frameDuration > 16.667) {
        jankFrameCount++;
      }
      if (frameDuration > maxFrameTimeMs) {
        maxFrameTimeMs = frameDuration;
      }
    }

    const iterDurationMs = Math.max(0.001, performance.now() - iterStart);
    rawIterations.push({
      iteration: it + 1,
      wall_time_ms: Math.round(iterDurationMs * 1000) / 1000,
      steps_processed: iterSteps,
      steps_per_sec: Math.round((iterSteps / (iterDurationMs / 1000)) * 100) / 100,
      peak_rss_bytes: process.memoryUsage().rss,
      checksum: '',
      raw_frame_times_ms: iterFrames.map((f) => Math.round(f * 1000) / 1000),
    });
  }

  hasher.update(`final_gui:steps=${totalSteps}:cards=${totalCards}:acts=${totalActs}`);
  const checksum = hasher.digest('hex');
  const endTime = performance.now();
  const totalDurationMs = Math.max(0.001, endTime - startTime);
  const totalFrames = frameTimes.length;
  const avgFps = totalFrames / (totalDurationMs / 1000);

  frameTimes.sort((a, b) => a - b);
  const onePctIdx = Math.floor(frameTimes.length * 0.99);
  const zeroPointOneIdx = Math.floor(frameTimes.length * 0.999);
  const onePctDuration = frameTimes[onePctIdx] ?? 0.001;
  const zeroPointOneDuration = frameTimes[zeroPointOneIdx] ?? 0.001;

  const onePctFps = 1000 / Math.max(0.001, onePctDuration);
  const zeroPointOneFps = 1000 / Math.max(0.001, zeroPointOneDuration);
  const jankPercentage = (jankFrameCount / Math.max(1, totalFrames)) * 100;

  for (const raw of rawIterations) {
    raw.checksum = checksum;
  }

  return {
    target: targetName,
    steps_processed: totalSteps,
    parse_duration_ms: 0,
    replay_duration_ms: Math.round(totalDurationMs * 100) / 100,
    total_duration_ms: Math.round(totalDurationMs * 100) / 100,
    steps_per_sec: Math.round(avgFps * 100) / 100,
    checksum,
    total_frames_rendered: totalFrames,
    avg_fps: Math.round(avgFps * 10) / 10,
    one_percent_low_fps: Math.round(onePctFps * 10) / 10,
    zero_point_one_percent_low_fps: Math.round(zeroPointOneFps * 10) / 10,
    jank_frame_count: jankFrameCount,
    jank_percentage: Math.round(jankPercentage * 100) / 100,
    max_frame_time_ms: Math.round(maxFrameTimeMs * 100) / 100,
    raw_iterations: rawIterations,
  };
}

export { runGenericTarget } from './runner.ts';

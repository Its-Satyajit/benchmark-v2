import { createHash } from 'node:crypto';
export { runGenericTarget } from './runner.ts';

export interface CardState {
  readonly id: number;
  readonly serial: number;
  readonly name: string;
  readonly playerIndex: number;
}

export interface PlayerState {
  readonly active: readonly unknown[];
  readonly bench: readonly unknown[];
  readonly deck: readonly CardState[];
  readonly hand: readonly CardState[];
  readonly discard: readonly CardState[];
  readonly prize: readonly CardState[];
}

export interface StepObservation {
  readonly current: {
    readonly players: readonly PlayerState[];
    readonly turn?: number;
    readonly result?: number;
  } | null;
  readonly logs?: readonly unknown[];
  readonly step: number;
}

export interface StepItem {
  readonly action?: readonly unknown[];
  readonly observation?: StepObservation;
  readonly obs?: unknown;
  readonly select?: unknown;
  readonly logs?: readonly unknown[];
  readonly reward?: number;
  readonly status?: string;
}

export interface ReplayLog {
  readonly name: string;
  readonly schema_version: number;
  readonly description?: string;
  readonly id?: string;
  readonly steps: readonly (readonly StepItem[])[];
  readonly configuration?: {
    readonly seed?: number;
    readonly runTimeout?: number;
    readonly episodeSteps?: number;
  };
}

export interface TargetBenchmarkResult {
  readonly target: string;
  readonly steps_processed: number;
  readonly parse_duration_ms: number;
  readonly replay_duration_ms: number;
  readonly total_duration_ms: number;
  readonly steps_per_sec: number;
  readonly checksum: string;
  readonly snapshots_retained?: number;
  readonly p50_latency_ms?: number;
  readonly p95_latency_ms?: number;
  readonly p99_latency_ms?: number;
  // GUI Jank & Frame Pacing metrics
  readonly total_frames_rendered?: number;
  readonly avg_fps?: number;
  readonly one_percent_low_fps?: number;
  readonly zero_point_one_percent_low_fps?: number;
  readonly jank_frame_count?: number;
  readonly jank_percentage?: number;
  readonly max_frame_time_ms?: number;
}

export interface StressOptions {
  readonly iterations?: number;
  readonly concurrency?: number;
  readonly retainSnapshots?: boolean;
  readonly targetName?: string;
}

export function simulateReplay(
  replay: ReplayLog,
  targetName = 'reference-ts'
): TargetBenchmarkResult {
  const startTime = performance.now();

  const hasher = createHash('sha256');
  let stepsProcessed = 0;
  let totalCardsObserved = 0;
  let actionTransitions = 0;

  if (replay.configuration?.seed !== undefined) {
    hasher.update(`seed:${replay.configuration.seed};`);
  }

  for (let i = 0; i < replay.steps.length; i++) {
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
  }

  hasher.update(`final:steps=${stepsProcessed}:cards=${totalCardsObserved}:acts=${actionTransitions}`);
  const checksum = hasher.digest('hex');
  const endTime = performance.now();
  const replayDurationMs = Math.max(0.001, endTime - startTime);
  const stepsPerSec = (stepsProcessed / (replayDurationMs / 1000));

  return {
    target: targetName,
    steps_processed: stepsProcessed,
    parse_duration_ms: 0,
    replay_duration_ms: Math.round(replayDurationMs * 100) / 100,
    total_duration_ms: Math.round(replayDurationMs * 100) / 100,
    steps_per_sec: Math.round(stepsPerSec * 100) / 100,
    checksum,
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
  const latencies: number[] = [];
  const snapshotTree: unknown[] = [];

  let totalSteps = 0;
  let totalCards = 0;
  let totalActs = 0;

  for (let it = 0; it < iterations; it++) {
    hasher.update(`iter:${it};`);
    if (replay.configuration?.seed !== undefined) {
      hasher.update(`seed:${replay.configuration.seed};`);
    }

    for (let i = 0; i < replay.steps.length; i++) {
      const stepStart = performance.now();
      const stepBatch = replay.steps[i];
      if (!stepBatch) continue;
      totalSteps++;

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

      latencies.push(performance.now() - stepStart);
    }
  }

  hasher.update(`final_stress:steps=${totalSteps}:cards=${totalCards}:acts=${totalActs}`);
  const checksum = hasher.digest('hex');
  const endTime = performance.now();
  const totalDurationMs = Math.max(0.001, endTime - startTime);
  const stepsPerSec = (totalSteps / (totalDurationMs / 1000));

  latencies.sort((a, b) => a - b);
  const p50 = latencies[Math.floor(latencies.length * 0.5)] ?? 0;
  const p95 = latencies[Math.floor(latencies.length * 0.95)] ?? 0;
  const p99 = latencies[Math.floor(latencies.length * 0.99)] ?? 0;

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
  };
}

export function simulateGuiJankReplay(
  replay: ReplayLog,
  options: StressOptions = {}
): TargetBenchmarkResult {
  const iterations = Math.max(1, options.iterations ?? 10);
  const targetName = options.targetName ?? 'gui-jank-target';

  const frameTimesMs: number[] = [];
  const VSYNC_BUDGET_MS = 16.667; // 60 FPS threshold

  const startTime = performance.now();
  const hasher = createHash('sha256');
  let totalSteps = 0;
  let totalCards = 0;
  let totalActs = 0;

  for (let it = 0; it < iterations; it++) {
    hasher.update(`iter:${it};`);
    if (replay.configuration?.seed !== undefined) {
      hasher.update(`seed:${replay.configuration.seed};`);
    }

    for (let i = 0; i < replay.steps.length; i++) {
      const frameStart = performance.now();
      const stepBatch = replay.steps[i];
      if (!stepBatch) continue;
      totalSteps++;

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
      frameTimesMs.push(frameDuration);
    }
  }

  hasher.update(`final_gui:steps=${totalSteps}:cards=${totalCards}:acts=${totalActs}`);
  const checksum = hasher.digest('hex');
  const totalDurationMs = performance.now() - startTime;

  let jankCount = 0;
  let maxFrameTime = 0;
  for (const ft of frameTimesMs) {
    if (ft > VSYNC_BUDGET_MS) jankCount++;
    if (ft > maxFrameTime) maxFrameTime = ft;
  }

  frameTimesMs.sort((a, b) => b - a); // descending for 1% slow frames
  const onePctIndex = Math.floor(frameTimesMs.length * 0.01);
  const zeroPointOneIndex = Math.floor(frameTimesMs.length * 0.001);

  const onePctWorstMs = frameTimesMs[onePctIndex] ?? 0.001;
  const zeroPointOneWorstMs = frameTimesMs[zeroPointOneIndex] ?? 0.001;

  const avgFps = frameTimesMs.length / (totalDurationMs / 1000.0);
  const onePctFps = 1000.0 / Math.max(0.001, onePctWorstMs);
  const zeroPointOneFps = 1000.0 / Math.max(0.001, zeroPointOneWorstMs);
  const jankPct = (jankCount / Math.max(1, frameTimesMs.length)) * 100.0;

  return {
    target: targetName,
    steps_processed: totalSteps,
    parse_duration_ms: 0,
    replay_duration_ms: Math.round(totalDurationMs * 100) / 100,
    total_duration_ms: Math.round(totalDurationMs * 100) / 100,
    steps_per_sec: Math.round((totalSteps / (totalDurationMs / 1000)) * 100) / 100,
    checksum,
    total_frames_rendered: frameTimesMs.length,
    avg_fps: Math.round(avgFps * 10) / 10,
    one_percent_low_fps: Math.round(onePctFps * 10) / 10,
    zero_point_one_percent_low_fps: Math.round(zeroPointOneFps * 10) / 10,
    jank_frame_count: jankCount,
    jank_percentage: Math.round(jankPct * 100) / 100,
    max_frame_time_ms: Math.round(maxFrameTime * 100) / 100,
  };
}

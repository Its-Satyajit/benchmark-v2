import { createHash } from 'node:crypto';

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

  // Process configuration seed
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
  const totalDurationMs = Math.max(0.001, endTime - startTime);
  const stepsPerSec = (stepsProcessed / totalDurationMs) * 1000;

  return {
    target: targetName,
    steps_processed: stepsProcessed,
    parse_duration_ms: 0,
    replay_duration_ms: totalDurationMs,
    total_duration_ms: totalDurationMs,
    steps_per_sec: Math.round(stepsPerSec * 100) / 100,
    checksum,
  };
}

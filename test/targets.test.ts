import { describe, it } from 'node:test';
import assert from 'node:assert/strict';
import { join } from 'node:path';
import { executeTargetWithProfiling, type TargetDescriptor } from '../src/runner/harness.ts';

describe('Multi-Stack Target Verification Suite', () => {
  const replayPath = join(process.cwd(), 'data', '92139349.json');

  const targets: TargetDescriptor[] = [
    {
      id: 'rust-native-cli',
      name: 'Rust Native Binary',
      category: 'Native',
      buildCommand: 'cargo build --release --manifest-path crates/replay-engine/Cargo.toml',
      buildArtifactPath: 'crates/replay-engine/target/release/benchmark_replay',
      runCommand: `./crates/replay-engine/target/release/benchmark_replay --replay ${replayPath}`,
    },
    {
      id: 'node-cli',
      name: 'Node / TS Strict CLI',
      category: 'CLI',
      runCommand: `nub src/targets/cli-ts.ts --replay ${replayPath}`,
      buildArtifactPath: 'src/targets/cli-ts.ts',
    },
    {
      id: 'python-cli',
      name: 'Python 3 CLI',
      category: 'CLI',
      runCommand: `python3 src/targets/cli-python.py --replay ${replayPath}`,
      buildArtifactPath: 'src/targets/cli-python.py',
    },
    {
      id: 'elysia-backend',
      name: 'ElysiaJS Web Backend',
      category: 'Web Backend',
      runCommand: `nub src/targets/backend-elysia.ts --replay ${replayPath}`,
      buildArtifactPath: 'src/targets/backend-elysia.ts',
    },
    {
      id: 'nextjs-ssr',
      name: 'Next.js SSR Metaframework',
      category: 'Metaframework',
      runCommand: `nub src/targets/metaframework-ssr.ts --replay ${replayPath}`,
      buildArtifactPath: 'src/targets/metaframework-ssr.ts',
    },
    {
      id: 'desktop-tauri-electron',
      name: 'Desktop App (Tauri / Electron)',
      category: 'Desktop',
      runCommand: `nub src/targets/desktop-app.ts --replay ${replayPath}`,
      buildArtifactPath: 'src/targets/desktop-app.ts',
    },
  ];

  it('should build, profile, and match deterministic checksums across all targets', async () => {
    let referenceChecksum: string | null = null;

    for (const target of targets) {
      const report = await executeTargetWithProfiling(target, replayPath);
      assert.ok(report.success, `Target ${target.id} failed: ${report.error}`);
      assert.ok(report.metrics, `Target ${target.id} missing metrics`);

      if (!referenceChecksum) {
        referenceChecksum = report.metrics.checksum;
      } else {
        assert.equal(
          report.metrics.checksum,
          referenceChecksum,
          `Checksum mismatch for target ${target.id}: expected ${referenceChecksum}, got ${report.metrics.checksum}`
        );
      }
    }
  });
});

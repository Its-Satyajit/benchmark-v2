import { exec, spawn } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { readdir, stat } from "node:fs/promises";
import { join } from "node:path";
import { promisify } from "node:util";
import type { TargetBenchmarkResult } from "../core/replay.ts";

const execAsync = promisify(exec);

export interface TargetDescriptor {
	readonly id: string;
	readonly name: string;
	readonly category:
		| "CLI"
		| "Native"
		| "Cross-Platform Desktop"
		| "Desktop"
		| "Native UI Engine"
		| "Rust Native UI"
		| "Web Frontend"
		| "Web Metaframework"
		| "Web Backend"
		| "Metaframework";
	readonly buildCommand?: string;
	readonly buildArtifactPath?: string;
	readonly runCommand: string;
	readonly warmUpRuns?: number;
	readonly measurementRuns?: number;
}

export interface TargetBenchmarkReport {
	readonly targetId: string;
	readonly targetName: string;
	readonly category: string;
	readonly buildDurationMs: number;
	readonly artifactSizeBytes: number;
	readonly totalWallTimeMs: number;
	readonly peakRssBytes: number;
	readonly success: boolean;
	readonly error?: string;
	readonly metrics?: TargetBenchmarkResult;
}

export async function calculateArtifactSize(path: string): Promise<number> {
	if (!existsSync(path)) {
		return 0;
	}

	const fileStat = await stat(path);
	if (fileStat.isFile()) {
		return fileStat.size;
	}

	if (fileStat.isDirectory()) {
		let total = 0;
		const entries = await readdir(path, { withFileTypes: true });
		for (const entry of entries) {
			const fullPath = join(path, entry.name);
			total += await calculateArtifactSize(fullPath);
		}
		return total;
	}

	return 0;
}

function getProcessRss(pid: number): number {
	try {
		const statmPath = `/proc/${pid}/statm`;
		if (existsSync(statmPath)) {
			const content = readFileSync(statmPath, "utf8");
			const parts = content.trim().split(/\s+/);
			const rssPages = parseInt(parts[1] ?? "0", 10);
			return rssPages * 4096; // 4KB page size
		}
	} catch {
		// Fallback if procfs isn't directly readable
	}
	return 0;
}

export async function executeTargetWithProfiling(
	descriptor: TargetDescriptor,
	replayPath: string,
): Promise<TargetBenchmarkReport> {
	let buildDurationMs = 0;
	let artifactSizeBytes = 0;

	// 1. Build Phase (if build command specified)
	if (descriptor.buildCommand) {
		const buildStart = performance.now();
		try {
			await execAsync(descriptor.buildCommand);
		} catch (err: unknown) {
			const msg = err instanceof Error ? err.message : String(err);
			return {
				targetId: descriptor.id,
				targetName: descriptor.name,
				category: descriptor.category,
				buildDurationMs: 0,
				artifactSizeBytes: 0,
				totalWallTimeMs: 0,
				peakRssBytes: 0,
				success: false,
				error: `Build failed: ${msg}`,
			};
		}
		buildDurationMs = performance.now() - buildStart;
	}

	// 2. Measure Artifact Size
	if (descriptor.buildArtifactPath) {
		artifactSizeBytes = await calculateArtifactSize(
			descriptor.buildArtifactPath,
		);
	}

	// 3. Execution & Memory Sampling Phase
	const runStart = performance.now();
	let peakRss = 0;

	return new Promise<TargetBenchmarkReport>((resolve) => {
		const [command, ...args] = descriptor.runCommand.split(" ");
		if (!command) {
			return resolve({
				targetId: descriptor.id,
				targetName: descriptor.name,
				category: descriptor.category,
				buildDurationMs: 0,
				artifactSizeBytes,
				totalWallTimeMs: 0,
				peakRssBytes: 0,
				success: false,
				error: "Empty run command",
			});
		}

		const child = spawn(command, args, {
			stdio: ["ignore", "pipe", "pipe"],
		});

		let stdout = "";
		let stderr = "";

		child.stdout.on("data", (chunk: Buffer) => {
			stdout += chunk.toString("utf8");
		});

		child.stderr.on("data", (chunk: Buffer) => {
			stderr += chunk.toString("utf8");
		});

		const interval = setInterval(() => {
			if (child.pid) {
				const currentRss = getProcessRss(child.pid);
				if (currentRss > peakRss) {
					peakRss = currentRss;
				}
			}
		}, 10);

		child.on("close", (code) => {
			clearInterval(interval);
			const totalWallTimeMs = performance.now() - runStart;

			if (code !== 0) {
				return resolve({
					targetId: descriptor.id,
					targetName: descriptor.name,
					category: descriptor.category,
					buildDurationMs: Math.round(buildDurationMs * 100) / 100,
					artifactSizeBytes,
					totalWallTimeMs: Math.round(totalWallTimeMs * 100) / 100,
					peakRssBytes: peakRss,
					success: false,
					error: `Process exited with code ${code}: ${stderr || stdout}`,
				});
			}

			try {
				const lastLine =
					stdout.trim().split("\n").filter(Boolean).pop() ?? "{}";
				const parsedMetrics = JSON.parse(lastLine) as TargetBenchmarkResult;

				resolve({
					targetId: descriptor.id,
					targetName: descriptor.name,
					category: descriptor.category,
					buildDurationMs: Math.round(buildDurationMs * 100) / 100,
					artifactSizeBytes,
					totalWallTimeMs: Math.round(totalWallTimeMs * 100) / 100,
					peakRssBytes: peakRss,
					success: true,
					metrics: parsedMetrics,
				});
			} catch (err: unknown) {
				const msg = err instanceof Error ? err.message : String(err);
				resolve({
					targetId: descriptor.id,
					targetName: descriptor.name,
					category: descriptor.category,
					buildDurationMs: Math.round(buildDurationMs * 100) / 100,
					artifactSizeBytes,
					totalWallTimeMs: Math.round(totalWallTimeMs * 100) / 100,
					peakRssBytes: peakRss,
					success: false,
					error: `Failed to parse benchmark metrics from stdout: ${msg} (Output: ${stdout})`,
				});
			}
		});

		child.on("error", (err) => {
			clearInterval(interval);
			const totalWallTimeMs = performance.now() - runStart;
			resolve({
				targetId: descriptor.id,
				targetName: descriptor.name,
				category: descriptor.category,
				buildDurationMs: Math.round(buildDurationMs * 100) / 100,
				artifactSizeBytes,
				totalWallTimeMs: Math.round(totalWallTimeMs * 100) / 100,
				peakRssBytes: peakRss,
				success: false,
				error: `Process spawn error: ${err.message}`,
			});
		});
	});
}

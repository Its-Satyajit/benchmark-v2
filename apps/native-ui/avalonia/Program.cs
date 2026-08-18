using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Security.Cryptography;
using System.Text;
using System.Text.Json;

class Program
{
    static void Main(string[] args)
    {
        string? replayPath = null;
        bool isStress = false;
        bool isGui = false;
        int iterations = 20;

        for (int i = 0; i < args.Length; i++)
        {
            if (args[i] == "--replay" && i + 1 < args.Length)
            {
                replayPath = args[i + 1];
            }
            else if (args[i] == "--stress")
            {
                isStress = true;
            }
            else if (args[i] == "--gui" || args[i] == "--gui-jank")
            {
                isGui = true;
            }
            else if (args[i] == "--iterations" && i + 1 < args.Length)
            {
                int.TryParse(args[i + 1], out iterations);
            }
        }

        if (string.IsNullOrEmpty(replayPath))
        {
            Console.Error.WriteLine("Error: Missing --replay <path>");
            Environment.Exit(1);
        }

        var parseSw = Stopwatch.StartNew();
        var rawJson = File.ReadAllText(replayPath);
        using var doc = JsonDocument.Parse(rawJson);
        parseSw.Stop();
        var parseMs = parseSw.Elapsed.TotalMilliseconds;

        var replaySw = Stopwatch.StartNew();
        using var sha = SHA256.Create();
        var sb = new StringBuilder();

        var root = doc.RootElement;
        var steps = root.GetProperty("steps");
        int totalSteps = 0;
        int totalCards = 0;
        int totalActs = 0;
        int actualIters = (isStress || isGui) ? iterations : 1;

        var stepTimes = new List<double>();
        var snapshotTree = new List<object>();

        for (int it = 0; it < actualIters; it++)
        {
            if (isStress || isGui) sb.Append($"iter:{it};");
            if (root.TryGetProperty("configuration", out var cfg) && cfg.TryGetProperty("seed", out var seedProp) && seedProp.ValueKind == JsonValueKind.Number)
            {
                sb.Append($"seed:{seedProp.GetInt64()};");
            }

            int stepBatchIdx = 0;
            foreach (var stepBatch in steps.EnumerateArray())
            {
                var stepSw = Stopwatch.StartNew();
                totalSteps++;
                int stepIdx = 0;
                foreach (var step in stepBatch.EnumerateArray())
                {
                    if (step.TryGetProperty("action", out var act) && act.ValueKind == JsonValueKind.Array && act.GetArrayLength() > 0)
                    {
                        totalActs += act.GetArrayLength();
                        if (isStress || isGui)
                            sb.Append($"act:{it}:{stepBatchIdx}:{stepIdx}:{act.GetRawText()};");
                        else
                            sb.Append($"act:{stepBatchIdx}:{stepIdx}:{act.GetRawText()};");
                    }

                    if (step.TryGetProperty("observation", out var obs) && obs.ValueKind == JsonValueKind.Object &&
                        obs.TryGetProperty("current", out var cur) && cur.ValueKind == JsonValueKind.Object &&
                        cur.TryGetProperty("players", out var players) && players.ValueKind == JsonValueKind.Array)
                    {
                        if (isStress)
                        {
                            snapshotTree.Add(players.Clone());
                        }

                        int pIdx = 0;
                        foreach (var player in players.EnumerateArray())
                        {
                            int dLen = player.TryGetProperty("deck", out var d) && d.ValueKind == JsonValueKind.Array ? d.GetArrayLength() : 0;
                            int hLen = player.TryGetProperty("hand", out var h) && h.ValueKind == JsonValueKind.Array ? h.GetArrayLength() : 0;
                            int aLen = player.TryGetProperty("active", out var a) && a.ValueKind == JsonValueKind.Array ? a.GetArrayLength() : 0;
                            int bLen = player.TryGetProperty("bench", out var b) && b.ValueKind == JsonValueKind.Array ? b.GetArrayLength() : 0;
                            totalCards += dLen + hLen;
                            sb.Append($"p:{pIdx}:d{dLen}:h{hLen}:a{aLen}:b{bLen};");
                            pIdx++;
                        }
                    }

                    if (step.TryGetProperty("status", out var st) && st.ValueKind == JsonValueKind.String)
                    {
                        sb.Append($"st:{st.GetString()};");
                    }
                    stepIdx++;
                }
                stepBatchIdx++;
                stepSw.Stop();
                stepTimes.Add(stepSw.Elapsed.TotalMilliseconds);
            }
        }

        if (isGui)
            sb.Append($"final_gui:steps={totalSteps}:cards={totalCards}:acts={totalActs}");
        else if (isStress)
            sb.Append($"final_stress:steps={totalSteps}:cards={totalCards}:acts={totalActs}");
        else
            sb.Append($"final:steps={totalSteps}:cards={totalCards}:acts={totalActs}");

        var hashBytes = sha.ComputeHash(Encoding.UTF8.GetBytes(sb.ToString()));
        var checksum = Convert.ToHexString(hashBytes).ToLowerInvariant();
        replaySw.Stop();
        var replayMs = replaySw.Elapsed.TotalMilliseconds;
        var totalMs = parseMs + replayMs;
        var stepsPerSec = replayMs > 0 ? (totalSteps / (replayMs / 1000.0)) : 0.0;

        var result = new Dictionary<string, object?>
        {
            ["target"] = "avalonia-dotnet-desktop",
            ["steps_processed"] = totalSteps,
            ["parse_duration_ms"] = Math.Round(parseMs, 2),
            ["replay_duration_ms"] = Math.Round(replayMs, 2),
            ["total_duration_ms"] = Math.Round(totalMs, 2),
            ["steps_per_sec"] = Math.Round(stepsPerSec, 2),
            ["checksum"] = checksum
        };

        if (isStress)
        {
            result["snapshots_retained"] = snapshotTree.Count;
            stepTimes.Sort();
            int p50Idx = (int)(stepTimes.Count * 0.50);
            int p95Idx = (int)(stepTimes.Count * 0.95);
            int p99Idx = (int)(stepTimes.Count * 0.99);
            if (p50Idx >= stepTimes.Count) p50Idx = stepTimes.Count - 1;
            if (p95Idx >= stepTimes.Count) p95Idx = stepTimes.Count - 1;
            if (p99Idx >= stepTimes.Count) p99Idx = stepTimes.Count - 1;

            result["p50_latency_ms"] = Math.Round(stepTimes[p50Idx], 3);
            result["p95_latency_ms"] = Math.Round(stepTimes[p95Idx], 3);
            result["p99_latency_ms"] = Math.Round(stepTimes[p99Idx], 3);
        }

        if (isGui && stepTimes.Count > 0)
        {
            int jankCount = 0;
            double maxFt = 0.0;
            foreach (var ft in stepTimes)
            {
                if (ft > 16.667) jankCount++;
                if (ft > maxFt) maxFt = ft;
            }

            stepTimes.Sort();
            int onePctIdx = (int)(stepTimes.Count * 0.99);
            int zeroPointOneIdx = (int)(stepTimes.Count * 0.999);
            if (onePctIdx >= stepTimes.Count) onePctIdx = stepTimes.Count - 1;
            if (zeroPointOneIdx >= stepTimes.Count) zeroPointOneIdx = stepTimes.Count - 1;

            double onePctMs = stepTimes[onePctIdx] > 0 ? stepTimes[onePctIdx] : 0.001;
            double zeroPointOneMs = stepTimes[zeroPointOneIdx] > 0 ? stepTimes[zeroPointOneIdx] : 0.001;

            double avgFps = stepTimes.Count / (replayMs / 1000.0);
            double onePctFps = 1000.0 / onePctMs;
            double zeroPointOneFps = 1000.0 / zeroPointOneMs;
            double jankPct = ((double)jankCount / stepTimes.Count) * 100.0;

            result["total_frames_rendered"] = stepTimes.Count;
            result["avg_fps"] = Math.Round(avgFps, 1);
            result["one_percent_low_fps"] = Math.Round(onePctFps, 1);
            result["zero_point_one_percent_low_fps"] = Math.Round(zeroPointOneFps, 1);
            result["jank_frame_count"] = jankCount;
            result["jank_percentage"] = Math.Round(jankPct, 2);
            result["max_frame_time_ms"] = Math.Round(maxFt, 2);
        }

        Console.WriteLine(JsonSerializer.Serialize(result));
    }
}

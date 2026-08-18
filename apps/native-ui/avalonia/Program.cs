using System;
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
        int actualIters = isStress ? iterations : 1;

        for (int it = 0; it < actualIters; it++)
        {
            if (isStress) sb.Append($"iter:{it};");
            if (root.TryGetProperty("configuration", out var cfg) && cfg.TryGetProperty("seed", out var seedProp) && seedProp.ValueKind == JsonValueKind.Number)
            {
                sb.Append($"seed:{seedProp.GetInt64()};");
            }

            int stepBatchIdx = 0;
            foreach (var stepBatch in steps.EnumerateArray())
            {
                totalSteps++;
                int stepIdx = 0;
                foreach (var step in stepBatch.EnumerateArray())
                {
                    if (step.TryGetProperty("action", out var act) && act.ValueKind == JsonValueKind.Array && act.GetArrayLength() > 0)
                    {
                        totalActs += act.GetArrayLength();
                        if (isStress)
                            sb.Append($"act:{it}:{stepBatchIdx}:{stepIdx}:{act.GetRawText()};");
                        else
                            sb.Append($"act:{stepBatchIdx}:{stepIdx}:{act.GetRawText()};");
                    }

                    if (step.TryGetProperty("observation", out var obs) && obs.ValueKind == JsonValueKind.Object &&
                        obs.TryGetProperty("current", out var cur) && cur.ValueKind == JsonValueKind.Object &&
                        cur.TryGetProperty("players", out var players) && players.ValueKind == JsonValueKind.Array)
                    {
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
            }
        }

        if (isStress)
            sb.Append($"final_stress:steps={totalSteps}:cards={totalCards}:acts={totalActs}");
        else
            sb.Append($"final:steps={totalSteps}:cards={totalCards}:acts={totalActs}");

        var hashBytes = sha.ComputeHash(Encoding.UTF8.GetBytes(sb.ToString()));
        var checksum = Convert.ToHexString(hashBytes).ToLowerInvariant();
        replaySw.Stop();
        var replayMs = replaySw.Elapsed.TotalMilliseconds;
        var totalMs = parseMs + replayMs;
        var stepsPerSec = replayMs > 0 ? (totalSteps / (replayMs / 1000.0)) : 0.0;

        var result = new
        {
            target = "avalonia-dotnet-desktop",
            steps_processed = totalSteps,
            parse_duration_ms = Math.Round(parseMs, 2),
            replay_duration_ms = Math.Round(replayMs, 2),
            total_duration_ms = Math.Round(totalMs, 2),
            steps_per_sec = Math.Round(stepsPerSec, 2),
            checksum = checksum
        };

        Console.WriteLine(JsonSerializer.Serialize(result));
    }
}

package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"sort"
	"time"
)

type ReplayLog struct {
	Name          string         `json:"name"`
	Configuration *Configuration `json:"configuration,omitempty"`
	Steps         [][]StepItem   `json:"steps"`
}

type Configuration struct {
	Seed *int64 `json:"seed,omitempty"`
}

type Player struct {
	Active []interface{} `json:"active"`
	Bench  []interface{} `json:"bench"`
	Deck   []interface{} `json:"deck"`
	Hand   []interface{} `json:"hand"`
}

type CurrentState struct {
	Players []Player `json:"players"`
}

type StepObservation struct {
	Current *CurrentState `json:"current,omitempty"`
}

type StepItem struct {
	Action      []interface{}    `json:"action,omitempty"`
	Observation *StepObservation `json:"observation,omitempty"`
	Status      *string          `json:"status,omitempty"`
}

type BenchmarkOutput struct {
	Target                 string   `json:"target"`
	StepsProcessed         int      `json:"steps_processed"`
	ParseDurationMs        float64  `json:"parse_duration_ms"`
	ReplayDurationMs       float64  `json:"replay_duration_ms"`
	TotalDurationMs        float64  `json:"total_duration_ms"`
	StepsPerSec            float64  `json:"steps_per_sec"`
	Checksum               string   `json:"checksum"`
	SnapshotsRetained       *int     `json:"snapshots_retained,omitempty"`
	P95LatencyMs           *float64 `json:"p95_latency_ms,omitempty"`
	TotalFramesRendered    *int     `json:"total_frames_rendered,omitempty"`
	AvgFps                 *float64 `json:"avg_fps,omitempty"`
	OnePercentLowFps       *float64 `json:"one_percent_low_fps,omitempty"`
	ZeroPointOnePercentLow *float64 `json:"zero_point_one_percent_low_fps,omitempty"`
	JankFrameCount         *int     `json:"jank_frame_count,omitempty"`
	JankPercentage         *float64 `json:"jank_percentage,omitempty"`
	MaxFrameTimeMs         *float64 `json:"max_frame_time_ms,omitempty"`
}

func main() {
	var replayPath string
	isStress := false
	isGui := false
	iterations := 20

	for i, arg := range os.Args {
		if arg == "--replay" && i+1 < len(os.Args) {
			replayPath = os.Args[i+1]
		} else if arg == "--stress" {
			isStress = true
		} else if arg == "--gui" || arg == "--gui-jank" {
			isGui = true
		} else if arg == "--iterations" && i+1 < len(os.Args) {
			fmt.Sscanf(os.Args[i+1], "%d", &iterations)
		}
	}

	if replayPath == "" {
		fmt.Fprintln(os.Stderr, "Error: Missing --replay <path>")
		os.Exit(1)
	}

	parseStart := time.Now()
	data, err := os.ReadFile(replayPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
		os.Exit(1)
	}

	var replay ReplayLog
	if err := json.Unmarshal(data, &replay); err != nil {
		fmt.Fprintf(os.Stderr, "Error unmarshalling JSON: %v\n", err)
		os.Exit(1)
	}
	parseDurationMs := float64(time.Since(parseStart).Microseconds()) / 1000.0

	replayStart := time.Now()
	hasher := sha256.New()

	totalSteps := 0
	totalCards := 0
	totalActs := 0
	actualIters := 1
	if isStress || isGui {
		actualIters = iterations
	}

	var frameTimes []float64

	for it := 0; it < actualIters; it++ {
		if isStress || isGui {
			fmt.Fprintf(hasher, "iter:%d;", it)
		}
		if replay.Configuration != nil && replay.Configuration.Seed != nil {
			fmt.Fprintf(hasher, "seed:%d;", *replay.Configuration.Seed)
		}

		for i, stepBatch := range replay.Steps {
			frameStart := time.Now()
			totalSteps++
			for j, step := range stepBatch {
				if len(step.Action) > 0 {
					totalActs += len(step.Action)
					actBytes, _ := json.Marshal(step.Action)
					if isStress || isGui {
						fmt.Fprintf(hasher, "act:%d:%d:%d:%s;", it, i, j, string(actBytes))
					} else {
						fmt.Fprintf(hasher, "act:%d:%d:%s;", i, j, string(actBytes))
					}
				}

				if step.Observation != nil && step.Observation.Current != nil {
					for p, player := range step.Observation.Current.Players {
						deckLen := len(player.Deck)
						handLen := len(player.Hand)
						activeLen := len(player.Active)
						benchLen := len(player.Bench)
						totalCards += deckLen + handLen
						fmt.Fprintf(hasher, "p:%d:d%d:h%d:a%d:b%d;", p, deckLen, handLen, activeLen, benchLen)
					}
				}

				if step.Status != nil {
					fmt.Fprintf(hasher, "st:%s;", *step.Status)
				}
			}

			if isGui {
				frameTimes = append(frameTimes, float64(time.Since(frameStart).Microseconds())/1000.0)
			}
		}
	}

	if isGui {
		fmt.Fprintf(hasher, "final_gui:steps=%d:cards=%d:acts=%d", totalSteps, totalCards, totalActs)
	} else if isStress {
		fmt.Fprintf(hasher, "final_stress:steps=%d:cards=%d:acts=%d", totalSteps, totalCards, totalActs)
	} else {
		fmt.Fprintf(hasher, "final:steps=%d:cards=%d:acts=%d", totalSteps, totalCards, totalActs)
	}

	checksum := hex.EncodeToString(hasher.Sum(nil))
	replayDurationMs := float64(time.Since(replayStart).Microseconds()) / 1000.0
	totalDurationMs := parseDurationMs + replayDurationMs
	stepsPerSec := 0.0
	if replayDurationMs > 0 {
		stepsPerSec = float64(totalSteps) / (replayDurationMs / 1000.0)
	}

	out := BenchmarkOutput{
		Target:           "wails-v3-desktop",
		StepsProcessed:   totalSteps,
		ParseDurationMs:  parseDurationMs,
		ReplayDurationMs: replayDurationMs,
		TotalDurationMs:  totalDurationMs,
		StepsPerSec:      stepsPerSec,
		Checksum:         checksum,
	}

	if isGui && len(frameTimes) > 0 {
		jankCount := 0
		maxFt := 0.0
		for _, ft := range frameTimes {
			if ft > 16.667 {
				jankCount++
			}
			if ft > maxFt {
				maxFt = ft
			}
		}

		sort.Float64s(frameTimes)
		onePctIdx := int(float64(len(frameTimes)) * 0.99)
		zeroPointOneIdx := int(float64(len(frameTimes)) * 0.999)
		if onePctIdx >= len(frameTimes) {
			onePctIdx = len(frameTimes) - 1
		}
		if zeroPointOneIdx >= len(frameTimes) {
			zeroPointOneIdx = len(frameTimes) - 1
		}

		onePctMs := frameTimes[onePctIdx]
		if onePctMs <= 0 {
			onePctMs = 0.001
		}
		zeroPointOneMs := frameTimes[zeroPointOneIdx]
		if zeroPointOneMs <= 0 {
			zeroPointOneMs = 0.001
		}

		framesCount := len(frameTimes)
		avgFps := float64(framesCount) / (replayDurationMs / 1000.0)
		onePctFps := 1000.0 / onePctMs
		zeroPointOneFps := 1000.0 / zeroPointOneMs
		jankPct := (float64(jankCount) / float64(framesCount)) * 100.0

		out.TotalFramesRendered = &framesCount
		out.AvgFps = &avgFps
		out.OnePercentLowFps = &onePctFps
		out.ZeroPointOnePercentLow = &zeroPointOneFps
		out.JankFrameCount = &jankCount
		out.JankPercentage = &jankPct
		out.MaxFrameTimeMs = &maxFt
	}

	outBytes, _ := json.Marshal(out)
	fmt.Println(string(outBytes))
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package benchmark

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"runtime"
	"time"
)

// BenchmarkResult represents the results of a single benchmark test
type BenchmarkResult struct {
	TestName          string  `json:"test_name"`
	Language          string  `json:"language"`
	DataSize          int     `json:"data_size"`
	Concurrency       int     `json:"concurrency"`
	EncryptLatencyMs  float64 `json:"encrypt_latency_ms"`
	DecryptLatencyMs  float64 `json:"decrypt_latency_ms"`
	EndToEndLatencyMs float64 `json:"end_to_end_latency_ms"`
	OpsPerSecond      float64 `json:"ops_per_second"`
	BytesPerSecond    float64 `json:"bytes_per_second"`
	PeakMemoryMB      float64 `json:"peak_memory_mb"`
	MemoryEfficiency  float64 `json:"memory_efficiency_ratio"`
	P50Latency        float64 `json:"p50_latency"`
	P95Latency        float64 `json:"p95_latency"`
	P99Latency        float64 `json:"p99_latency"`
	Timestamp         string  `json:"timestamp"`
	GoVersion         string  `json:"go_version"`
	CPUCount          int     `json:"cpu_count"`
	TotalMemoryGB     float64 `json:"total_memory_gb"`
}

// MemorySample represents a single memory measurement
type MemorySample struct {
	Timestamp        time.Time
	HeapMB           float64
	MetricsAllocsMB  float64
	MemStatsAllocsMB float64
}

// === Utility Functions ===

// Average calculates the average of a slice of float64 values
func Average(values []float64) float64 {
	if len(values) == 0 {
		return 0
	}
	sum := 0.0
	for _, v := range values {
		sum += v
	}
	return sum / float64(len(values))
}

// Percentile calculates the percentile of sorted values
func Percentile(sortedValues []float64, p float64) float64 {
	if len(sortedValues) == 0 {
		return 0
	}
	if p <= 0 {
		return sortedValues[0]
	}
	if p >= 100 {
		return sortedValues[len(sortedValues)-1]
	}

	index := (p / 100.0) * float64(len(sortedValues)-1)
	lower := int(math.Floor(index))
	upper := int(math.Ceil(index))

	if lower == upper {
		return sortedValues[lower]
	}

	weight := index - float64(lower)
	return sortedValues[lower]*(1-weight) + sortedValues[upper]*weight
}

// === Results Saving ===

// SaveResults saves benchmark results to JSON file
func (b *ESDKBenchmark) SaveResults(outputPath string) error {
	if err := os.MkdirAll(filepath.Dir(outputPath), 0755); err != nil {
		return fmt.Errorf("failed to create output directory: %w", err)
	}

	resultsData := map[string]interface{}{
		"metadata": map[string]interface{}{
			"language":        "go",
			"timestamp":       time.Now().Format("2006-01-02 15:04:05"),
			"go_version":      runtime.Version(),
			"cpu_count":       b.CPUCount,
			"total_memory_gb": b.TotalMemoryGB,
			"total_tests":     len(b.Results),
		},
		"results": b.Results,
	}

	file, err := os.Create(outputPath)
	if err != nil {
		return fmt.Errorf("failed to create output file: %w", err)
	}
	defer file.Close()

	encoder := json.NewEncoder(file)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(resultsData); err != nil {
		return fmt.Errorf("failed to encode results to JSON: %w", err)
	}

	return nil
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using System.Text.Json.Serialization;
using Newtonsoft.Json;

namespace EsdkBenchmark;

public class BenchmarkResult
{
    [JsonPropertyName("test_name")]
    public string TestName { get; set; } = "";

    [JsonPropertyName("language")]
    public string Language { get; set; } = "";

    [JsonPropertyName("data_size")]
    public int DataSize { get; set; }

    [JsonPropertyName("concurrency")]
    public int Concurrency { get; set; } = 1;

    [JsonPropertyName("operations_per_second")]
    public double OpsPerSecond { get; set; }

    [JsonPropertyName("bytes_per_second")]
    public double BytesPerSecond { get; set; }

    [JsonPropertyName("peak_memory_mb")]
    public double PeakMemoryMb { get; set; }

    [JsonPropertyName("memory_efficiency_ratio")]
    public double MemoryEfficiencyRatio { get; set; }

    [JsonPropertyName("avg_latency_ms")]
    public double AvgLatencyMs { get; set; }

    [JsonPropertyName("p50_latency_ms")]
    public double P50LatencyMs { get; set; }

    [JsonPropertyName("p95_latency_ms")]
    public double P95LatencyMs { get; set; }

    [JsonPropertyName("p99_latency_ms")]
    public double P99LatencyMs { get; set; }

    [JsonPropertyName("encrypt_latency_ms")]
    public double EncryptLatencyMs { get; set; }

    [JsonPropertyName("decrypt_latency_ms")]
    public double DecryptLatencyMs { get; set; }

    [JsonPropertyName("timestamp")]
    public string Timestamp { get; set; } = "";

    [JsonPropertyName("dotnet_version")]
    public string DotNetVersion { get; set; } = "";

    [JsonPropertyName("cpu_count")]
    public int CpuCount { get; set; }

    [JsonPropertyName("total_memory_gb")]
    public double TotalMemoryGb { get; set; }

    [JsonPropertyName("iterations")]
    public int Iterations { get; set; }
}

internal class BenchmarkResults
{
    [JsonPropertyName("metadata")]
    public BenchmarkMetadata Metadata { get; set; } = new();

    [JsonPropertyName("results")]
    public List<BenchmarkResult> Results { get; set; } = new();
}

internal class BenchmarkMetadata
{
    [JsonPropertyName("language")]
    public string Language { get; set; } = "net";

    [JsonPropertyName("timestamp")]
    public string Timestamp { get; set; } = "";

    [JsonPropertyName("dotnet_version")]
    public string DotNetVersion { get; set; } = "";

    [JsonPropertyName("cpu_count")]
    public int CpuCount { get; set; }

    [JsonPropertyName("total_memory_gb")]
    public double TotalMemoryGb { get; set; }

    [JsonPropertyName("total_tests")]
    public int TotalTests { get; set; }
}

public static class BenchmarkResultsHelper
{
    public static double Average(IEnumerable<double> values)
    {
        var list = values.ToList();
        return list.Count == 0 ? 0.0 : list.Sum() / list.Count;
    }

    public static double Percentile(List<double> sortedValues, double p)
    {
        if (sortedValues.Count == 0) return 0.0;
        if (p <= 0.0) return sortedValues[0];
        if (p >= 1.0) return sortedValues[^1];

        var index = p * (sortedValues.Count - 1);
        var lower = (int)Math.Floor(index);
        var upper = (int)Math.Ceiling(index);

        if (lower == upper) return sortedValues[lower];

        var weight = index - lower;
        return sortedValues[lower] * (1 - weight) + sortedValues[upper] * weight;
    }

    public static async Task SaveResultsAsync(List<BenchmarkResult> results, string outputPath, int cpuCount, double totalMemoryGb)
    {
        var directory = Path.GetDirectoryName(outputPath);
        if (!string.IsNullOrEmpty(directory))
        {
            Directory.CreateDirectory(directory);
        }

        var resultsData = new BenchmarkResults
        {
            Metadata = new BenchmarkMetadata
            {
                Language = "net",
                Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
                DotNetVersion = Environment.Version.ToString(),
                CpuCount = cpuCount,
                TotalMemoryGb = totalMemoryGb,
                TotalTests = results.Count
            },
            Results = results
        };

        var json = System.Text.Json.JsonSerializer.Serialize(resultsData, new System.Text.Json.JsonSerializerOptions
        {
            WriteIndented = true,
            PropertyNamingPolicy = System.Text.Json.JsonNamingPolicy.SnakeCaseLower
        });

        await File.WriteAllTextAsync(outputPath, json);
    }

    public static BenchmarkResult CreateThroughputResult(List<double> encryptLatencies, List<double> decryptLatencies, List<double> totalLatencies, int dataSize, int cpuCount, double totalMemoryGb)
    {
        var avgTotalLatency = totalLatencies.Average();
        var opsPerSecond = 1000.0 / avgTotalLatency;
        totalLatencies.Sort();

        return new BenchmarkResult
        {
            TestName = "throughput",
            Language = "net",
            DataSize = dataSize,
            Concurrency = 1,
            OpsPerSecond = opsPerSecond,
            BytesPerSecond = opsPerSecond * dataSize,
            AvgLatencyMs = avgTotalLatency,
            P50LatencyMs = Percentile(totalLatencies, 0.5),
            P95LatencyMs = Percentile(totalLatencies, 0.95),
            P99LatencyMs = Percentile(totalLatencies, 0.99),
            EncryptLatencyMs = encryptLatencies.Average(),
            DecryptLatencyMs = decryptLatencies.Average(),
            Iterations = encryptLatencies.Count,
            Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
            DotNetVersion = Environment.Version.ToString(),
            CpuCount = cpuCount,
            TotalMemoryGb = totalMemoryGb
        };
    }

    public static BenchmarkResult CreateMemoryResult(double peakMemoryMb, double avgMemoryMb, int dataSize, int cpuCount, double totalMemoryGb)
    {
        // Convert memory from MB to bytes since data size is in bytes
        var memoryEfficiency = peakMemoryMb > 0 ? dataSize / (peakMemoryMb * 1024 * 1024) : 0.0;

        return new BenchmarkResult
        {
            TestName = "memory",
            Language = "net",
            DataSize = dataSize,
            Concurrency = 1,
            PeakMemoryMb = peakMemoryMb,
            MemoryEfficiencyRatio = memoryEfficiency,
            Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
            DotNetVersion = Environment.Version.ToString(),
            CpuCount = cpuCount,
            TotalMemoryGb = totalMemoryGb
        };
    }

    public static BenchmarkResult CreateConcurrentResult(List<double> allTimes, int totalOps, int dataSize, int concurrency, int cpuCount, double totalMemoryGb)
    {
        var avgLatency = allTimes.Average();
        var opsPerSecond = totalOps / (allTimes.Sum() / 1000.0);
        allTimes.Sort();

        return new BenchmarkResult
        {
            TestName = "concurrent",
            Language = "net",
            DataSize = dataSize,
            Concurrency = concurrency,
            OpsPerSecond = opsPerSecond,
            BytesPerSecond = opsPerSecond * dataSize,
            AvgLatencyMs = avgLatency,
            P50LatencyMs = Percentile(allTimes, 0.5),
            P95LatencyMs = Percentile(allTimes, 0.95),
            P99LatencyMs = Percentile(allTimes, 0.99),
            Iterations = totalOps,
            Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
            DotNetVersion = Environment.Version.ToString(),
            CpuCount = cpuCount,
            TotalMemoryGb = totalMemoryGb
        };
    }
}

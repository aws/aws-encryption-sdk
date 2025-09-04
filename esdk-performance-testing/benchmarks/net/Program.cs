// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using System.Diagnostics;
using System.Runtime;
using System.Text.Json;
using CommandLine;
using Newtonsoft.Json;
using ShellProgressBar;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;
using MathNet.Numerics.Statistics;
using AWS.Cryptography.EncryptionSDK;
using AWS.Cryptography.MaterialProviders;
using System.Security.Cryptography;

namespace Amazon.Esdk.Benchmark;

/// <summary>
/// ESDK Performance Benchmark Suite - .NET Implementation
/// 
/// This application provides comprehensive performance testing for the AWS Encryption SDK (ESDK)
/// .NET runtime, measuring throughput, latency, memory usage, and scalability.
/// Follows the same structure and configuration approach as the Java implementation.
/// </summary>
public class Program
{

    public static async Task Main(string[] args)
    {
        var options = ParseArgs(args);
        if (options == null) return;

        try
        {
            var benchmark = new ESDKBenchmark(options.ConfigPath);
            
            if (options.QuickTest)
            {
                benchmark.AdjustForQuickTest();
            }

            var results = await benchmark.RunAllBenchmarksAsync(options.QuickTest);
            await benchmark.SaveResultsAsync(options.OutputPath);
            PrintSummary(results, options.OutputPath);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Benchmark failed: {ex.Message}");
        }
    }

    private static CommandLineOptions? ParseArgs(string[] args)
    {
        var options = new CommandLineOptions
        {
            ConfigPath = "../../config/test-scenarios.yaml",
            OutputPath = "../../results/raw-data/net_results.json"
        };
        
        for (int i = 0; i < args.Length; i++)
        {
            switch (args[i])
            {
                case "--config":
                case "-c":
                    if (i + 1 < args.Length) options.ConfigPath = args[++i];
                    break;
                case "--output":
                case "-o":
                    if (i + 1 < args.Length) options.OutputPath = args[++i];
                    break;
                case "--quick":
                case "-q":
                    options.QuickTest = true;
                    break;
                case "--help":
                case "-h":
                    PrintHelp();
                    return null;
            }
        }
        return options;
    }

    private static void PrintHelp()
    {
        Console.WriteLine("ESDK .NET Benchmark");
        Console.WriteLine("Usage: EsdkBenchmark [options]");
        Console.WriteLine("Options:");
        Console.WriteLine("  --config, -c    Path to test configuration file (default: ../../config/test-scenarios.yaml)");
        Console.WriteLine("  --output, -o    Path to output results file (default: ../../results/raw-data/net_results.json)");
        Console.WriteLine("  --quick, -q     Run quick test with reduced iterations");
        Console.WriteLine("  --help, -h      Show this help message");
    }

    private static void PrintSummary(List<BenchmarkResult> results, string outputPath)
    {
        Console.WriteLine("\n=== ESDK .NET Benchmark Summary ===");
        Console.WriteLine($"Total tests completed: {results.Count}");
        Console.WriteLine($"Results saved to: {outputPath}");

        if (results.Any())
        {
            var throughputResults = results.Where(r => r.TestName == "throughput").ToList();
            if (throughputResults.Any())
            {
                var maxThroughput = throughputResults.Max(r => r.OpsPerSecond);
                Console.WriteLine($"Maximum throughput: {maxThroughput:F2} ops/sec");
            }
        }
    }
}

/// <summary>
/// Command line options for the benchmark
/// </summary>
public class CommandLineOptions
{
    [Option('c', "config", Default = "../../config/test-scenarios.yaml", HelpText = "Path to test configuration file")]
    public string ConfigPath { get; set; } = string.Empty;

    [Option('o', "output", Default = "../../results/raw-data/net_results.json", HelpText = "Path to output results file")]
    public string OutputPath { get; set; } = string.Empty;

    [Option('q', "quick", Default = false, HelpText = "Run quick test with reduced iterations")]
    public bool QuickTest { get; set; }

    [Option('v', "verbose", Default = false, HelpText = "Enable verbose logging")]
    public bool Verbose { get; set; }

    [Option('h', "help", Default = false, HelpText = "Show help message")]
    public bool Help { get; set; }
}

/// <summary>
/// Benchmark result for a single test
/// </summary>
public class BenchmarkResult
{
    [JsonProperty("test_name")]
    public string TestName { get; set; } = string.Empty;
    
    [JsonProperty("language")]
    public string Language { get; set; } = "net";
    
    [JsonProperty("data_size")]
    public int DataSize { get; set; }
    
    [JsonProperty("concurrency")]
    public int Concurrency { get; set; }

    // Performance metrics
    [JsonProperty("encrypt_latency_ms")]
    public double EncryptLatencyMs { get; set; }
    
    [JsonProperty("decrypt_latency_ms")]
    public double DecryptLatencyMs { get; set; }
    
    [JsonProperty("end_to_end_latency_ms")]
    public double EndToEndLatencyMs { get; set; }
    
    [JsonProperty("ops_per_second")]
    public double OpsPerSecond { get; set; }
    
    [JsonProperty("bytes_per_second")]
    public double BytesPerSecond { get; set; }

    // Memory metrics
    [JsonProperty("peak_memory_mb")]
    public double PeakMemoryMb { get; set; }
    
    [JsonProperty("memory_efficiency_ratio")]
    public double MemoryEfficiencyRatio { get; set; }

    // Statistical metrics
    [JsonProperty("p50_latency")]
    public double P50Latency { get; set; }
    
    [JsonProperty("p95_latency")]
    public double P95Latency { get; set; }
    
    [JsonProperty("p99_latency")]
    public double P99Latency { get; set; }

    // Environment info
    [JsonProperty("timestamp")]
    public string Timestamp { get; set; } = string.Empty;
    
    [JsonProperty("dotnet_version")]
    public string DotNetVersion { get; set; } = string.Empty;
    
    [JsonProperty("cpu_count")]
    public int CpuCount { get; set; }
    
    [JsonProperty("total_memory_gb")]
    public double TotalMemoryGb { get; set; }
}

/// <summary>
/// Test configuration loaded from YAML - matches YAML structure
/// </summary>
public class TestConfig
{
    [YamlMember(Alias = "data_sizes")]
    public DataSizes DataSizes { get; set; } = new();

    [YamlMember(Alias = "iterations")]
    public IterationConfig Iterations { get; set; } = new();

    [YamlMember(Alias = "concurrency_levels")]
    public List<int> ConcurrencyLevels { get; set; } = new();

    [YamlMember(Alias = "quick_config")]
    public QuickConfig? QuickConfig { get; set; }
}

public class DataSizes
{
    [YamlMember(Alias = "small")]
    public List<int> Small { get; set; } = new();

    [YamlMember(Alias = "medium")]
    public List<int> Medium { get; set; } = new();

    [YamlMember(Alias = "large")]
    public List<int> Large { get; set; } = new();
}

public class IterationConfig
{
    [YamlMember(Alias = "warmup")]
    public int Warmup { get; set; }

    [YamlMember(Alias = "measurement")]
    public int Measurement { get; set; }
}

public class QuickConfig
{
    [YamlMember(Alias = "data_sizes")]
    public QuickDataSizes DataSizes { get; set; } = new();

    [YamlMember(Alias = "iterations")]
    public QuickIterationConfig Iterations { get; set; } = new();

    [YamlMember(Alias = "concurrency_levels")]
    public List<int> ConcurrencyLevels { get; set; } = new();

    [YamlMember(Alias = "test_types")]
    public List<string> TestTypes { get; set; } = new();
}

public class QuickDataSizes
{
    [YamlMember(Alias = "small")]
    public List<int> Small { get; set; } = new();
}

public class QuickIterationConfig
{
    [YamlMember(Alias = "warmup")]
    public int Warmup { get; set; }

    [YamlMember(Alias = "measurement")]
    public int Measurement { get; set; }
}

/// <summary>
/// Main benchmark class - follows Go/Rust implementation structure
/// </summary>
public class ESDKBenchmark
{
    private TestConfig _config;
    private readonly List<BenchmarkResult> _results = new();
    private readonly Random _random = new();

    // Constants for memory testing
    private const int MemoryTestIterations = 5;
    private const int SamplingIntervalMs = 1;
    private const int GcSettleTimeMs = 5;
    private const int FinalSampleWaitMs = 2;

    // System information
    private readonly int _cpuCount;
    private readonly double _totalMemoryGb;
    
    // ESDK components
    private MaterialProviders _materialProviders = null!;
    private ESDK _encryptionSdk = null!;
    private IKeyring _keyring = null!;

    public ESDKBenchmark(string configPath)
    {
        _config = LoadConfig(configPath);

        // Get system information
        _cpuCount = Environment.ProcessorCount;
        _totalMemoryGb = GetTotalMemoryGb();

        // Configure GC for performance
        GCSettings.LatencyMode = GCLatencyMode.SustainedLowLatency;
        
        // Setup ESDK
        var setupError = SetupEsdk();
        if (setupError != null)
        {
            throw new InvalidOperationException($"Failed to setup ESDK: {setupError}");
        }

        Console.WriteLine($"Initialized ESDK Benchmark - CPU cores: {_cpuCount}, Memory: {_totalMemoryGb:F1}GB");
    }

    private string? SetupEsdk()
    {
        try
        {
            // Initialize material providers client
            _materialProviders = new MaterialProviders(new MaterialProvidersConfig());

            // Create 256-bit AES key using .NET crypto
            var key = new byte[32];
            using (var rng = RandomNumberGenerator.Create())
            {
                rng.GetBytes(key);
            }

            // Create raw AES keyring
            var createKeyringInput = new CreateRawAesKeyringInput
            {
                KeyNamespace = "esdk-performance-test",
                KeyName = "test-aes-256-key",
                WrappingKey = new MemoryStream(key),
                WrappingAlg = AesWrappingAlg.ALG_AES256_GCM_IV12_TAG16
            };
            _keyring = _materialProviders.CreateRawAesKeyring(createKeyringInput);

            // Create ESDK client with commitment policy
            var esdkConfig = new AwsEncryptionSdkConfig
            {
                CommitmentPolicy = ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT
            };
            _encryptionSdk = new ESDK(esdkConfig);

            Console.WriteLine("ESDK client initialized successfully");
            return null;
        }
        catch (Exception ex)
        {
            return ex.Message;
        }
    }

    /// <summary>
    /// Load test configuration from YAML file - matches Java approach
    /// </summary>
    private TestConfig LoadConfig(string configPath)
    {
        if (!File.Exists(configPath))
        {
            throw new FileNotFoundException($"Config file not found: {configPath}");
        }

        try
        {
            var yaml = File.ReadAllText(configPath);
            var deserializer = new DeserializerBuilder()
                .WithNamingConvention(UnderscoredNamingConvention.Instance)
                .IgnoreUnmatchedProperties()
                .Build();

            return deserializer.Deserialize<TestConfig>(yaml);
        }
        catch (Exception ex)
        {
            throw new InvalidOperationException($"Failed to load config file: {ex.Message}", ex);
        }
    }

    /// <summary>
    /// Create default configuration - matches Java defaults
    /// </summary>
    private static TestConfig CreateDefaultConfig()
    {
        return new TestConfig
        {
            DataSizes = new DataSizes
            {
                Small = new() { 1024, 5120, 10240 },
                Medium = new() { 102400, 512000, 1048576 },
                Large = new() { 10485760, 52428800, 104857600 }
            },
            Iterations = new IterationConfig
            {
                Warmup = 5,
                Measurement = 10
            },
            ConcurrencyLevels = new() { 1, 2, 4, 8 }
        };
    }

    /// <summary>
    /// Check if a test type should run based on configuration
    /// </summary>
    private bool ShouldRunTestType(string testType, bool isQuickMode)
    {
        if (isQuickMode)
        {
            if (_config.QuickConfig != null && _config.QuickConfig.TestTypes.Count > 0)
            {
                return _config.QuickConfig.TestTypes.Contains(testType);
            }
        }
        return true; // Run all tests if not in quick mode or no test_types specified
    }

    /// <summary>
    /// Adjust configuration for quick test - uses quick_config from YAML
    /// </summary>
    public void AdjustForQuickTest()
    {
        if (_config.QuickConfig != null)
        {
            _config.Iterations = new IterationConfig
            {
                Warmup = _config.QuickConfig.Iterations.Warmup,
                Measurement = _config.QuickConfig.Iterations.Measurement
            };
            _config.DataSizes = new DataSizes
            {
                Small = _config.QuickConfig.DataSizes.Small,
                Medium = new List<int>(),
                Large = new List<int>()
            };
            _config.ConcurrencyLevels = _config.QuickConfig.ConcurrencyLevels;
        }
        else
        {
            throw new InvalidOperationException("Quick mode requested but no quick_config found in config file");
        }
    }

    /// <summary>
    /// Generate test data of specified size
    /// </summary>
    private byte[] GenerateTestData(int size)
    {
        var data = new byte[size];
        _random.NextBytes(data);
        return data;
    }

    /// <summary>
    /// Calculate average of a list of values
    /// </summary>
    private static double Average(List<double> values)
    {
        return values.Count > 0 ? values.Average() : 0.0;
    }

    /// <summary>
    /// Calculate percentile of a sorted list of values
    /// </summary>
    private static double Percentile(List<double> sortedValues, double percentile)
    {
        if (sortedValues.Count == 0) return 0.0;
        
        var index = percentile * (sortedValues.Count - 1);
        var lower = (int)Math.Floor(index);
        var upper = (int)Math.Ceiling(index);
        
        if (lower == upper)
            return sortedValues[lower];
        
        var weight = index - lower;
        return sortedValues[lower] * (1 - weight) + sortedValues[upper] * weight;
    }

    /// <summary>
    /// Run encrypt-decrypt cycle matching Go/Rust implementation
    /// </summary>
    private (double encryptMs, double decryptMs, string? error) RunEncryptDecryptCycle(byte[] data)
    {
        try
        {
            var plaintext = new MemoryStream(data);
            
            // Create encryption context matching Go/Rust
            var encryptionContext = new Dictionary<string, string>()
            {
                {"purpose", "performance-test"},
                {"size", data.Length.ToString()}
            };

            // Encrypt
            var encryptStart = Stopwatch.GetTimestamp();
            var encryptInput = new EncryptInput
            {
                Plaintext = plaintext,
                Keyring = _keyring,
                EncryptionContext = encryptionContext
            };
            var encryptOutput = _encryptionSdk.Encrypt(encryptInput);
            var encryptMs = Stopwatch.GetElapsedTime(encryptStart).TotalMilliseconds;

            // Decrypt
            var decryptStart = Stopwatch.GetTimestamp();
            var decryptInput = new DecryptInput
            {
                Ciphertext = encryptOutput.Ciphertext,
                Keyring = _keyring
            };
            var decryptOutput = _encryptionSdk.Decrypt(decryptInput);
            var decryptMs = Stopwatch.GetElapsedTime(decryptStart).TotalMilliseconds;

            // Verify data integrity
            if (!data.SequenceEqual(decryptOutput.Plaintext.ToArray()))
            {
                return (0, 0, "data integrity check failed");
            }

            return (encryptMs, decryptMs, null);
        }
        catch (Exception ex)
        {
            return (0, 0, ex.Message);
        }
    }

    /// <summary>
    /// Run throughput benchmark test - matches Java structure
    /// </summary>
    /// <summary>
    /// Run throughput test matching Go/Rust implementation
    /// </summary>
    public BenchmarkResult? RunThroughputTest(int dataSize, int iterations, ProgressBar? progressBar = null)
    {
        Action<string> log = progressBar != null ? progressBar.WriteLine : Console.WriteLine;
        log($"Running throughput test - Size: {dataSize} bytes, Iterations: {iterations}");

        var testData = GenerateTestData(dataSize);

        // Warmup
        for (int i = 0; i < _config.Iterations.Warmup; i++)
        {
            var (_, _, error) = RunEncryptDecryptCycle(testData);
            if (error != null)
            {
                Console.WriteLine($"Warmup iteration {i} failed: {error}");
                return null;
            }
        }

        // Measurement runs
        var encryptLatencies = new List<double>();
        var decryptLatencies = new List<double>();
        var endToEndLatencies = new List<double>();
        long totalBytes = 0;

        var startTime = Stopwatch.GetTimestamp();
        for (int i = 0; i < iterations; i++)
        {
            var iterationStart = Stopwatch.GetTimestamp();
            var (encryptMs, decryptMs, error) = RunEncryptDecryptCycle(testData);
            if (error != null)
            {
                Console.WriteLine($"Measurement iteration {i} failed: {error}");
                continue;
            }
            var iterationDuration = Stopwatch.GetElapsedTime(iterationStart).TotalMilliseconds;

            encryptLatencies.Add(encryptMs);
            decryptLatencies.Add(decryptMs);
            endToEndLatencies.Add(iterationDuration);
            totalBytes += dataSize;
        }
        var totalDuration = Stopwatch.GetElapsedTime(startTime).TotalSeconds;

        if (!encryptLatencies.Any())
        {
            Console.WriteLine("All test iterations failed");
            return null;
        }

        // Calculate metrics
        endToEndLatencies.Sort();
        var result = new BenchmarkResult
        {
            TestName = "throughput",
            Language = "net",
            DataSize = dataSize,
            Concurrency = 1,
            EncryptLatencyMs = Average(encryptLatencies),
            DecryptLatencyMs = Average(decryptLatencies),
            EndToEndLatencyMs = Average(endToEndLatencies),
            OpsPerSecond = iterations / totalDuration,
            BytesPerSecond = totalBytes / totalDuration,
            P50Latency = Percentile(endToEndLatencies, 0.50),
            P95Latency = Percentile(endToEndLatencies, 0.95),
            P99Latency = Percentile(endToEndLatencies, 0.99),
            Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
            DotNetVersion = Environment.Version.ToString(),
            CpuCount = _cpuCount,
            TotalMemoryGb = _totalMemoryGb
        };

        Console.WriteLine($"Throughput test completed - Ops/sec: {result.OpsPerSecond:F2}, MB/sec: {result.BytesPerSecond / (1024 * 1024):F2}");

        return result;
    }

    /// <summary>
    /// Run memory usage benchmark test - matches Java structure
    /// </summary>
    /// <summary>
    /// Run memory test matching Go/Rust implementation
    /// </summary>
    public BenchmarkResult? RunMemoryTest(int dataSize, ProgressBar? progressBar = null)
    {
        Action<string> log = progressBar != null ? progressBar.WriteLine : Console.WriteLine;
        log($"Running memory test - Size: {dataSize} bytes ({MemoryTestIterations} iterations, continuous sampling)");

        var data = GenerateTestData(dataSize);
        var peakHeap = 0.0;
        var peakAllocations = 0.0;
        var avgHeapValues = new List<double>();

        // Run iterations
        for (int i = 0; i < MemoryTestIterations; i++)
        {
            GC.Collect();
            GC.WaitForPendingFinalizers();
            GC.Collect();
            Thread.Sleep(GcSettleTimeMs);

            // Get baseline
            var beforeHeap = GC.GetTotalMemory(false);
            var beforeAllocated = (long)GC.GetTotalAllocatedBytes(false);

            // Start continuous sampling
            var stopSampling = new CancellationTokenSource();
            var continuousSamples = new List<(double HeapMB, double AllocsMB)>();
            var samplingTask = Task.Run(() => SampleMemoryContinuously(beforeHeap, (ulong)beforeAllocated, stopSampling.Token, continuousSamples));

            // Run operation
            var operationStart = Stopwatch.GetTimestamp();
            var (_, _, error) = RunEncryptDecryptCycle(data);
            var operationDuration = Stopwatch.GetElapsedTime(operationStart);

            stopSampling.Cancel();
            Thread.Sleep(FinalSampleWaitMs);

            if (error != null)
            {
                Console.WriteLine($"Memory test iteration {i + 1} failed: {error}");
                continue;
            }

            // Analyze samples
            var iterPeakHeap = 0.0;
            var iterTotalAllocs = 0.0;
            var iterAvgHeap = 0.0;

            if (continuousSamples.Count > 0)
            {
                var heapSum = 0.0;
                foreach (var sample in continuousSamples)
                {
                    if (sample.HeapMB > iterPeakHeap)
                        iterPeakHeap = sample.HeapMB;
                    if (sample.AllocsMB > iterTotalAllocs)
                        iterTotalAllocs = sample.AllocsMB;
                    heapSum += sample.HeapMB;
                }
                iterAvgHeap = heapSum / continuousSamples.Count;
            }

            if (iterPeakHeap > peakHeap)
                peakHeap = iterPeakHeap;
            if (iterTotalAllocs > peakAllocations)
                peakAllocations = iterTotalAllocs;

            avgHeapValues.Add(iterAvgHeap);

            log($"=== Iteration {i + 1} === Peak Heap: {iterPeakHeap:F2} MB, Total Allocs: {iterTotalAllocs:F2} MB, Avg Heap: {iterAvgHeap:F2} MB ({operationDuration.TotalMilliseconds:F0}ms, {continuousSamples.Count} samples)");
        }

        var avgHeap = avgHeapValues.Count > 0 ? avgHeapValues.Average() : 0.0;
        var memoryEfficiency = dataSize > 0 ? (dataSize / (1024.0 * 1024.0)) / Math.Max(peakHeap, 1.0) : 0.0;

        log("\nMemory Summary:");
        log($"- Absolute Peak Heap: {peakHeap:F2} MB (across all runs)");
        log($"- Average Heap: {avgHeap:F2} MB (across all runs)");
        log($"- Total Allocations: {peakAllocations:F2} MB (max across all runs)");

        var result = new BenchmarkResult
        {
            TestName = "memory",
            Language = "net",
            DataSize = dataSize,
            Concurrency = 1,
            PeakMemoryMb = peakHeap,
            MemoryEfficiencyRatio = memoryEfficiency,
            Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
            DotNetVersion = Environment.Version.ToString(),
            CpuCount = _cpuCount,
            TotalMemoryGb = _totalMemoryGb
        };

        Console.WriteLine($"Memory test completed: {result.PeakMemoryMb:F2} MB peak");
        return result;
    }

    private void SampleMemoryContinuously(long baselineHeap, ulong baselineAllocs, CancellationToken cancellationToken, List<(double HeapMB, double AllocsMB)> samples)
    {
        while (!cancellationToken.IsCancellationRequested)
        {
            try
            {
                var currentHeap = GC.GetTotalMemory(false);
                var currentAllocs = GC.GetTotalAllocatedBytes(false);

                var heapMB = (currentHeap - baselineHeap) / (1024.0 * 1024.0);
                var allocsMB = ((long)currentAllocs - (long)baselineAllocs) / (1024.0 * 1024.0);

                lock (samples)
                {
                    samples.Add((Math.Max(0, heapMB), Math.Max(0, allocsMB)));
                }

                Thread.Sleep(SamplingIntervalMs);
            }
            catch (OperationCanceledException)
            {
                break;
            }
        }
    }

    /// <summary>
    /// Run concurrent operations benchmark test - matches Java structure
    /// </summary>
    /// <summary>
    /// Run concurrent test matching Go/Rust implementation
    /// </summary>
    public BenchmarkResult? RunConcurrentTest(int dataSize, int concurrency, int iterationsPerWorker, ProgressBar? progressBar = null)
    {
        Console.WriteLine($"Running concurrent test - Size: {dataSize} bytes, Concurrency: {concurrency}");

        var data = GenerateTestData(dataSize);
        var allTimes = new List<double>();
        var timesMutex = new object();
        var errorOccurred = false;

        var startTime = Stopwatch.GetTimestamp();

        // Launch workers
        var tasks = new Task[concurrency];
        for (int i = 0; i < concurrency; i++)
        {
            int workerID = i;
            tasks[i] = Task.Run(() =>
            {
                var workerTimes = new List<double>();
                for (int j = 0; j < iterationsPerWorker; j++)
                {
                    var iterStart = Stopwatch.GetTimestamp();
                    var (_, _, error) = RunEncryptDecryptCycle(data);
                    if (error != null)
                    {
                        Console.WriteLine($"Worker {workerID} iteration {j} failed: {error}");
                        errorOccurred = true;
                        return;
                    }
                    workerTimes.Add(Stopwatch.GetElapsedTime(iterStart).TotalMilliseconds);
                }

                lock (timesMutex)
                {
                    allTimes.AddRange(workerTimes);
                }
            });
        }

        Task.WaitAll(tasks);
        var totalDuration = Stopwatch.GetElapsedTime(startTime).TotalSeconds;

        if (errorOccurred || !allTimes.Any())
        {
            Console.WriteLine("Concurrent test failed - no successful operations");
            return null;
        }

        // Calculate metrics
        allTimes.Sort();
        var totalOperations = allTimes.Count;
        var concurrentOpsPerSecond = totalOperations / totalDuration;

        var result = new BenchmarkResult
        {
            TestName = "concurrent",
            Language = "net",
            DataSize = dataSize,
            Concurrency = concurrency,
            EndToEndLatencyMs = Average(allTimes),
            OpsPerSecond = concurrentOpsPerSecond,
            BytesPerSecond = dataSize * concurrentOpsPerSecond,
            P50Latency = Percentile(allTimes, 0.50),
            P95Latency = Percentile(allTimes, 0.95),
            P99Latency = Percentile(allTimes, 0.99),
            Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
            DotNetVersion = Environment.Version.ToString(),
            CpuCount = _cpuCount,
            TotalMemoryGb = _totalMemoryGb
        };

        Console.WriteLine($"Concurrent test completed: {result.OpsPerSecond:F2} ops/sec @ {concurrency} threads");

        return result;
    }

    /// <summary>
    /// Run all configured benchmark tests - matches Java structure
    /// </summary>
    public async Task<List<BenchmarkResult>> RunAllBenchmarksAsync(bool isQuickMode = false)
    {
        Console.WriteLine("Starting comprehensive ESDK benchmark suite");

        var allResults = new List<BenchmarkResult>();

        // Get test parameters from config - matches Java approach
        var dataSizes = new List<int>();
        // Collect all data sizes from all categories
        dataSizes.AddRange(_config.DataSizes.Small);
        dataSizes.AddRange(_config.DataSizes.Medium);
        dataSizes.AddRange(_config.DataSizes.Large);

        var concurrencyLevels = _config.ConcurrencyLevels;
        var iterations = _config.Iterations.Measurement;

        var totalTests = dataSizes.Count * (concurrencyLevels.Count + 2);

        Console.WriteLine($"Running {totalTests} total tests");

        var progressOptions = new ProgressBarOptions
        {
            ProgressCharacter = '█',
            ProgressBarOnBottom = true,
            ForegroundColor = ConsoleColor.Green,
            BackgroundColor = ConsoleColor.DarkGray,
            CollapseWhenFinished = false,
            DisplayTimeInRealTime = true,
            ShowEstimatedDuration = true
        };

        using var overallProgress = new ProgressBar(totalTests, "Overall Progress", progressOptions);
        int completedTests = 0;

        // Throughput tests
        if (ShouldRunTestType("throughput", isQuickMode))
        {
            overallProgress.WriteLine("Running throughput tests...");
            foreach (var dataSize in dataSizes)
            {
                try
                {
                    var result = RunThroughputTest(dataSize, iterations, overallProgress);
                    if (result != null)
                    {
                        allResults.Add(result);
                    }
                }
                catch (Exception ex)
                {
                    overallProgress.WriteLine($"Throughput test failed: {ex.Message}");
                }
                
                completedTests++;
                overallProgress.Tick();
            }
        }
        else
        {
            overallProgress.WriteLine("Skipping throughput tests (not in test_types)");
        }

        // Memory tests
        if (ShouldRunTestType("memory", isQuickMode))
        {
            overallProgress.WriteLine("Running memory tests...");
            foreach (var dataSize in dataSizes)
            {
                try
                {
                    var result = RunMemoryTest(dataSize, overallProgress);
                    if (result != null)
                    {
                        allResults.Add(result);
                    }
                }
                catch (Exception ex)
                {
                    overallProgress.WriteLine($"Memory test failed: {ex.Message}");
                }
                
                completedTests++;
                overallProgress.Tick();
            }
        }
        else
        {
            overallProgress.WriteLine("Skipping memory tests (not in test_types)");
        }

        // Concurrent tests
        if (ShouldRunTestType("concurrency", isQuickMode))
        {
            overallProgress.WriteLine("Running concurrency tests...");
            foreach (var dataSize in dataSizes)
            {
                foreach (var concurrency in concurrencyLevels.Where(c => c > 1))
                {
                    try
                    {
                        var result = RunConcurrentTest(dataSize, concurrency, 5, overallProgress);
                        if (result != null)
                        {
                            allResults.Add(result);
                        }
                    }
                    catch (Exception ex)
                    {
                        overallProgress.WriteLine($"Concurrent test failed: {ex.Message}");
                    }
                    
                    completedTests++;
                    overallProgress.Tick();
                }
            }
        }
        else
        {
            overallProgress.WriteLine("Skipping concurrency tests (not in test_types)");
        }

        _results.AddRange(allResults);
        Console.WriteLine($"Benchmark suite completed. Total results: {allResults.Count}");
        return allResults;
    }

    /// <summary>
    /// Save benchmark results to JSON file - matches Java structure
    /// </summary>
    public async Task SaveResultsAsync(string outputPath)
    {
        // Create output directory if it doesn't exist
        var outputDir = Path.GetDirectoryName(outputPath);
        if (!string.IsNullOrEmpty(outputDir) && !Directory.Exists(outputDir))
        {
            Directory.CreateDirectory(outputDir);
        }

        // Prepare results data - matches other languages structure
        var resultsData = new
        {
            metadata = new
            {
                language = "net",
                timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
                dotnet_version = Environment.Version.ToString(),
                cpu_count = _cpuCount,
                total_memory_gb = _totalMemoryGb,
                total_tests = _results.Count
            },
            results = _results
        };

        // Write to file
        var json = JsonConvert.SerializeObject(resultsData, Formatting.Indented);
        await File.WriteAllTextAsync(outputPath, json);

        Console.WriteLine($"Results saved to {outputPath}");
    }

    /// <summary>
    /// Get total system memory in GB
    /// </summary>
    private static double GetTotalMemoryGb()
    {
        try
        {
            var gcMemoryInfo = GC.GetGCMemoryInfo();
            return gcMemoryInfo.TotalAvailableMemoryBytes / (1024.0 * 1024.0 * 1024.0);
        }
        catch
        {
            // Fallback - estimate based on process memory
            return Environment.WorkingSet / (1024.0 * 1024.0 * 1024.0) * 4; // Rough estimate
        }
    }

}

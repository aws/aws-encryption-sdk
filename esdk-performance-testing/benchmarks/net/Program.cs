// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using System.Diagnostics;
using System.Runtime;
using System.Text.Json;
using CommandLine;
using Microsoft.Extensions.Logging;
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
    private static ILogger<Program>? _logger;

    public static async Task<int> Main(string[] args)
    {
        return await Parser.Default.ParseArguments<CommandLineOptions>(args)
            .MapResult(
                async options => await RunBenchmarkAsync(options),
                errors => Task.FromResult(1)
            );
    }

    private static async Task<int> RunBenchmarkAsync(CommandLineOptions options)
    {
        try
        {
            // Initialize logging
            using var loggerFactory = LoggerFactory.Create(builder =>
            {
                builder.AddConsole();
                builder.SetMinimumLevel(options.Verbose ? LogLevel.Debug : LogLevel.Information);
            });
            _logger = loggerFactory.CreateLogger<Program>();

            // Initialize benchmark
            var benchmark = new ESDKBenchmark(options.ConfigPath, _logger);

            // Adjust config for quick test
            if (options.QuickTest)
            {
                benchmark.AdjustForQuickTest();
            }

            // Run benchmarks
            var results = await benchmark.RunAllBenchmarksAsync();

            // Save results
            await benchmark.SaveResultsAsync(options.OutputPath);

            // Print summary
            PrintSummary(results, options);

            return 0;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"Benchmark failed: {ex.Message}");
            if (options.Verbose)
            {
                Console.Error.WriteLine(ex.StackTrace);
            }
            return 1;
        }
    }

    private static void PrintSummary(List<BenchmarkResult> results, CommandLineOptions options)
    {
        Console.WriteLine("\n=== ESDK .NET Benchmark Summary ===");
        Console.WriteLine($"Total tests completed: {results.Count}");
        Console.WriteLine($"Results saved to: {options.OutputPath}");

        var throughputResults = results.Where(r => r.TestName == "throughput").ToList();
        if (throughputResults.Any())
        {
            var maxThroughput = throughputResults.Max(r => r.OpsPerSecond);
            Console.WriteLine($"Maximum throughput: {maxThroughput:F2} ops/sec");
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
    public string TestName { get; set; } = string.Empty;
    public string Language { get; set; } = "net";
    public int DataSize { get; set; }
    public string AlgorithmSuite { get; set; } = string.Empty;
    public long? FrameLength { get; set; }
    public int Concurrency { get; set; }

    // Performance metrics
    public double EncryptLatencyMs { get; set; }
    public double DecryptLatencyMs { get; set; }
    public double EndToEndLatencyMs { get; set; }
    public double OpsPerSecond { get; set; }
    public double BytesPerSecond { get; set; }

    // Memory metrics
    public double PeakMemoryMb { get; set; }
    public double MemoryEfficiencyRatio { get; set; }

    // Statistical metrics
    public double P50Latency { get; set; }
    public double P95Latency { get; set; }
    public double P99Latency { get; set; }

    // Environment info
    public string Timestamp { get; set; } = string.Empty;
    public string DotNetVersion { get; set; } = string.Empty;
    public int CpuCount { get; set; }
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
    private readonly ILogger _logger;
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

    public ESDKBenchmark(string configPath, ILogger logger)
    {
        _logger = logger;
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

        _logger.LogInformation("Initialized ESDK Benchmark - CPU cores: {CpuCount}, Memory: {Memory:F1}GB", 
            _cpuCount, _totalMemoryGb);
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

            _logger.LogInformation("ESDK client initialized successfully");
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
            _logger.LogWarning("Config file not found, using default configuration");
            return CreateDefaultConfig();
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
            _logger.LogError(ex, "Failed to load config file, using default configuration");
            return CreateDefaultConfig();
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
            // Map QuickDataSizes to DataSizes
            _config.DataSizes = new DataSizes
            {
                Small = _config.QuickConfig.DataSizes.Small,
                Medium = new List<int>(),
                Large = new List<int>()
            };
            _config.ConcurrencyLevels = _config.QuickConfig.ConcurrencyLevels;
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
    public BenchmarkResult? RunThroughputTest(int dataSize, int iterations)
    {
        _logger.LogInformation("Running throughput test - Size: {DataSize} bytes, Iterations: {Iterations}", dataSize, iterations);

        var testData = GenerateTestData(dataSize);

        // Warmup
        for (int i = 0; i < _config.Iterations.Warmup; i++)
        {
            var (_, _, error) = RunEncryptDecryptCycle(testData);
            if (error != null)
            {
                _logger.LogError("Warmup iteration {Iteration} failed: {Error}", i, error);
                return null;
            }
        }

        // Measurement runs
        var encryptLatencies = new List<double>();
        var decryptLatencies = new List<double>();
        var endToEndLatencies = new List<double>();
        long totalBytes = 0;

        var progressOptions = new ProgressBarOptions
        {
            ProgressCharacter = '█',
            ProgressBarOnBottom = true,
            ForegroundColor = ConsoleColor.Cyan,
            BackgroundColor = ConsoleColor.DarkGray,
            ForegroundColorDone = ConsoleColor.Green
        };

        using var progressBar = new ProgressBar(iterations, "Throughput test", progressOptions);

        var startTime = Stopwatch.GetTimestamp();
        for (int i = 0; i < iterations; i++)
        {
            var iterationStart = Stopwatch.GetTimestamp();
            var (encryptMs, decryptMs, error) = RunEncryptDecryptCycle(testData);
            if (error != null)
            {
                _logger.LogError("Measurement iteration {Iteration} failed: {Error}", i, error);
                continue;
            }
            var iterationDuration = Stopwatch.GetElapsedTime(iterationStart).TotalMilliseconds;

            encryptLatencies.Add(encryptMs);
            decryptLatencies.Add(decryptMs);
            endToEndLatencies.Add(iterationDuration);
            totalBytes += dataSize;

            progressBar.Tick();
        }
        var totalDuration = Stopwatch.GetElapsedTime(startTime).TotalSeconds;

        if (!encryptLatencies.Any())
        {
            _logger.LogError("All test iterations failed");
            return null;
        }

        // Calculate metrics
        endToEndLatencies.Sort();
        var result = new BenchmarkResult
        {
            TestName = "throughput",
            Language = "net",
            DataSize = dataSize,
            AlgorithmSuite = "AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384",
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

        _logger.LogInformation("Throughput test completed - Ops/sec: {OpsPerSec:F2}, MB/sec: {MBPerSec:F2}",
            result.OpsPerSecond, result.BytesPerSecond / (1024 * 1024));

        return result;
    }

    /// <summary>
    /// Run memory usage benchmark test - matches Java structure
    /// </summary>
    /// <summary>
    /// Run memory test matching Go/Rust implementation
    /// </summary>
    public BenchmarkResult? RunMemoryTest(int dataSize)
    {
        _logger.LogInformation("Running memory test - Size: {DataSize} bytes ({Iterations} iterations, continuous sampling)", 
            dataSize, MemoryTestIterations);

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
                _logger.LogWarning("Memory test iteration {Iteration} failed: {Error}", i + 1, error);
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

            _logger.LogDebug("Memory iteration {Iteration}: Peak={PeakMB:F2}MB, Avg={AvgMB:F2}MB, Duration={DurationMs:F2}ms",
                i + 1, iterPeakHeap, iterAvgHeap, operationDuration.TotalMilliseconds);
        }

        var avgHeap = avgHeapValues.Count > 0 ? avgHeapValues.Average() : 0.0;
        var memoryEfficiency = dataSize > 0 ? (dataSize / (1024.0 * 1024.0)) / Math.Max(peakHeap, 1.0) : 0.0;

        var result = new BenchmarkResult
        {
            TestName = "memory",
            Language = "net",
            DataSize = dataSize,
            AlgorithmSuite = "AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384",
            PeakMemoryMb = peakHeap,
            MemoryEfficiencyRatio = memoryEfficiency,
            Timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss"),
            DotNetVersion = Environment.Version.ToString(),
            CpuCount = _cpuCount,
            TotalMemoryGb = _totalMemoryGb
        };

        _logger.LogInformation("Memory test completed: {PeakMB:F2} MB peak", result.PeakMemoryMb);
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
    public BenchmarkResult? RunConcurrentTest(int dataSize, int concurrency, int iterationsPerWorker)
    {
        _logger.LogInformation("Running concurrent test - Size: {DataSize} bytes, Concurrency: {Concurrency}", 
            dataSize, concurrency);

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
                        _logger.LogWarning("Worker {WorkerID} iteration {Iteration} failed: {Error}", workerID, j, error);
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
            _logger.LogError("Concurrent test failed - no successful operations");
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
            AlgorithmSuite = "AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384",
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

        _logger.LogInformation("Concurrent test completed: {OpsPerSec:F2} ops/sec @ {Concurrency} threads", 
            result.OpsPerSecond, concurrency);

        return result;
    }

    /// <summary>
    /// Run all configured benchmark tests - matches Java structure
    /// </summary>
    public async Task<List<BenchmarkResult>> RunAllBenchmarksAsync()
    {
        _logger.LogInformation("Starting comprehensive ESDK benchmark suite");

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

        _logger.LogInformation("Running {TotalTests} total tests", totalTests);

        var overallProgressOptions = new ProgressBarOptions
        {
            ProgressCharacter = '█',
            ProgressBarOnBottom = true,
            ForegroundColor = ConsoleColor.Green,
            BackgroundColor = ConsoleColor.DarkGray
        };

        using var overallProgress = new ProgressBar(totalTests, "Running benchmarks", overallProgressOptions);

        // Throughput tests
        foreach (var dataSize in dataSizes)
        {
            overallProgress.Message = $"Throughput test: {dataSize:N0} bytes";
            
            try
            {
                var result = RunThroughputTest(dataSize, iterations);
                if (result != null)
                {
                    _logger.LogInformation("Throughput test completed: {OpsPerSecond:F2} ops/sec", result.OpsPerSecond);
                    allResults.Add(result);
                }
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Throughput test failed");
            }
            
            overallProgress.Tick();
        }

        // Memory tests
        foreach (var dataSize in dataSizes)
        {
            overallProgress.Message = $"Memory test: {dataSize:N0} bytes";
            
            try
            {
                var result = RunMemoryTest(dataSize);
                if (result != null)
                {
                    _logger.LogInformation("Memory test completed: {PeakMemory:F2} MB peak", result.PeakMemoryMb);
                    allResults.Add(result);
                }
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Memory test failed");
            }
            
            overallProgress.Tick();
        }

        // Concurrent tests
        foreach (var dataSize in dataSizes)
        {
            foreach (var concurrency in concurrencyLevels.Where(c => c > 1))
            {
                overallProgress.Message = $"Concurrent test: {dataSize:N0} bytes @ {concurrency} threads";
                
                try
                {
                    var result = RunConcurrentTest(dataSize, concurrency, 5);
                    if (result != null)
                    {
                        _logger.LogInformation("Concurrent test completed: {OpsPerSecond:F2} ops/sec @ {Concurrency} threads", 
                            result.OpsPerSecond, concurrency);
                        allResults.Add(result);
                    }
                }
                catch (Exception ex)
                {
                    _logger.LogError(ex, "Concurrent test failed");
                }
                
                overallProgress.Tick();
            }
        }

        _results.AddRange(allResults);
        _logger.LogInformation("Benchmark suite completed. Total results: {ResultCount}", allResults.Count);
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

        // Prepare results data - matches Java structure
        var resultsData = new
        {
            metadata = new
            {
                language = "net",
                timestamp = DateTime.UtcNow.ToString("O"),
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

        _logger.LogInformation("Results saved to {OutputPath}", outputPath);
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

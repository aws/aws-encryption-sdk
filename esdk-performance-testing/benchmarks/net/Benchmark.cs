// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using System.Diagnostics;
using System.Runtime;
using System.Text.Json;
using System.Security.Cryptography;
using Microsoft.Extensions.Logging;
using AWS.Cryptography.EncryptionSDK;
using AWS.Cryptography.MaterialProviders;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;
using ShellProgressBar;
using Newtonsoft.Json;

namespace EsdkBenchmark;

public partial class ESDKBenchmark
{
    private readonly ILogger _logger;
    private readonly TestConfig _config;
    private readonly int _cpuCount;
    private readonly double _totalMemoryGb;
    private static readonly Random _random = new();
    private readonly List<BenchmarkResult> _results = new();

    // Public properties to match Go structure
    public TestConfig Config => _config;
    public List<BenchmarkResult> Results => _results;

    // ESDK components
    private MaterialProviders _materialProviders = null!;
    private ESDK _encryptionSdk = null!;
    private IKeyring _keyring = null!;

    // Constants for memory testing
    private const int MemoryTestIterations = 5;
    private const int SamplingIntervalMs = 1;
    private const int GcSettleTimeMs = 5;
    private const int FinalSampleWaitMs = 2;

    public ESDKBenchmark(string configPath, ILogger logger)
    {
        _logger = logger;
        _config = ConfigLoader.LoadConfig(configPath);

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

            return null;
        }
        catch (Exception ex)
        {
            return ex.Message;
        }
    }

    public static double GetTotalMemoryGb()
    {
        try
        {
            var gcMemoryInfo = GC.GetGCMemoryInfo();
            // Convert from bytes to GB
            return gcMemoryInfo.TotalAvailableMemoryBytes / (1024.0 * 1024.0 * 1024.0);
        }
        catch
        {
            // Fallback - estimate based on process memory
            return Environment.WorkingSet / (1024.0 * 1024.0 * 1024.0) * 4; // Rough estimate
        }
    }

    private byte[] GenerateTestData(int size)
    {
        var data = new byte[size];
        _random.NextBytes(data);
        return data;
    }



    public void RunAllBenchmarks()
    {
        _results.Clear();
        Console.WriteLine("Starting comprehensive ESDK benchmark suite");

        // Combine all data sizes
        var dataSizes = _config.DataSizes.Small
            .Concat(_config.DataSizes.Medium)
            .Concat(_config.DataSizes.Large);

        // Run test suites
        if (ConfigLoader.ShouldRunTestType(_config, "throughput"))
        {
            RunThroughputTests(dataSizes, _config.Iterations.Measurement);
        }
        else
        {
            Console.WriteLine("Skipping throughput tests (not in test_types)");
        }

        if (ConfigLoader.ShouldRunTestType(_config, "memory"))
        {
            RunMemoryTests(dataSizes);
        }
        else
        {
            Console.WriteLine("Skipping memory tests (not in test_types)");
        }

        if (ConfigLoader.ShouldRunTestType(_config, "concurrency"))
        {
            RunConcurrencyTests(dataSizes, _config.ConcurrencyLevels);
        }
        else
        {
            Console.WriteLine("Skipping concurrency tests (not in test_types)");
        }

        Console.WriteLine($"Benchmark suite completed. Total results: {_results.Count}");
    }
}

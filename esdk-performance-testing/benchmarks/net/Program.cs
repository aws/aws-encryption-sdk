// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using Microsoft.Extensions.Logging;
using CommandLine;

namespace EsdkBenchmark;

public class Options
{
    [Option('c', "config", Default = "../../config/test-scenarios.yaml", HelpText = "Path to test configuration file")]
    public string ConfigPath { get; set; } = string.Empty;

    [Option('o', "output", Default = "../../results/raw-data/net_results.json", HelpText = "Path to output results file")]
    public string OutputPath { get; set; } = string.Empty;

    [Option('q', "quick", Default = false, HelpText = "Run quick test with reduced iterations")]
    public bool Quick { get; set; }
}

public class Program
{
    public static async Task Main(string[] args)
    {
        var result = Parser.Default.ParseArguments<Options>(args);

        await result.WithParsedAsync(async options =>
        {
            try
            {
                // Initialize benchmark
                using var loggerFactory = LoggerFactory.Create(builder => builder.AddConsole());
                var logger = loggerFactory.CreateLogger<ESDKBenchmark>();
                var bench = new ESDKBenchmark(options.ConfigPath, logger);

                // Adjust config for quick test
                if (options.Quick)
                {
                    if (bench.Config.QuickConfig == null)
                    {
                        Console.WriteLine("Quick mode requested but no quick_config found in config file");
                        return;
                    }
                    ConfigLoader.AdjustForQuickTest(bench.Config);
                }

                // Run benchmarks
                bench.RunAllBenchmarks();

                // Save results  
                await BenchmarkResultsHelper.SaveResultsAsync(bench.Results, options.OutputPath, Environment.ProcessorCount, ESDKBenchmark.GetTotalMemoryGb());

                // Print summary
                Console.WriteLine("\n=== ESDK .NET Benchmark Summary ===");
                Console.WriteLine($"Total tests completed: {bench.Results.Count}");
                Console.WriteLine($"Results saved to: {options.OutputPath}");

                if (bench.Results.Count > 0)
                {
                    var maxThroughput = 0.0;
                    foreach (var result in bench.Results)
                    {
                        if (result.TestName == "throughput" && result.OpsPerSecond > maxThroughput)
                        {
                            maxThroughput = result.OpsPerSecond;
                        }
                    }
                    if (maxThroughput > 0)
                    {
                        Console.WriteLine($"Maximum throughput: {maxThroughput:F2} ops/sec");
                    }
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Benchmark failed: {ex}");
            }
        });
    }
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace EsdkBenchmark;

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

public static class ConfigLoader
{
    public static TestConfig LoadConfig(string configPath)
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

    public static void AdjustForQuickTest(TestConfig config)
    {
        if (config.QuickConfig != null)
        {
            if (config.QuickConfig.DataSizes != null)
            {
                config.DataSizes.Small = config.QuickConfig.DataSizes.Small;
                config.DataSizes.Medium = new List<int>();
                config.DataSizes.Large = new List<int>();
            }

            if (config.QuickConfig.Iterations != null)
            {
                config.Iterations.Warmup = config.QuickConfig.Iterations.Warmup;
                config.Iterations.Measurement = config.QuickConfig.Iterations.Measurement;
            }

            if (config.QuickConfig.ConcurrencyLevels.Count > 0)
            {
                config.ConcurrencyLevels = config.QuickConfig.ConcurrencyLevels;
            }
        }
    }

    public static bool ShouldRunTestType(TestConfig config, string testType)
    {
        if (config.QuickConfig != null && config.QuickConfig.TestTypes.Count > 0)
        {
            return config.QuickConfig.TestTypes.Contains(testType);
        }
        return true;
    }
}
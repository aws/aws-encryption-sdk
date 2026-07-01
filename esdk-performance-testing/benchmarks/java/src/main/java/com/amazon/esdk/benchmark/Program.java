// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package com.amazon.esdk.benchmark;

import com.amazon.esdk.benchmark.model.Report;
import com.amazon.esdk.benchmark.model.TestResult;
import java.util.List;
import java.util.OptionalDouble;

public final class Program {

  public static void main(final String[] args) {
    final CommandLineOptions options = parseArgs(args);
    if (options == null) return;

    try {
      final ESDKBenchmark benchmark = new ESDKBenchmark(options.configPath());

      if (options.quickTest()) {
        benchmark.config.adjustForQuickTest();
      }

      final List<TestResult> results = benchmark.runAllBenchmarks();
      Report.saveResults(
        results,
        options.outputPath(),
        benchmark.cpuCount,
        benchmark.totalMemoryMB
      );
      printSummary(results, options.outputPath());
    } catch (final Exception ex) {
      System.err.println("Benchmark failed: " + ex.getMessage());
      ex.printStackTrace();
      System.exit(1);
    }
  }

  private static CommandLineOptions parseArgs(final String[] args) {
    // Default options
    String configPath = "../../config/test-scenarios.yaml";
    String outputPath = "../../results/raw-data/java_results.json";
    boolean quickTest = false;

    // Simple argument parsing
    for (int i = 0; i < args.length; i++) {
      switch (args[i]) {
        case "--config":
        case "-c":
          if (i + 1 < args.length) configPath = args[++i];
          break;
        case "--output":
        case "-o":
          if (i + 1 < args.length) outputPath = args[++i];
          break;
        case "--quick":
        case "-q":
          quickTest = true;
          break;
        case "--help":
        case "-h":
          printUsage();
          return null;
      }
    }

    return new CommandLineOptions(configPath, outputPath, quickTest);
  }

  private static void printUsage() {
    System.out.println("ESDK Java Performance Benchmark");
    System.out.println("Usage: java -jar esdk-benchmark.jar [options]");
    System.out.println("Options:");
    System.out.println(
      "  --config, -c    Path to test configuration file (default: ../../config/test-scenarios.yaml)"
    );
    System.out.println(
      "  --output, -o    Path to output results file (default: ../../results/raw-data/java_results.json)"
    );
    System.out.println(
      "  --quick, -q     Run quick test with reduced iterations"
    );
    System.out.println("  --help, -h      Show this help message");
  }

  private static void printSummary(
    final List<TestResult> results,
    final String outputPath
  ) {
    System.out.println("\n=== ESDK Java Benchmark Summary ===");
    System.out.println("Total tests completed: " + results.size());
    System.out.println("Results saved to: " + outputPath);

    // Print some basic statistics
    if (!results.isEmpty()) {
      final OptionalDouble avgOps = results
        .stream()
        .filter(r -> "throughput".equals(r.testName))
        .mapToDouble(r -> r.opsPerSecond)
        .average();
      final OptionalDouble maxOps = results
        .stream()
        .filter(r -> "throughput".equals(r.testName))
        .mapToDouble(r -> r.opsPerSecond)
        .max();

      if (avgOps.isPresent() && maxOps.isPresent()) {
        System.out.printf(
          "Throughput - Avg: %.2f ops/sec, Max: %.2f ops/sec%n",
          avgOps.getAsDouble(),
          maxOps.getAsDouble()
        );
      }
    }
  }

  public record CommandLineOptions(
    String configPath,
    String outputPath,
    boolean quickTest
  ) {}
}

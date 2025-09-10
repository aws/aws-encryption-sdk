// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package com.amazon.esdk.benchmark;

import com.amazon.esdk.benchmark.model.TestResult;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import me.tongfei.progressbar.ProgressBar;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public final class Tests {

  private static final Logger logger = LoggerFactory.getLogger(Tests.class);

  // Constants for memory testing
  private static final int MemoryTestIterations = 5;
  private static final int SamplingIntervalMs = 1;
  private static final int GcSettleTimeMs = 5;
  private static final int FinalSampleWaitMs = 2;

  /** Run throughput benchmark test */
  public static TestResult runThroughputTest(
    final ESDKBenchmark benchmark,
    final int dataSize,
    final int iterations
  ) {
    System.out.println(
      "Running throughput test - Size: " +
      dataSize +
      " bytes, Iterations: " +
      iterations
    );
    System.out.flush();

    final byte[] data = new byte[dataSize];
    new java.security.SecureRandom().nextBytes(data);

    // Warmup - run measurement but drop results
    runMeasurementIterations(
      benchmark,
      data,
      benchmark.config.iterations.warmup
    );

    // Measurement runs
    final var results = runMeasurementIterations(benchmark, data, iterations);
    final var encryptLatencies = results.encryptLatencies;
    final var decryptLatencies = results.decryptLatencies;
    final var totalLatencies = results.totalLatencies;

    if (encryptLatencies.isEmpty()) {
      System.out.println("All test iterations failed");
      return null;
    }

    return TestResult.createThroughputResult(
      encryptLatencies,
      decryptLatencies,
      totalLatencies,
      dataSize,
      benchmark.cpuCount,
      benchmark.totalMemoryMB
    );
  }

  private static long getTotalAllocatedBytes() {
    final var threadBean =
      java.lang.management.ManagementFactory.getThreadMXBean();
    final var sunThreadBean = (com.sun.management.ThreadMXBean) threadBean;

    if (!sunThreadBean.isThreadAllocatedMemoryEnabled()) {
      sunThreadBean.setThreadAllocatedMemoryEnabled(true);
    }

    return sunThreadBean.getCurrentThreadAllocatedBytes();
  }

  private static MeasurementResults runMeasurementIterations(
    final ESDKBenchmark benchmark,
    final byte[] data,
    final int iterations
  ) {
    final var encryptLatencies = new ArrayList<Double>();
    final var decryptLatencies = new ArrayList<Double>();
    final var totalLatencies = new ArrayList<Double>();

    for (int i = 0; i < iterations; i++) {
      try {
        final long iterationStart = System.nanoTime();
        final var result = benchmark.runEncryptDecryptCycle(data);
        final double totalMs =
          (System.nanoTime() - iterationStart) / 1_000_000.0;

        encryptLatencies.add(result.encryptTimeMs());
        decryptLatencies.add(result.decryptTimeMs());
        totalLatencies.add(totalMs);
      } catch (final Exception e) {
        System.out.println("Iteration " + i + " failed: " + e.getMessage());
      }
    }

    return new MeasurementResults(
      encryptLatencies,
      decryptLatencies,
      totalLatencies
    );
  }

  /** Run memory usage benchmark test */
  public static TestResult runMemoryTest(
    final ESDKBenchmark benchmark,
    final int dataSize
  ) {
    System.out.println(
      "Running memory test - Size: " +
      dataSize +
      " bytes (" +
      MemoryTestIterations +
      " iterations, continuous sampling)"
    );
    System.out.flush();

    final byte[] data = new byte[dataSize];
    new java.security.SecureRandom().nextBytes(data);
    final var memoryResults = sampleMemoryDuringOperations(benchmark, data);

    if (memoryResults == null) {
      throw new RuntimeException(
        "Memory test failed: Unable to collect memory samples for data size " +
        dataSize +
        " bytes"
      );
    }

    return TestResult.createMemoryResult(
      memoryResults.peakMemoryMb,
      memoryResults.avgMemoryMb,
      dataSize,
      benchmark.cpuCount,
      benchmark.totalMemoryMB
    );
  }

  private static MemoryResults sampleMemoryDuringOperations(
    final ESDKBenchmark benchmark,
    final byte[] data
  ) {
    double peakMemoryDelta = 0.0;
    double peakAllocations = 0.0;
    final var avgMemoryValues = new ArrayList<Double>();

    for (int i = 0; i < MemoryTestIterations; i++) {
      final var iterationResult = runSingleMemoryIteration(
        benchmark,
        data,
        i + 1
      );

      if (iterationResult.peakMemory > peakMemoryDelta) {
        peakMemoryDelta = iterationResult.peakMemory;
      }
      if (iterationResult.totalAllocs > peakAllocations) {
        peakAllocations = iterationResult.totalAllocs;
      }
      avgMemoryValues.add(iterationResult.avgMemory);
    }

    final double overallAvgMemory = avgMemoryValues.isEmpty()
      ? 0.0
      : avgMemoryValues
        .stream()
        .mapToDouble(Double::doubleValue)
        .average()
        .orElse(0.0);

    System.out.println("\nMemory Summary:");
    System.out.println(
      "- Absolute Peak Heap: " +
      String.format("%.2f", peakMemoryDelta) +
      " MB (across all runs)"
    );
    System.out.println(
      "- Average Heap: " +
      String.format("%.2f", overallAvgMemory) +
      " MB (across all runs)"
    );
    System.out.println(
      "- Total Allocations: " +
      String.format("%.2f", peakAllocations) +
      " MB (max across all runs)"
    );
    System.out.flush();

    return new MemoryResults(peakMemoryDelta, overallAvgMemory);
  }

  private static IterationResult runSingleMemoryIteration(
    final ESDKBenchmark benchmark,
    final byte[] data,
    final int iteration
  ) {
    // Force GC and settle
    System.gc();
    System.gc();
    try {
      Thread.sleep(GcSettleTimeMs);
    } catch (final InterruptedException e) {
      Thread.currentThread().interrupt();
      throw new RuntimeException(
        "Memory test interrupted during GC settle phase",
        e
      );
    }

    final long baselineMemory =
      Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory();
    final long baselineAllocations = getTotalAllocatedBytes();
    final var memorySamples = new ArrayList<MemorySample>();

    final long operationStart = System.nanoTime();

    // Start background sampling
    final var samplingTask = new Thread(() -> {
      try {
        while (System.nanoTime() - operationStart < 100_000_000) { // 100ms
          final long currentMemory =
            Runtime.getRuntime().totalMemory() -
            Runtime.getRuntime().freeMemory();
          final long currentAllocations = getTotalAllocatedBytes();
          final double heapDelta =
            (currentMemory - baselineMemory) / (1024.0 * 1024.0);
          final double cumulativeAllocs =
            (currentAllocations - baselineAllocations) / (1024.0 * 1024.0);

          if (heapDelta > 0 || cumulativeAllocs > 0) {
            synchronized (memorySamples) {
              memorySamples.add(
                new MemorySample(
                  Math.max(0, heapDelta),
                  Math.max(0, cumulativeAllocs)
                )
              );
            }
          }
          Thread.sleep(SamplingIntervalMs);
        }
      } catch (final InterruptedException e) {}
    });

    samplingTask.start();

    // Run the actual operation
    try {
      benchmark.runEncryptDecryptCycle(data);
    } catch (final Exception e) {
      System.out.println(
        "Memory test iteration " + iteration + " failed: " + e.getMessage()
      );
      return new IterationResult(0.0, 0.0, 0.0);
    }

    final double operationDurationMs =
      (System.nanoTime() - operationStart) / 1_000_000.0;

    // Wait for sampling to complete
    try {
      Thread.sleep(FinalSampleWaitMs);
      samplingTask.join(100);
    } catch (final InterruptedException e) {}

    return calculateIterationMetrics(
      baselineMemory,
      baselineAllocations,
      memorySamples,
      iteration,
      operationDurationMs
    );
  }

  private static IterationResult calculateIterationMetrics(
    final long baselineMemory,
    final long baselineAllocations,
    final ArrayList<MemorySample> memorySamples,
    final int iteration,
    final double operationDurationMs
  ) {
    final long finalMemory =
      Runtime.getRuntime().totalMemory() - Runtime.getRuntime().freeMemory();
    final long finalAllocations = getTotalAllocatedBytes();
    final double finalHeapDelta =
      (finalMemory - baselineMemory) / (1024.0 * 1024.0);
    final double finalCumulativeAllocs =
      (finalAllocations - baselineAllocations) / (1024.0 * 1024.0);

    final double iterPeakMemory;
    final double iterTotalAllocs;
    final double iterAvgMemory;

    synchronized (memorySamples) {
      if (memorySamples.isEmpty()) {
        iterPeakMemory = Math.max(0, finalHeapDelta);
        iterTotalAllocs = Math.max(0, finalCumulativeAllocs);
        iterAvgMemory = Math.max(0, finalHeapDelta);
      } else {
        iterPeakMemory =
          memorySamples.stream().mapToDouble(s -> s.heapMB).max().orElse(0.0);
        iterTotalAllocs = Math.max(0, finalCumulativeAllocs);
        iterAvgMemory =
          memorySamples
            .stream()
            .mapToDouble(s -> s.heapMB)
            .average()
            .orElse(0.0);
      }
    }

    System.out.println(
      "=== Iteration " +
      iteration +
      " === Peak Heap: " +
      String.format("%.2f", iterPeakMemory) +
      " MB, Total Allocs: " +
      String.format("%.2f", iterTotalAllocs) +
      " MB, Avg Heap: " +
      String.format("%.2f", iterAvgMemory) +
      " MB (" +
      String.format("%.0f", operationDurationMs) +
      "ms, " +
      memorySamples.size() +
      " samples)"
    );
    System.out.flush();

    return new IterationResult(iterPeakMemory, iterTotalAllocs, iterAvgMemory);
  }

  private record IterationResult(
    double peakMemory,
    double totalAllocs,
    double avgMemory
  ) {}

  /** Run concurrent operations benchmark test */
  public static TestResult runConcurrentTest(
    final ESDKBenchmark benchmark,
    final int dataSize,
    final int concurrency,
    final int iterationsPerThread
  ) {
    System.out.println(
      "Running concurrent test - Size: " +
      dataSize +
      " bytes, Concurrency: " +
      concurrency +
      ", Iterations per thread: " +
      iterationsPerThread
    );
    System.out.flush();

    if (concurrency <= 0) {
      throw new IllegalArgumentException(
        "Concurrency must be positive, got: " + concurrency
      );
    }
    if (iterationsPerThread <= 0) {
      throw new IllegalArgumentException(
        "Iterations per thread must be positive, got: " + iterationsPerThread
      );
    }

    final byte[] data = new byte[dataSize];
    new java.security.SecureRandom().nextBytes(data);
    final List<Double> allTimes = Collections.synchronizedList(
      new ArrayList<>()
    );

    final ExecutorService executor = Executors.newFixedThreadPool(concurrency);
    final List<Future<Void>> futures = new ArrayList<>();

    // Create progress bar for concurrent operations
    final int expectedOperations = concurrency * iterationsPerThread;
    try (
      final ProgressBar concurrentPb = new ProgressBar(
        "Concurrent test",
        expectedOperations
      )
    ) {
      // Submit concurrent tasks
      for (int i = 0; i < concurrency; i++) {
        final Future<Void> future = executor.submit(() -> {
          for (int j = 0; j < iterationsPerThread; j++) {
            try {
              final long threadStartTime = System.nanoTime();
              benchmark.runEncryptDecryptCycle(data);
              final double elapsed =
                (System.nanoTime() - threadStartTime) / 1_000_000.0;
              allTimes.add(elapsed);

              System.out.flush();
              concurrentPb.step();
              System.out.flush();
            } catch (final Exception e) {
              System.err.println(
                "Concurrent test iteration failed: " + e.getMessage()
              );
            }
          }
          return null;
        });
        futures.add(future);
      }

      // Wait for all tasks to complete
      for (final Future<Void> future : futures) {
        try {
          future.get();
        } catch (final Exception e) {
          System.err.println("Concurrent thread failed: " + e.getMessage());
        }
      }
    }

    executor.shutdown();

    if (allTimes.isEmpty()) {
      throw new RuntimeException(
        "Concurrent test failed: No operations completed successfully. " +
        "Concurrency: " +
        concurrency +
        ", Expected operations: " +
        (concurrency * iterationsPerThread)
      );
    }

    // Calculate metrics
    final int totalOperations = allTimes.size();

    return TestResult.createConcurrentResult(
      allTimes,
      totalOperations,
      dataSize,
      concurrency,
      benchmark.cpuCount,
      benchmark.totalMemoryMB
    );
  }

  // Helper records
  private record MeasurementResults(
    List<Double> encryptLatencies,
    List<Double> decryptLatencies,
    List<Double> totalLatencies
  ) {}

  private record MemoryResults(double peakMemoryMb, double avgMemoryMb) {}

  private record MemorySample(double heapMB, double allocsMB) {}
}

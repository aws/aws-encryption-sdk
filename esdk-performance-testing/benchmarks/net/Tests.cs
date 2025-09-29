// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using System.Diagnostics;
using AWS.Cryptography.EncryptionSDK;
using ShellProgressBar;

namespace EsdkBenchmark;

public partial class ESDKBenchmark
{
    private (double encryptMs, double decryptMs, string? error) RunEncryptDecryptCycle(byte[] data)
    {
        try
        {
            var plaintext = new MemoryStream(data);

            // Create encryption context matching Rust
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

            // Verify integrity
            decryptOutput.Plaintext.Position = 0;
            var decryptedData = new byte[decryptOutput.Plaintext.Length];
            decryptOutput.Plaintext.Read(decryptedData, 0, decryptedData.Length);

            if (!data.SequenceEqual(decryptedData))
            {
                return (0, 0, "Data integrity check failed");
            }

            return (encryptMs, decryptMs, null);
        }
        catch (Exception ex)
        {
            return (0, 0, ex.Message);
        }
    }

    public BenchmarkResult? RunThroughputTest(int dataSize, int iterations, ProgressBar? progressBar = null)
    {
        Action<string> log = progressBar != null ? progressBar.WriteLine : Console.WriteLine;
        log($"Running throughput test - Size: {dataSize} bytes, Iterations: {iterations}");

        var data = GenerateTestData(dataSize);

        // Warmup (ignore results)
        RunIterations(data, _config.Iterations.Warmup);

        // Measurement runs
        var (encryptLatencies, decryptLatencies, totalLatencies) = RunIterations(data, iterations);

        if (!encryptLatencies.Any())
        {
            Console.WriteLine("All test iterations failed");
            return null;
        }

        return BenchmarkResultsHelper.CreateThroughputResult(encryptLatencies, decryptLatencies, totalLatencies, dataSize, _cpuCount, _totalMemoryGb);
    }

    private (List<double> encrypt, List<double> decrypt, List<double> total) RunIterations(byte[] data, int iterations)
    {
        var encryptLatencies = new List<double>();
        var decryptLatencies = new List<double>();
        var totalLatencies = new List<double>();

        for (int i = 0; i < iterations; i++)
        {
            var iterationStart = Stopwatch.GetTimestamp();
            var (encryptMs, decryptMs, error) = RunEncryptDecryptCycle(data);

            if (error != null)
            {
                Console.WriteLine($"Iteration {i} failed: {error}");
                continue;
            }

            var totalMs = Stopwatch.GetElapsedTime(iterationStart).TotalMilliseconds;

            encryptLatencies.Add(encryptMs);
            decryptLatencies.Add(decryptMs);
            totalLatencies.Add(totalMs);
        }

        return (encryptLatencies, decryptLatencies, totalLatencies);
    }

    public BenchmarkResult? RunMemoryTest(int dataSize, ProgressBar? progressBar = null)
    {
        Action<string> log = progressBar != null ? progressBar.WriteLine : Console.WriteLine;
        log($"Running memory test - Size: {dataSize} bytes ({MemoryTestIterations} iterations, continuous sampling)");

        var data = GenerateTestData(dataSize);
        var (peakMemoryMb, avgMemoryMb) = SampleMemoryDuringOperations(data, progressBar);

        return BenchmarkResultsHelper.CreateMemoryResult(peakMemoryMb, avgMemoryMb, dataSize, _cpuCount, _totalMemoryGb);
    }

    private (double peakMemoryMb, double avgMemoryMb) SampleMemoryDuringOperations(byte[] data, ProgressBar? progressBar = null)
    {
        Action<string> log = progressBar != null ? progressBar.WriteLine : Console.WriteLine;
        var peakHeap = 0.0;
        var peakAllocations = 0.0;
        var avgHeapValues = new List<double>();

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

        log("\nMemory Summary:");
        log($"- Absolute Peak Heap: {peakHeap:F2} MB (across all runs)");
        log($"- Average Heap: {avgHeap:F2} MB (across all runs)");
        log($"- Total Allocations: {peakAllocations:F2} MB (max across all runs)");

        return (peakHeap, avgHeap);
    }

    private void SampleMemoryContinuously(long baselineHeap, ulong baselineAllocs, CancellationToken cancellationToken, List<(double HeapMB, double AllocsMB)> samples)
    {
        while (!cancellationToken.IsCancellationRequested)
        {
            try
            {
                var currentHeap = GC.GetTotalMemory(false);
                var currentAllocs = GC.GetTotalAllocatedBytes(false);

                // Convert from bytes to MB
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

    public BenchmarkResult? RunConcurrentTest(int dataSize, int concurrency, int iterationsPerWorker, ProgressBar? progressBar = null)
    {
        Action<string> log = progressBar != null ? progressBar.WriteLine : Console.WriteLine;
        log($"Running concurrent test - Size: {dataSize} bytes, Concurrency: {concurrency}");

        var data = GenerateTestData(dataSize);
        var allTimes = RunConcurrentWorkers(data, concurrency, iterationsPerWorker);

        if (!allTimes.Any())
        {
            Console.WriteLine("All concurrent workers failed");
            return null;
        }

        return BenchmarkResultsHelper.CreateConcurrentResult(allTimes, concurrency * iterationsPerWorker, dataSize, concurrency, _cpuCount, _totalMemoryGb);
    }

    private List<double> RunConcurrentWorkers(byte[] data, int concurrency, int iterationsPerWorker)
    {
        var allTimes = new List<double>();
        var tasks = new List<Task<List<double>>>();

        for (int i = 0; i < concurrency; i++)
        {
            tasks.Add(CreateWorkerTask(data, iterationsPerWorker, i));
        }

        Task.WaitAll(tasks.ToArray());

        foreach (var task in tasks)
        {
            if (task.IsCompletedSuccessfully)
            {
                allTimes.AddRange(task.Result);
            }
        }

        return allTimes;
    }

    private Task<List<double>> CreateWorkerTask(byte[] data, int iterations, int workerID)
    {
        return Task.Run(() =>
        {
            var workerTimes = new List<double>();

            for (int i = 0; i < iterations; i++)
            {
                var start = Stopwatch.GetTimestamp();
                var (_, _, error) = RunEncryptDecryptCycle(data);
                if (error != null)
                {
                    Console.WriteLine($"Worker {workerID} iteration {i} failed: {error}");
                    continue;
                }
                workerTimes.Add(Stopwatch.GetElapsedTime(start).TotalMilliseconds);
            }
            return workerTimes;
        });
    }

    private void RunThroughputTests(IEnumerable<int> dataSizes, int iterations)
    {
        Console.WriteLine("Running throughput tests...");
        var dataSizesList = dataSizes.ToList();

        var progressOptions = new ProgressBarOptions
        {
            ProgressCharacter = '█',
            ProgressBarOnBottom = true,
            ForegroundColor = ConsoleColor.Green,
            BackgroundColor = ConsoleColor.DarkGreen
        };

        using var progressBar = new ProgressBar(dataSizesList.Count, "Throughput Tests", progressOptions);

        foreach (var dataSize in dataSizesList)
        {
            var result = RunThroughputTest(dataSize, iterations, progressBar);
            if (result != null)
            {
                _results.Add(result);
                progressBar.WriteLine($"Throughput test completed: {result.OpsPerSecond:F2} ops/sec");
            }
            progressBar.Tick();
        }
    }

    private void RunMemoryTests(IEnumerable<int> dataSizes)
    {
        Console.WriteLine("Running memory tests...");
        var dataSizesList = dataSizes.ToList();

        var progressOptions = new ProgressBarOptions
        {
            ProgressCharacter = '█',
            ProgressBarOnBottom = true,
            ForegroundColor = ConsoleColor.Blue,
            BackgroundColor = ConsoleColor.DarkBlue
        };

        using var progressBar = new ProgressBar(dataSizesList.Count, "Memory Tests", progressOptions);

        foreach (var dataSize in dataSizesList)
        {
            var result = RunMemoryTest(dataSize, progressBar);
            if (result != null)
            {
                _results.Add(result);
                progressBar.WriteLine($"Memory test completed: {result.PeakMemoryMb:F2} MB peak");
            }
            progressBar.Tick();
        }
    }

    private void RunConcurrencyTests(IEnumerable<int> dataSizes, IEnumerable<int> concurrencyLevels)
    {
        Console.WriteLine("Running concurrency tests...");
        var dataSizesList = dataSizes.ToList();
        var concurrencyList = concurrencyLevels.Where(c => c > 1).ToList(); // Skip single-threaded
        var totalTests = dataSizesList.Count * concurrencyList.Count;

        var progressOptions = new ProgressBarOptions
        {
            ProgressCharacter = '█',
            ProgressBarOnBottom = true,
            ForegroundColor = ConsoleColor.Yellow,
            BackgroundColor = ConsoleColor.DarkYellow
        };

        using var progressBar = new ProgressBar(totalTests, "Concurrency Tests", progressOptions);

        foreach (var dataSize in dataSizesList)
        {
            foreach (var concurrency in concurrencyList)
            {
                var result = RunConcurrentTest(dataSize, concurrency, 5, progressBar);
                if (result != null)
                {
                    _results.Add(result);
                    progressBar.WriteLine($"Concurrent test completed: {result.OpsPerSecond:F2} ops/sec @ {concurrency} threads");
                }
                progressBar.Tick();
            }
        }
    }
}

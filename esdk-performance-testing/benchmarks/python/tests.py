#!/usr/bin/env python3
"""
Test implementations for ESDK Python benchmark
"""

import gc
import os
import statistics
import threading
import time
import tracemalloc
from concurrent.futures import ThreadPoolExecutor, as_completed

import psutil
from memory_profiler import memory_usage
from tqdm import tqdm

from results import BenchmarkResult


def run_encrypt_decrypt_cycle(esdk_client, keyring, data: bytes) -> tuple[float, float]:
    """Run a single encrypt-decrypt cycle and return timing"""

    # Encryption context
    encryption_context = {"purpose": "performance-test", "size": str(len(data))}

    # Encrypt
    encrypt_start = time.time()
    encrypted_result, _ = esdk_client.encrypt(
        source=data, keyring=keyring, encryption_context=encryption_context
    )
    encrypt_time = (time.time() - encrypt_start) * 1000

    # Decrypt
    decrypt_start = time.time()
    decrypted_result, _ = esdk_client.decrypt(source=encrypted_result, keyring=keyring)
    decrypt_time = (time.time() - decrypt_start) * 1000

    # Verify data integrity
    if decrypted_result != data:
        raise ValueError("Decrypted data does not match original")

    return encrypt_time, decrypt_time


def run_throughput_test(
    benchmark,
    data_size: int,
    iterations: int,
) -> BenchmarkResult:
    """Run throughput benchmark test"""
    data = os.urandom(data_size)

    # Warmup
    warmup_iterations = benchmark.config.get("iterations", {}).get("warmup", 2)
    for _ in range(warmup_iterations):
        run_encrypt_decrypt_cycle(benchmark.esdk_client, benchmark.keyring, data)

    # Collect timing data
    timing_data = _collect_timing_data(benchmark, data, iterations)

    # Calculate statistics
    return _create_throughput_result(timing_data, data_size)


def _collect_timing_data(benchmark, data, iterations):
    """Collect timing data for throughput test"""
    encrypt_times = []
    decrypt_times = []
    end_to_end_times = []

    with tqdm(total=iterations, desc="Throughput test", leave=False) as pbar:
        for _ in range(iterations):
            start_time = time.time()
            encrypt_time, decrypt_time = run_encrypt_decrypt_cycle(
                benchmark.esdk_client, benchmark.keyring, data
            )
            end_to_end_time = (time.time() - start_time) * 1000

            encrypt_times.append(encrypt_time)
            decrypt_times.append(decrypt_time)
            end_to_end_times.append(end_to_end_time)
            pbar.update(1)

    return {
        "encrypt_times": encrypt_times,
        "decrypt_times": decrypt_times,
        "end_to_end_times": end_to_end_times,
    }


def _create_throughput_result(timing_data, data_size):
    """Create throughput benchmark result from timing data"""
    avg_encrypt = statistics.mean(timing_data["encrypt_times"])
    avg_decrypt = statistics.mean(timing_data["decrypt_times"])
    avg_end_to_end = statistics.mean(timing_data["end_to_end_times"])

    ops_per_second = 1000.0 / avg_end_to_end
    bytes_per_second = ops_per_second * data_size

    # Calculate percentiles
    sorted_times = sorted(timing_data["end_to_end_times"])
    n = len(sorted_times)
    p50 = sorted_times[int(0.50 * n)] if n > 0 else 0
    p95 = sorted_times[int(0.95 * n)] if n > 0 else 0
    p99 = sorted_times[int(0.99 * n)] if n > 0 else 0

    return BenchmarkResult(
        test_name="throughput",
        data_size=data_size,
        encrypt_latency_ms=avg_encrypt,
        decrypt_latency_ms=avg_decrypt,
        end_to_end_latency_ms=avg_end_to_end,
        ops_per_second=ops_per_second,
        bytes_per_second=bytes_per_second,
        p50_latency=p50,
        p95_latency=p95,
        p99_latency=p99,
    )


def run_memory_test(benchmark, data_size: int) -> BenchmarkResult:
    """Run memory usage benchmark test"""
    data = os.urandom(data_size)
    iterations = 5

    tracemalloc.start()
    process = psutil.Process()

    memory_samples = []
    total_allocations = 0

    def memory_test_function():
        nonlocal total_allocations
        iteration_peaks, iteration_avgs, iteration_allocs = [], [], []

        for i in range(iterations):
            peak, avg, allocs = _run_memory_iteration(benchmark, data, process, i)
            iteration_peaks.append(peak)
            iteration_avgs.append(avg)
            iteration_allocs.append(allocs)
            total_allocations += allocs * 1024 * 1024  # Convert back to bytes

        _log_memory_summary(
            benchmark, iteration_peaks, iteration_avgs, iteration_allocs
        )
        return True

    mem_usage = memory_usage(memory_test_function, interval=0.01, timeout=60)
    tracemalloc.stop()

    if not mem_usage:
        raise RuntimeError("Failed to collect memory usage data")

    return _create_memory_result(
        data_size,
        mem_usage + memory_samples,
        total_allocations,
        data_size,
    )


def _run_memory_iteration(benchmark, data, process, iteration_num):
    """Run a single memory test iteration"""
    iteration_samples = []
    snapshot_before = tracemalloc.take_snapshot()

    # Sample memory during operation
    stop_sampling = threading.Event()
    sampler = threading.Thread(
        target=lambda: _sample_memory_continuously(
            process, iteration_samples, stop_sampling
        )
    )
    sampler.daemon = True
    sampler.start()

    start_time = time.time()
    run_encrypt_decrypt_cycle(benchmark.esdk_client, benchmark.keyring, data)
    end_time = time.time()

    stop_sampling.set()
    sampler.join(timeout=1.0)

    snapshot_after = tracemalloc.take_snapshot()
    top_stats = snapshot_after.compare_to(snapshot_before, "lineno")
    iteration_allocations = sum(
        stat.size_diff for stat in top_stats if stat.size_diff > 0
    )

    if iteration_samples:
        peak = max(iteration_samples)
        avg = sum(iteration_samples) / len(iteration_samples)
        allocs_mb = iteration_allocations / 1024 / 1024

        duration = end_time - start_time
        benchmark.logger.info(
            f"=== Iteration {iteration_num + 1} === Peak: {peak:.2f} MB, "
            f"Allocs: {allocs_mb:.2f} MB, Avg: {avg:.2f} MB "
            f"({duration:.3f}s, {len(iteration_samples)} samples)"
        )
        return peak, avg, allocs_mb

    return 0, 0, 0


def _sample_memory_continuously(process, samples, stop_event):
    """Continuously sample memory usage"""
    while not stop_event.is_set():
        current_memory = process.memory_info().rss / 1024 / 1024
        samples.append(current_memory)
        time.sleep(0.001)


def _log_memory_summary(benchmark, peaks, avgs, allocs):
    """Log memory test summary"""
    if peaks and avgs and allocs:
        abs_peak = max(peaks)
        overall_avg = sum(avgs) / len(avgs)
        max_allocs = max(allocs)

        benchmark.logger.info("")
        benchmark.logger.info("Memory Summary:")
        benchmark.logger.info(f"- Absolute Peak Heap: {abs_peak:.2f} MB")
        benchmark.logger.info(f"- Average Heap: {overall_avg:.2f} MB")
        benchmark.logger.info(f"- Total Allocations: {max_allocs:.2f} MB")


def _create_memory_result(
    data_size, all_samples, total_allocations, original_data_size
):
    """Create memory benchmark result"""
    peak_memory_mb = max(all_samples)
    memory_efficiency = (
        original_data_size / (peak_memory_mb * 1024 * 1024) if peak_memory_mb > 0 else 0
    )

    return BenchmarkResult(
        test_name="memory",
        data_size=data_size,
        peak_memory_mb=peak_memory_mb,
        memory_efficiency_ratio=memory_efficiency,
    )


def run_concurrent_test(benchmark, data_size: int, concurrency: int) -> BenchmarkResult:
    """Run concurrent benchmark test"""
    data = os.urandom(data_size)
    operations_per_worker = 5
    total_operations = concurrency * operations_per_worker

    start_time = time.time()
    all_times, errors = _execute_concurrent_workers(
        benchmark, data, concurrency, operations_per_worker
    )
    total_duration = time.time() - start_time

    if errors:
        raise RuntimeError(
            f"Concurrent test failed with {len(errors)} errors: {errors[0]}"
        )

    if not all_times:
        raise RuntimeError("No timing data collected from concurrent test")

    return _create_concurrent_result(
        all_times, total_operations, total_duration, data_size, concurrency
    )


def _execute_concurrent_workers(benchmark, data, concurrency, operations_per_worker):
    """Execute concurrent workers and collect results"""
    all_times = []
    errors = []

    def worker_function():
        worker_times = []
        try:
            for _ in range(operations_per_worker):
                start_time = time.time()
                run_encrypt_decrypt_cycle(
                    benchmark.esdk_client, benchmark.keyring, data
                )
                operation_time = (time.time() - start_time) * 1000
                worker_times.append(operation_time)
        except Exception as e:
            errors.append(e)
        return worker_times

    with ThreadPoolExecutor(max_workers=concurrency) as executor:
        futures = [executor.submit(worker_function) for _ in range(concurrency)]

        for future in as_completed(futures):
            try:
                worker_times = future.result()
                all_times.extend(worker_times)
            except Exception as e:
                errors.append(e)

    return all_times, errors


def _create_concurrent_result(
    all_times, total_operations, total_duration, data_size, concurrency
):
    """Create concurrent benchmark result"""
    avg_latency = statistics.mean(all_times)
    ops_per_second = total_operations / total_duration
    bytes_per_second = ops_per_second * data_size

    return BenchmarkResult(
        test_name="concurrent",
        data_size=data_size,
        concurrency=concurrency,
        end_to_end_latency_ms=avg_latency,
        ops_per_second=ops_per_second,
        bytes_per_second=bytes_per_second,
    )


def run_all_benchmarks(benchmark, is_quick_mode: bool = False) -> list[BenchmarkResult]:
    """Run all configured benchmark tests"""
    benchmark.logger.info("Starting comprehensive ESDK benchmark suite")
    results = []

    test_params = _get_test_parameters(benchmark.config)
    total_tests = _calculate_total_tests(test_params)

    with tqdm(total=total_tests, desc="Running benchmarks") as pbar:
        if benchmark.should_run_test_type("throughput", is_quick_mode):
            _run_throughput_tests(benchmark, test_params, results, pbar)
        else:
            benchmark.logger.info("Skipping throughput tests (not in test_types)")

        if benchmark.should_run_test_type("memory", is_quick_mode):
            _run_memory_tests(benchmark, test_params, results, pbar)
        else:
            benchmark.logger.info("Skipping memory tests (not in test_types)")

        if benchmark.should_run_test_type("concurrency", is_quick_mode):
            _run_concurrent_tests(benchmark, test_params, results, pbar)
        else:
            benchmark.logger.info("Skipping concurrency tests (not in test_types)")

    benchmark.results = results
    benchmark.logger.info(f"Benchmark suite completed. Total results: {len(results)}")
    return results


def _get_test_parameters(config):
    """Extract test parameters from config"""
    data_sizes = []
    for category in ["small", "medium", "large"]:
        if category in config.get("data_sizes", {}):
            data_sizes.extend(config["data_sizes"][category])

    return {
        "data_sizes": data_sizes,
        "concurrency_levels": config.get("concurrency_levels", [1, 2, 4]),
        "iterations": config.get("iterations", {}).get("measurement", 10),
    }


def _calculate_total_tests(params):
    """Calculate total number of tests to run"""
    return len(params["data_sizes"]) * (1 + len(params["concurrency_levels"]) + 1)


def _run_throughput_tests(benchmark, params, results, pbar):
    """Run all throughput tests"""
    for data_size in params["data_sizes"]:
        try:
            benchmark.logger.info(
                f"Running throughput test - Size: {data_size} bytes, "
                f"Iterations: {params['iterations']}"
            )
            result = run_throughput_test(
                benchmark,
                data_size,
                params["iterations"],
            )
            results.append(result)
            benchmark.logger.info(
                f"Throughput test completed: " f"{result.ops_per_second:.2f} ops/sec"
            )
        except Exception as e:
            benchmark.logger.error(f"Throughput test failed: {e}")
        pbar.update(1)


def _run_memory_tests(benchmark, params, results, pbar):
    """Run all memory tests"""
    for data_size in params["data_sizes"]:
        try:
            benchmark.logger.info(f"Running memory test - Size: {data_size} bytes")
            gc.collect()
            result = run_memory_test(benchmark, data_size)
            results.append(result)
            benchmark.logger.info(
                f"Memory test completed: {result.peak_memory_mb:.2f} MB peak"
            )
        except Exception as e:
            benchmark.logger.error(f"Memory test failed: {e}")
        pbar.update(1)


def _run_concurrent_tests(benchmark, params, results, pbar):
    """Run all concurrent tests"""
    for data_size in params["data_sizes"]:
        for concurrency in params["concurrency_levels"]:
            if concurrency > 1:
                try:
                    benchmark.logger.info(
                        f"Running concurrent test - Size: {data_size} bytes, "
                        f"Concurrency: {concurrency}"
                    )
                    result = run_concurrent_test(benchmark, data_size, concurrency)
                    results.append(result)
                    benchmark.logger.info(
                        f"Concurrent test completed: "
                        f"{result.ops_per_second:.2f} ops/sec "
                        f"@ {concurrency} threads"
                    )
                except Exception as e:
                    benchmark.logger.error(f"Concurrent test failed: {e}")
                pbar.update(1)

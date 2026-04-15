/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#ifndef ESDK_BENCHMARK_TESTS_H
#define ESDK_BENCHMARK_TESTS_H

#include "benchmark.h"

/**
 * Run a throughput test: warmup + measurement iterations, collect latencies,
 * compute ops/sec and percentiles.
 */
void run_throughput_test(struct esdk_benchmark *bench, int64_t data_size, int iterations);

/**
 * Run a memory test: 5 iterations with getrusage sampling.
 */
void run_memory_test(struct esdk_benchmark *bench, int64_t data_size);

/**
 * Run a concurrency test using pthreads.
 */
void run_concurrent_test(struct esdk_benchmark *bench, int64_t data_size,
                         int concurrency, int iterations_per_worker);

#endif /* ESDK_BENCHMARK_TESTS_H */

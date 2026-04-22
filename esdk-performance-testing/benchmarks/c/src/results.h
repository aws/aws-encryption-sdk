/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#ifndef ESDK_BENCHMARK_RESULTS_H
#define ESDK_BENCHMARK_RESULTS_H

#include <stddef.h>
#include <stdint.h>

struct benchmark_result {
    char test_name[32];
    char language[8]; /* "c" */
    int64_t data_size;
    int concurrency;
    double ops_per_second;
    double bytes_per_second;
    double peak_memory_mb;
    double memory_efficiency_ratio;
    double end_to_end_latency_ms;
    double p50_latency_ms;
    double p95_latency_ms;
    double p99_latency_ms;
    double encrypt_latency_ms;
    double decrypt_latency_ms;
    char timestamp[32];
    char c_compiler[64];
    int cpu_count;
    double total_memory_gb;
    int iterations;
};

/**
 * Save an array of benchmark results to a JSON file.
 * Returns 0 on success, -1 on failure.
 */
int save_results(const struct benchmark_result *results, size_t count, const char *output_path);

/* ---- statistical helpers ------------------------------------------------ */

double average(const double *arr, size_t n);
double percentile(const double *sorted_arr, size_t n, double p);
void get_timestamp(char *buf, size_t size);

#endif /* ESDK_BENCHMARK_RESULTS_H */

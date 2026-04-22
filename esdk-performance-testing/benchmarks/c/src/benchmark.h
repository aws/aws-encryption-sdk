/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#ifndef ESDK_BENCHMARK_BENCHMARK_H
#define ESDK_BENCHMARK_BENCHMARK_H

#include "config.h"
#include "results.h"

#include <aws/common/allocator.h>
#include <aws/cryptosdk/raw_aes_keyring.h>
#include <aws/cryptosdk/session.h>

struct esdk_benchmark {
    struct aws_cryptosdk_keyring *keyring;
    struct aws_allocator *alloc;
    struct test_config *config;
    int cpu_count;
    double total_memory_gb;
    char c_compiler[64];
    struct benchmark_result *results;
    size_t results_count;
    size_t results_capacity;
};

/**
 * Log a message with a timestamp prefix (like Go's log.Printf).
 */
void log_msg(const char *fmt, ...);

/**
 * Initialise the benchmark: create allocator, generate AES-256 key, build keyring.
 * Returns 0 on success, -1 on failure.
 */
int benchmark_init(struct esdk_benchmark *bench, struct test_config *config);

/**
 * Run a single encrypt-then-decrypt cycle.
 * Writes encrypt and decrypt latencies (ms) to *encrypt_ms and *decrypt_ms.
 * Returns 0 on success, -1 on integrity failure or SDK error.
 */
int run_encrypt_decrypt_cycle(struct esdk_benchmark *bench,
                              const uint8_t *data, int64_t data_len,
                              double *encrypt_ms, double *decrypt_ms);

/**
 * Run the full benchmark suite (throughput, memory, concurrency).
 */
void run_all_benchmarks(struct esdk_benchmark *bench);

/**
 * Add a result to the benchmark's results array (grows as needed).
 */
void benchmark_add_result(struct esdk_benchmark *bench, const struct benchmark_result *result);

/**
 * Generate random test data of the given size using a CSPRNG.
 * Caller must free the returned pointer with aws_mem_release.
 */
uint8_t *generate_test_data(struct aws_allocator *alloc, int64_t size);

/**
 * Release all resources held by the benchmark.
 */
void benchmark_cleanup(struct esdk_benchmark *bench);

#endif /* ESDK_BENCHMARK_BENCHMARK_H */

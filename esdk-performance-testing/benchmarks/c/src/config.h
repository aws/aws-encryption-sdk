/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#ifndef ESDK_BENCHMARK_CONFIG_H
#define ESDK_BENCHMARK_CONFIG_H

#include <stddef.h>
#include <stdint.h>

struct data_sizes {
    int64_t *small;
    size_t small_count;
    int64_t *medium;
    size_t medium_count;
    int64_t *large;
    size_t large_count;
};

struct iterations_config {
    int warmup;
    int measurement;
};

struct quick_config {
    struct data_sizes data_sizes;
    struct iterations_config iterations;
    int *concurrency_levels;
    size_t concurrency_count;
    char **test_types;
    size_t test_types_count;
};

struct test_config {
    struct data_sizes data_sizes;
    struct iterations_config iterations;
    int *concurrency_levels;
    size_t concurrency_count;
    struct quick_config *quick_config;
};

/**
 * Load test configuration from a YAML file.
 * Returns NULL on failure.
 */
struct test_config *load_config(const char *path);

/**
 * Adjust configuration for quick test mode by applying quick_config overrides.
 */
void adjust_for_quick_test(struct test_config *config);

/**
 * Check if a given test type should be run based on quick_config test_types.
 * Returns 1 if the test should run, 0 otherwise.
 */
int should_run_test_type(const struct test_config *config, const char *type);

/**
 * Collect all data sizes (small + medium + large) into a single array.
 * Caller must free *out_sizes.
 */
void get_all_data_sizes(const struct test_config *config, int64_t **out_sizes, size_t *out_count);

/**
 * Free all memory associated with a test_config.
 */
void free_config(struct test_config *config);

#endif /* ESDK_BENCHMARK_CONFIG_H */

/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#include "tests.h"
#include "results.h"

#include <inttypes.h>
#include <math.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include <sys/resource.h>

#ifdef __APPLE__
#include <mach/mach.h>
#include <malloc/malloc.h>
#endif

/* ---- helpers ------------------------------------------------------------ */

static int compare_double(const void *a, const void *b) {
    double da = *(const double *)a;
    double db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

static double elapsed_ms(const struct timespec *start, const struct timespec *end) {
    return ((double)(end->tv_sec - start->tv_sec) * 1000.0)
         + ((double)(end->tv_nsec - start->tv_nsec) / 1e6);
}

static double get_current_rss_mb(void) {
#ifdef __APPLE__
    mach_task_basic_info_data_t info;
    mach_msg_type_number_t count = MACH_TASK_BASIC_INFO_COUNT;
    if (task_info(mach_task_self(), MACH_TASK_BASIC_INFO,
                  (task_info_t)&info, &count) == KERN_SUCCESS) {
        return (double)info.resident_size / (1024.0 * 1024.0);
    }
    return 0.0;
#else
    /* Linux: read VmRSS from /proc/self/status */
    FILE *f = fopen("/proc/self/status", "r");
    if (!f) return 0.0;
    char line[256];
    double rss_mb = 0.0;
    while (fgets(line, sizeof(line), f)) {
        long rss_kb;
        if (sscanf(line, "VmRSS: %ld kB", &rss_kb) == 1) {
            rss_mb = (double)rss_kb / 1024.0;
            break;
        }
    }
    fclose(f);
    return rss_mb;
#endif
}

/* ---- throughput test ---------------------------------------------------- */

void run_throughput_test(struct esdk_benchmark *bench, int64_t data_size, int iterations) {
    log_msg("Running throughput test - Size: %" PRId64 " bytes, Iterations: %d",
           data_size, iterations);

    uint8_t *data = generate_test_data(bench->alloc, data_size);
    if (!data) {
        fprintf(stderr, "Failed to allocate test data\n");
        return;
    }

    /* Warmup */
    for (int i = 0; i < bench->config->iterations.warmup; i++) {
        double enc_ms, dec_ms;
        if (run_encrypt_decrypt_cycle(bench, data, data_size, &enc_ms, &dec_ms) != 0) {
            fprintf(stderr, "  Warmup iteration %d failed\n", i);
            aws_mem_release(bench->alloc, data);
            return;
        }
    }

    /* Measurement */
    double *encrypt_latencies = malloc((size_t)iterations * sizeof(double));
    double *decrypt_latencies = malloc((size_t)iterations * sizeof(double));
    double *e2e_latencies     = malloc((size_t)iterations * sizeof(double));
    if (!encrypt_latencies || !decrypt_latencies || !e2e_latencies) {
        aws_mem_release(bench->alloc, data); free(encrypt_latencies); free(decrypt_latencies); free(e2e_latencies);
        return;
    }

    struct timespec total_start, total_end;
    clock_gettime(CLOCK_MONOTONIC, &total_start);

    for (int i = 0; i < iterations; i++) {
        struct timespec iter_start, iter_end;
        double enc_ms, dec_ms;

        clock_gettime(CLOCK_MONOTONIC, &iter_start);
        if (run_encrypt_decrypt_cycle(bench, data, data_size, &enc_ms, &dec_ms) != 0) {
            fprintf(stderr, "  Measurement iteration %d failed\n", i);
            aws_mem_release(bench->alloc, data); free(encrypt_latencies); free(decrypt_latencies); free(e2e_latencies);
            return;
        }
        clock_gettime(CLOCK_MONOTONIC, &iter_end);

        encrypt_latencies[i] = enc_ms;
        decrypt_latencies[i] = dec_ms;
        e2e_latencies[i] = elapsed_ms(&iter_start, &iter_end);
    }

    clock_gettime(CLOCK_MONOTONIC, &total_end);
    double total_sec = elapsed_ms(&total_start, &total_end) / 1000.0;

    /* Sort for percentiles */
    qsort(e2e_latencies, (size_t)iterations, sizeof(double), compare_double);

    struct benchmark_result result;
    memset(&result, 0, sizeof(result));
    snprintf(result.test_name, sizeof(result.test_name), "%s", "throughput");
    snprintf(result.language, sizeof(result.language), "%s", "c");
    result.data_size = data_size;
    result.concurrency = 1;
    result.encrypt_latency_ms = average(encrypt_latencies, (size_t)iterations);
    result.decrypt_latency_ms = average(decrypt_latencies, (size_t)iterations);
    result.end_to_end_latency_ms = average(e2e_latencies, (size_t)iterations);
    result.ops_per_second = (double)iterations / total_sec;
    result.bytes_per_second = (double)iterations * (double)data_size / total_sec;
    result.p50_latency_ms = percentile(e2e_latencies, (size_t)iterations, 0.50);
    result.p95_latency_ms = percentile(e2e_latencies, (size_t)iterations, 0.95);
    result.p99_latency_ms = percentile(e2e_latencies, (size_t)iterations, 0.99);
    result.cpu_count = bench->cpu_count;
    result.total_memory_gb = bench->total_memory_gb;
    snprintf(result.c_compiler, sizeof(result.c_compiler), "%s", bench->c_compiler);
    result.iterations = iterations;
    get_timestamp(result.timestamp, sizeof(result.timestamp));

    benchmark_add_result(bench, &result);

    log_msg("Throughput test completed: %.2f ops/sec", result.ops_per_second);

    aws_mem_release(bench->alloc, data);
    free(encrypt_latencies);
    free(decrypt_latencies);
    free(e2e_latencies);
}

/* ---- memory test -------------------------------------------------------- */

/* ---- tracking allocator ------------------------------------------------- */

struct alloc_tracker {
    struct aws_allocator wrapper;
    struct aws_allocator *inner;
    size_t total_allocated;
    size_t current_allocated;
    size_t peak_allocated;
};

static void *tracking_acquire(struct aws_allocator *allocator, size_t size) {
    struct alloc_tracker *t = allocator->impl;
    /* Allocate extra header to store the size for release */
    size_t *ptr = t->inner->mem_acquire(t->inner, size + sizeof(size_t));
    if (!ptr) return NULL;
    *ptr = size;
    __atomic_fetch_add(&t->total_allocated, size, __ATOMIC_RELAXED);
    size_t cur = __atomic_add_fetch(&t->current_allocated, size, __ATOMIC_RELAXED);
    size_t peak;
    do {
        peak = __atomic_load_n(&t->peak_allocated, __ATOMIC_RELAXED);
    } while (cur > peak && !__atomic_compare_exchange_n(&t->peak_allocated, &peak, cur, 1, __ATOMIC_RELAXED, __ATOMIC_RELAXED));
    return ptr + 1;
}

static void tracking_release(struct aws_allocator *allocator, void *ptr) {
    if (!ptr) return;
    struct alloc_tracker *t = allocator->impl;
    size_t *header = (size_t *)ptr - 1;
    __atomic_fetch_sub(&t->current_allocated, *header, __ATOMIC_RELAXED);
    t->inner->mem_release(t->inner, header);
}

static void *tracking_realloc(struct aws_allocator *allocator, void *oldptr, size_t oldsize, size_t newsize) {
    (void)oldsize;
    if (!oldptr) return tracking_acquire(allocator, newsize);
    struct alloc_tracker *t = allocator->impl;
    size_t *old_header = (size_t *)oldptr - 1;
    size_t real_oldsize = *old_header;
    size_t *ptr = t->inner->mem_realloc(t->inner, old_header, real_oldsize + sizeof(size_t), newsize + sizeof(size_t));
    if (!ptr) return NULL;
    *ptr = newsize;
    __atomic_fetch_sub(&t->current_allocated, real_oldsize, __ATOMIC_RELAXED);
    __atomic_fetch_add(&t->total_allocated, newsize, __ATOMIC_RELAXED);
    size_t cur = __atomic_add_fetch(&t->current_allocated, newsize, __ATOMIC_RELAXED);
    size_t peak;
    do {
        peak = __atomic_load_n(&t->peak_allocated, __ATOMIC_RELAXED);
    } while (cur > peak && !__atomic_compare_exchange_n(&t->peak_allocated, &peak, cur, 1, __ATOMIC_RELAXED, __ATOMIC_RELAXED));
    return ptr + 1;
}

static void alloc_tracker_init(struct alloc_tracker *t, struct aws_allocator *inner) {
    t->inner = inner;
    t->total_allocated = 0;
    t->current_allocated = 0;
    t->peak_allocated = 0;
    t->wrapper.mem_acquire = tracking_acquire;
    t->wrapper.mem_release = tracking_release;
    t->wrapper.mem_realloc = inner->mem_realloc ? tracking_realloc : NULL;
    t->wrapper.mem_calloc = NULL; /* falls back to acquire */
    t->wrapper.impl = t;
}

static void alloc_tracker_reset(struct alloc_tracker *t) {
    t->total_allocated = 0;
    t->current_allocated = 0;
    t->peak_allocated = 0;
}

/* ---- RSS background sampler --------------------------------------------- */

struct rss_sampler {
    pthread_t thread;
    double baseline_mb;
    double peak_mb;
    double sum_mb;
    int count;
    int stop;  /* accessed via __atomic builtins */
};

static void *rss_sampler_fn(void *arg) {
    struct rss_sampler *s = arg;
    while (!__atomic_load_n(&s->stop, __ATOMIC_RELAXED)) {
        double rss = get_current_rss_mb();
        if (rss > s->peak_mb) s->peak_mb = rss;
        s->sum_mb += rss;
        s->count++;
        usleep(1000); /* 1ms polling */
    }
    return NULL;
}

#define MEMORY_TEST_ITERATIONS 5

void run_memory_test(struct esdk_benchmark *bench, int64_t data_size) {
    log_msg("Running memory test - Size: %" PRId64 " bytes (%d iterations, continuous sampling)",
           data_size, MEMORY_TEST_ITERATIONS);

    uint8_t *data = generate_test_data(bench->alloc, data_size);
    if (!data) {
        fprintf(stderr, "Failed to allocate test data\n");
        return;
    }

    /* Wrap the allocator to track allocations, including keyring internals. */
    struct alloc_tracker tracker;
    alloc_tracker_init(&tracker, bench->alloc);
    struct aws_allocator *orig_alloc = bench->alloc;
    bench->alloc = &tracker.wrapper;

    /* Rebuild keyring with tracking allocator so its allocations are counted. */
    struct aws_cryptosdk_keyring *orig_keyring = bench->keyring;
    {
        /* Use a fixed key — this keyring is only for memory measurement, not security. */
        uint8_t wrapping_key[32];
        memset(wrapping_key, 0x42, sizeof(wrapping_key));
        struct aws_string *ns = aws_string_new_from_c_str(&tracker.wrapper, "esdk-performance-test");
        struct aws_string *nm = aws_string_new_from_c_str(&tracker.wrapper, "test-aes-256-key");
        bench->keyring = aws_cryptosdk_raw_aes_keyring_new(
            &tracker.wrapper, ns, nm, wrapping_key, AWS_CRYPTOSDK_AES256);
        aws_string_destroy((void *)ns);
        aws_string_destroy((void *)nm);
        if (!bench->keyring) {
            fprintf(stderr, "Failed to rebuild keyring for memory test\n");
            bench->alloc = orig_alloc;
            bench->keyring = orig_keyring;
            aws_mem_release(orig_alloc, data);
            return;
        }
    }

    double overall_peak_heap = 0.0;
    double overall_peak_allocs = 0.0;
    double avg_heap_values[MEMORY_TEST_ITERATIONS];
    int valid_samples = 0;

    for (int i = 0; i < MEMORY_TEST_ITERATIONS; i++) {
#ifdef __APPLE__
        malloc_zone_pressure_relief(malloc_default_zone(), 0);
#elif defined(__GLIBC__)
        malloc_trim(0);
#endif
        usleep(50000);

        alloc_tracker_reset(&tracker);

        struct rss_sampler sampler = {
            .baseline_mb = get_current_rss_mb(), .peak_mb = get_current_rss_mb(),
            .sum_mb = 0.0, .count = 0, .stop = 0
        };
        pthread_create(&sampler.thread, NULL, rss_sampler_fn, &sampler);

        struct timespec op_start, op_end;
        clock_gettime(CLOCK_MONOTONIC, &op_start);

        double enc_ms, dec_ms;
        int rc = run_encrypt_decrypt_cycle(bench, data, data_size, &enc_ms, &dec_ms);

        clock_gettime(CLOCK_MONOTONIC, &op_end);
        __atomic_store_n(&sampler.stop, 1, __ATOMIC_RELAXED);
        pthread_join(sampler.thread, NULL);

        if (rc != 0) {
            fprintf(stderr, "  Iteration %d failed\n", i + 1);
            continue;
        }

        double peak_heap_mb = (double)tracker.peak_allocated / (1024.0 * 1024.0);
        double total_allocs_mb = (double)tracker.total_allocated / (1024.0 * 1024.0);
        double avg_rss = (sampler.count > 0) ? sampler.sum_mb / sampler.count : 0.0;
        double duration_ms = elapsed_ms(&op_start, &op_end);

        avg_heap_values[valid_samples++] = peak_heap_mb;
        if (peak_heap_mb > overall_peak_heap) overall_peak_heap = peak_heap_mb;
        if (total_allocs_mb > overall_peak_allocs) overall_peak_allocs = total_allocs_mb;

        log_msg("=== Iteration %d === Peak Heap: %.2f MB, Total Allocs: %.2f MB, Avg RSS: %.2f MB (%.3fs, %d samples)",
                i + 1, peak_heap_mb, total_allocs_mb, avg_rss, duration_ms / 1000.0, sampler.count);
    }

    /* Restore original allocator and keyring */
    aws_cryptosdk_keyring_release(bench->keyring);
    bench->keyring = orig_keyring;
    bench->alloc = orig_alloc;

    if (valid_samples == 0) {
        fprintf(stderr, "  All memory test iterations failed\n");
        aws_mem_release(bench->alloc, data);
        return;
    }

    double overall_avg_heap = average(avg_heap_values, (size_t)valid_samples);
    double memory_efficiency = (overall_avg_heap > 0.0)
        ? (double)data_size / (overall_avg_heap * 1024.0 * 1024.0)
        : 0.0;

    log_msg("Memory Summary:");
    log_msg("- Absolute Peak Heap: %.2f MB (across all runs)", overall_peak_heap);
    log_msg("- Average Heap: %.2f MB (across all runs)", overall_avg_heap);
    log_msg("- Total Allocations: %.2f MB (max across all runs)", overall_peak_allocs);

    struct benchmark_result result;
    memset(&result, 0, sizeof(result));
    snprintf(result.test_name, sizeof(result.test_name), "%s", "memory");
    snprintf(result.language, sizeof(result.language), "%s", "c");
    result.data_size = data_size;
    result.concurrency = 1;
    result.peak_memory_mb = overall_peak_heap;
    result.memory_efficiency_ratio = memory_efficiency;
    result.cpu_count = bench->cpu_count;
    result.total_memory_gb = bench->total_memory_gb;
    snprintf(result.c_compiler, sizeof(result.c_compiler), "%s", bench->c_compiler);
    result.iterations = MEMORY_TEST_ITERATIONS;
    get_timestamp(result.timestamp, sizeof(result.timestamp));

    benchmark_add_result(bench, &result);

    log_msg("Memory test completed: %.2f MB peak", overall_peak_heap);

    aws_mem_release(bench->alloc, data);
}

/* ---- concurrency test --------------------------------------------------- */

struct worker_args {
    struct esdk_benchmark *bench;
    const uint8_t *data;
    int64_t data_len;
    int iterations;
    double *latencies;   /* pre-allocated, length = iterations */
    int success;
};

static void *concurrent_worker(void *arg) {
    struct worker_args *wa = (struct worker_args *)arg;
    wa->success = 1;

    for (int i = 0; i < wa->iterations; i++) {
        struct timespec iter_start, iter_end;
        double enc_ms, dec_ms;

        clock_gettime(CLOCK_MONOTONIC, &iter_start);
        if (run_encrypt_decrypt_cycle(wa->bench, wa->data, wa->data_len, &enc_ms, &dec_ms) != 0) {
            fprintf(stderr, "  Concurrent worker iteration %d failed\n", i);
            wa->success = 0;
            return NULL;
        }
        clock_gettime(CLOCK_MONOTONIC, &iter_end);
        wa->latencies[i] = elapsed_ms(&iter_start, &iter_end);
    }
    return NULL;
}

void run_concurrent_test(struct esdk_benchmark *bench, int64_t data_size,
                         int concurrency, int iterations_per_worker) {
    log_msg("Running concurrent test - Size: %" PRId64 " bytes, Concurrency: %d",
           data_size, concurrency);

    uint8_t *data = generate_test_data(bench->alloc, data_size);
    if (!data) {
        fprintf(stderr, "Failed to allocate test data\n");
        return;
    }

    int total_ops = concurrency * iterations_per_worker;

    /* Allocate per-worker latency arrays */
    double **worker_latencies = malloc((size_t)concurrency * sizeof(double *));
    struct worker_args *args = malloc((size_t)concurrency * sizeof(struct worker_args));
    pthread_t *threads = malloc((size_t)concurrency * sizeof(pthread_t));
    if (!worker_latencies || !args || !threads) {
        aws_mem_release(bench->alloc, data); free(worker_latencies); free(args); free(threads);
        return;
    }

    for (int i = 0; i < concurrency; i++) {
        worker_latencies[i] = malloc((size_t)iterations_per_worker * sizeof(double));
        if (!worker_latencies[i]) {
            for (int j = 0; j < i; j++) free(worker_latencies[j]);
            free(worker_latencies); free(args); free(threads); aws_mem_release(bench->alloc, data);
            return;
        }
        args[i].bench = bench;
        args[i].data = data;
        args[i].data_len = data_size;
        args[i].iterations = iterations_per_worker;
        args[i].latencies = worker_latencies[i];
        args[i].success = 0;
    }

    struct timespec total_start, total_end;
    clock_gettime(CLOCK_MONOTONIC, &total_start);

    /* Launch workers */
    for (int i = 0; i < concurrency; i++) {
        if (pthread_create(&threads[i], NULL, concurrent_worker, &args[i]) != 0) {
            fprintf(stderr, "  Failed to create thread %d\n", i);
        }
    }

    /* Join workers */
    for (int i = 0; i < concurrency; i++) {
        pthread_join(threads[i], NULL);
    }

    clock_gettime(CLOCK_MONOTONIC, &total_end);
    double total_sec = elapsed_ms(&total_start, &total_end) / 1000.0;

    /* Check for errors */
    for (int i = 0; i < concurrency; i++) {
        if (!args[i].success) {
            fprintf(stderr, "  Concurrent test failed (worker error)\n");
            for (int j = 0; j < concurrency; j++) free(worker_latencies[j]);
            free(worker_latencies); free(args); free(threads); aws_mem_release(bench->alloc, data);
            return;
        }
    }

    /* Collect all latencies */
    double *all_latencies = malloc((size_t)total_ops * sizeof(double));
    if (!all_latencies) {
        for (int j = 0; j < concurrency; j++) free(worker_latencies[j]);
        free(worker_latencies); free(args); free(threads); aws_mem_release(bench->alloc, data);
        return;
    }

    int idx = 0;
    for (int i = 0; i < concurrency; i++) {
        for (int j = 0; j < iterations_per_worker; j++) {
            all_latencies[idx++] = worker_latencies[i][j];
        }
    }

    qsort(all_latencies, (size_t)total_ops, sizeof(double), compare_double);

    struct benchmark_result result;
    memset(&result, 0, sizeof(result));
    snprintf(result.test_name, sizeof(result.test_name), "%s", "concurrent");
    snprintf(result.language, sizeof(result.language), "%s", "c");
    result.data_size = data_size;
    result.concurrency = concurrency;
    result.end_to_end_latency_ms = average(all_latencies, (size_t)total_ops);
    result.ops_per_second = (double)total_ops / total_sec;
    result.bytes_per_second = (double)total_ops * (double)data_size / total_sec;
    result.p50_latency_ms = percentile(all_latencies, (size_t)total_ops, 0.50);
    result.p95_latency_ms = percentile(all_latencies, (size_t)total_ops, 0.95);
    result.p99_latency_ms = percentile(all_latencies, (size_t)total_ops, 0.99);
    result.cpu_count = bench->cpu_count;
    result.total_memory_gb = bench->total_memory_gb;
    snprintf(result.c_compiler, sizeof(result.c_compiler), "%s", bench->c_compiler);
    result.iterations = total_ops;
    get_timestamp(result.timestamp, sizeof(result.timestamp));

    benchmark_add_result(bench, &result);

    log_msg("Concurrent test completed: %.2f ops/sec @ %d threads",
           result.ops_per_second, concurrency);

    free(all_latencies);
    for (int i = 0; i < concurrency; i++) free(worker_latencies[i]);
    free(worker_latencies);
    free(args);
    free(threads);
    aws_mem_release(bench->alloc, data);
}

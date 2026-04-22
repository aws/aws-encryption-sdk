/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#include "benchmark.h"
#include "tests.h"

#include <inttypes.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef __APPLE__
#include <sys/sysctl.h>
#endif

#include <aws/common/common.h>
#include <aws/common/hash_table.h>
#include <aws/common/string.h>
#include <aws/cryptosdk/enc_ctx.h>
#include <aws/cryptosdk/error.h>
#include <aws/cryptosdk/raw_aes_keyring.h>
#include <aws/cryptosdk/session.h>

/* ---- platform helpers --------------------------------------------------- */

#ifndef __APPLE__
#include <unistd.h>
#endif

static int get_cpu_count(void) {
#ifdef __APPLE__
    int count = 0;
    size_t len = sizeof(count);
    if (sysctlbyname("hw.logicalcpu", &count, &len, NULL, 0) == 0) return count;
    return 1;
#else
    long n = sysconf(_SC_NPROCESSORS_ONLN);
    return n > 0 ? (int)n : 1;
#endif
}

static double get_total_memory_gb(void) {
#ifdef __APPLE__
    int64_t mem = 0;
    size_t len = sizeof(mem);
    if (sysctlbyname("hw.memsize", &mem, &len, NULL, 0) == 0)
        return (double)mem / (1024.0 * 1024.0 * 1024.0);
    return 0.0;
#else
    long pages = sysconf(_SC_PHYS_PAGES);
    long page_size = sysconf(_SC_PAGE_SIZE);
    if (pages > 0 && page_size > 0)
        return (double)pages * (double)page_size / (1024.0 * 1024.0 * 1024.0);
    return 0.0;
#endif
}

static int secure_random(uint8_t *buf, size_t len) {
#ifdef __APPLE__
    arc4random_buf(buf, len);
    return 0;
#else
    FILE *f = fopen("/dev/urandom", "rb");
    if (!f) {
        fprintf(stderr, "Failed to open /dev/urandom\n");
        return -1;
    }
    size_t n = fread(buf, 1, len, f);
    fclose(f);
    if (n != len) {
        fprintf(stderr, "Short read from /dev/urandom: got %zu, expected %zu\n", n, len);
        return -1;
    }
    return 0;
#endif
}

/* ---- public API --------------------------------------------------------- */

void log_msg(const char *fmt, ...) {
    time_t now = time(NULL);
    struct tm *tm = localtime(&now);
    fprintf(stdout, "%04d/%02d/%02d %02d:%02d:%02d ",
            tm->tm_year + 1900, tm->tm_mon + 1, tm->tm_mday,
            tm->tm_hour, tm->tm_min, tm->tm_sec);
    va_list args;
    va_start(args, fmt);
    vfprintf(stdout, fmt, args);
    va_end(args);
    fprintf(stdout, "\n");
    fflush(stdout);
}

uint8_t *generate_test_data(struct aws_allocator *alloc, int64_t size) {
    uint8_t *data = aws_mem_acquire(alloc, (size_t)size);
    if (!data) return NULL;
    if (secure_random(data, (size_t)size) != 0) {
        aws_mem_release(alloc, data);
        return NULL;
    }
    return data;
}

int benchmark_init(struct esdk_benchmark *bench, struct test_config *config) {
    memset(bench, 0, sizeof(*bench));
    bench->config = config;
    bench->alloc = aws_default_allocator();
    bench->cpu_count = get_cpu_count();
    bench->total_memory_gb = get_total_memory_gb();

#if defined(__clang__)
    snprintf(bench->c_compiler, sizeof(bench->c_compiler),
             "clang %d.%d.%d", __clang_major__, __clang_minor__, __clang_patchlevel__);
#elif defined(__GNUC__)
    snprintf(bench->c_compiler, sizeof(bench->c_compiler),
             "gcc %d.%d.%d", __GNUC__, __GNUC_MINOR__, __GNUC_PATCHLEVEL__);
#else
    snprintf(bench->c_compiler, sizeof(bench->c_compiler), "unknown");
#endif

    /* Pre-allocate results array */
    bench->results_capacity = 64;
    bench->results = calloc(bench->results_capacity, sizeof(struct benchmark_result));
    if (!bench->results) {
        fprintf(stderr, "Failed to allocate results array\n");
        return -1;
    }

    /* Generate a random 256-bit AES wrapping key */
    uint8_t wrapping_key[32];
    if (secure_random(wrapping_key, sizeof(wrapping_key)) != 0) {
        fprintf(stderr, "Failed to generate random wrapping key\n");
        free(bench->results);
        return -1;
    }

    struct aws_string *key_namespace = aws_string_new_from_c_str(bench->alloc, "esdk-performance-test");
    struct aws_string *key_name = aws_string_new_from_c_str(bench->alloc, "test-aes-256-key");

    bench->keyring = aws_cryptosdk_raw_aes_keyring_new(
        bench->alloc, key_namespace, key_name,
        wrapping_key, AWS_CRYPTOSDK_AES256);

    aws_string_destroy((void *)key_namespace);
    aws_string_destroy((void *)key_name);

    if (!bench->keyring) {
        fprintf(stderr, "Failed to create raw AES keyring\n");
        free(bench->results);
        return -1;
    }

    /* The raw AES keyring is thread-safe for concurrent encrypt/decrypt:
     * it only reads immutable fields (key_namespace, key_name, raw_key)
     * and uses atomic refcounting. Each session gets its own CMM. */

    log_msg("ESDK client initialized successfully");
    log_msg("Initialized ESDK Benchmark - CPU cores: %d, Memory: %.1fGB",
            bench->cpu_count, bench->total_memory_gb);

    return 0;
}

void benchmark_add_result(struct esdk_benchmark *bench, const struct benchmark_result *result) {
    if (bench->results_count >= bench->results_capacity) {
        bench->results_capacity *= 2;
        struct benchmark_result *tmp = realloc(bench->results,
            bench->results_capacity * sizeof(struct benchmark_result));
        if (!tmp) {
            fprintf(stderr, "Failed to grow results array\n");
            return;
        }
        bench->results = tmp;
    }
    bench->results[bench->results_count++] = *result;
}

int run_encrypt_decrypt_cycle(struct esdk_benchmark *bench,
                              const uint8_t *data, int64_t data_len,
                              double *encrypt_ms, double *decrypt_ms) {
    struct timespec ts_start, ts_end;
    int rc;

    /* Allocate buffers — account for ESDK overhead:
     * header (~600 bytes) + per-frame overhead (32 bytes per 4096-byte frame)
     * + footer. For safety, add ~1% + 64KB. */
    size_t overhead = (size_t)(data_len / 100) + 65536;
    size_t ct_buf_size = (size_t)data_len + overhead;
    uint8_t *ciphertext = aws_mem_acquire(bench->alloc, ct_buf_size);
    if (!ciphertext) return -1;

    size_t pt_buf_size = (size_t)data_len + overhead;
    uint8_t *plaintext = aws_mem_acquire(bench->alloc, pt_buf_size);
    if (!plaintext) { aws_mem_release(bench->alloc, ciphertext); return -1; }

    size_t ct_len = 0;
    size_t pt_len = 0;

    /* ---- Encrypt ---- */
    struct aws_cryptosdk_session *enc_session =
        aws_cryptosdk_session_new_from_keyring_2(bench->alloc, AWS_CRYPTOSDK_ENCRYPT, bench->keyring);
    if (!enc_session) { aws_mem_release(bench->alloc, ciphertext); aws_mem_release(bench->alloc, plaintext); return -1; }

    rc = aws_cryptosdk_session_set_commitment_policy(enc_session,
        COMMITMENT_POLICY_REQUIRE_ENCRYPT_REQUIRE_DECRYPT);
    if (rc != AWS_OP_SUCCESS) {
        fprintf(stderr, "set_commitment_policy failed: %s\n", aws_error_str(aws_last_error()));
        aws_cryptosdk_session_destroy(enc_session);
        aws_mem_release(bench->alloc, ciphertext); aws_mem_release(bench->alloc, plaintext);
        return -1;
    }

    /* Set encryption context to match Go implementation.
     * The enc_ctx hash table takes ownership of keys and values on successful put
     * (it uses aws_hash_callback_string_destroy). We must free on failure. */
    struct aws_hash_table *enc_ctx = aws_cryptosdk_session_get_enc_ctx_ptr_mut(enc_session);
    if (enc_ctx) {
        char size_str[32];
        snprintf(size_str, sizeof(size_str), "%" PRId64, data_len);
        struct aws_string *k1 = aws_string_new_from_c_str(bench->alloc, "purpose");
        struct aws_string *v1 = aws_string_new_from_c_str(bench->alloc, "performance-test");
        if (k1 && v1) {
            if (aws_hash_table_put(enc_ctx, k1, (void *)v1, NULL) != AWS_OP_SUCCESS) {
                aws_string_destroy(k1);
                aws_string_destroy(v1);
            }
        } else {
            aws_string_destroy(k1);
            aws_string_destroy(v1);
        }
        struct aws_string *k2 = aws_string_new_from_c_str(bench->alloc, "size");
        struct aws_string *v2 = aws_string_new_from_c_str(bench->alloc, size_str);
        if (k2 && v2) {
            if (aws_hash_table_put(enc_ctx, k2, (void *)v2, NULL) != AWS_OP_SUCCESS) {
                aws_string_destroy(k2);
                aws_string_destroy(v2);
            }
        } else {
            aws_string_destroy(k2);
            aws_string_destroy(v2);
        }
    }

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    rc = aws_cryptosdk_session_process_full(enc_session, ciphertext, ct_buf_size, &ct_len,
                                            data, (size_t)data_len);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    *encrypt_ms = ((double)(ts_end.tv_sec - ts_start.tv_sec) * 1000.0)
                + ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1e6);

    aws_cryptosdk_session_destroy(enc_session);

    if (rc != AWS_OP_SUCCESS) {
        fprintf(stderr, "Encryption failed: %s\n", aws_error_str(aws_last_error()));
        aws_mem_release(bench->alloc, ciphertext);
        aws_mem_release(bench->alloc, plaintext);
        return -1;
    }

    /* ---- Decrypt ---- */
    struct aws_cryptosdk_session *dec_session =
        aws_cryptosdk_session_new_from_keyring_2(bench->alloc, AWS_CRYPTOSDK_DECRYPT, bench->keyring);
    if (!dec_session) { aws_mem_release(bench->alloc, ciphertext); aws_mem_release(bench->alloc, plaintext); return -1; }

    aws_cryptosdk_session_set_commitment_policy(dec_session,
        COMMITMENT_POLICY_REQUIRE_ENCRYPT_REQUIRE_DECRYPT);

    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    rc = aws_cryptosdk_session_process_full(dec_session, plaintext, pt_buf_size, &pt_len,
                                            ciphertext, ct_len);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    *decrypt_ms = ((double)(ts_end.tv_sec - ts_start.tv_sec) * 1000.0)
                + ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1e6);

    aws_cryptosdk_session_destroy(dec_session);

    aws_mem_release(bench->alloc, ciphertext);

    if (rc != AWS_OP_SUCCESS) {
        fprintf(stderr, "Decryption failed: %s\n", aws_error_str(aws_last_error()));
        aws_mem_release(bench->alloc, plaintext);
        return -1;
    }

    /* ---- Verify integrity ---- */
    if ((int64_t)pt_len != data_len || memcmp(data, plaintext, (size_t)data_len) != 0) {
        fprintf(stderr, "Data integrity check failed\n");
        aws_mem_release(bench->alloc, plaintext);
        return -1;
    }

    aws_mem_release(bench->alloc, plaintext);
    return 0;
}

void run_all_benchmarks(struct esdk_benchmark *bench) {
    log_msg("Starting comprehensive ESDK C benchmark suite");

    int64_t *data_sizes = NULL;
    size_t data_sizes_count = 0;
    get_all_data_sizes(bench->config, &data_sizes, &data_sizes_count);

    if (!data_sizes || data_sizes_count == 0) {
        fprintf(stderr, "No data sizes configured\n");
        return;
    }

    /* Throughput tests */
    if (should_run_test_type(bench->config, "throughput")) {
        log_msg("Running throughput tests...");
        for (size_t i = 0; i < data_sizes_count; i++) {
            run_throughput_test(bench, data_sizes[i], bench->config->iterations.measurement);
        }
    } else {
        log_msg("Skipping throughput tests (not in test_types)");
    }

    /* Memory tests */
    if (should_run_test_type(bench->config, "memory")) {
        log_msg("Running memory tests...");
        for (size_t i = 0; i < data_sizes_count; i++) {
            run_memory_test(bench, data_sizes[i]);
        }
    } else {
        log_msg("Skipping memory tests (not in test_types)");
    }

    /* Concurrency tests */
    if (should_run_test_type(bench->config, "concurrency")) {
        log_msg("Running concurrency tests...");
        for (size_t i = 0; i < data_sizes_count; i++) {
            for (size_t c = 0; c < bench->config->concurrency_count; c++) {
                int concurrency = bench->config->concurrency_levels[c];
                if (concurrency > 1) {
                    run_concurrent_test(bench, data_sizes[i], concurrency, 5);
                }
            }
        }
    } else {
        log_msg("Skipping concurrency tests (not in test_types)");
    }

    free(data_sizes);
    log_msg("Benchmark suite completed. Total results: %zu", bench->results_count);
}

void benchmark_cleanup(struct esdk_benchmark *bench) {
    if (bench->keyring) {
        aws_cryptosdk_keyring_release(bench->keyring);
        bench->keyring = NULL;
    }
    free(bench->results);
    bench->results = NULL;
    bench->results_count = 0;
    bench->results_capacity = 0;
}

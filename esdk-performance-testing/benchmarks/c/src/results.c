/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#include "results.h"

#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>

#include <json-c/json.h>

/* ---- statistical helpers ------------------------------------------------ */

double average(const double *arr, size_t n) {
    if (n == 0) return 0.0;
    double sum = 0.0;
    for (size_t i = 0; i < n; i++) sum += arr[i];
    return sum / (double)n;
}

double percentile(const double *sorted_arr, size_t n, double p) {
    if (n == 0) return 0.0;
    if (n == 1) return sorted_arr[0];
    double idx = p * (double)(n - 1);
    size_t lo = (size_t)idx;
    double frac = idx - (double)lo;
    if (lo + 1 >= n) return sorted_arr[n - 1];
    return sorted_arr[lo] + frac * (sorted_arr[lo + 1] - sorted_arr[lo]);
}

void get_timestamp(char *buf, size_t size) {
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    strftime(buf, size, "%Y-%m-%d %H:%M:%S", tm_info);
}

/* ---- JSON output -------------------------------------------------------- */

static void ensure_parent_dir(const char *path) {
    char *tmp = strdup(path);
    if (!tmp) return;

    /* find last '/' */
    char *last_slash = strrchr(tmp, '/');
    if (last_slash) {
        *last_slash = '\0';
        /* simple recursive mkdir — works for typical relative paths */
        for (char *p = tmp + 1; *p; p++) {
            if (*p == '/') {
                *p = '\0';
                mkdir(tmp, 0755);
                *p = '/';
            }
        }
        mkdir(tmp, 0755);
    }
    free(tmp);
}

static json_object *result_to_json(const struct benchmark_result *r) {
    json_object *obj = json_object_new_object();

    json_object_object_add(obj, "test_name", json_object_new_string(r->test_name));
    json_object_object_add(obj, "language", json_object_new_string(r->language));
    json_object_object_add(obj, "data_size", json_object_new_int64(r->data_size));
    json_object_object_add(obj, "concurrency", json_object_new_int(r->concurrency));
    json_object_object_add(obj, "encrypt_latency_ms", json_object_new_double(r->encrypt_latency_ms));
    json_object_object_add(obj, "decrypt_latency_ms", json_object_new_double(r->decrypt_latency_ms));
    json_object_object_add(obj, "end_to_end_latency_ms", json_object_new_double(r->end_to_end_latency_ms));
    json_object_object_add(obj, "ops_per_second", json_object_new_double(r->ops_per_second));
    json_object_object_add(obj, "bytes_per_second", json_object_new_double(r->bytes_per_second));
    json_object_object_add(obj, "peak_memory_mb", json_object_new_double(r->peak_memory_mb));
    json_object_object_add(obj, "memory_efficiency_ratio", json_object_new_double(r->memory_efficiency_ratio));
    json_object_object_add(obj, "p50_latency", json_object_new_double(r->p50_latency_ms));
    json_object_object_add(obj, "p95_latency", json_object_new_double(r->p95_latency_ms));
    json_object_object_add(obj, "p99_latency", json_object_new_double(r->p99_latency_ms));
    json_object_object_add(obj, "timestamp", json_object_new_string(r->timestamp));
    json_object_object_add(obj, "c_compiler", json_object_new_string(r->c_compiler));
    json_object_object_add(obj, "cpu_count", json_object_new_int(r->cpu_count));
    json_object_object_add(obj, "total_memory_gb", json_object_new_double(r->total_memory_gb));
    json_object_object_add(obj, "iterations", json_object_new_int(r->iterations));

    return obj;
}

int save_results(const struct benchmark_result *results, size_t count, const char *output_path) {
    ensure_parent_dir(output_path);

    /* metadata */
    json_object *metadata = json_object_new_object();
    json_object_object_add(metadata, "language", json_object_new_string("c"));

    char ts[32];
    get_timestamp(ts, sizeof(ts));
    json_object_object_add(metadata, "timestamp", json_object_new_string(ts));
    json_object_object_add(metadata, "total_tests", json_object_new_int((int)count));

    if (count > 0) {
        json_object_object_add(metadata, "c_compiler", json_object_new_string(results[0].c_compiler));
        json_object_object_add(metadata, "cpu_count", json_object_new_int(results[0].cpu_count));
        json_object_object_add(metadata, "total_memory_gb", json_object_new_double(results[0].total_memory_gb));
    }

    /* results array */
    json_object *results_arr = json_object_new_array();
    for (size_t i = 0; i < count; i++) {
        json_object_array_add(results_arr, result_to_json(&results[i]));
    }

    /* root object */
    json_object *root = json_object_new_object();
    json_object_object_add(root, "metadata", metadata);
    json_object_object_add(root, "results", results_arr);

    FILE *fp = fopen(output_path, "w");
    if (!fp) {
        fprintf(stderr, "Failed to open output file: %s (%s)\n", output_path, strerror(errno));
        json_object_put(root);
        return -1;
    }

    const char *json_str = json_object_to_json_string_ext(root, JSON_C_TO_STRING_PRETTY);
    fprintf(fp, "%s\n", json_str);
    fclose(fp);

    json_object_put(root);
    return 0;
}

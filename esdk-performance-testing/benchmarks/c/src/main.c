/* Copyright Amazon.com Inc. or its affiliates. All Rights Reserved. */
/* SPDX-License-Identifier: Apache-2.0 */

#include "benchmark.h"
#include "config.h"
#include "results.h"

#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <aws/common/common.h>
#include <aws/cryptosdk/error.h>

static void print_usage(const char *prog) {
    printf("Usage: %s [OPTIONS]\n", prog);
    printf("  -c, --config PATH   Path to test configuration YAML (default: ../../config/test-scenarios.yaml)\n");
    printf("  -o, --output PATH   Path to output results JSON   (default: ../../results/raw-data/c_results.json)\n");
    printf("  -q, --quick         Run quick test with reduced iterations\n");
    printf("  -h, --help          Show this help message\n");
}

int main(int argc, char *argv[]) {
    const char *config_path = "../../config/test-scenarios.yaml";
    const char *output_path = "../../results/raw-data/c_results.json";
    int quick = 0;

    static struct option long_options[] = {
        {"config", required_argument, NULL, 'c'},
        {"output", required_argument, NULL, 'o'},
        {"quick",  no_argument,       NULL, 'q'},
        {"help",   no_argument,       NULL, 'h'},
        {NULL, 0, NULL, 0}
    };

    int opt;
    while ((opt = getopt_long(argc, argv, "c:o:qh", long_options, NULL)) != -1) {
        switch (opt) {
        case 'c': config_path = optarg; break;
        case 'o': output_path = optarg; break;
        case 'q': quick = 1; break;
        case 'h': print_usage(argv[0]); return 0;
        default:  print_usage(argv[0]); return 1;
        }
    }

    /* Initialise AWS libraries */
    aws_common_library_init(aws_default_allocator());
    aws_cryptosdk_load_error_strings();

    /* Load configuration */
    struct test_config *config = load_config(config_path);
    if (!config) {
        fprintf(stderr, "Failed to load config: %s\n", config_path);
        return 1;
    }

    /* Adjust for quick mode */
    if (quick) {
        if (!config->quick_config) {
            fprintf(stderr, "Quick mode requested but no quick_config found in config file\n");
            free_config(config);
            return 1;
        }
        adjust_for_quick_test(config);
    }

    /* Initialise benchmark */
    struct esdk_benchmark bench;
    if (benchmark_init(&bench, config) != 0) {
        fprintf(stderr, "Failed to initialise benchmark\n");
        free_config(config);
        return 1;
    }

    /* Run benchmarks */
    run_all_benchmarks(&bench);

    /* Save results */
    if (save_results(bench.results, bench.results_count, output_path) != 0) {
        fprintf(stderr, "Failed to save results to %s\n", output_path);
    }

    /* Print summary */
    printf("\n=== ESDK C Benchmark Summary ===\n");
    printf("Total tests completed: %zu\n", bench.results_count);
    printf("Results saved to: %s\n", output_path);

    double max_throughput = 0.0;
    for (size_t i = 0; i < bench.results_count; i++) {
        if (strcmp(bench.results[i].test_name, "throughput") == 0 &&
            bench.results[i].ops_per_second > max_throughput) {
            max_throughput = bench.results[i].ops_per_second;
        }
    }
    if (max_throughput > 0.0) {
        printf("Maximum throughput: %.2f ops/sec\n", max_throughput);
    }

    /* Cleanup */
    benchmark_cleanup(&bench);
    free_config(config);
    aws_common_library_clean_up();

    return 0;
}

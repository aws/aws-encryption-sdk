#!/usr/bin/env python3
"""
ESDK Performance Benchmark Suite - Python Implementation

This module provides comprehensive performance testing for the AWS Encryption SDK (ESDK)
Python runtime, measuring throughput, latency, memory usage, and scalability.
"""

import sys
import argparse
from benchmark import ESDKBenchmark
from tests import run_all_benchmarks


def main():
    """Main entry point for the benchmark suite"""
    args = _parse_arguments()

    try:
        benchmark = ESDKBenchmark(config_path=args.config)

        if args.quick:
            _adjust_config_for_quick_mode(benchmark)

        results = run_all_benchmarks(benchmark, is_quick_mode=args.quick)

        _save_and_summarize_results(results, args.output)

    except Exception as e:
        print(f"Benchmark failed: {e}")
        sys.exit(1)


def _parse_arguments():
    """Parse command line arguments"""
    parser = argparse.ArgumentParser(description="ESDK Python Performance Benchmark")
    parser.add_argument(
        "--config",
        default="../../config/test-scenarios.yaml",
        help="Path to test configuration file",
    )
    parser.add_argument(
        "--output",
        default="../../results/raw-data/python_results.json",
        help="Path to output results file",
    )
    parser.add_argument(
        "--quick", action="store_true", help="Run quick test with reduced iterations"
    )
    return parser.parse_args()


def _adjust_config_for_quick_mode(benchmark):
    """Adjust benchmark configuration for quick mode"""
    quick_config = benchmark.config.get("quick_config")
    if not quick_config:
        raise RuntimeError(
            "Quick mode requested but no quick_config found in config file"
        )

    benchmark.config["iterations"]["measurement"] = quick_config["iterations"][
        "measurement"
    ]
    benchmark.config["iterations"]["warmup"] = quick_config["iterations"]["warmup"]
    benchmark.config["data_sizes"]["small"] = quick_config["data_sizes"]["small"]
    benchmark.config["data_sizes"]["medium"] = []
    benchmark.config["data_sizes"]["large"] = []
    benchmark.config["concurrency_levels"] = quick_config["concurrency_levels"]


def _save_and_summarize_results(results, output_path):
    """Save results and print summary"""
    from results import save_results

    save_results(results, output_path)

    print("\n=== ESDK Python Benchmark Summary ===")
    print(f"Total tests completed: {len(results)}")
    print(f"Results saved to: {output_path}")

    if results:
        throughput_results = [r for r in results if r.test_name == "throughput"]
        if throughput_results:
            max_throughput = max(r.ops_per_second for r in throughput_results)
            print("Maximum throughput: {:.2f} ops/sec".format(max_throughput))


if __name__ == "__main__":
    main()

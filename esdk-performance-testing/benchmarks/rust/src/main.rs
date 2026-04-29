// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::collapsible_if)]

mod alloc;
mod benchmark;
mod config;
mod results;
mod size_limit_smoke_test;
mod tests;

use anyhow::Result;
use clap::{Arg, Command};

use benchmark::EsdkBenchmark;

fn main() -> Result<()> {
    // Set larger stack size for tokio runtime
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_stack_size(8 * 1024 * 1024) // 8MB stack size
        .enable_all()
        .build()?;

    rt.block_on(async { run_benchmark().await })
}

async fn run_benchmark() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let matches = Command::new("ESDK Rust Benchmark")
        .version("1.0")
        .about("AWS Encryption SDK Performance Benchmark Suite - Rust Implementation")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Path to test configuration file")
                .default_value("../../config/test-scenarios.yaml"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("FILE")
                .help("Path to output results file")
                .default_value("../../results/raw-data/rust_results.json"),
        )
        .arg(
            Arg::new("quick")
                .long("quick")
                .help("Run quick test with reduced iterations")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    let output_path = matches.get_one::<String>("output").unwrap();
    let quick = matches.get_flag("quick");

    // Initialize benchmark
    let mut bench = EsdkBenchmark::new(config_path).await?;

    // Adjust config for quick test
    if quick {
        let quick_config = bench.config.quick_config.as_ref().ok_or_else(|| {
            anyhow::anyhow!("Quick mode requested but no quick_config found in config file")
        })?;

        bench.config.iterations.measurement = quick_config.iterations.measurement;
        bench.config.iterations.warmup = quick_config.iterations.warmup;
        bench.config.data_sizes.small = quick_config.data_sizes.small.clone();
        bench.config.data_sizes.medium = Vec::new();
        bench.config.data_sizes.large = Vec::new();
        bench.config.concurrency_levels = quick_config.concurrency_levels.clone();
    }

    // Run benchmarks
    bench.run_all_benchmarks(quick).await?;

    // Save results
    bench.save_results(output_path)?;

    // Print summary
    println!("\n=== ESDK Rust Benchmark Summary ===");
    println!("Total tests completed: {}", bench.results.len());
    println!("Results saved to: {}", output_path);

    if !bench.results.is_empty() {
        let mut max_throughput = 0.0;
        for result in &bench.results {
            if result.test_name == "throughput" && result.ops_per_second > max_throughput {
                max_throughput = result.ops_per_second;
            }
        }
        if max_throughput > 0.0 {
            println!("Maximum throughput: {:.2} ops/sec", max_throughput);
        }
    }

    Ok(())
}

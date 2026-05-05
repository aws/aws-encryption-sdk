// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::alloc;
use anyhow::Result;
use aws_esdk::client as esdk_client;
use chrono::Utc;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use memory_stats::memory_stats;
use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;

use crate::benchmark::{EsdkBenchmark, MEMORY_TEST_ITERATIONS};
use crate::results::{BenchmarkResult, average, percentile};

impl EsdkBenchmark {
    // === Helper Functions ===

    async fn run_encrypt_decrypt_cycle(&self, data: &[u8]) -> Result<(f64, f64)> {
        // Prepare encryption context
        let encryption_context = HashMap::from([
            ("purpose".to_string(), "performance-test".to_string()),
            ("size".to_string(), data.len().to_string()),
        ]);

        // Convert data to Vec on heap
        let data_vec = Vec::from(data);

        // Encrypt
        let encrypt_start = Instant::now();
        let encrypt_output = self
            .esdk_client
            .encrypt()
            .keyring(self.raw_keyring.clone())
            .plaintext(data_vec)
            .encryption_context(encryption_context)
            .send()
            .await?;
        let encrypt_duration = encrypt_start.elapsed().as_secs_f64() * 1000.0;

        // Decrypt
        let decrypt_start = Instant::now();
        let decrypt_output = self
            .esdk_client
            .decrypt()
            .keyring(self.raw_keyring.clone())
            .ciphertext(encrypt_output.ciphertext.unwrap())
            .send()
            .await?;
        let decrypt_duration = decrypt_start.elapsed().as_secs_f64() * 1000.0;

        // Verify data integrity
        let decrypted_plaintext = decrypt_output.plaintext.unwrap();
        if data != decrypted_plaintext.as_ref() {
            return Err(anyhow::anyhow!("data integrity check failed"));
        }

        Ok((encrypt_duration, decrypt_duration))
    }

    fn should_run_test_type(&self, test_type: &str, is_quick_mode: bool) -> bool {
        if is_quick_mode {
            if let Some(quick_config) = &self.config.quick_config {
                if !quick_config.test_types.is_empty() {
                    return quick_config.test_types.contains(&test_type.to_string());
                }
            }
        }
        true
    }

    // === Throughput Test Implementation ===

    /// Runs throughput test measuring operations per second and data processing rate
    async fn run_throughput_test(
        &mut self,
        data_size: usize,
        iterations: usize,
    ) -> Result<BenchmarkResult> {
        info!(
            "Running throughput test - Size: {} bytes, Iterations: {}",
            data_size, iterations
        );

        // Generate test data on heap to avoid stack overflow
        let data = {
            let mut data = vec![0u8; data_size];
            rand::rng().fill(&mut data[..]);
            data.into_boxed_slice()
        };

        // Warmup
        for i in 0..self.config.iterations.warmup {
            if let Err(e) = self.run_encrypt_decrypt_cycle(&data).await {
                return Err(anyhow::anyhow!("Warmup iteration {} failed: {}", i, e));
            }
        }

        // Measurement runs
        let mut encrypt_latencies = Vec::new();
        let mut decrypt_latencies = Vec::new();
        let mut end_to_end_latencies = Vec::new();
        let mut total_bytes = 0u64;

        // Progress bar
        let pb = ProgressBar::new(iterations as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} {percent}% |{bar:50.cyan/blue}| ({pos}/{len}) [{elapsed}<{eta}]")
                .unwrap()
                .progress_chars("██ "),
        );
        pb.set_message("Throughput test");

        let start_time = Instant::now();
        let alloc = alloc::ResourceTracker::new();
        for i in 0..iterations {
            let iteration_start = Instant::now();
            let (encrypt_ms, decrypt_ms) = self
                .run_encrypt_decrypt_cycle(&data)
                .await
                .map_err(|e| anyhow::anyhow!("Measurement iteration {} failed: {}", i, e))?;
            let iteration_duration = iteration_start.elapsed().as_secs_f64() * 1000.0;

            encrypt_latencies.push(encrypt_ms);
            decrypt_latencies.push(decrypt_ms);
            end_to_end_latencies.push(iteration_duration);
            total_bytes += data_size as u64;

            pb.inc(1);
        }
        let total_duration = start_time.elapsed().as_secs_f64();
        pb.finish_and_clear();

        // Calculate metrics
        end_to_end_latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let result = BenchmarkResult {
            test_name: "throughput".to_string(),
            language: "rust".to_string(),
            data_size,
            concurrency: 1,
            encrypt_latency_ms: average(&encrypt_latencies),
            decrypt_latency_ms: average(&decrypt_latencies),
            end_to_end_latency_ms: average(&end_to_end_latencies),
            ops_per_second: iterations as f64 / total_duration,
            bytes_per_second: total_bytes as f64 / total_duration,
            p50_latency: percentile(&end_to_end_latencies, 50.0),
            p95_latency: percentile(&end_to_end_latencies, 95.0),
            p99_latency: percentile(&end_to_end_latencies, 99.0),
            peak_memory_mb: 0.0,
            memory_efficiency_ratio: 0.0,
            timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            cpu_count: self.cpu_count,
            total_memory_gb: self.total_memory_gb,
            alloc: alloc.get_results(),
        };

        info!(
            "Throughput test completed - Ops/sec: {:.2}, MB/sec: {:.2}",
            result.ops_per_second,
            result.bytes_per_second / (1024.0 * 1024.0)
        );

        Ok(result)
    }

    // === Memory Test Implementation ===

    /// Runs memory usage test by measuring memory delta during ESDK operations
    async fn run_memory_test(&mut self, data_size: usize) -> Result<BenchmarkResult> {
        info!(
            "Running memory test - Size: {} bytes ({} iterations)",
            data_size, MEMORY_TEST_ITERATIONS
        );

        // Generate test data on heap to avoid stack overflow
        let data = {
            let mut data = vec![0u8; data_size];
            rand::rng().fill(&mut data[..]);
            data.into_boxed_slice()
        };

        let mut peak_memory_delta_mb = 0.0;
        let mut avg_memory_samples = Vec::new();
        let alloc = alloc::ResourceTracker::new();

        // Run iterations with memory sampling
        for i in 0..MEMORY_TEST_ITERATIONS {
            // Force garbage collection and get baseline memory
            std::hint::black_box(&data); // Prevent optimization
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

            let baseline_memory = if let Some(stats) = memory_stats() {
                stats.physical_mem as f64 / (1024.0 * 1024.0)
            } else {
                0.0
            };

            let mut iteration_samples = Vec::new();
            let mut iteration_peak = baseline_memory;

            let operation_start = Instant::now();

            // Run ESDK operation with memory sampling
            let data_clone = data.clone();
            let esdk_client = self.esdk_client.clone();
            let keyring = self.raw_keyring.clone();

            let operation_task = tokio::spawn(async move {
                Self::run_encrypt_decrypt_cycle_static(&esdk_client, &keyring, &data_clone).await
            });

            // Sample memory during operation
            let mut sample_count = 0;
            while !operation_task.is_finished() {
                if let Some(stats) = memory_stats() {
                    let current_physical = stats.physical_mem as f64 / (1024.0 * 1024.0);
                    let delta = current_physical - baseline_memory;

                    // Only track positive deltas for average (memory increases)
                    if delta > 0.0 {
                        iteration_samples.push(delta);
                    }

                    // Track peak regardless of sign
                    if current_physical > iteration_peak {
                        iteration_peak = current_physical;
                    }
                    sample_count += 1;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }

            // Wait for operation to complete
            let _ = operation_task.await.unwrap().unwrap();
            let operation_duration = operation_start.elapsed();

            // Get final memory delta
            let final_stats = memory_stats().unwrap_or(memory_stats::MemoryStats {
                physical_mem: 0,
                virtual_mem: 0,
            });
            let final_physical = final_stats.physical_mem as f64 / (1024.0 * 1024.0);
            let final_delta = final_physical - baseline_memory;

            // Calculate metrics for this iteration (memory delta from baseline)
            let iter_peak_delta_mb = (iteration_peak - baseline_memory).max(final_delta);

            let iter_avg_delta_mb = if !iteration_samples.is_empty() {
                iteration_samples.iter().sum::<f64>() / iteration_samples.len() as f64
            } else {
                // If no positive deltas, use final delta if positive, otherwise 0
                final_delta.max(0.0)
            };

            // Update global maximum
            if iter_peak_delta_mb > peak_memory_delta_mb {
                peak_memory_delta_mb = iter_peak_delta_mb;
            }

            avg_memory_samples.push(iter_avg_delta_mb);

            info!(
                "=== Iteration {} === Peak Delta: {:.2} MB, Avg Delta: {:.2} MB ({:?}, {} samples)",
                i + 1,
                iter_peak_delta_mb,
                iter_avg_delta_mb,
                operation_duration,
                sample_count
            );
        }

        // Calculate overall averages
        let overall_avg_delta_mb = if !avg_memory_samples.is_empty() {
            avg_memory_samples.iter().sum::<f64>() / avg_memory_samples.len() as f64
        } else {
            0.0
        };

        // Calculate memory efficiency
        let memory_efficiency = if peak_memory_delta_mb > 0.0 {
            data_size as f64 / (peak_memory_delta_mb * 1024.0 * 1024.0)
        } else {
            0.0
        };

        info!("\nMemory Summary:");
        info!(
            "- Peak Memory Delta: {:.2} MB (operation overhead)",
            peak_memory_delta_mb
        );
        info!(
            "- Average Memory Delta: {:.2} MB (operation overhead)",
            overall_avg_delta_mb
        );

        let result = BenchmarkResult {
            test_name: "memory".to_string(),
            language: "rust".to_string(),
            data_size,
            concurrency: 1,
            encrypt_latency_ms: 0.0,
            decrypt_latency_ms: 0.0,
            end_to_end_latency_ms: 0.0,
            ops_per_second: 0.0,
            bytes_per_second: 0.0,
            peak_memory_mb: peak_memory_delta_mb,
            memory_efficiency_ratio: memory_efficiency,
            p50_latency: 0.0,
            p95_latency: 0.0,
            p99_latency: 0.0,
            timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            cpu_count: self.cpu_count,
            total_memory_gb: self.total_memory_gb,
            alloc: alloc.get_results(),
        };

        Ok(result)
    }

    // Static helper for memory test
    async fn run_encrypt_decrypt_cycle_static(
        esdk_client: &esdk_client::Client,
        keyring: &aws_esdk::material_providers::types::keyring::KeyringRef,
        data: &[u8],
    ) -> Result<(f64, f64)> {
        // Prepare encryption context
        let encryption_context = HashMap::from([
            ("purpose".to_string(), "performance-test".to_string()),
            ("size".to_string(), data.len().to_string()),
        ]);

        // Convert data to Vec on heap
        let data_vec = Vec::from(data);

        // Encrypt
        let encrypt_start = Instant::now();
        let encrypt_output = esdk_client
            .encrypt()
            .keyring(keyring.clone())
            .plaintext(data_vec)
            .encryption_context(encryption_context)
            .send()
            .await?;
        let encrypt_duration = encrypt_start.elapsed().as_secs_f64() * 1000.0;

        // Decrypt
        let decrypt_start = Instant::now();
        let decrypt_output = esdk_client
            .decrypt()
            .keyring(keyring.clone())
            .ciphertext(encrypt_output.ciphertext.unwrap())
            .send()
            .await?;
        let decrypt_duration = decrypt_start.elapsed().as_secs_f64() * 1000.0;

        // Verify data integrity
        let decrypted_plaintext = decrypt_output.plaintext.unwrap();
        if data != decrypted_plaintext.as_ref() {
            return Err(anyhow::anyhow!("data integrity check failed"));
        }

        Ok((encrypt_duration, decrypt_duration))
    }

    // === Concurrent Test Implementation ===

    /// Runs concurrent test with multiple workers to measure parallel performance
    async fn run_concurrent_test(
        &mut self,
        data_size: usize,
        concurrency: usize,
        iterations_per_worker: usize,
    ) -> Result<BenchmarkResult> {
        info!(
            "Running concurrent test - Size: {} bytes, Concurrency: {}",
            data_size, concurrency
        );

        // Create tasks for concurrent execution
        let mut tasks = Vec::new();
        for worker_id in 0..concurrency {
            let esdk_client = self.esdk_client.clone();
            let keyring = self.raw_keyring.clone();

            let task = tokio::spawn(async move {
                let mut worker_times = Vec::new();
                for j in 0..iterations_per_worker {
                    let iter_start = Instant::now();

                    // Generate data per worker on heap to avoid stack overflow
                    let worker_data = {
                        let mut data = vec![0u8; data_size];
                        rand::rng().fill(&mut data[..]);
                        data
                    };

                    // Run encrypt-decrypt cycle
                    let encryption_context = HashMap::from([
                        ("purpose".to_string(), "performance-test".to_string()),
                        ("size".to_string(), worker_data.len().to_string()),
                    ]);

                    let encrypt_output = esdk_client
                        .encrypt()
                        .keyring(keyring.clone())
                        .plaintext(worker_data.clone())
                        .encryption_context(encryption_context)
                        .send()
                        .await
                        .map_err(|e| {
                            anyhow::anyhow!(
                                "Worker {} iteration {} encrypt failed: {}",
                                worker_id,
                                j,
                                e
                            )
                        })?;

                    let _decrypt_output = esdk_client
                        .decrypt()
                        .keyring(keyring.clone())
                        .ciphertext(encrypt_output.ciphertext.unwrap())
                        .send()
                        .await
                        .map_err(|e| {
                            anyhow::anyhow!(
                                "Worker {} iteration {} decrypt failed: {}",
                                worker_id,
                                j,
                                e
                            )
                        })?;

                    // Verify data integrity
                    let decrypted_plaintext = _decrypt_output.plaintext.unwrap();
                    if worker_data != decrypted_plaintext.as_ref() {
                        return Err(anyhow::anyhow!(
                            "Worker {} iteration {} data integrity check failed",
                            worker_id,
                            j
                        ));
                    }

                    worker_times.push(iter_start.elapsed().as_secs_f64() * 1000.0);
                }
                Ok::<Vec<f64>, anyhow::Error>(worker_times)
            });
            tasks.push(task);
        }

        let start_time = Instant::now();
        let alloc = alloc::ResourceTracker::new();
        let results = join_all(tasks).await;
        let total_duration = start_time.elapsed().as_secs_f64();

        // Collect all times
        let mut all_times = Vec::new();
        for result in results {
            let worker_times = result??;
            all_times.extend(worker_times);
        }

        // Calculate metrics
        let total_ops = concurrency * iterations_per_worker;
        let total_bytes = total_ops * data_size;

        all_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let result = BenchmarkResult {
            test_name: "concurrent".to_string(),
            language: "rust".to_string(),
            data_size,
            concurrency,
            encrypt_latency_ms: 0.0,
            decrypt_latency_ms: 0.0,
            end_to_end_latency_ms: average(&all_times),
            ops_per_second: total_ops as f64 / total_duration,
            bytes_per_second: total_bytes as f64 / total_duration,
            p50_latency: percentile(&all_times, 50.0),
            p95_latency: percentile(&all_times, 95.0),
            p99_latency: percentile(&all_times, 99.0),
            peak_memory_mb: 0.0,
            memory_efficiency_ratio: 0.0,
            timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            cpu_count: self.cpu_count,
            total_memory_gb: self.total_memory_gb,
            alloc: alloc.get_results(),
        };

        info!(
            "Concurrent test completed - Ops/sec: {:.2}, Avg latency: {:.2} ms",
            result.ops_per_second, result.end_to_end_latency_ms
        );

        Ok(result)
    }

    // === Test Orchestration ===

    async fn run_throughput_tests(&mut self, data_sizes: &[usize], iterations: usize) {
        info!("Running throughput tests...");
        for &data_size in data_sizes {
            match self.run_throughput_test(data_size, iterations).await {
                Ok(result) => {
                    info!(
                        "Throughput test completed: {:.2} ops/sec",
                        result.ops_per_second
                    );
                    self.results.push(result);
                }
                Err(e) => {
                    warn!("Throughput test failed: {}", e);
                }
            }
        }
    }

    async fn run_memory_tests(&mut self, data_sizes: &[usize]) {
        info!("Running memory tests...");
        for &data_size in data_sizes {
            match self.run_memory_test(data_size).await {
                Ok(result) => {
                    info!(
                        "Memory test completed: {:.2} MB peak",
                        result.peak_memory_mb
                    );
                    self.results.push(result);
                }
                Err(e) => {
                    warn!("Memory test failed: {}", e);
                }
            }
        }
    }

    async fn run_concurrency_tests(&mut self, data_sizes: &[usize], concurrency_levels: &[usize]) {
        info!("Running concurrency tests...");
        for &data_size in data_sizes {
            for &concurrency in concurrency_levels {
                if concurrency > 1 {
                    // Skip single-threaded
                    match self.run_concurrent_test(data_size, concurrency, 5).await {
                        Ok(result) => {
                            info!(
                                "Concurrent test completed: {:.2} ops/sec @ {} threads",
                                result.ops_per_second, concurrency
                            );
                            self.results.push(result);
                        }
                        Err(e) => {
                            warn!("Concurrent test failed: {}", e);
                        }
                    }
                }
            }
        }
    }

    pub async fn run_all_benchmarks(&mut self, is_quick_mode: bool) -> Result<()> {
        info!("Starting comprehensive ESDK benchmark suite");

        // Combine all data sizes
        let mut data_sizes = Vec::new();
        data_sizes.extend(&self.config.data_sizes.small);
        data_sizes.extend(&self.config.data_sizes.medium);
        data_sizes.extend(&self.config.data_sizes.large);

        let concurrency_levels = self.config.concurrency_levels.clone();

        // Run test suites
        if self.should_run_test_type("throughput", is_quick_mode) {
            self.run_throughput_tests(&data_sizes, self.config.iterations.measurement)
                .await;
        } else {
            info!("Skipping throughput tests (not in test_types)");
        }

        if self.should_run_test_type("memory", is_quick_mode) {
            self.run_memory_tests(&data_sizes).await;
        } else {
            info!("Skipping memory tests (not in test_types)");
        }

        if self.should_run_test_type("concurrency", is_quick_mode) {
            self.run_concurrency_tests(&data_sizes, &concurrency_levels)
                .await;
        } else {
            info!("Skipping concurrency tests (not in test_types)");
        }

        if self.should_run_test_type("size_limits", is_quick_mode) {
            self.run_size_limit_smoke_test().await?;
        } else {
            info!("Skipping size limit smoke tests (not in test_types)");
        }

        info!(
            "Benchmark suite completed. Total results: {}",
            self.results.len()
        );
        Ok(())
    }
}

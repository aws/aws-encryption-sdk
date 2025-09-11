// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::alloc;
use anyhow::Result;
use chrono::Utc;
use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Clone)]
pub struct BenchmarkResult {
    pub test_name: String,
    pub language: String,
    pub data_size: usize,
    pub concurrency: usize,
    pub encrypt_latency_ms: f64,
    pub decrypt_latency_ms: f64,
    pub end_to_end_latency_ms: f64,
    pub ops_per_second: f64,
    pub bytes_per_second: f64,
    pub peak_memory_mb: f64,
    pub memory_efficiency_ratio: f64,
    pub p50_latency: f64,
    pub p95_latency: f64,
    pub p99_latency: f64,
    pub timestamp: String,
    pub rust_version: String,
    pub cpu_count: usize,
    pub total_memory_gb: f64,
    pub alloc: alloc::ResourceResults,
}

#[derive(Debug, Serialize)]
struct BenchmarkResults {
    metadata: BenchmarkMetadata,
    results: Vec<BenchmarkResult>,
}

#[derive(Debug, Serialize)]
struct BenchmarkMetadata {
    language: String,
    timestamp: String,
    rust_version: String,
    cpu_count: usize,
    total_memory_gb: f64,
    total_tests: usize,
}

// === Utility Functions ===

pub fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn percentile(sorted_values: &[f64], p: f64) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    if p <= 0.0 {
        return sorted_values[0];
    }
    if p >= 100.0 {
        return sorted_values[sorted_values.len() - 1];
    }

    let index = (p / 100.0) * (sorted_values.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        return sorted_values[lower];
    }

    let weight = index - lower as f64;
    sorted_values[lower] * (1.0 - weight) + sorted_values[upper] * weight
}

// === Results Saving ===

pub fn save_results(
    results: &[BenchmarkResult],
    output_path: &str,
    cpu_count: usize,
    total_memory_gb: f64,
) -> Result<()> {
    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let results_data = BenchmarkResults {
        metadata: BenchmarkMetadata {
            language: "rust".to_string(),
            timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            cpu_count,
            total_memory_gb,
            total_tests: results.len(),
        },
        results: results.to_vec(),
    };

    let json_data = serde_json::to_string_pretty(&results_data)?;
    let mut file = File::create(output_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

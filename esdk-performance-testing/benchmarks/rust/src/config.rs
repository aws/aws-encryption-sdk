// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub data_sizes: DataSizes,
    pub iterations: IterationConfig,
    pub concurrency_levels: Vec<usize>,
    pub quick_config: Option<QuickConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSizes {
    pub small: Vec<usize>,
    pub medium: Vec<usize>,
    pub large: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationConfig {
    pub warmup: usize,
    pub measurement: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickConfig {
    pub data_sizes: QuickDataSizes,
    pub iterations: QuickIterationConfig,
    pub concurrency_levels: Vec<usize>,
    pub test_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickDataSizes {
    pub small: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickIterationConfig {
    pub warmup: usize,
    pub measurement: usize,
}

pub fn load_config(config_path: &str) -> Result<TestConfig> {
    if !std::path::Path::new(config_path).exists() {
        return Err(anyhow::anyhow!("Config file not found: {}", config_path));
    }

    let config_content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path))?;

    let config: TestConfig =
        serde_yaml::from_str(&config_content).with_context(|| "Failed to parse config file")?;

    Ok(config)
}

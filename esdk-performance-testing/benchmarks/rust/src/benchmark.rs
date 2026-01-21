// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use aws_esdk::client as esdk_client;
use aws_esdk::material_providers::client as mpl_client;
use aws_esdk::material_providers::types::AesWrappingAlg;
use aws_esdk::material_providers::types::EsdkCommitmentPolicy;
use aws_esdk::material_providers::types::material_providers_config::MaterialProvidersConfig;
use aws_esdk::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig;
use log::info;
use rand::Rng;
use sysinfo::System;

use crate::config::{TestConfig, load_config};
use crate::results::BenchmarkResult;

// Constants for memory testing
pub const MEMORY_TEST_ITERATIONS: usize = 5;

pub struct EsdkBenchmark {
    pub esdk_client: esdk_client::Client,
    pub raw_keyring: aws_esdk::material_providers::types::keyring::KeyringRef,
    pub raw_keyring_new: aws_mpl_legacy::types::keyring::KeyringRef,
    pub config: TestConfig,
    pub results: Vec<BenchmarkResult>,
    pub cpu_count: usize,
    pub total_memory_gb: f64,
}

impl EsdkBenchmark {
    pub async fn new(config_path: &str) -> Result<Self> {
        // Get system info
        let mut system = System::new_all();
        system.refresh_all();
        let cpu_count = system.cpus().len();
        let total_memory_gb = system.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

        // Load configuration
        let config = load_config(config_path)?;

        // Setup ESDK
        let (esdk_client, raw_keyring) = Self::setup_esdk().await?;
        let raw_keyring_new = Self::setup_new_keyring().await?;

        info!(
            "Initialized ESDK Benchmark - CPU cores: {}, Memory: {:.1}GB",
            cpu_count, total_memory_gb
        );

        Ok(Self {
            esdk_client,
            raw_keyring,
            raw_keyring_new,
            config,
            results: Vec::new(),
            cpu_count,
            total_memory_gb,
        })
    }

    async fn setup_esdk() -> Result<(
        esdk_client::Client,
        aws_esdk::material_providers::types::keyring::KeyringRef,
    )> {
        // Initialize ESDK client with commitment policy
        let esdk_config = AwsEncryptionSdkConfig::builder()
            .commitment_policy(EsdkCommitmentPolicy::RequireEncryptRequireDecrypt)
            .build()?;
        let esdk_client = esdk_client::Client::from_conf(esdk_config)?;

        // Initialize material providers client
        let mpl_config = MaterialProvidersConfig::builder().build()?;
        let mpl_client = mpl_client::Client::from_conf(mpl_config)?;

        // Create default AES-256 keyring
        let mut key = [0u8; 32]; // 256-bit key
        rand::rng().fill(&mut key);

        let raw_keyring = mpl_client
            .create_raw_aes_keyring()
            .key_name("test-aes-256-key")
            .key_namespace("esdk-performance-test")
            .wrapping_key(key.to_vec())
            .wrapping_alg(AesWrappingAlg::AlgAes256GcmIv12Tag16)
            .send()
            .await
            .context("Failed to create keyring")?;

        info!("ESDK client initialized successfully");
        Ok((esdk_client, raw_keyring))
    }

    async fn setup_new_keyring() -> Result<aws_mpl_legacy::types::keyring::KeyringRef> {
        // Initialize material providers client
        let mpl_client = aws_esdk_rs::mpl();

        // Create default AES-256 keyring
        let mut key = [0u8; 32]; // 256-bit key
        rand::rng().fill(&mut key);

        let raw_keyring = mpl_client
            .create_raw_aes_keyring()
            .key_name("test-aes-256-key")
            .key_namespace("esdk-performance-test")
            .wrapping_key(key.to_vec())
            .wrapping_alg(aws_mpl_legacy::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
            .send()
            .await
            .context("Failed to create keyring")?;

        info!("raw_keyring initialized successfully");
        Ok(raw_keyring)
    }

    pub fn save_results(&self, output_path: &str) -> Result<()> {
        crate::results::save_results(
            &self.results,
            output_path,
            self.cpu_count,
            self.total_memory_gb,
        )
    }
}

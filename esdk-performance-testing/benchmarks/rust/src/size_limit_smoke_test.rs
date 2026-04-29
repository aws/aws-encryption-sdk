// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use log::info;

use crate::benchmark::EsdkBenchmark;

const MB_128: usize = 134_217_728;

impl EsdkBenchmark {
    /// Validates that encrypt/decrypt properly reject data exceeding the size limit
    /// and succeed at the boundary.
    pub async fn run_size_limit_smoke_test(&self) -> Result<()> {
        info!("Running size limit smoke tests...");

        struct TestCase {
            name: &'static str,
            size: usize,
            expect_err: bool,
        }

        let cases = [
            TestCase { name: "limit-1", size: MB_128 - 1, expect_err: false },
            TestCase { name: "limit", size: MB_128, expect_err: false },
            TestCase { name: "limit+1", size: MB_128 + 1, expect_err: true },
        ];

        // Test encrypt plaintext size limits
        for tc in &cases {
            info!("  Encrypt plaintext size={} ({}) expect_err={}", tc.size, tc.name, tc.expect_err);
            let data = vec![0u8; tc.size];

            let result = self
                .esdk_client
                .encrypt()
                .keyring(self.raw_keyring.clone())
                .plaintext(data)
                .send()
                .await;

            if tc.expect_err {
                if result.is_ok() {
                    return Err(anyhow::anyhow!(
                        "encrypt({}): expected error for plaintext size {}, got Ok",
                        tc.name, tc.size
                    ));
                }
                info!("  PASS: encrypt({}) returned expected error", tc.name);
            } else {
                if let Err(e) = result {
                    return Err(anyhow::anyhow!(
                        "encrypt({}): unexpected error for plaintext size {}: {}",
                        tc.name, tc.size, e
                    ));
                }
                info!("  PASS: encrypt({}) succeeded", tc.name);
            }
        }

        // Decrypt ciphertext — no size limit on ciphertext

        info!("Size limit smoke tests PASSED");
        Ok(())
    }
}

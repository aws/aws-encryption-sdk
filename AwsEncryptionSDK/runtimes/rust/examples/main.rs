// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

pub mod client_supplier;
pub mod cryptographic_materials_manager;
pub mod keyring;
pub mod example_utils;
pub mod set_encryption_algorithm_suite_example;
pub mod set_commitment_policy_example;
pub mod limit_encrypted_data_keys_example;
use std::convert::From;

// Why two types?
// return type from main must impl Debug
// but if impl Debug for BoxError
// then I can't impl<T : std::fmt::Debug> From<T> for BoxError
// because there's already a impl<T> From<T> for T;

pub struct BoxError(String);
pub struct BoxError2(String);

impl std::fmt::Debug for BoxError2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BoxError> for BoxError2 {
    fn from(error: BoxError) -> Self {
        BoxError2(error.0)
    }
}

impl<T: std::fmt::Debug> From<T> for BoxError {
    fn from(error: T) -> Self {
        let my_str = format!("{:?}", error);
        BoxError(my_str)
    }
}

#[tokio::main]
pub async fn main() -> Result<(), BoxError2> {
    use example_utils::utils;

    // Example use of KMS Keyring
    keyring::aws_kms_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID
    ).await?;

    Ok(())
}
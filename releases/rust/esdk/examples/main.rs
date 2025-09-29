// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

pub mod client_supplier;
pub mod cryptographic_materials_manager;
pub mod example_utils;
pub mod keyring;
pub mod limit_encrypted_data_keys_example;
pub mod set_commitment_policy_example;
pub mod set_encryption_algorithm_suite_example;
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
        let my_str = format!("{error:?}");
        BoxError(my_str)
    }
}

#[tokio::main]
pub async fn main() -> Result<(), BoxError2> {
    use example_utils::utils;

    keyring::aws_kms_discovery_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
    )
    .await?;

    let aws_regions: Vec<String> = vec!["us-east-1".to_string(), "us-west-2".to_string()];

    keyring::aws_kms_discovery_multi_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
        aws_regions,
    )
    .await?;

    keyring::aws_kms_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
    )
    .await?;

    let mrk_encrypt_region: String = "us-east-1".to_string();
    let mrk_replica_decrypt_region: String = "eu-west-1".to_string();

    keyring::aws_kms_mrk_discovery_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
        mrk_encrypt_region,
        mrk_replica_decrypt_region,
    )
    .await?;

    let mrk_encrypt_region: String = "us-east-1".to_string();
    let aws_regions: Vec<String> = vec!["us-east-1".to_string(), "us-west-2".to_string()];

    keyring::aws_kms_mrk_discovery_multi_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        mrk_encrypt_region,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
        aws_regions,
    )
    .await?;

    let mrk_encrypt_region: String = "us-east-1".to_string();
    let mrk_replica_decrypt_region: String = "eu-west-1".to_string();

    keyring::aws_kms_mrk_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        utils::TEST_MRK_KEY_ID_EU_WEST_1,
        mrk_encrypt_region,
        mrk_replica_decrypt_region,
    )
    .await?;

    let mrk_replica_decrypt_region: String = "eu-west-1".to_string();

    keyring::aws_kms_mrk_multi_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        utils::TEST_DEFAULT_KMS_KEY_ID,
        utils::TEST_MRK_KEY_ID_EU_WEST_1,
        mrk_replica_decrypt_region,
    )
    .await?;

    let default_region: String = "us-west-2".to_string();
    let second_region: String = "eu-central-1".to_string();

    keyring::aws_kms_multi_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
        utils::TEST_SECOND_REGION_KMS_KEY_ID,
        default_region,
        second_region,
    )
    .await?;

    keyring::aws_kms_rsa_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_KMS_RSA_KEY_ID,
        utils::TEST_KMS_RSA_PUBLIC_KEY,
    )
    .await?;

    keyring::multi_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
    )
    .await?;

    keyring::raw_aes_keyring_example::encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA)
        .await?;

    keyring::raw_rsa_keyring_example::encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA)
        .await?;

    use aws_esdk::aws_cryptography_primitives::types::EcdhCurveSpec;

    // If you provide the key for Raw ECDH Keyring, it MUST
    // be a key on curve P256 to run the example as is. If you want to provide a key on a
    // different curve, update the input to encrypt_and_decrypt_with_keyring
    keyring::ecdh::raw_ecdh_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        EcdhCurveSpec::EccNistP256,
    )
    .await?;

    // If you provide the key for Ephemeral Raw ECDH Keyring, it MUST
    // be a key on curve P256 to run the example as is. If you want to provide a key on a
    // different curve, update the input to encrypt_with_keyring
    keyring::ecdh::ephemeral_raw_ecdh_keyring_example::encrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        EcdhCurveSpec::EccNistP256,
    )
    .await?;

    // If you provide the key for Public Key Discovery Raw ECDH Keyring, it MUST
    // be a key on curve P256 to run the example as is. If you want to provide a key on a
    // different curve, update the input to decrypt_with_keyring
    keyring::ecdh::public_key_discovery_raw_ecdh_keyring_example::decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        EcdhCurveSpec::EccNistP256,
    )
    .await?;

    // If you provide the public keys for KMS ECDH Keyring, it MUST
    // be a key on curve P256 to run the example as is. If you want to provide a key on a
    // different curve, update the input to encrypt_and_decrypt_with_keyring
    keyring::ecdh::kms_ecdh_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_KMS_ECDH_KEY_ID_P256_SENDER,
        EcdhCurveSpec::EccNistP256,
        Some(utils::TEST_KMS_ECDH_KEY_ID_P256_RECIPIENT),
    )
    .await?;

    keyring::ecdh::kms_ecdh_discovery_keyring_example::decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        EcdhCurveSpec::EccNistP256,
        utils::TEST_KMS_ECDH_KEY_ID_P256_RECIPIENT,
    )
    .await?;

    keyring::aws_kms_hierarchical::aws_kms_hierarchical_keyring_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_KEY_STORE_NAME,
        utils::TEST_LOGICAL_KEY_STORE_NAME,
        utils::TEST_KEY_STORE_KMS_KEY_ID
    ).await?;

    keyring::aws_kms_hierarchical::version_branch_key_id_example::create_and_version_branch_key_id(
    )
    .await?;

    keyring::aws_kms_hierarchical::shared_cache_across_hierarchical_keyrings_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_KEY_STORE_NAME,
        utils::TEST_LOGICAL_KEY_STORE_NAME,
        utils::TEST_KEY_STORE_KMS_KEY_ID
    ).await?;

    cryptographic_materials_manager::required_encryption_context::required_encryption_context_example::encrypt_and_decrypt_with_cmm(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID
    ).await?;

    cryptographic_materials_manager::restrict_algorithm_suite::signing_only_example::encrypt_and_decrypt_with_cmm(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID
    ).await?;

    let aws_regions: Vec<String> = vec!["eu-west-1".to_string()];

    client_supplier::client_supplier_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
        aws_regions,
    )
    .await?;

    set_encryption_algorithm_suite_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
    )
    .await?;

    set_commitment_policy_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
    )
    .await?;

    // max_encrypted_data_keys MUST be greater than 0
    let max_encrypted_data_keys: u16 = 3;

    limit_encrypted_data_keys_example::encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        max_encrypted_data_keys,
    )
    .await?;

    println!("All examples completed successfully.");

    Ok(())
}

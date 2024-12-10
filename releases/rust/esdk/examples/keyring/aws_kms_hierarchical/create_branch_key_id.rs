// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::aws_cryptography_keyStore::client as keystore_client;
use aws_esdk::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig;
use aws_esdk::aws_cryptography_keyStore::types::KmsConfiguration;

/*
 The Hierarchical Keyring Example relies on the existence
 of a DDB-backed key store with pre-existing
 branch key material.

 This example demonstrates configuring a KeyStore and then
 using a helper method to create a branch key.
*/
pub async fn create_branch_key_id(
    key_store_table_name: &str,
    logical_key_store_name: &str,
    kms_key_arn: &str
) -> Result<String, crate::BoxError> {
    // Create a Key Store
    // The KMS Configuration you use in the KeyStore MUST have the right access to the resources in the KeyStore.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let key_store_config = KeyStoreConfig::builder()
        .kms_client(aws_sdk_kms::Client::new(&sdk_config))
        .ddb_client(aws_sdk_dynamodb::Client::new(&sdk_config))
        .ddb_table_name(key_store_table_name)
        .logical_key_store_name(logical_key_store_name)
        .kms_configuration(KmsConfiguration::KmsKeyArn(kms_key_arn.to_string()))
        .build()?;

    let keystore = keystore_client::Client::from_conf(key_store_config)?;

    // Create a branch key identifier with the AWS KMS Key configured in the KeyStore Configuration.
    let new_key = keystore.create_key().send().await?;
    Ok(new_key.branch_key_identifier.unwrap())
}

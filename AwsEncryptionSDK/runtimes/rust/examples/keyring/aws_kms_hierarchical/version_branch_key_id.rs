// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::aws_cryptography_keyStore::client as keystore_client;
use aws_esdk::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig;
use aws_esdk::aws_cryptography_keyStore::types::KmsConfiguration;

/*
 This example demonstrates configuring a KeyStore and then
 uses a helper method to version a branch key.
*/
pub async fn version_branch_key_id(
    key_store_table_name: &str,
    logical_key_store_name: &str,
    kms_key_arn: &str,
    branch_key_id: &str
) -> Result<(), crate::BoxError> {
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

    // To version a branch key you MUST have access to kms:ReEncrypt* and kms:GenerateDataKeyWithoutPlaintext
    keystore.version_key()
        .branch_key_identifier(branch_key_id)
        .send()
        .await?;
    Ok(())
}

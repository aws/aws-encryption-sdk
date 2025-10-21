// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 This example sets up the Hierarchical Keyring, which establishes a key hierarchy where "branch"
 keys are persisted in DynamoDb. These branch keys are used to protect your data keys, and these
 branch keys are themselves protected by a KMS Key.

 Establishing a key hierarchy like this has two benefits:
 First, by caching the branch key material, and only calling KMS to re-establish authentication
 regularly according to your configured TTL, you limit how often you need to call KMS to protect
 your data. This is a performance security tradeoff, where your authentication, audit, and logging
 from KMS is no longer one-to-one with every encrypt or decrypt call. Additionally, KMS Cloudtrail
 cannot be used to distinguish Encrypt and Decrypt calls, and you cannot restrict who has
 Encryption rights from who has Decryption rights since they both ONLY need KMS:Decrypt. However,
 the benefit is that you no longer have to make a network call to KMS for every encrypt or
 decrypt.

 Second, this key hierarchy facilitates cryptographic isolation of a tenant's data in a
 multi-tenant data store. Each tenant can have a unique Branch Key, that is only used to protect
 the tenant's data. You can either statically configure a single branch key to ensure you are
 restricting access to a single tenant, or you can implement an interface that selects the Branch
 Key based on the Encryption Context.

 This example demonstrates configuring a Hierarchical Keyring with a Branch Key ID Supplier to
 encrypt and decrypt data for two separate tenants.

 This example requires access to the DDB Table where you are storing the Branch Keys. This
 table must be configured with the following primary key configuration: - Partition key is named
 "partition_key" with type (S) - Sort key is named "sort_key" with type (S).

 This example also requires using a KMS Key. You need the following access on this key:
 - GenerateDataKeyWithoutPlaintext
 - Decrypt

 For more information on how to use Hierarchical Keyrings, see
 https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-hierarchical-keyring.html
*/

use super::create_branch_key_id::create_branch_key_id;
use super::example_branch_key_id_supplier::ExampleBranchKeyIdSupplier;
use aws_esdk::Client as EsdkClient;
use aws_esdk::*;
use aws_mpl_rs::aws_cryptography_keyStore::client as keystore_client;
use aws_mpl_rs::aws_cryptography_keyStore::types::KmsConfiguration;
use aws_mpl_rs::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig;
use aws_mpl_rs::client as mpl_client;
use aws_mpl_rs::types::material_providers_config::MaterialProvidersConfig;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    key_store_table_name: &str,
    logical_key_store_name: &str,
    key_store_kms_key_id: &str,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::default();
    let esdk_client = EsdkClient::from_conf(esdk_config)?;

    // 2. Create a KMS client and DynamoDB client.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);
    let ddb_client = aws_sdk_dynamodb::Client::new(&sdk_config);

    // 3. Configure your KeyStore resource.
    //    This SHOULD be the same configuration that you used
    //    to initially create and populate your KeyStore.
    let key_store_config = KeyStoreConfig::builder()
        .kms_client(kms_client)
        .ddb_client(ddb_client)
        .ddb_table_name(key_store_table_name)
        .logical_key_store_name(logical_key_store_name)
        .kms_configuration(KmsConfiguration::KmsKeyArn(
            key_store_kms_key_id.to_string(),
        ))
        .build()?;

    let key_store = keystore_client::Client::from_conf(key_store_config)?;

    // 4. Call CreateKey to create two new active branch keys
    let branch_key_id_a: String = create_branch_key_id(
        key_store_table_name,
        logical_key_store_name,
        key_store_kms_key_id,
    )
    .await?;
    let branch_key_id_b: String = create_branch_key_id(
        key_store_table_name,
        logical_key_store_name,
        key_store_kms_key_id,
    )
    .await?;

    // 5. Create a branch key supplier that maps the branch key id to a more readable format
    let branch_key_id_supplier =
        ExampleBranchKeyIdSupplier::new(&branch_key_id_a, &branch_key_id_b);

    // 6. Create the Hierarchical Keyring.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    let hierarchical_keyring = mpl
        .create_aws_kms_hierarchical_keyring()
        .key_store(key_store.clone())
        .branch_key_id_supplier(branch_key_id_supplier)
        .ttl_seconds(600)
        .send()
        .await?;

    // 7. Create encryption context for both tenants.
    //    The Branch Key Id supplier uses the encryption context to determine which branch key id will
    //    be used to encrypt data.
    // Remember that your encryption context is NOT SECRET.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context

    // Create encryption context for TenantA
    let encryption_context_a = EncryptionContext::from([
        ("tenant".to_string(), "TenantA".to_string()),
        ("encryption".to_string(), "context".to_string()),
        ("is not".to_string(), "secret".to_string()),
        ("but adds".to_string(), "useful metadata".to_string()),
        (
            "that can help you".to_string(),
            "be confident that".to_string(),
        ),
        (
            "the data you are handling".to_string(),
            "is what you think it is".to_string(),
        ),
    ]);

    // Create encryption context for TenantB
    let encryption_context_b = EncryptionContext::from([
        ("tenant".to_string(), "TenantB".to_string()),
        ("encryption".to_string(), "context".to_string()),
        ("is not".to_string(), "secret".to_string()),
        ("but adds".to_string(), "useful metadata".to_string()),
        (
            "that can help you".to_string(),
            "be confident that".to_string(),
        ),
        (
            "the data you are handling".to_string(),
            "is what you think it is".to_string(),
        ),
    ]);

    // 8. Encrypt the data with encryption_contextA & encryption_contextB
    let plaintext = example_data.as_bytes();

    let mut encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(hierarchical_keyring.clone())
        .encryption_context(&encryption_context_a)
        .build()?;
    let encryption_response_a = esdk_client.encrypt(&encrypt_input).await?;

    let ciphertext_a = encryption_response_a.ciphertext;

    encrypt_input.encryption_context = Some(&encryption_context_b);
    let encryption_response_b = esdk_client.encrypt(&encrypt_input).await?;

    let ciphertext_b = encryption_response_b.ciphertext;

    // 9. Demonstrate that the ciphertexts and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext_a, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    assert_ne!(
        ciphertext_b, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 10. To attest that TenantKeyB cannot decrypt a message written by TenantKeyA,
    //    and vice versa, let's construct more restrictive hierarchical keyrings.
    let hierarchical_keyring_a = mpl
        .create_aws_kms_hierarchical_keyring()
        .key_store(key_store.clone())
        .branch_key_id(branch_key_id_a)
        .ttl_seconds(600)
        .send()
        .await?;

    let hierarchical_keyring_b = mpl
        .create_aws_kms_hierarchical_keyring()
        .key_store(key_store)
        .branch_key_id(branch_key_id_b)
        .ttl_seconds(600)
        .send()
        .await?;

    // 11. Demonstrate that data encrypted by one tenant's key
    // cannot be decrypted with by a keyring specific to another tenant.

    // Keyring with tenant B's branch key cannot decrypt data encrypted with tenant A's branch key
    // This will fail and raise a AwsCryptographicMaterialProvidersError,
    // which we swallow ONLY for demonstration purposes.
    let mut decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext_a)
        .keyring(hierarchical_keyring_b.clone())
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(&encryption_context_a)
        .build()?;
    let decryption_response_mismatch_1 = esdk_client.decrypt(&decrypt_input).await;

    if decryption_response_mismatch_1.is_ok() {
        panic!(
            "Decrypt with wrong tenant's hierarchical keyring MUST raise AwsCryptographicMaterialProvidersError"
        );
    }

    // Keyring with tenant A's branch key cannot decrypt data encrypted with tenant B's branch key.
    // This will fail and raise a AwsCryptographicMaterialProvidersError,
    // which we swallow ONLY for demonstration purposes.
    decrypt_input.ciphertext = &ciphertext_b;
    decrypt_input.keyring = Some(hierarchical_keyring_a.clone());
    decrypt_input.encryption_context = Some(&encryption_context_b);
    let decryption_response_mismatch_2 = esdk_client.decrypt(&decrypt_input).await;

    if decryption_response_mismatch_2.is_ok() {
        panic!(
            "Decrypt with wrong tenant's hierarchical keyring MUST raise AwsCryptographicMaterialProvidersError"
        )
    }

    // 12. Demonstrate that data encrypted by one tenant's branch key can be decrypted by that tenant,
    //     and that the decrypted data matches the input data.
    decrypt_input.ciphertext = &ciphertext_a;
    decrypt_input.encryption_context = Some(&encryption_context_a);
    let decryption_response_a = esdk_client.decrypt(&decrypt_input).await?;

    let decrypted_plaintext_a = decryption_response_a.plaintext;

    // Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_a, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // Similarly for TenantB
    decrypt_input.ciphertext = &ciphertext_b;
    decrypt_input.encryption_context = Some(&encryption_context_b);
    decrypt_input.keyring = Some(hierarchical_keyring_b.clone());
    let decryption_response_b = esdk_client.decrypt(&decrypt_input).await?;
    let decrypted_plaintext_b = decryption_response_b.plaintext;

    // Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_b, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("Hierarchical Keyring Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Hierarchical Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_KEY_STORE_NAME,
        utils::TEST_LOGICAL_KEY_STORE_NAME,
        utils::TEST_KEY_STORE_KMS_KEY_ID,
    )
    .await?;

    Ok(())
}

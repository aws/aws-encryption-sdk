// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
 This example demonstrates how to use a shared cache across multiple Hierarchical Keyrings.
 With this functionality, users only need to maintain one common shared cache across multiple
 Hierarchical Keyrings with different Key Stores instances/KMS Clients/KMS Keys.

 If you want to use a Shared Cache, you need to initialize it only once, and
 pass the same cache `shared_cache` to different hierarchical keyrings.

 There are three important parameters that users need to carefully set while providing the shared cache:

 1. Partition ID - Partition ID is an optional parameter provided to the Hierarchical Keyring input,
 which distinguishes Cryptographic Material Providers (i.e: Keyrings) writing to a cache.
 - If the Partition ID is set and is the same for two Hierarchical Keyrings (or another Material Provider),
   they CAN share the same cache entries in the cache.
 - If the Partition ID is set and is different for two Hierarchical Keyrings (or another Material Provider),
   they CANNOT share the same cache entries in the cache.
 - If the Partition ID is not set by the user, it is initialized as a random 16-byte UUID which makes
   it unique for every Hierarchical Keyring, and two Hierarchical Keyrings (or another Material Provider)
   CANNOT share the same cache entries in the cache.

 2. Logical Key Store Name - This parameter is set by the user when configuring the Key Store for
 the Hierarchical Keyring. This is a logical name for the branch key store.
 Suppose you have a physical Key Store (K). You create two instances of K (K1 and K2). Now, you create
 two Hierarchical Keyrings (HK1 and HK2) with these Key Store instances (K1 and K2 respectively).
 - If you want to share cache entries across these two keyrings, you should set the Logical Key Store Names
   for both the Key Store instances (K1 and K2) to be the same.
 - If you set the Logical Key Store Names for K1 and K2 to be different, HK1 (which uses Key Store instance K1)
   and HK2 (which uses Key Store instance K2) will NOT be able to share cache entries.

 3. Branch Key ID - Choose an effective Branch Key ID Schema

 This is demonstrated in the example below.
 Notice that both K1 and K2 are instances of the same physical Key Store (K).
 You MUST NEVER have two different physical Key Stores with the same Logical Key Store Name.

 Important Note: If you have two or more Hierarchy Keyrings with:
 - Same Partition ID
 - Same Logical Key Store Name of the Key Store for the Hierarchical Keyring
 - Same Branch Key ID
 then they WILL share the cache entries in the Shared Cache.
 Please make sure that you set all of Partition ID, Logical Key Store Name and Branch Key ID
 to be the same for two Hierarchical Keyrings if and only if you want them to share cache entries.

 This example first creates a shared cache that you can use across multiple Hierarchical Keyrings.
 The example then configures a Hierarchical Keyring (HK1 and HK2) with the shared cache,
 a Branch Key ID and two instances (K1 and K2) of the same physical Key Store (K) respectively,
 i.e. HK1 with K1 and HK2 with K2. The example demonstrates that if you set the same Partition ID
 for HK1 and HK2, the two keyrings can share cache entries.
 If you set different Partition ID of the Hierarchical Keyrings, or different
 Logical Key Store Names of the Key Store instances, then the keyrings will NOT
 be able to share cache entries.

 This example requires access to the DDB Table (K) where you are storing the Branch Keys. This
 table must be configured with the following primary key configuration: - Partition key is named
 "partition_key" with type (S) - Sort key is named "sort_key" with type (S)

 This example also requires using a KMS Key. You need the following access on this key:
 - GenerateDataKeyWithoutPlaintext
 - Decrypt
*/

use super::create_branch_key_id::create_branch_key_id;
use aws_esdk::*;
use aws_mpl_legacy::aws_cryptography_keyStore::client as keystore_client;
use aws_mpl_legacy::aws_cryptography_keyStore::types::KmsConfiguration;
use aws_mpl_legacy::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig;
use aws_mpl_legacy::types::CacheType;
use aws_mpl_legacy::types::DefaultCache;
use aws_mpl_legacy::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    key_store_table_name: &str,
    logical_key_store_name: &str,
    key_store_kms_key_id: &str,
) -> Result<(), crate::BoxError> {
    // 1a. Create the CryptographicMaterialsCache (CMC) to share across multiple Hierarchical Keyrings
    // using the Material Providers Library
    //    This CMC takes in:
    //     - CacheType
    let mpl = mpl();

    let cache: CacheType = CacheType::Default(DefaultCache::builder().entry_capacity(100).build()?);

    let shared_cryptographic_materials_cache: CryptographicMaterialsCacheRef = mpl
        .create_cryptographic_materials_cache()
        .cache(cache)
        .send()
        .await?;

    // 1b. Create a CacheType object for the shared_cryptographic_materials_cache
    // Note that the `cache` parameter in the Hierarchical Keyring Input takes a `CacheType` as input
    // Here, we pass a `Shared` CacheType that passes an already initialized shared cache.

    // If you want to use a Shared Cache, you need to initialize it only once, and
    // pass the same cache `shared_cache` to different hierarchical keyrings.

    // CryptographicMaterialsCacheRef is an Rc (Reference Counted), so if you clone it to
    // pass it to different Hierarchical Keyrings, it will still point to the same
    // underlying cache, and increment the reference count accordingly.
    let shared_cache: CacheType = CacheType::Shared(shared_cryptographic_materials_cache);

    // 3. Configure your Key Store resource key_store1.
    //    This SHOULD be the same configuration that you used
    //    to initially create and populate your physical Key Store.
    // Note that key_store_table_name is the physical Key Store,
    // and key_store1 is instances of this physical Key Store.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let key_store_config = KeyStoreConfig::builder()
        .kms_client(aws_sdk_kms::Client::new(&sdk_config))
        .ddb_client(aws_sdk_dynamodb::Client::new(&sdk_config))
        .ddb_table_name(key_store_table_name)
        .logical_key_store_name(logical_key_store_name)
        .kms_configuration(KmsConfiguration::KmsKeyArn(
            key_store_kms_key_id.to_string(),
        ))
        .build()?;

    let key_store1 = keystore_client::Client::from_conf(key_store_config.clone())?;

    // 4. Call create_branch_key_id to create one new active branch key
    let branch_key_id: String = create_branch_key_id(
        key_store_table_name,
        logical_key_store_name,
        key_store_kms_key_id,
    )
    .await?;

    // 5. Create the Hierarchical Keyring HK1 with Key Store instance K1, partition_id,
    // the shared_cache and the branch_key_id.
    // Note that we are now providing an already initialized shared cache instead of just mentioning
    // the cache type and the Hierarchical Keyring initializing a cache at initialization.

    // partition_id for this example is a random UUID
    let partition_id = "91c1b6a2-6fc3-4539-ad5e-938d597ed730".to_string();

    // Please make sure that you read the guidance on how to set Partition ID, Logical Key Store Name and
    // Branch Key ID at the top of this example before creating Hierarchical Keyrings with a Shared Cache
    let keyring1 = mpl
        .create_aws_kms_hierarchical_keyring()
        .key_store(key_store1)
        .branch_key_id(branch_key_id.clone())
        // CryptographicMaterialsCacheRef is an Rc (Reference Counted), so if you clone it to
        // pass it to different Hierarchical Keyrings, it will still point to the same
        // underlying cache, and increment the reference count accordingly.
        .cache(shared_cache.clone())
        .ttl_seconds(600)
        .partition_id(partition_id.clone())
        .send()
        .await?;

    // 6. Create encryption context.
    // Remember that your encryption context is NOT SECRET.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
    let encryption_context = EncryptionContext::from([
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

    // 7. Encrypt the data for encryption_context using keyring1
    let plaintext = example_data.as_bytes();
    let mut encrypt_input =
        EncryptInput::with_keyring(plaintext, encryption_context.clone(), keyring1.clone());
    let encryption_response1 = encrypt(&encrypt_input).await?;
    let ciphertext1 = encryption_response1.ciphertext;

    // 8. Demonstrate that the ciphertexts and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext1, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 9. Decrypt your encrypted data using the same keyring HK1 you used on encrypt.
    // Provide the encryption context that was supplied to the encrypt method
    let mut decrypt_input = DecryptInput::with_keyring(&ciphertext1, encryption_context, keyring1);
    let decryption_response1 = decrypt(&decrypt_input).await?;

    let decrypted_plaintext1 = decryption_response1.plaintext;
    // 10. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext1, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // 11. Through the above encrypt and decrypt roundtrip, the cache will be populated and
    // the cache entries can be used by another Hierarchical Keyring with the
    // - Same Partition ID
    // - Same Logical Key Store Name of the Key Store for the Hierarchical Keyring
    // - Same Branch Key ID

    // Configure your Key Store resource key_store2.
    //    This SHOULD be the same configuration that you used
    //    to initially create and populate your physical Key Store.
    // Note that key_store_table_name is the physical Key Store,
    // and key_store2 is instances of this physical Key Store.

    // Note that for this example, key_store2 is identical to key_store1.
    // You can optionally change configurations like KMS Client or KMS Key ID based
    // on your use-case.
    // Make sure you have the required permissions to use different configurations.

    // - If you want to share cache entries across two keyrings HK1 and HK2,
    //   you should set the Logical Key Store Names for both
    //   Key Store instances (K1 and K2) to be the same.
    // - If you set the Logical Key Store Names for K1 and K2 to be different,
    //   HK1 (which uses Key Store instance K1) and HK2 (which uses Key Store
    //   instance K2) will NOT be able to share cache entries.
    let key_store2 = keystore_client::Client::from_conf(key_store_config.clone())?;

    // 12. Create the Hierarchical Keyring HK2 with Key Store instance K2, the shared_cache
    // and the same partition_id and branch_key_id used in HK1 because we want to share cache entries
    // (and experience cache HITS).

    // Please make sure that you read the guidance on how to set Partition ID, Logical Key Store Name and
    // Branch Key ID at the top of this example before creating Hierarchical Keyrings with a Shared Cache
    let keyring2 = mpl
        .create_aws_kms_hierarchical_keyring()
        .key_store(key_store2)
        .branch_key_id(branch_key_id)
        .cache(shared_cache)
        .ttl_seconds(600)
        .partition_id(partition_id)
        .send()
        .await?;

    // 13. This encrypt-decrypt roundtrip with HK2 will experience Cache HITS from previous HK1 roundtrip
    // Encrypt the data for encryption_context using keyring2
    encrypt_input.keyring = Some(keyring2.clone());
    let encryption_response2 = encrypt(&encrypt_input).await?;
    let ciphertext2 = encryption_response2.ciphertext;

    // 14. Demonstrate that the ciphertexts and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext2, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 15. Decrypt your encrypted data using the same keyring HK2 you used on encrypt.
    decrypt_input.ciphertext = &ciphertext2;
    decrypt_input.keyring = Some(keyring2);
    let decryption_response2 = decrypt(&decrypt_input).await?;

    let decrypted_plaintext2 = decryption_response2.plaintext;

    // 10. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext2, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("Shared Cache Across Hierarchical Keyrings Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Shared Cache Across Hierarchical Keyrings example
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

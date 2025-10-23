// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the AWS KMS Keyring

The AWS KMS keyring uses symmetric encryption KMS keys to generate, encrypt and
decrypt data keys. This example creates a KMS Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

AWS KMS keyrings can be used independently or in a multi-keyring with other keyrings
of the same or a different type.

For more information on how to use KMS keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

use aws_esdk::*;
use aws_mpl_rs::client as mpl_client;
use aws_mpl_rs::types::material_providers_config::MaterialProvidersConfig;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    kms_key_id: &str,
) -> Result<(), crate::BoxError> {
    // 2. Create a KMS client.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);

    // 3. Create encryption context.
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

    // 4. Create a KMS keyring
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(kms_key_id)
        .kms_client(kms_client)
        .send()
        .await?;

    // 5. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();

    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(kms_keyring.clone())
        .encryption_context(&encryption_context)
        .build()?;
    let encryption_response = encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 6. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 7. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(kms_keyring)
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(&encryption_context)
        .build()?;
    let decryption_response = decrypt(&decrypt_input).await?;

    let decrypted_plaintext = decryption_response.plaintext;

    // 8. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("KMS Keyring Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the AWS KMS Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA, utils::TEST_DEFAULT_KMS_KEY_ID)
        .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring_async() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the AWS KMS Keyring example (async)
    use crate::example_utils::utils;

    let handle = tokio::spawn(async move {
        encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA, utils::TEST_DEFAULT_KMS_KEY_ID)
            .await
    });

    assert!(handle.await.unwrap().is_ok());
    Ok(())
}

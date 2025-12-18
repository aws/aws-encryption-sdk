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

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    kms_key_id: &str,
) -> Result<(), crate::BoxError> {
    // 1. Create a KMS client.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);

    // 2. Create encryption context.
    // Remember that your encryption context is NOT SECRET.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
    let context = EncryptionContext::from([
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

    // 3. Create a KMS keyring
    let kms_keyring = mpl()
        .create_aws_kms_keyring()
        .kms_key_id(kms_key_id)
        .kms_client(kms_client)
        .send()
        .await?;

    // 4. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();
    let encrypt_input = EncryptInput::with_keyring(plaintext, context, kms_keyring);
    let encryption_response = encrypt(&encrypt_input).await?;
    let ciphertext = encryption_response.ciphertext;

    // 5. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 6. Decrypt your encrypted data using the same keyring you used on encrypt.
    // Provide the encryption context that was supplied to the encrypt method
    let decrypt_input = DecryptInput::from_encrypt(&ciphertext, &encrypt_input);
    let decryption_response = decrypt(&decrypt_input).await?;

    // 7. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decryption_response.plaintext, plaintext,
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

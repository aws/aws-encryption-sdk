// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Raw AES Keyring

The Raw AES keyring lets you use an AES symmetric key that you provide as a wrapping key that
protects your data key. You need to generate, store, and protect the key material,
preferably in a hardware security module (HSM) or key management system. Use a Raw AES keyring
when you need to provide the wrapping key and encrypt the data keys locally or offline.

This example creates a Raw AES Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

The Raw AES keyring encrypts data by using the AES-GCM algorithm and a wrapping key that
you specify as a byte array. You can specify only one wrapping key in each Raw AES keyring,
but you can include multiple Raw AES keyrings, alone or with other keyrings, in a multi-keyring.

For more information on how to use Raw AES keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-aes-keyring.html
*/

use aws_esdk::*;
use aws_mpl_legacy::types::AesWrappingAlg;
use rand::TryRngCore;

pub async fn encrypt_and_decrypt_with_legacy_keyring(
    example_data: &str,
) -> Result<(), crate::BoxError> {
    // 2. The key namespace and key name are defined by you.
    // and are used by the Raw AES keyring to determine
    // whether it should attempt to decrypt an encrypted data key.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-aes-keyring.html
    let key_namespace: &str = "my-key-namespace";
    let key_name: &str = "my-aes-key-name";

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

    // 4. Generate a 256-bit AES key to use with your keyring.
    // In practice, you should get this key from a secure key management system such as an HSM.
    let aes_key_bytes = generate_aes_key_bytes();

    // 5. Create a Raw AES Keyring
    let raw_aes_keyring = mpl()
        .create_raw_aes_keyring()
        .key_name(key_name)
        .key_namespace(key_namespace)
        .wrapping_key(aes_key_bytes)
        .wrapping_alg(AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await?;

    // 6. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();

    let encrypt_input: EncryptInput<'_> =
        EncryptInput::with_legacy_keyring(plaintext, encryption_context, raw_aes_keyring);
    let encryption_response = encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 7. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 8. Decrypt your encrypted data using the same keyring you used on encrypt.
    // Provide the encryption context that was supplied to the encrypt method
    let decrypt_input = DecryptInput::from_encrypt(&ciphertext, &encrypt_input);
    let decryption_response = decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 9. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("Raw AES Keyring Example Completed Successfully");

    Ok(())
}

fn generate_aes_key_bytes() -> Vec<u8> {
    // This example returns a random AES key.
    // In practice, you should not generate this key in your code, and should instead
    //     retrieve this key from a secure key management system (e.g. HSM).
    // This key is created here for example purposes only and should not be used for any other purpose.
    let mut random_bytes = [0u8; 32];
    rand::rngs::OsRng.try_fill_bytes(&mut random_bytes).unwrap();

    random_bytes.to_vec()
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_legacy_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Raw AES Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_legacy_keyring(utils::TEST_EXAMPLE_DATA).await?;

    Ok(())
}

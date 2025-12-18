// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example demonstrates how to set an algorithm suite while using the Raw AES Keyring
in the AWS Encryption SDK.

The algorithm suite used in the encrypt() method is the algorithm used to protect your
data using the data key. By setting this algorithm, you can configure the algorithm used
to encrypt and decrypt your data.

Algorithm suites can be set in a similar manner in other keyrings as well. However,
please make sure that you're using a logical algorithm suite that is compatible with your
keyring. For more information on algorithm suites supported by the AWS Encryption SDK, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html

The AES wrapping algorithm (AesWrappingAlg::AlgAes256GcmIv12Tag16) protects your data key using
the user-provided wrapping key. In contrast, the algorithm suite used in the encrypt() method
is the algorithm used to protect your data using the data key. This example demonstrates setting the
latter, which is the algorithm suite for protecting your data. When the commitment policy is
RequireEncryptRequireDecrypt, the default algorithm used in the encrypt method is
AlgAes256GcmHkdfSha512CommitKeyEcdsaP384, which is a committing and signing algorithm.
Signature verification ensures the integrity of a digital message as it goes across trust
boundaries. However, signature verification adds a significant performance cost to encryption
and decryption. If encryptors and decryptors are equally trusted, we can consider using an algorithm
suite that does not include signing. This example sets the algorithm suite as
AlgAes256GcmHkdfSha512CommitKey, which is a committing but non-signing algorithm.
For more information on digital signatures, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#digital-sigs

This example creates a Raw AES Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context and the algorithm suite AlgAes256GcmHkdfSha512CommitKey.
This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on how to use Raw AES keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-aes-keyring.html
*/

use aws_esdk::*;
use aws_mpl_legacy::types::AesWrappingAlg;
use aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey;
use rand::TryRngCore;

pub async fn encrypt_and_decrypt_with_keyring(example_data: &str) -> Result<(), crate::BoxError> {
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
    // The wrapping algorithm here is NOT the algorithm suite we set in this example.
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

    // This is the important step in this example where we specify the algorithm suite
    // you want to use for encrypting your data
    let mut encrypt_input =
        EncryptInput::with_keyring(plaintext, encryption_context, raw_aes_keyring);
    encrypt_input.algorithm_suite_id = Some(AlgAes256GcmHkdfSha512CommitKey);
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

    println!("Set Encryption Algorithm Suite Example Completed Successfully");

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
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Set Encryption Algorithm Suite example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA).await?;

    Ok(())
}

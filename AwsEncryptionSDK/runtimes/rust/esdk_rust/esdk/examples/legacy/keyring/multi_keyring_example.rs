// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Multi Keyring

A multi-keyring is a keyring that consists of one or more individual keyrings of the
same or a different type. The effect is like using several keyrings in a series.
When you use a multi-keyring to encrypt data, any of the wrapping keys in any of its
keyrings can decrypt that data.

When you create a multi-keyring to encrypt data, you designate one of the keyrings as
the generator keyring. All other keyrings are known as child keyrings. The generator keyring
generates and encrypts the plaintext data key. Then, all of the wrapping keys in all of the
child keyrings encrypt the same plaintext data key. The multi-keyring returns the plaintext
key and one encrypted data key for each wrapping key in the multi-keyring. If you create a
multi-keyring with no generator keyring, you can use it to decrypt data, but not to encrypt.
If the generator keyring is a KMS keyring, the generator key in the AWS KMS keyring generates
and encrypts the plaintext key. Then, all additional AWS KMS keys in the AWS KMS keyring,
and all wrapping keys in all child keyrings in the multi-keyring, encrypt the same plaintext key.

When decrypting, the AWS Encryption SDK uses the keyrings to try to decrypt one of the encrypted
data keys. The keyrings are called in the order that they are specified in the multi-keyring.
Processing stops as soon as any key in any keyring can decrypt an encrypted data key.

This example creates a Multi Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decryption of ciphertext is possible using the multi_keyring,
and every one of the keyrings from the multi_keyring separately
3. All decrypted plaintext value match EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

This example creates a multi_keyring using a KMS keyring as generator keyring and a raw AES keyring
as a child keyring. You can use different combinations of keyrings in the multi_keyring.

For more information on how to use Multi keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-multi-keyring.html

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

use aws_esdk::*;
use aws_mpl_legacy::types::AesWrappingAlg;
use rand::TryRngCore;

pub async fn encrypt_and_decrypt_with_legacy_keyring(
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
    let mpl = mpl();

    let kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(kms_key_id)
        .kms_client(kms_client)
        .send()
        .await?;

    // 5. Create a raw AES keyring to additionally encrypt under as child_keyring

    // The key namespace and key name are defined by you.
    // and are used by the Raw AES keyring to determine
    // whether it should attempt to decrypt an encrypted data key.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-aes-keyring.html
    let key_namespace: &str = "my-key-namespace";
    let key_name: &str = "my-aes-key-name";

    // Generate a 256-bit AES key to use with your raw AES keyring.
    // In practice, you should get this key from a secure key management system such as an HSM.
    let aes_key_bytes = generate_aes_key_bytes();

    let raw_aes_keyring = mpl
        .create_raw_aes_keyring()
        .key_name(key_name)
        .key_namespace(key_namespace)
        .wrapping_key(aes_key_bytes)
        .wrapping_alg(AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await?;

    // 6. Create a multi_keyring that consists of the previously created keyrings.
    // When using this multi_keyring to encrypt data, either `kms_keyring` or
    // `raw_aes_keyring` (or a multi_keyring containing either) may be used to decrypt the data.
    let multi_keyring = mpl
        .create_multi_keyring()
        .generator(kms_keyring.clone())
        .child_keyrings(vec![raw_aes_keyring.clone()])
        .send()
        .await?;

    // 7. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();
    let encrypt_input =
        EncryptInput::with_legacy_keyring(plaintext, encryption_context, multi_keyring);
    let encryption_response = encrypt(&encrypt_input).await?;
    let ciphertext = encryption_response.ciphertext;

    // 8. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 9a. Decrypt your encrypted data using the same multi_keyring you used on encrypt.
    // Provide the encryption context that was supplied to the encrypt method
    let mut decrypt_input = DecryptInput::from_encrypt(&ciphertext, &encrypt_input);
    let decryption_response_multi_keyring = decrypt(&decrypt_input).await?;
    let decrypted_plaintext_multi_keyring = decryption_response_multi_keyring.plaintext;

    // 9b. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_multi_keyring, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // Because you used a multi_keyring on Encrypt, you can use either the
    // `kms_keyring` or `raw_aes_keyring` individually to decrypt the data.

    // 10. Demonstrate that you can successfully decrypt data using just the `kms_keyring`
    // directly.
    // (This is an example for demonstration; you do not need to do this in your own code.)

    // 10a. Decrypt your encrypted data using the kms_keyring.
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(kms_keyring));
    let decryption_response_kms_keyring = decrypt(&decrypt_input).await?;
    let decrypted_plaintext_kms_keyring = decryption_response_kms_keyring.plaintext;

    // 10b. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_kms_keyring, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // 11. Demonstrate that you can also successfully decrypt data using the `raw_aes_keyring`
    // directly.
    // (This is an example for demonstration; you do not need to do this in your own code.)

    // 11a. Decrypt your encrypted data using the raw_aes_keyring.
    decrypt_input.source = Some(MaterialSource::LegacyKeyring(raw_aes_keyring));
    let decryption_response_raw_aes_keyring = decrypt(&decrypt_input).await?;
    let decrypted_plaintext_raw_aes_keyring = decryption_response_raw_aes_keyring.plaintext;

    // 11b. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_raw_aes_keyring, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("Multi Keyring Example Completed Successfully");

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
    // Test function for encrypt and decrypt using the Multi Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_legacy_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
    )
    .await?;

    Ok(())
}

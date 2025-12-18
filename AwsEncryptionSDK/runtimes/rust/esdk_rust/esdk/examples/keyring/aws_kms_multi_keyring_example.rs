// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the AWS KMS Multi Keyring made up of multiple AWS KMS Keyrings.

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

This example creates a multi_keyring using a KMS keyring as generator keyring and
another KMS keyring as a child keyring.

For more information on how to use Multi keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-multi-keyring.html

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

use aws_config::Region;
use aws_esdk::*;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    default_region_kms_key_id: &str,
    second_region_kms_key_id: &str,
    default_region: String,
    second_region: String,
) -> Result<(), crate::BoxError> {
    // 2. Create encryption context.
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

    // 3. Create an AwsKmsMultiKeyring that protects your data under two different KMS Keys.
    // Either KMS Key individually is capable of decrypting data encrypted under this Multi Keyring.
    let mpl = mpl();

    let kms_multi_keyring = mpl
        .create_aws_kms_multi_keyring()
        .generator(default_region_kms_key_id)
        .kms_key_ids(vec![second_region_kms_key_id.to_string()])
        .send()
        .await?;

    // 4. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();
    let encrypt_input =
        EncryptInput::with_keyring(plaintext, encryption_context, kms_multi_keyring);
    let encryption_response = encrypt(&encrypt_input).await?;
    let ciphertext = encryption_response.ciphertext;

    // 5. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 6a. Decrypt your encrypted data using the same multi_keyring you used on encrypt.
    let mut decrypt_input = DecryptInput::from_encrypt(&ciphertext, &encrypt_input);
    let decryption_response_multi_keyring = decrypt(&decrypt_input).await?;
    let decrypted_plaintext_multi_keyring = decryption_response_multi_keyring.plaintext;

    // 6b. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_multi_keyring, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // Because you used a multi_keyring on Encrypt, you can use either of the two
    // kms keyrings individually to decrypt the data.

    // 7. Demonstrate that you can successfully decrypt data using a KMS keyring with just the
    // `default_region_kms_key_id` directly.
    // (This is an example for demonstration; you do not need to do this in your own code.)

    // 7a. Create a client for KMS for the default region.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let default_region_kms_config = aws_sdk_kms::config::Builder::from(&sdk_config)
        .region(Region::new(default_region))
        .build();
    let default_region_kms_client = aws_sdk_kms::Client::from_conf(default_region_kms_config);

    // 7b. Create KMS keyring
    let default_region_kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(default_region_kms_key_id)
        .kms_client(default_region_kms_client)
        .send()
        .await?;

    // 7c. Decrypt your encrypted data using the default_region_kms_keyring.
    decrypt_input.keyring = Some(default_region_kms_keyring);
    let decryption_response_default_region_kms_keyring = decrypt(&decrypt_input).await?;

    let decrypted_plaintext_default_region_kms_keyring =
        decryption_response_default_region_kms_keyring.plaintext;

    // 7d. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_default_region_kms_keyring, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // 8. Demonstrate that you can also successfully decrypt data using a KMS keyring with just the
    // `second_region_kms_key_id` directly.
    // (This is an example for demonstration; you do not need to do this in your own code.)

    // 8a. Create a client for KMS for the second region.
    let second_region_kms_config = aws_sdk_kms::config::Builder::from(&sdk_config)
        .region(Region::new(second_region))
        .build();
    let second_region_kms_client = aws_sdk_kms::Client::from_conf(second_region_kms_config);

    // 8b. Create KMS keyring
    let second_region_kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(second_region_kms_key_id)
        .kms_client(second_region_kms_client)
        .send()
        .await?;

    // 8c. Decrypt your encrypted data using the second_region_kms_keyring.
    decrypt_input.keyring = Some(second_region_kms_keyring);
    let decryption_response_second_region_kms_keyring = decrypt(&decrypt_input).await?;

    let decrypted_plaintext_second_region_kms_keyring =
        decryption_response_second_region_kms_keyring.plaintext;
    // 8d. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext_second_region_kms_keyring, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("KMS Multi Keyring Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the AWS KMS Multi Keyring example
    use crate::example_utils::utils;

    let default_region: String = "us-west-2".to_string();
    let second_region: String = "eu-central-1".to_string();

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
        utils::TEST_SECOND_REGION_KMS_KEY_ID,
        default_region,
        second_region,
    )
    .await?;

    Ok(())
}

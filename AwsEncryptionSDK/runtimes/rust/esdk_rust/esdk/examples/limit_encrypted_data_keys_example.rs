// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
Demonstrate limiting the number of Encrypted Data Keys [EDKs] allowed
when encrypting or decrypting a message.
Limiting encrypted data keys is most valuable when you are decrypting
messages from an untrusted source.
By default, the ESDK will allow up to 65,535 encrypted data keys.
A malicious actor might construct an encrypted message with thousands of
encrypted data keys, none of which can be decrypted.
As a result, the AWS Encryption SDK would attempt to decrypt each
encrypted data key until it exhausted the encrypted data keys in the message.

For more information on limiting EDKs, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/configure.html#config-limit-keys
*/

use aws_esdk::Client as EsdkClient;
use aws_esdk::*;
use aws_mpl_rs::client as mpl_client;
use aws_mpl_rs::types::AesWrappingAlg;
use aws_mpl_rs::types::keyring::KeyringRef;
use aws_mpl_rs::types::material_providers_config::MaterialProvidersConfig;
use rand::TryRngCore;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    max_encrypted_data_keys: u16,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    // Also, set the EncryptionSDK's max_encrypted_data_keys parameter here
    let esdk_config = AwsEncryptionSdkConfigBuilder::default()
        .max_encrypted_data_keys(max_encrypted_data_keys)
        .build()?;
    let esdk_client = EsdkClient::from_conf(esdk_config)?;

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

    // 4. Generate `max_encrypted_data_keys` AES keyrings to use with your keyring.
    // In practice, you should get this key from a secure key management system such as an HSM.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    let mut raw_aes_keyrings: Vec<KeyringRef> = vec![];

    assert!(
        max_encrypted_data_keys > 0,
        "max_encrypted_data_keys MUST be greater than 0"
    );

    let mut i = 0;
    while i < max_encrypted_data_keys {
        let aes_key_bytes = generate_aes_key_bytes();

        let raw_aes_keyring = mpl
            .create_raw_aes_keyring()
            .key_name(key_name)
            .key_namespace(key_namespace)
            .wrapping_key(aes_key_bytes)
            .wrapping_alg(AesWrappingAlg::AlgAes256GcmIv12Tag16)
            .send()
            .await?;

        raw_aes_keyrings.push(raw_aes_keyring);
        i += 1;
    }

    // 5. Create a Multi Keyring with `max_encrypted_data_keys` AES Keyrings
    let generator_keyring = raw_aes_keyrings.remove(0);

    let multi_keyring = mpl
        .create_multi_keyring()
        .generator(generator_keyring)
        .child_keyrings(raw_aes_keyrings)
        .send()
        .await?;

    // 6. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();

    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(multi_keyring.clone())
        .encryption_context(&encryption_context)
        .build()?;
    let encryption_response = esdk_client.encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 7. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 8. Decrypt your encrypted data using the same keyring you used on encrypt.
    let mut decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(multi_keyring.clone())
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(&encryption_context)
        .build()?;
    let decryption_response = esdk_client.decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 9. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    // 10. Demonstrate that an EncryptionSDK with a lower MaxEncryptedDataKeys
    // will fail to decrypt the encrypted message.
    let esdk_config = AwsEncryptionSdkConfigBuilder::default()
        .max_encrypted_data_keys(max_encrypted_data_keys - 1)
        .build()?;
    let esdk_client_incorrect_max_encrypted_keys = EsdkClient::from_conf(esdk_config)?;

    decrypt_input.keyring = Some(multi_keyring);
    let decryption_response_incorrect_max_encrypted_keys = esdk_client_incorrect_max_encrypted_keys
        .decrypt(&decrypt_input)
        .await;

    if decryption_response_incorrect_max_encrypted_keys.is_ok() {
        panic!(
            "Decrypt using discovery keyring with wrong AWS Account ID MUST raise AwsCryptographicMaterialProvidersError"
        );
    }

    println!("Limit Encrypted Data Keys Example Completed Successfully");

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
    // Test function for encrypt and decrypt using the Limit Encrypted Data Keys example
    use crate::example_utils::utils;

    // max_encrypted_data_keys MUST be greater than 0
    let max_encrypted_data_keys: u16 = 3;

    encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA, max_encrypted_data_keys).await?;

    Ok(())
}

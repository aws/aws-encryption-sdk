// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the AWS KMS MRK (multi-region key) Keyring

The AWS Key Management Service (AWS KMS) MRK keyring interacts with AWS KMS to
create, encrypt, and decrypt data keys with multi-region AWS KMS keys (MRKs).
This example creates a KMS MRK Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

AWS KMS MRK keyrings can be used independently or in a multi-keyring with other keyrings
of the same or a different type.

For more information on how to use KMS keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html

For more info on KMS MRK (multi-region keys), see the KMS documentation:
https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

use aws_config::Region;
use aws_esdk::Client as EsdkClient;
use aws_esdk::*;
use aws_mpl_rs::client as mpl_client;
use aws_mpl_rs::types::material_providers_config::MaterialProvidersConfig;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    mrk_key_id_encrypt: &str,
    mrk_replica_key_id_decrypt: &str,
    mrk_encrypt_region: String,
    mrk_replica_decrypt_region: String,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_client = EsdkClient::default();

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

    // 3. Create a keyring that will encrypt your data, using a KMS MRK in the first region.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    // Create a KMS client in the first region
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let encrypt_kms_config = aws_sdk_kms::config::Builder::from(&sdk_config)
        .region(Region::new(mrk_encrypt_region))
        .build();
    let encrypt_kms_client = aws_sdk_kms::Client::from_conf(encrypt_kms_config);

    // Create the keyring that determines how your data keys are protected.
    let encrypt_kms_keyring = mpl
        .create_aws_kms_mrk_keyring()
        .kms_key_id(mrk_key_id_encrypt)
        .kms_client(encrypt_kms_client)
        .send()
        .await?;

    // 4. Encrypt the data with the encryption_context using the encrypt_keyring.
    let plaintext = example_data.as_bytes();

    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(encrypt_kms_keyring)
        .encryption_context(&encryption_context)
        .build()?;
    let encryption_response = esdk_client.encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 5. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 6. Create a keyring that will decrypt your data, using the same KMS MRK replicated
    // to the second region. This example assumes you have already replicated your key

    // Create a KMS Client in the second region.
    let decrypt_kms_config = aws_sdk_kms::config::Builder::from(&sdk_config)
        .region(Region::new(mrk_replica_decrypt_region))
        .build();
    let decrypt_kms_client = aws_sdk_kms::Client::from_conf(decrypt_kms_config);

    let decrypt_kms_keyring = mpl
        .create_aws_kms_mrk_keyring()
        .kms_key_id(mrk_replica_key_id_decrypt)
        .kms_client(decrypt_kms_client)
        .send()
        .await?;

    // 7. Decrypt your encrypted data using the decrypt keyring.
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(decrypt_kms_keyring)
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(&encryption_context)
        .build()?;
    let decryption_response = esdk_client.decrypt(&decrypt_input).await?;

    let decrypted_plaintext = decryption_response.plaintext;

    // 8. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("KMS MRK Keyring Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the AWS KMS MRK Keyring example
    use crate::example_utils::utils;

    let mrk_encrypt_region: String = "us-east-1".to_string();
    let mrk_replica_decrypt_region: String = "eu-west-1".to_string();

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        utils::TEST_MRK_KEY_ID_EU_WEST_1,
        mrk_encrypt_region,
        mrk_replica_decrypt_region,
    )
    .await?;

    Ok(())
}

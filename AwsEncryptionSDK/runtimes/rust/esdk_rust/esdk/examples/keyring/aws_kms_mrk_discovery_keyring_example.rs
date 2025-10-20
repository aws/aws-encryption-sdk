// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the AWS KMS MRK (multi-region key) Discovery Keyring

The AWS KMS discovery keyring is an AWS KMS keyring that doesn't specify any wrapping keys.

When decrypting, an MRK discovery keyring allows the AWS Encryption SDK to ask AWS KMS to decrypt
any encrypted data key by using the AWS KMS MRK that encrypted it, regardless of who owns or
has access to that AWS KMS key. The call succeeds only when the caller has kms:Decrypt
permission on the AWS KMS MRK.

The AWS Encryption SDK provides a standard AWS KMS discovery keyring and a discovery keyring
for AWS KMS multi-Region keys. Because it doesn't specify any wrapping keys, a discovery keyring
can't encrypt data. If you use a discovery keyring to encrypt data, alone or in a multi-keyring,
the encrypt operation fails.

The AWS Key Management Service (AWS KMS) MRK keyring interacts with AWS KMS to
create, encrypt, and decrypt data keys with multi-region AWS KMS keys (MRKs).
This example creates a KMS MRK Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This encrypted ciphertext is then decrypted using an
MRK Discovery keyring. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For information about using multi-Region keys with the AWS Encryption SDK, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/configure.html#config-mrks

For more info on KMS MRKs (multi-region keys), see the KMS documentation:
https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html

For more information on how to use KMS Discovery keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html#kms-keyring-discovery

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

use aws_config::Region;
use aws_esdk::client as esdk_client;
use aws_esdk::material_providers::client as mpl_client;
use aws_esdk::material_providers::types::DiscoveryFilter;
use aws_esdk::material_providers::types::material_providers_config::MaterialProvidersConfig;
use aws_esdk::types::*;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    mrk_key_id_encrypt: &str,
    aws_account_id: &str,
    mrk_encrypt_region: String,
    mrk_replica_decrypt_region: String,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::default();
    let esdk_client = esdk_client::Client::from_conf(esdk_config)?;

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

    // 3. Create the keyring that determines how your data keys are protected.
    // Although this example highlights Discovery keyrings, Discovery keyrings cannot
    // be used to encrypt, so for encryption we create an MRK keyring without discovery mode.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    // Create a KMS client in the first region
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let encrypt_kms_config = aws_sdk_kms::config::Builder::from(&sdk_config)
        .region(Region::new(mrk_encrypt_region))
        .build();
    let encrypt_kms_client = aws_sdk_kms::Client::from_conf(encrypt_kms_config);

    // Create a keyring that will encrypt your data, using a KMS MRK in the first region.
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

    // 6. Now create a Discovery keyring to use for decryption.
    // In order to illustrate the MRK behavior of this keyring, we configure
    // the keyring to use the second KMS region where the MRK (mrk_key_id_encrypt) is replicated to.
    // This example assumes you have already replicated your key, but since we
    // are using a discovery keyring, we don't need to provide the mrk replica key id

    // Create a KMS Client in the second region.
    let decrypt_kms_config = aws_sdk_kms::config::Builder::from(&sdk_config)
        .region(Region::new(mrk_replica_decrypt_region.clone()))
        .build();
    let decrypt_kms_client = aws_sdk_kms::Client::from_conf(decrypt_kms_config);

    let discovery_filter = DiscoveryFilter::builder()
        .account_ids(vec![aws_account_id.to_string()])
        .partition("aws".to_string())
        .build()?;

    let discovery_keyring = mpl
        .create_aws_kms_mrk_discovery_keyring()
        .kms_client(decrypt_kms_client)
        .region(mrk_replica_decrypt_region)
        .discovery_filter(discovery_filter)
        .send()
        .await?;

    // 7. Decrypt your encrypted data using the discovery keyring.
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(discovery_keyring)
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

    println!("KMS MRK Discovery Keyring Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the AWS KMS MRK Discovery Keyring example
    use crate::example_utils::utils;

    let mrk_encrypt_region: String = "us-east-1".to_string();
    let mrk_replica_decrypt_region: String = "eu-west-1".to_string();

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_MRK_KEY_ID_US_EAST_1,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
        mrk_encrypt_region,
        mrk_replica_decrypt_region,
    )
    .await?;

    Ok(())
}

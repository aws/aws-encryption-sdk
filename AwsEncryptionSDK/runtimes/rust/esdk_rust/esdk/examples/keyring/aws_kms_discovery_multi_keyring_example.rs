// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the AWS KMS Discovery Multi Keyring and demonstrates decryption
using a Multi-Keyring containing multiple AWS KMS Discovery Keyrings.

The AWS Encryption SDK provides a standard AWS KMS discovery keyring and a discovery keyring
for AWS KMS multi-Region keys. For information about using multi-Region keys with the
AWS Encryption SDK, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/configure.html#config-mrks

Because it doesn't specify any wrapping keys, a discovery keyring can't encrypt data.
If you use a discovery keyring to encrypt data, alone or in a multi-keyring, the encrypt
operation fails.

When decrypting, a discovery keyring allows the AWS Encryption SDK to ask AWS KMS to decrypt
any encrypted data key by using the AWS KMS key that encrypted it, regardless of who owns or
has access to that AWS KMS key. The call succeeds only when the caller has kms:Decrypt
permission on the AWS KMS key.

This example creates a KMS Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This encrypted ciphertext is then decrypted using the Discovery Multi
keyring. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on how to use KMS Discovery keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-keyring.html#kms-keyring-discovery

For more information on KMS Key identifiers, see
https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id
*/

use aws_esdk::*;
use aws_mpl_legacy::types::DiscoveryFilter;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    kms_key_id: &str,
    aws_account_id: &str,
    aws_regions: Vec<String>,
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

    // 4. Create the keyring that determines how your data keys are protected.
    //    Although this example highlights Discovery keyrings, Discovery keyrings cannot
    //    be used to encrypt, so for encryption we create a KMS keyring without discovery mode.
    let mpl = mpl();

    let encrypt_kms_keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(kms_key_id)
        .kms_client(kms_client.clone())
        .send()
        .await?;

    // 5. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();
    let encrypt_input = EncryptInput::with_keyring(plaintext, encryption_context.clone(), encrypt_kms_keyring);
    let encryption_response = encrypt(&encrypt_input).await?;
    let ciphertext = encryption_response.ciphertext;

    // 6. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 7. Now create a Discovery Multi keyring to use for decryption. We'll add a discovery filter
    //    so that we limit the set of ciphertexts we are willing to decrypt to only ones
    //    created by KMS keys in our account and partition.
    let discovery_filter = DiscoveryFilter::builder()
        .account_ids(vec![aws_account_id.to_string()])
        .partition("aws".to_string())
        .build()?;

    // This is a Multi Keyring composed of Discovery Keyrings.
    // There is a keyring for every region in `regions`.
    // All the keyrings have the same Discovery Filter.
    // Each keyring has its own KMS Client, which is created for the keyring's region.
    let discovery_multi_keyring = mpl
        .create_aws_kms_discovery_multi_keyring()
        .regions(aws_regions)
        .discovery_filter(discovery_filter)
        .send()
        .await?;

    // 8. On Decrypt, the header of the encrypted message (ciphertext) will be parsed.
    // The header contains the Encrypted Data Keys (EDKs), which, if the EDK
    // was encrypted by a KMS Keyring, includes the KMS Key ARN.
    // For each member of the Multi Keyring, every EDK will try to be decrypted until a decryption
    // is successful.
    // Since every member of the Multi Keyring is a Discovery Keyring:
    //   Each Keyring will filter the EDKs by the Discovery Filter
    //       For the filtered EDKs, the keyring will try to decrypt it with the keyring's client.
    // All of this is done serially, until a success occurs or all keyrings have
    // failed all (filtered) EDKs.
    // KMS Discovery Keyrings will attempt to decrypt Multi Region Keys (MRKs) and regular KMS Keys.
    let decrypt_input = DecryptInput::with_keyring(&ciphertext, encryption_context, discovery_multi_keyring);
    let decryption_response = decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 9. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("KMS Discovery Multi Keyring Example Completed Successfully");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the AWS KMS Discovery Multi Keyring example
    use crate::example_utils::utils;

    let aws_regions: Vec<String> = vec!["us-east-1".to_string(), "us-west-2".to_string()];

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_DEFAULT_KMS_KEY_ID,
        utils::TEST_DEFAULT_KMS_KEY_ACCOUNT_ID,
        aws_regions,
    )
    .await?;

    Ok(())
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the KMS ECDH Discovery Keyring.

This example takes in the recipient's KMS ECC key ARN.
This example attempts to decrypt a ciphertext using the provided ecc_recipient_key_arn,
it does so by checking if the message header contains the recipient's public key.

This example also requires access to a KMS ECC key.
Our tests provide a KMS ECC Key ARN that anyone can use, but you
can also provide your own KMS ECC key.
To use your own KMS ECC key, you must have:
    - kms:GetPublicKey permissions on that key.
This example will call kms:GetPublicKey on keyring creation.
You must also have kms:DeriveSharedSecret permissions on the KMS ECC key.

This example creates a KMS ECDH Discovery Keyring and then decrypts a ciphertext.
For getting the ciphertext, we create a KMS ECDH keyring without discovery
because kms_ecdh_discovery_keyring cannot encrypt data.
This example also includes some sanity checks for demonstration:
1. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-discovery
*/

use crate::example_utils::utils::TEST_KMS_ECDH_KEY_ID_P256_SENDER;
use crate::example_utils::utils::generate_kms_ecc_public_key;
use aws_esdk::Client as EsdkClient;
use aws_esdk::*;
use aws_mpl_rs::aws_cryptography_primitives::types::EcdhCurveSpec;
use aws_mpl_rs::client as mpl_client;
use aws_mpl_rs::types::KmsEcdhStaticConfigurations;
use aws_mpl_rs::types::KmsPrivateKeyToStaticPublicKeyInput;
use aws_mpl_rs::types::KmsPublicKeyDiscoveryInput;
use aws_mpl_rs::types::material_providers_config::MaterialProvidersConfig;

pub async fn decrypt_with_keyring(
    example_data: &str,
    ecdh_curve_spec: EcdhCurveSpec,
    ecc_recipient_key_arn: &str,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::default();
    let esdk_client = EsdkClient::from_conf(esdk_config)?;

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

    // 4. Create the KmsPublicKeyDiscoveryInput
    let kms_ecdh_discovery_static_configuration_input = KmsPublicKeyDiscoveryInput::builder()
        .recipient_kms_identifier(ecc_recipient_key_arn)
        .build()?;

    let kms_ecdh_discovery_static_configuration =
        KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery(
            kms_ecdh_discovery_static_configuration_input,
        );

    // 5. Create the KMS ECDH keyring.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    // Create a KMS ECDH Discovery keyring.
    // This keyring uses the KmsPublicKeyDiscovery configuration.
    // On encrypt, the keyring will fail as it is not allowed to encrypt data under this configuration.
    // On decrypt, the keyring will check if its corresponding public key is stored in the message header. It
    // will AWS KMS to derive the shared from the recipient's KMS ECC Key ARN and the sender's public key;
    // For more information on this configuration see:
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-discovery
    // This keyring takes in:
    //  - kmsClient
    //  - recipientKmsIdentifier: Must be an ARN representing a KMS ECC key meant for KeyAgreement
    //  - curveSpec: The curve name where the public keys lie
    let kms_ecdh_discovery_keyring = mpl
        .create_aws_kms_ecdh_keyring()
        .kms_client(kms_client.clone())
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(kms_ecdh_discovery_static_configuration)
        .send()
        .await?;

    // 6. Get ciphertext by creating a KMS ECDH keyring WITHOUT discovery
    // because the KMS ECDH keyring WITH discovery CANNOT encrypt data.
    let plaintext = example_data.as_bytes();

    // Get ciphertext by creating a KMS ECDH keyring WITHOUT discovery.
    // The recipient's public key used in the encrypting KMS ECDH keyring WITHOUT discovery
    // is a public key generated from ecc_recipient_key_arn, the same ecc key used
    // when creating the KMS ECDH keyring WITH discovery used for decryption in this example.
    // We then decrypt this ciphertext using a KMS ECDH keyring WITH discovery
    let ciphertext = get_ciphertext(
        example_data,
        &encryption_context,
        ecc_recipient_key_arn,
        ecdh_curve_spec,
        kms_client,
        &esdk_client,
    )
    .await?;

    // 7. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(kms_ecdh_discovery_keyring)
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

    println!("KMS ECDH Discovery Keyring Example Completed Successfully");

    Ok(())
}

async fn get_ciphertext(
    example_data: &str,
    encryption_context: &EncryptionContext,
    ecc_recipient_key_arn: &str,
    ecdh_curve_spec: EcdhCurveSpec,
    kms_client: aws_sdk_kms::Client,
    esdk_client: &EsdkClient,
) -> Result<Vec<u8>, crate::BoxError> {
    // 1. Create the public keys for sender and recipient
    // Recipient keys are taken as input for this example
    // Sender ECC key used in this example is TEST_KMS_ECDH_KEY_ID_P256_SENDER
    let public_key_sender_utf8_bytes =
        generate_kms_ecc_public_key(TEST_KMS_ECDH_KEY_ID_P256_SENDER).await?;
    let public_key_recipient_utf8_bytes =
        generate_kms_ecc_public_key(ecc_recipient_key_arn).await?;

    // 2. Create the KmsPrivateKeyToStaticPublicKeyInput
    let kms_ecdh_static_configuration_input = KmsPrivateKeyToStaticPublicKeyInput::builder()
        .sender_kms_identifier(TEST_KMS_ECDH_KEY_ID_P256_SENDER)
        // Must be a UTF8 DER-encoded X.509 public key
        .sender_public_key(public_key_sender_utf8_bytes)
        // Must be a UTF8 DER-encoded X.509 public key
        .recipient_public_key(public_key_recipient_utf8_bytes)
        .build()?;

    let kms_ecdh_static_configuration = KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(
        kms_ecdh_static_configuration_input,
    );

    // 3. Create the KMS ECDH keyring.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    let kms_ecdh_keyring = mpl
        .create_aws_kms_ecdh_keyring()
        .kms_client(kms_client)
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(kms_ecdh_static_configuration)
        .send()
        .await?;

    // 4. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();

    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(kms_ecdh_keyring)
        .encryption_context(encryption_context)
        .build()?;
    let encryption_response = esdk_client.encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 5. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    Ok(ciphertext)
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for decrypt using the KMS ECDH Discovery Keyring example
    use crate::example_utils::utils;

    decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        EcdhCurveSpec::EccNistP256,
        utils::TEST_KMS_ECDH_KEY_ID_P256_RECIPIENT,
    )
    .await?;

    Ok(())
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the KMS ECDH Keyring.

This example takes in the sender's KMS ECC key ARN, the sender's public key,
the recipient's public key, and the algorithm definition where the ECC keys lie.

Both public keys MUST be UTF8 PEM-encoded X.509 public key,
also known as SubjectPublicKeyInfo (SPKI),

This keyring, depending on its KeyAgreement scheme,
takes in the sender's KMS ECC Key ARN, and the recipient's ECC Public Key
to derive a shared secret.
The keyring uses the shared secret to derive a data key to protect the
data keys that encrypt and decrypt example_data.

This example also requires access to a KMS ECC key.
Our tests provide a KMS ECC Key ARN that anyone can use, but you
can also provide your own KMS ECC key.
To use your own KMS ECC key, you must have either:
- Its public key downloaded in a UTF-8 encoded PEM file
- kms:GetPublicKey permissions on that key.
If you do not have the public key downloaded, running this example
through its main method will download the public key for you
by calling kms:GetPublicKey.
You must also have kms:DeriveSharedSecret permissions on the KMS ECC key.
This example also requires a recipient ECC Public Key that lies on the same
curve as the sender public key. This examples uses another distinct
KMS ECC Public Key, it does not have to be a KMS key; it can be a
valid SubjectPublicKeyInfo (SPKI) Public Key.

This example creates a KMS ECDH Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-create
*/

use crate::example_utils::utils::EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_RECIPIENT;
use crate::example_utils::utils::EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_SENDER;
use crate::example_utils::utils::exists;
use crate::example_utils::utils::write_kms_ecdh_ecc_public_key;
use aws_esdk::client as esdk_client;
use aws_esdk::material_providers::client as mpl_client;
use aws_esdk::material_providers::types::KmsEcdhStaticConfigurations;
use aws_esdk::material_providers::types::KmsPrivateKeyToStaticPublicKeyInput;
use aws_esdk::material_providers::types::material_providers_config::MaterialProvidersConfig;
use aws_esdk::types::*;
use aws_mpl_rs::aws_cryptography_primitives::types::EcdhCurveSpec;
use pem::parse;
use std::path::Path;

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    ecc_key_arn: &str,
    ecdh_curve_spec: EcdhCurveSpec,
    ecc_recipient_key_arn: Option<&str>,
) -> Result<(), crate::BoxError> {
    // 1. If ecc_recipient_key_arn is not provided, set the private key for the recipient to TEST_KMS_ECDH_KEY_ID_P256_RECIPIENT
    let ecc_recipient_key_arn = ecc_recipient_key_arn
        .unwrap_or(crate::example_utils::utils::TEST_KMS_ECDH_KEY_ID_P256_RECIPIENT);

    // 2. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::default();
    let esdk_client = esdk_client::Client::from_conf(esdk_config)?;

    // 3. Create a KMS client.
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);

    // 4. Create encryption context.
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

    // 5. You may provide your own ECC keys in the files located at
    // - EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_SENDER
    // - EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_RECIPIENT

    // If not, the main method in this class will call
    // the KMS ECC key, retrieve its public key, and store it
    // in a PEM file for example use.
    if should_generate_new_kms_ecc_public_key(EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_SENDER)? {
        write_kms_ecdh_ecc_public_key(ecc_key_arn, EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_SENDER)
            .await?;
    }

    if should_generate_new_kms_ecc_public_key(EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)? {
        write_kms_ecdh_ecc_public_key(
            ecc_recipient_key_arn,
            EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_RECIPIENT,
        )
        .await?;
    }

    // 6. Load public key from UTF-8 encoded PEM files into a DER encoded public key.
    let public_key_file_content_sender =
        std::fs::read_to_string(Path::new(EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_SENDER))?;
    let parsed_public_key_file_content_sender = parse(public_key_file_content_sender)?;
    let public_key_sender_utf8_bytes = parsed_public_key_file_content_sender.contents();

    let public_key_file_content_recipient =
        std::fs::read_to_string(Path::new(EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_RECIPIENT))?;
    let parsed_public_key_file_content_recipient = parse(public_key_file_content_recipient)?;
    let public_key_recipient_utf8_bytes = parsed_public_key_file_content_recipient.contents();

    // 7. Create the KmsPrivateKeyToStaticPublicKeyInput
    let kms_ecdh_static_configuration_input = KmsPrivateKeyToStaticPublicKeyInput::builder()
        .sender_kms_identifier(ecc_key_arn)
        // Must be a UTF8 DER-encoded X.509 public key
        .sender_public_key(public_key_sender_utf8_bytes)
        // Must be a UTF8 DER-encoded X.509 public key
        .recipient_public_key(public_key_recipient_utf8_bytes)
        .build()?;

    let kms_ecdh_static_configuration = KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(
        kms_ecdh_static_configuration_input,
    );

    // 8. Create the KMS ECDH keyring.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    // Create a KMS ECDH keyring.
    // This keyring uses the KmsPrivateKeyToStaticPublicKey configuration. This configuration calls for both of
    // the keys to be on the same curve (P256, P384, P521).
    // On encrypt, the keyring calls AWS KMS to derive the shared secret from the sender's KMS ECC Key ARN and the recipient's public key.
    // For this example, on decrypt, the keyring calls AWS KMS to derive the shared secret from the sender's KMS ECC Key ARN and the recipient's public key;
    // however, on decrypt, the recipient can construct a keyring such that the shared secret is calculated with
    // the recipient's private key and the sender's public key. In both scenarios the shared secret will be the same.
    // For more information on this configuration see:
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-kms-ecdh-keyring.html#kms-ecdh-create
    // This keyring takes in:
    //  - kmsClient
    //  - kmsKeyId: Must be an ARN representing a KMS ECC key meant for KeyAgreement
    //  - curveSpec: The curve name where the public keys lie
    //  - senderPublicKey: A ByteBuffer of a UTF-8 encoded public
    //               key for the key passed into kmsKeyId in DER format
    //  - recipientPublicKey: A ByteBuffer of a UTF-8 encoded public
    //               key for the key passed into kmsKeyId in DER format
    let kms_ecdh_keyring = mpl
        .create_aws_kms_ecdh_keyring()
        .kms_client(kms_client)
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(kms_ecdh_static_configuration)
        .send()
        .await?;

    // 9. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();

    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(kms_ecdh_keyring.clone())
        .encryption_context(&encryption_context)
        .build()?;
    let encryption_response = esdk_client.encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 10. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 11. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(kms_ecdh_keyring)
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(&encryption_context)
        .build()?;
    let decryption_response = esdk_client.decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 12. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("KMS ECDH Keyring Example Completed Successfully");

    Ok(())
}

fn should_generate_new_kms_ecc_public_key(ecc_public_key_filename: &str) -> Result<bool, String> {
    // If key already exists: do not overwrite existing key
    if exists(ecc_public_key_filename) {
        Ok(false)
    }
    // If file is not present, generate a new key pair
    else {
        Ok(true)
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the KMS ECDH Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        utils::TEST_KMS_ECDH_KEY_ID_P256_SENDER,
        EcdhCurveSpec::EccNistP256,
        Some(utils::TEST_KMS_ECDH_KEY_ID_P256_RECIPIENT),
    )
    .await?;

    Ok(())
}

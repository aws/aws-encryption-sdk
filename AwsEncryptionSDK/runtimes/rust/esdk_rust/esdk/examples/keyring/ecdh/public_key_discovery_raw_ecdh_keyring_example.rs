// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Public Key Discovery Raw ECDH Keyring.

A public key discovery Raw ECDH Keyring takes in the recipient's private key located
at EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
as a UTF8 PEM-encoded (PKCS #8 PrivateKeyInfo structures) private key,
and the Curve Specification where the key lies.

If you provide the EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT, make sure to also
provide the recipient's public key located at EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
in the directory that you run this example. Even though the Public Key Discovery Raw ECDH keyring
uses the EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT to decrypt the data,
the EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT is needed to generate the ciphertext to decrypt.

This example loads ECC keys from PEM files and the ciphertext with paths defined in
 - EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
 - EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT

If you do not provide these files, running this example through this
class' main method will generate three files required for all raw ECDH examples
EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER, EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
and EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT for you.
In practice, users of this library should not generate new key pairs
like this, and should instead retrieve an existing key from a secure
key management system (e.g. an HSM).
You may also provide your own key pair by placing PEM files in the
directory where the example is run or modifying the paths in the code
below. These files must be valid PEM encodings of the key pair as UTF-8
encoded bytes. If you do provide your own key pair, or if a key pair
already exists, this class' main method will not generate a new key pair.

This examples creates a RawECDH keyring with the PublicKeyDiscovery key agreement scheme.
This scheme is only available on decrypt.

This example creates a Public Key Discovery Raw ECDH Keyring and takes in a ciphertext to decrypt it.
This example also includes some sanity checks for demonstration:
1. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-ecdh-keyring.html#raw-ecdh-PublicKeyDiscovery
*/

use crate::example_utils::utils::EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT;
use crate::example_utils::utils::EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT;
use crate::example_utils::utils::exists;
use crate::example_utils::utils::write_raw_ecdh_ecc_keys;
use aws_esdk::Client as EsdkClient;
use aws_esdk::*;
use aws_mpl_rs::aws_cryptography_primitives::types::EcdhCurveSpec;
use aws_mpl_rs::client as mpl_client;
use aws_mpl_rs::types::EphemeralPrivateKeyToStaticPublicKeyInput;
use aws_mpl_rs::types::PublicKeyDiscoveryInput;
use aws_mpl_rs::types::RawEcdhStaticConfigurations;
use aws_mpl_rs::types::material_providers_config::MaterialProvidersConfig;
use pem::parse;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub async fn decrypt_with_keyring(
    example_data: &str,
    ecdh_curve_spec: EcdhCurveSpec,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::default();
    let esdk_client = EsdkClient::from_conf(esdk_config)?;

    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

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

    // 3. You may provide your own ECC keys in the files located at
    // - EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT

    // If you do not provide these files, running this example through this
    // class' main method will generate three files required for all raw ECDH examples
    // EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER, EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
    // and EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT for you.

    // Do not use these files for any other purpose.
    if should_generate_new_ecc_key_pair_discovery_raw_ecdh()? {
        write_raw_ecdh_ecc_keys(ecdh_curve_spec)?;
    }

    // 4. Load keys from UTF-8 encoded PEM files.
    let mut file = File::open(Path::new(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT))?;
    let mut private_key_recipient_utf8_bytes = Vec::new();
    file.read_to_end(&mut private_key_recipient_utf8_bytes)?;

    // Generate the ciphertext
    let ciphertext = get_ciphertext(
        example_data,
        ecdh_curve_spec,
        &encryption_context,
        &esdk_client,
        &mpl,
    )
    .await?;

    // 5. Create the PublicKeyDiscoveryInput
    let discovery_raw_ecdh_static_configuration_input = PublicKeyDiscoveryInput::builder()
        // Must be a UTF8 PEM-encoded private key
        .recipient_static_private_key(private_key_recipient_utf8_bytes)
        .build()?;

    let discovery_raw_ecdh_static_configuration = RawEcdhStaticConfigurations::PublicKeyDiscovery(
        discovery_raw_ecdh_static_configuration_input,
    );

    // 6. Create the Public Key Discovery Raw ECDH keyring.

    // Create the keyring.
    // This keyring uses a discovery configuration. This configuration will check on decrypt
    // if it is meant to decrypt the message by checking if the configured public key is stored on the message.
    // The discovery configuration can only decrypt messages and CANNOT encrypt messages.
    let discovery_raw_ecdh_keyring = mpl
        .create_raw_ecdh_keyring()
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(discovery_raw_ecdh_static_configuration)
        .send()
        .await?;

    // 7. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(discovery_raw_ecdh_keyring)
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(&encryption_context)
        .build()?;
    let decryption_response = esdk_client.decrypt(&decrypt_input).await?;

    let decrypted_plaintext = decryption_response.plaintext;

    // 8. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    let plaintext = example_data.as_bytes();

    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("Public Key Discovery Raw ECDH Keyring Example Completed Successfully");

    Ok(())
}

fn should_generate_new_ecc_key_pair_discovery_raw_ecdh() -> Result<bool, String> {
    // If keys already exist: do not overwrite existing keys
    if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    {
        Ok(false)
    }
    // If only one file is present: throw exception
    else if !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    {
        Err("Missing key file at ".to_string() + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
    } else if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && !exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    {
        Err("Missing key file at ".to_string() + EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    }
    // If neither file is present, generate a new key pair
    else {
        Ok(true)
    }
}

async fn get_ciphertext(
    example_data: &str,
    ecdh_curve_spec: EcdhCurveSpec,
    encryption_context: &EncryptionContext,
    esdk_client: &EsdkClient,
    mpl: &mpl_client::Client,
) -> Result<Vec<u8>, crate::BoxError> {
    // 1. Load keys from UTF-8 encoded PEM files.

    // Load public key from UTF-8 encoded PEM files into a DER encoded public key.
    let public_key_file_content =
        std::fs::read_to_string(Path::new(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT))?;
    let parsed_public_key_file_content = parse(public_key_file_content)?;
    let public_key_recipient_utf8_bytes = parsed_public_key_file_content.contents();

    // 2. Create the EphemeralPrivateKeyToStaticPublicKeyInput to generate the ciphertext
    let ephemeral_raw_ecdh_static_configuration_input =
        EphemeralPrivateKeyToStaticPublicKeyInput::builder()
            // Must be a UTF8 DER-encoded X.509 public key
            .recipient_public_key(public_key_recipient_utf8_bytes)
            .build()?;

    let ephemeral_raw_ecdh_static_configuration =
        RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(
            ephemeral_raw_ecdh_static_configuration_input,
        );

    // 3. Create the Ephemeral Raw ECDH keyring.

    // Create the keyring.
    // This keyring uses an ephemeral configuration. This configuration will always create a new
    // key pair as the sender key pair for the key agreement operation. The ephemeral configuration can only
    // encrypt data and CANNOT decrypt messages.
    let ephemeral_raw_ecdh_keyring = mpl
        .create_raw_ecdh_keyring()
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(ephemeral_raw_ecdh_static_configuration)
        .send()
        .await?;

    // 4. Encrypt the data with the encryption_context

    // A raw ecdh keyring with Ephemeral configuration cannot decrypt data since the key pair
    // used as the sender is ephemeral. This means that at decrypt time it does not have
    // the private key that corresponds to the public key that is stored on the message.
    let plaintext = example_data.as_bytes();

    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(ephemeral_raw_ecdh_keyring)
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
    // Test function for decrypt using the Public Key Discovery Raw ECDH Keyring example
    use crate::example_utils::utils;

    decrypt_with_keyring(utils::TEST_EXAMPLE_DATA, EcdhCurveSpec::EccNistP256).await?;

    Ok(())
}

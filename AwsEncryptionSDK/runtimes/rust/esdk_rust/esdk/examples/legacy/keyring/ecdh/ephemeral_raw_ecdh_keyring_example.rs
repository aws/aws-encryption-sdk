// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Ephemeral Raw ECDH Keyring.

This example takes in the recipient's public key located at
EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT as a
UTF8 PEM-encoded X.509 public key,
and the Curve Specification where the key lies.

This example loads ECC keys from PEM files with paths defined in
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

This examples creates a RawECDH keyring with the EphemeralPrivateKeyToStaticPublicKey key agreement scheme.
This configuration will always create a new key pair as the sender key pair for the key agreement operation.
The ephemeral configuration can only encrypt data and CANNOT decrypt messages.

This example creates an Ephemeral Raw ECDH Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-ecdh-keyring.html#raw-ecdh-EphemeralPrivateKeyToStaticPublicKey
*/

use crate::legacy::example_utils::utils::EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT;
use crate::legacy::example_utils::utils::exists;
use crate::legacy::example_utils::utils::write_raw_ecdh_ecc_keys;
use aws_esdk::*;
use aws_mpl_legacy::dafny::aws_cryptography_primitives::types::EcdhCurveSpec;
use aws_mpl_legacy::dafny::types::EphemeralPrivateKeyToStaticPublicKeyInput;
use aws_mpl_legacy::dafny::types::RawEcdhStaticConfigurations;
use pem::parse;
use std::path::Path;

pub async fn encrypt_with_legacy_keyring(
    example_data: &str,
    ecdh_curve_spec: EcdhCurveSpec,
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

    // 3. You may provide your own ECC keys in the files located at
    // - EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT

    // If you do not provide these files, running this example through this
    // class' main method will generate three files required for all raw ECDH examples
    // EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER, EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
    // and EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT for you.

    // Do not use these files for any other purpose.
    if should_generate_new_ecc_key_pair_ephemeral_raw_ecdh()? {
        write_raw_ecdh_ecc_keys(ecdh_curve_spec)?;
    }

    // 4. Load keys from UTF-8 encoded PEM files.

    // Load public key from UTF-8 encoded PEM files into a DER encoded public key.
    let public_key_file_content =
        std::fs::read_to_string(Path::new(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT))?;
    let parsed_public_key_file_content = parse(public_key_file_content)?;
    let public_key_recipient_utf8_bytes = parsed_public_key_file_content.contents();

    // 5. Create the EphemeralPrivateKeyToStaticPublicKeyInput
    let ephemeral_raw_ecdh_static_configuration_input =
        EphemeralPrivateKeyToStaticPublicKeyInput::builder()
            // Must be a UTF8 DER-encoded X.509 public key
            .recipient_public_key(public_key_recipient_utf8_bytes)
            .build()?;

    let ephemeral_raw_ecdh_static_configuration =
        RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(
            ephemeral_raw_ecdh_static_configuration_input,
        );

    // 6. Create the Ephemeral Raw ECDH keyring.
    // Create the keyring.
    // This keyring uses an ephemeral configuration. This configuration will always create a new
    // key pair as the sender key pair for the key agreement operation. The ephemeral configuration can only
    // encrypt data and CANNOT decrypt messages.
    let ephemeral_raw_ecdh_keyring = mpl()
        .create_raw_ecdh_keyring()
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(ephemeral_raw_ecdh_static_configuration)
        .send()
        .await?;

    // 7. Encrypt the data with the encryption_context

    // A raw ecdh keyring with Ephemeral configuration cannot decrypt data since the key pair
    // used as the sender is ephemeral. This means that at decrypt time it does not have
    // the private key that corresponds to the public key that is stored on the message.
    let plaintext = example_data.as_bytes();
    let encrypt_input = EncryptInput::with_legacy_keyring(
        plaintext,
        encryption_context,
        ephemeral_raw_ecdh_keyring,
    );
    let encryption_response = encrypt(&encrypt_input).await?;
    let ciphertext = encryption_response.ciphertext;

    // 8. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    println!("Ephemeral Raw ECDH Keyring Example Completed Successfully");

    Ok(())
}

fn should_generate_new_ecc_key_pair_ephemeral_raw_ecdh() -> Result<bool, String> {
    // If key already exists: do not overwrite existing key
    if exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT) {
        Ok(false)
    }
    // If file is not present, generate a new key pair
    else {
        Ok(true)
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_with_legacy_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt using the Ephemeral Raw ECDH Keyring example
    use crate::example_utils::utils;

    encrypt_with_legacy_keyring(utils::TEST_EXAMPLE_DATA, EcdhCurveSpec::EccNistP256).await?;

    Ok(())
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Raw ECDH Keyring.

This example takes in the sender's private key located at
EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER as a UTF8 PEM-encoded
(PKCS #8 PrivateKeyInfo structures) private key,
and the recipient's public key located at
EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT as a
UTF8 PEM-encoded X.509 public key,
also known as SubjectPublicKeyInfo (SPKI),
and the Curve Specification where the keys lie.

This example loads ECC keys from PEM files with paths defined in
 - EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER
 - EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT

If you do not provide these files, running this example through this
class' main method will generate three files required for all raw ECDH examples
EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER, EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
and EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT for you.
These files will be generated in the directory where the example is run.
In practice, users of this library should not generate new key pairs
like this, and should instead retrieve an existing key from a secure
key management system (e.g. an HSM).
You may also provide your own key pair by placing PEM files in the
directory where the example is run or modifying the paths in the code
below. These files must be valid PEM encodings of the key pair as UTF-8
encoded bytes. If you do provide your own key pair, or if a key pair
already exists, this class' main method will not generate a new key pair.

This example creates a RawECDH keyring with the RawPrivateKeyToStaticPublicKey key agreement scheme.
On encrypt, the shared secret is derived from the sender's private key and the recipient's public key.
On decrypt, the shared secret is derived from the sender's private key and the recipient's public key;
however, on decrypt the recipient can construct a keyring such that the shared secret is calculated with
the recipient's private key and the sender's public key. In both scenarios the shared secret will be the same.

This example creates a Raw ECDH Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on this configuration see:
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-ecdh-keyring.html#raw-ecdh-RawPrivateKeyToStaticPublicKey
*/

use crate::example_utils::utils::EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER;
use crate::example_utils::utils::EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT;
use crate::example_utils::utils::exists;
use crate::example_utils::utils::write_raw_ecdh_ecc_keys;
use aws_esdk::*;
use aws_mpl_legacy::aws_cryptography_primitives::types::EcdhCurveSpec;
use aws_mpl_legacy::types::RawEcdhStaticConfigurations;
use aws_mpl_legacy::types::RawPrivateKeyToStaticPublicKeyInput;
use pem::parse;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub async fn encrypt_and_decrypt_with_keyring(
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
    // - EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER
    // - EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT

    // If you do not provide these files, running this example through this
    // class' main method will generate three files required for all raw ECDH examples
    // EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER, EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
    // and EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT for you.

    // Do not use these files for any other purpose.
    if should_generate_new_ecc_key_pair_raw_ecdh()? {
        write_raw_ecdh_ecc_keys(ecdh_curve_spec)?;
    }

    // 4. Load keys from UTF-8 encoded PEM files.
    let mut file = File::open(Path::new(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER))?;
    let mut private_key_sender_utf8_bytes = Vec::new();
    file.read_to_end(&mut private_key_sender_utf8_bytes)?;

    // Load public key from UTF-8 encoded PEM files into a DER encoded public key.
    let public_key_file_content =
        std::fs::read_to_string(Path::new(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT))?;
    let parsed_public_key_file_content = parse(public_key_file_content)?;
    let public_key_recipient_utf8_bytes = parsed_public_key_file_content.contents();

    // 5. Create the RawPrivateKeyToStaticPublicKeyInput
    let raw_ecdh_static_configuration_input = RawPrivateKeyToStaticPublicKeyInput::builder()
        // Must be a UTF8 PEM-encoded private key
        .sender_static_private_key(private_key_sender_utf8_bytes)
        // Must be a UTF8 DER-encoded X.509 public key
        .recipient_public_key(public_key_recipient_utf8_bytes)
        .build()?;

    let raw_ecdh_static_configuration = RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(
        raw_ecdh_static_configuration_input,
    );

    // 6. Create the Raw ECDH keyring.

    // Create the keyring.
    // This keyring uses static sender and recipient keys. This configuration calls for both of
    // the keys to be on the same curve (P256 / P384 / P521).
    // On encrypt, the shared secret is derived from the sender's private key and the recipient's public key.
    // For this example, on decrypt, the shared secret is derived from the sender's private key and the recipient's public key;
    // However, on decrypt, the recipient can construct a keyring such that the shared secret is calculated with
    // the recipient's private key and the sender's public key. In both scenarios the shared secret will be the same.
    let raw_ecdh_keyring = mpl()
        .create_raw_ecdh_keyring()
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;

    // 7. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();
    let encrypt_input = EncryptInput::with_keyring(plaintext, encryption_context, raw_ecdh_keyring);
    let encryption_response = encrypt(&encrypt_input).await?;
    let ciphertext = encryption_response.ciphertext;

    // 8. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 9. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decrypt_input = DecryptInput::from_encrypt(&ciphertext, &encrypt_input);
    let decryption_response = decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 10. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("Raw ECDH Keyring Example Completed Successfully");

    Ok(())
}

fn should_generate_new_ecc_key_pair_raw_ecdh() -> Result<bool, String> {
    // If keys already exist: do not overwrite existing keys
    if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    {
        Ok(false)
    }
    // If only one file is present: throw exception
    else if !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    {
        Err("Missing key file at ".to_string() + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
    } else if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && !exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    {
        Err("Missing key file at ".to_string() + EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    }
    // If neither file is present, generate a new key pair
    else {
        Ok(true)
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Raw ECDH Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA, EcdhCurveSpec::EccNistP256).await?;

    Ok(())
}

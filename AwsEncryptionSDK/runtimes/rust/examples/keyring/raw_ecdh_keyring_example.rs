// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Raw ECDH Keyring

This keyring takes in the sender's ECC private key and the
recipient's ECC Public Key to derive a shared secret.
The keyring uses the shared secret to derive a data key to
protect the data keys that encrypt and decrypt example_data.

This example takes in the sender's private key, the recipient's
public key, and the algorithm definition where the ECC keys lie.
This parameter takes in the sender's private key as a
UTF8 PEM-encoded (PKCS #8 PrivateKeyInfo structures), the recipient's
DER-encoded X.509 public key, also known as SubjectPublicKeyInfo (SPKI),
and the Curve Specification where the keys lie.

This example encrypts the example_data using the provided ECC keys.
Then, it gets the ciphertext and decrypts it.

This example loads ECC keys from PEM files with paths defined in
 - EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER
 - EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
 - EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
If you do not provide these files, running this example through this
class' main method will generate these files for you. These files will
be generated in the directory where the example is run.
In practice, users of this library should not generate new key pairs
like this, and should instead retrieve an existing key from a secure
key management system (e.g. an HSM).
You may also provide your own key pair by placing PEM files in the
directory where the example is run or modifying the paths in the code
below. These files must be valid PEM encodings of the key pair as UTF-8
encoded bytes. If you do provide your own key pair, or if a key pair
already exists, this class' main method will not generate a new key pair.

This example creates a Raw ECDH Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
These sanity checks are for demonstration in the example only. You do not need these in your code.

For more information on how to use Raw ECDH keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-ecdh-keyring.html
*/

use aws_esdk::client as esdk_client;
use aws_esdk::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig;
use aws_esdk::aws_cryptography_materialProviders::client as mpl_client;
use aws_esdk::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig;
use aws_esdk::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations;
use aws_esdk::aws_cryptography_materialProviders::types::keyring::KeyringRef;
use aws_esdk::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput;
use aws_esdk::aws_cryptography_primitives::types::EcdhCurveSpec;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

const EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER: &str = "RawEcdhKeyringExamplePrivateKeySender.pem";
const EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT: &str = "RawEcdhKeyringExamplePrivateKeyRecipient.pem";
const EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT: &str = "RawEcdhKeyringExamplePublicKeyRecipient.pem";

pub async fn encrypt_and_decrypt_with_keyring(
    example_data: &str,
    ecdh_curve_spec: EcdhCurveSpec,
) -> Result<(), crate::BoxError> {
    // 1. You may provide your own ECC keys in the files located at
    // - EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER
    // - EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
    // If these files are not present, this will generate a pair for you
    if should_generate_new_ecc_key_pair()? {
        generate_ecc_key_pair(ecdh_curve_spec)?;
    }

    // 2. Load keys from UTF-8 encoded PEM files.
    //    You may provide your own PEM files to use here.
    //    If you do not, the main method in this class will generate PEM
    //    files for example use. Do not use these files for any other purpose.
    let mut file = File::open(Path::new(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER))?;
    let mut private_key_sender_utf8_bytes = Vec::new();
    file.read_to_end(&mut private_key_sender_utf8_bytes)?;

    let mut file = File::open(Path::new(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT))?;
    let mut public_key_recipient_utf8_bytes = Vec::new();
    file.read_to_end(&mut public_key_recipient_utf8_bytes)?;

    // 3. Create the RawPrivateKeyToStaticPublicKeyInput
    let raw_ecdh_static_configuration_input =
        RawPrivateKeyToStaticPublicKeyInput::builder()
            // Must be a UTF8 PEM-encoded private key
            .sender_static_private_key(private_key_sender_utf8_bytes)
            // Must be a DER-encoded X.509 public key
            .recipient_public_key(public_key_recipient_utf8_bytes)
            .build()?;

    let raw_ecdh_static_configuration = RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(raw_ecdh_static_configuration_input);

    // 4. Create the Raw ECDH keyring.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    let raw_ecdh_keyring = mpl
        .create_raw_ecdh_keyring()
        .curve_spec(ecdh_curve_spec)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;

    // 5. Encrypt and decrypt roundtrip using the raw_ecdh_keyring
    encrypt_decrypt_roundtrip(example_data, raw_ecdh_keyring).await?;

    println!("Raw ECDH Keyring Example Completed Successfully");

    Ok(())
}

async fn encrypt_decrypt_roundtrip(
    example_data: &str,
    raw_ecdh_keyring: KeyringRef,
) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::builder().build()?;
    let esdk_client = esdk_client::Client::from_conf(esdk_config)?;

    // 2. Create encryption context.
    // Remember that your encryption context is NOT SECRET.
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
    let encryption_context = HashMap::from([
        ("encryption".to_string(), "context".to_string()),
        ("is not".to_string(), "secret".to_string()),
        ("but adds".to_string(), "useful metadata".to_string()),
        ("that can help you".to_string(), "be confident that".to_string()),
        ("the data you are handling".to_string(), "is what you think it is".to_string()),
        ]);

    // 3. Encrypt the data with the encryptionContext
    let plaintext = example_data.as_bytes();

    let encryption_response = esdk_client.encrypt()
        .plaintext(plaintext)
        .keyring(raw_ecdh_keyring.clone())
        .encryption_context(encryption_context.clone())
        .send()
        .await?;

    let ciphertext = encryption_response
                        .ciphertext
                        .expect("Unable to unwrap ciphertext from encryption response");

    // 4. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(ciphertext, aws_smithy_types::Blob::new(plaintext),
        "Ciphertext and plaintext data are the same. Invalid encryption");

    // 5. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decryption_response = esdk_client.decrypt()
        .ciphertext(ciphertext)
        .keyring(raw_ecdh_keyring)
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(encryption_context)
        .send()
        .await?;

    let decrypted_plaintext = decryption_response
                                .plaintext
                                .expect("Unable to unwrap plaintext from decryption response");

    // 6. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(decrypted_plaintext, aws_smithy_types::Blob::new(plaintext),
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption");
    
    Ok(())
}

fn exists(f: &str) -> bool {
    Path::new(f).exists()
}

fn should_generate_new_ecc_key_pair() -> Result<bool, String> {
    // If keys already exist: do not overwrite existing keys
    if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            Ok(false)
        }
    // If only one file is present: throw exception
    else if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && !exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            Err(
                "Missing key file at ".to_string()
                + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
                + " and "
                + EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
            )
        }
    else if !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && !exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            Err(
                "Missing key file at ".to_string()
                + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER
                + " and "
                + EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
            )
        }
    else if !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            Err(
                "Missing key file at ".to_string()
                + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER
                + " and "
                + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
            )
        }
    // If only two files are present: throw exception
    else if !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            Err(
                "Missing key file at ".to_string()
                + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER
            )
        }
    else if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && !exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            Err(
                "Missing key file at ".to_string()
                + EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
            )
        }
    else if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        && exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        && !exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            Err(
                "Missing key file at ".to_string()
                + EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
            )
        }
    // If neither file is present, generate a new key pair
    else {
        Ok(true)
    }
}

fn generate_ecc_key_pair(
    _ecdh_curve_spec: EcdhCurveSpec
) -> Result<(), crate::BoxError> {
    use aws_lc_rs::encoding::AsDer;
    use aws_lc_rs::encoding::EcPrivateKeyRfc5915Der;
    use aws_lc_rs::encoding::PublicKeyX509Der;
    use aws_lc_rs::agreement;

    // Safety check: Validate neither file is present
    if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        || exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        || exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
        {
            return Err(crate::BoxError(
                "generate_ecc_key_pair will not overwrite existing PEM files".to_string(),
            ));
        }

    // This code will generate new ECC keys for example use.
    // The public and private keys will be written to the files:
    //  - public: EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT
    //  - private: EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER, EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT
    // This example uses aws-lc-rs's KeyPairGenerator to generate the key pair.
    // In practice, you should not generate this in your code, and should instead
    // retrieve this key from a secure key management system (e.g. HSM)
    // These examples only demonstrate using the P256 curve while the keyring accepts
    // P256, P384, or P521.
    // This key is created here for example purposes only.

    // let private_key =
    //     agreement::PrivateKey::generate(super::ECCUtils::get_alg(ecdh_curve_spec))
    //         .map_err(|e| format!("{:?}", e))?;

    // let public_key = private_key
    //     .compute_public_key()
    //     .map_err(|e| format!("{:?}", e))?;

    // let public_key: Vec<u8> = super::ECCUtils::X962_to_X509(public_key.as_ref(), alg)?;

    // let private_key_der = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key)
    //     .map_err(|e| format!("{:?}", e))?;
    // let private_key = pem::Pem::new("PRIVATE KEY", private_key_der.as_ref());
    // let private_key = pem::encode(&private_key);
    // let private_key: Vec<u8> = private_key.into_bytes();


    let private_key_sender = agreement::PrivateKey::generate(&agreement::ECDH_P256)?;

    let private_key_sender = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key_sender)?;
    let private_key_sender = pem::Pem::new("ECDH PRIVATE KEY SENDER", private_key_sender.as_ref());
    let private_key_sender = pem::encode(&private_key_sender);
    
    let private_key_recipient = agreement::PrivateKey::generate(&agreement::ECDH_P256)?;
    // Make `public_key_recipient` a byte slice containing private_key_recipient. In a real
    // application, this would be sent to the peer in an encoded protocol
    // message.
    let public_key_recipient = private_key_recipient.compute_public_key()?;

    let private_key_recipient = AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key_recipient)?;
    let private_key_recipient = pem::Pem::new("ECDH PRIVATE KEY RECIPIENT", private_key_recipient.as_ref());
    let private_key_recipient = pem::encode(&private_key_recipient);

    let public_key_recipient = AsDer::<PublicKeyX509Der>::as_der(&public_key_recipient)?;
    let public_key_recipient = pem::Pem::new("ECDH PUBLIC KEY RECIPIENT", public_key_recipient.as_ref());
    let public_key_recipient = pem::encode(&public_key_recipient);

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER))?
        .write_all(private_key_sender.as_bytes())?;

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT))?
        .write_all(private_key_recipient.as_bytes())?;

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT))?
        .write_all(public_key_recipient.as_bytes())?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Raw ECDH Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_keyring(
        utils::TEST_EXAMPLE_DATA,
        EcdhCurveSpec::EccNistP256
    ).await?;

    Ok(())
}

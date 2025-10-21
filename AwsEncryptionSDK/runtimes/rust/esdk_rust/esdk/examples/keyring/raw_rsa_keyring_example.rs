// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
This example sets up the Raw RSA Keyring

The Raw RSA keyring performs asymmetric encryption and decryption of data keys in local memory
with RSA public and private keys that you provide.

This keyring accepts PEM encodings of the key pair as UTF-8 interpreted bytes.
The encryption function encrypts the data key under the RSA public key. The decryption function
decrypts the data key using the private key.

This example loads a key pair from PEM files with paths defined in
 - EXAMPLE_RSA_PRIVATE_KEY_FILENAME
 - EXAMPLE_RSA_PUBLIC_KEY_FILENAME
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

This example creates a Raw RSA Keyring and then encrypts a custom input EXAMPLE_DATA
with an encryption context. This example also includes some sanity checks for demonstration:
1. Ciphertext and plaintext data are not the same
2. Decrypted plaintext value matches EXAMPLE_DATA
3. The original ciphertext is not decryptable using a keyring with a different RSA key pair
These sanity checks are for demonstration in the example only. You do not need these in your code.

A Raw RSA keyring that encrypts and decrypts must include an asymmetric public key and private
key pair. However, you can encrypt data with a Raw RSA keyring that has only a public key,
and you can decrypt data with a Raw RSA keyring that has only a private key. This example requires
the user to either provide both private and public keys, or not provide any keys and the example
generates both to test encryption and decryption. If you configure a Raw RSA keyring with a
public and private key, be sure that they are part of the same key pair. Some language
implementations of the AWS Encryption SDK will not construct a Raw RSA keyring with keys
from different pairs. Others rely on you to verify that your keys are from the same key pair.
You can include any Raw RSA keyring in a multi-keyring.

For more information on how to use Raw RSA keyrings, see
https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-rsa-keyring.html
*/

use aws_esdk::Client as EsdkClient;
use aws_esdk::*;
use aws_mpl_rs::client as mpl_client;
use aws_mpl_rs::types::PaddingScheme;
use aws_mpl_rs::types::material_providers_config::MaterialProvidersConfig;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

const EXAMPLE_RSA_PRIVATE_KEY_FILENAME: &str = "RawRsaKeyringExamplePrivateKey.pem";
const EXAMPLE_RSA_PUBLIC_KEY_FILENAME: &str = "RawRsaKeyringExamplePublicKey.pem";

pub async fn encrypt_and_decrypt_with_keyring(example_data: &str) -> Result<(), crate::BoxError> {
    // 1. Instantiate the encryption SDK client.
    // This builds the default client with the RequireEncryptRequireDecrypt commitment policy,
    // which enforces that this client only encrypts using committing algorithm suites and enforces
    // that this client will only decrypt encrypted messages that were created with a committing
    // algorithm suite.
    let esdk_config = AwsEncryptionSdkConfig::default();
    let esdk_client = EsdkClient::from_conf(esdk_config)?;

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

    // 3. You may provide your own RSA key pair in the files located at
    //  - EXAMPLE_RSA_PRIVATE_KEY_FILENAME
    //  - EXAMPLE_RSA_PUBLIC_KEY_FILENAME
    // If these files are not present, this will generate a pair for you
    if should_generate_new_rsa_key_pair()? {
        generate_rsa_key_pair()?;
    }

    // 4. Load key pair from UTF-8 encoded PEM files.
    //    You may provide your own PEM files to use here.
    //    If you do not, the main method in this class will generate PEM
    //    files for example use. Do not use these files for any other purpose.
    let mut file = File::open(Path::new(EXAMPLE_RSA_PUBLIC_KEY_FILENAME))?;
    let mut public_key_utf8_bytes = Vec::new();
    file.read_to_end(&mut public_key_utf8_bytes)?;

    let mut file = File::open(Path::new(EXAMPLE_RSA_PRIVATE_KEY_FILENAME))?;
    let mut private_key_utf8_bytes = Vec::new();
    file.read_to_end(&mut private_key_utf8_bytes)?;

    // 5. The key namespace and key name are defined by you.
    // and are used by the Raw RSA keyring
    // For more information, see
    // https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/use-raw-rsa-keyring.html
    let key_namespace: &str = "my-key-namespace";
    let key_name: &str = "my-rsa-key-name";

    // 6. Create the Raw RSA keyring.
    let mpl_config = MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::Client::from_conf(mpl_config)?;

    let raw_rsa_keyring = mpl
        .create_raw_rsa_keyring()
        .key_name(key_name)
        .key_namespace(key_namespace)
        .padding_scheme(PaddingScheme::OaepSha256Mgf1)
        .public_key(public_key_utf8_bytes)
        .private_key(private_key_utf8_bytes)
        .send()
        .await?;

    // 7. Encrypt the data with the encryption_context
    let plaintext = example_data.as_bytes();

    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .keyring(raw_rsa_keyring.clone())
        .encryption_context(&encryption_context)
        .build()?;
    let encryption_response = esdk_client.encrypt(&encrypt_input).await?;

    let ciphertext = encryption_response.ciphertext;

    // 8. Demonstrate that the ciphertext and plaintext are different.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext and plaintext data are the same. Invalid encryption"
    );

    // 9. Decrypt your encrypted data using the same keyring you used on encrypt.
    let decrypt_input = DecryptInputBuilder::default()
        .ciphertext(&ciphertext)
        .keyring(raw_rsa_keyring)
        // Provide the encryption context that was supplied to the encrypt method
        .encryption_context(&encryption_context)
        .build()?;
    let decryption_response = esdk_client.decrypt(&decrypt_input).await?;
    let decrypted_plaintext = decryption_response.plaintext;

    // 10. Demonstrate that the decrypted plaintext is identical to the original plaintext.
    // (This is an example for demonstration; you do not need to do this in your own code.)
    assert_eq!(
        decrypted_plaintext, plaintext,
        "Decrypted plaintext should be identical to the original plaintext. Invalid decryption"
    );

    println!("Raw RSA Keyring Example Completed Successfully");

    Ok(())
}

fn exists(f: &str) -> bool {
    Path::new(f).exists()
}

fn should_generate_new_rsa_key_pair() -> Result<bool, String> {
    // If a key pair already exists: do not overwrite existing key pair
    if exists(EXAMPLE_RSA_PRIVATE_KEY_FILENAME) && exists(EXAMPLE_RSA_PUBLIC_KEY_FILENAME) {
        Ok(false)
    }
    // If only one file is present: throw exception
    else if exists(EXAMPLE_RSA_PRIVATE_KEY_FILENAME) && !exists(EXAMPLE_RSA_PUBLIC_KEY_FILENAME) {
        Err("Missing public key file at ".to_string() + EXAMPLE_RSA_PUBLIC_KEY_FILENAME)
    } else if exists(EXAMPLE_RSA_PRIVATE_KEY_FILENAME) && !exists(EXAMPLE_RSA_PUBLIC_KEY_FILENAME) {
        Err("Missing private key file at ".to_string() + EXAMPLE_RSA_PRIVATE_KEY_FILENAME)
    }
    // If neither file is present, generate a new key pair
    else {
        Ok(true)
    }
}

fn generate_rsa_key_pair() -> Result<(), crate::BoxError> {
    use aws_lc_rs::encoding::AsDer;
    use aws_lc_rs::encoding::Pkcs8V1Der;
    use aws_lc_rs::encoding::PublicKeyX509Der;
    use aws_lc_rs::rsa::KeySize;
    use aws_lc_rs::rsa::PrivateDecryptingKey;

    // Safety check: Validate neither file is present
    if exists(EXAMPLE_RSA_PRIVATE_KEY_FILENAME) || exists(EXAMPLE_RSA_PUBLIC_KEY_FILENAME) {
        return Err(crate::BoxError(
            "generate_rsa_key_pair will not overwrite existing PEM files".to_string(),
        ));
    }

    // This code will generate a new RSA key pair for example use.
    // The public and private key will be written to the files:
    //  - public: EXAMPLE_RSA_PUBLIC_KEY_FILENAME
    //  - private: EXAMPLE_RSA_PRIVATE_KEY_FILENAME
    // This example uses aws-lc-rs's KeyPairGenerator to generate the key pair.
    // In practice, you should not generate this in your code, and should instead
    // retrieve this key from a secure key management system (e.g. HSM)
    // This key is created here for example purposes only.

    let private_key = PrivateDecryptingKey::generate(KeySize::Rsa2048)?;
    let public_key = private_key.public_key();

    let public_key = AsDer::<PublicKeyX509Der>::as_der(&public_key)?;
    let public_key = pem::Pem::new("RSA PUBLIC KEY", public_key.as_ref());
    let public_key = pem::encode(&public_key);

    let private_key = AsDer::<Pkcs8V1Der>::as_der(&private_key)?;
    let private_key = pem::Pem::new("RSA PRIVATE KEY", private_key.as_ref());
    let private_key = pem::encode(&private_key);

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_RSA_PRIVATE_KEY_FILENAME))?
        .write_all(private_key.as_bytes())?;

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(EXAMPLE_RSA_PUBLIC_KEY_FILENAME))?
        .write_all(public_key.as_bytes())?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_encrypt_and_decrypt_with_keyring() -> Result<(), crate::BoxError2> {
    // Test function for encrypt and decrypt using the Raw RSA Keyring example
    use crate::example_utils::utils;

    encrypt_and_decrypt_with_keyring(utils::TEST_EXAMPLE_DATA).await?;

    Ok(())
}

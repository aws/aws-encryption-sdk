// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

pub const TEST_EXAMPLE_DATA: &str = "Hello World!";

pub const TEST_DEFAULT_KMS_KEY_ID: &str =
    "arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7f";

pub const TEST_DEFAULT_KMS_KEY_ACCOUNT_ID: &str = "658956600833";

pub const TEST_SECOND_REGION_KMS_KEY_ID: &str =
    "arn:aws:kms:eu-central-1:658956600833:key/75414c93-5285-4b57-99c9-30c1cf0a22c2";

pub const TEST_KMS_RSA_KEY_ID: &str =
    "arn:aws:kms:us-west-2:370957321024:key/mrk-63d386cb70614ea59b32ad65c9315297";

pub const TEST_KMS_RSA_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----\n\
                        MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA27Uc/fBaMVhxCE/SpCMQ\
                        oSBRSzQJw+o2hBaA+FiPGtiJ/aPy7sn18aCkelaSj4kwoC79b/arNHlkjc7OJFsN\
                        /GoFKgNvaiY4lOeJqEiWQGSSgHtsJLdbO2u4OOSxh8qIRAMKbMgQDVX4FR/PLKeK\
                        fc2aCDvcNSpAM++8NlNmv7+xQBJydr5ce91eISbHkFRkK3/bAM+1iddupoRw4Wo2\
                        r3avzrg5xBHmzR7u1FTab22Op3Hgb2dBLZH43wNKAceVwKqKA8UNAxashFON7xK9\
                        yy4kfOL0Z/nhxRKe4jRZ/5v508qIzgzCksYy7Y3QbMejAtiYnr7s5/d5KWw0swou\
                        twIDAQAB\n\
                        -----END PUBLIC KEY-----";

pub const TEST_MRK_KEY_ID_US_EAST_1: &str =
    "arn:aws:kms:us-east-1:658956600833:key/mrk-80bd8ecdcd4342aebd84b7dc9da498a7";

pub const TEST_MRK_KEY_ID_EU_WEST_1: &str =
    "arn:aws:kms:eu-west-1:658956600833:key/mrk-80bd8ecdcd4342aebd84b7dc9da498a7";

pub const TEST_KEY_STORE_NAME: &str = "KeyStoreDdbTable";

pub const TEST_LOGICAL_KEY_STORE_NAME: &str = "KeyStoreDdbTable";

pub const TEST_KEY_STORE_KMS_KEY_ID: &str =
    "arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126";

// ECDH Utils
use aws_mpl_legacy::dafny::aws_cryptography_primitives::types::EcdhCurveSpec;
use std::io::Write;
use std::path::Path;

pub const TEST_KMS_ECDH_KEY_ID_P256_SENDER: &str =
    "arn:aws:kms:us-west-2:370957321024:key/eabdf483-6be2-4d2d-8ee4-8c2583d416e9";
pub const TEST_KMS_ECDH_KEY_ID_P256_RECIPIENT: &str =
    "arn:aws:kms:us-west-2:370957321024:key/0265c8e9-5b6a-4055-8f70-63719e09fda5";

pub const EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER: &str =
    "RawEcdhKeyringExamplePrivateKeySender.pem";
pub const EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT: &str =
    "RawEcdhKeyringExamplePrivateKeyRecipient.pem";
pub const EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT: &str =
    "RawEcdhKeyringExamplePublicKeyRecipient.pem";
pub const EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_SENDER: &str =
    "KmsEccKeyringExamplePublicKeySender.pem";
pub const EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_RECIPIENT: &str =
    "KmsEccKeyringExamplePublicKeyRecipient.pem";

// Following are the helper functions for running ECDH examples
fn get_alg(x: EcdhCurveSpec) -> &'static aws_lc_rs::agreement::Algorithm {
    match x {
        EcdhCurveSpec::EccNistP256 => &aws_lc_rs::agreement::ECDH_P256,
        EcdhCurveSpec::EccNistP384 => &aws_lc_rs::agreement::ECDH_P384,
        EcdhCurveSpec::EccNistP521 => &aws_lc_rs::agreement::ECDH_P521,
        EcdhCurveSpec::Sm2 => panic!("No SM2 in Rust"),
    }
}

pub(crate) fn exists(f: &str) -> bool {
    Path::new(f).exists()
}

pub(crate) fn write_raw_ecdh_ecc_keys(
    ecdh_curve_spec: EcdhCurveSpec,
) -> Result<(), crate::BoxError> {
    // Safety check: Validate neither file is present
    if exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_SENDER)
        || exists(EXAMPLE_ECC_PRIVATE_KEY_FILENAME_RECIPIENT)
        || exists(EXAMPLE_ECC_PUBLIC_KEY_FILENAME_RECIPIENT)
    {
        return Err(crate::BoxError(
            "write_raw_ecdh_ecc_keys will not overwrite existing PEM files".to_string(),
        ));
    }

    let (_public_key_sender, private_key_sender) = generate_raw_ecc_key_pair(ecdh_curve_spec)?;
    let (public_key_recipient, private_key_recipient) = generate_raw_ecc_key_pair(ecdh_curve_spec)?;

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

fn generate_raw_ecc_key_pair(
    ecdh_curve_spec: EcdhCurveSpec,
) -> Result<(String, String), crate::BoxError> {
    use aws_lc_rs::encoding::AsDer;
    use aws_lc_rs::encoding::EcPrivateKeyRfc5915Der;
    use aws_lc_rs::encoding::PublicKeyX509Der;
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
    let private_key = aws_lc_rs::agreement::PrivateKey::generate(get_alg(ecdh_curve_spec))
        .map_err(|e| format!("{e:?}"))?;

    let public_key = private_key
        .compute_public_key()
        .map_err(|e| format!("{e:?}"))?;

    let public_key =
        AsDer::<PublicKeyX509Der>::as_der(&public_key).map_err(|e| format!("{e:?}"))?;
    let public_key = pem::Pem::new("PUBLIC KEY", public_key.as_ref());
    let public_key = pem::encode(&public_key);

    let private_key_der =
        AsDer::<EcPrivateKeyRfc5915Der>::as_der(&private_key).map_err(|e| format!("{e:?}"))?;
    let private_key = pem::Pem::new("PRIVATE KEY", private_key_der.as_ref());
    let private_key = pem::encode(&private_key);

    Ok((public_key, private_key))
}

pub(crate) async fn write_kms_ecdh_ecc_public_key(
    ecc_key_arn: &str,
    public_key_file_path: &str,
) -> Result<(), crate::BoxError> {
    if exists(public_key_file_path) {
        return Err(crate::BoxError(
            "write_kms_ecdh_ecc_public_key will not overwrite existing PEM files".to_string(),
        ));
    }

    let public_key = generate_kms_ecc_public_key(ecc_key_arn).await?;

    let public_key = pem::Pem::new("PUBLIC KEY", public_key);
    let public_key = pem::encode(&public_key);

    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(public_key_file_path))?
        .write_all(public_key.as_bytes())?;

    Ok(())
}

pub(crate) async fn generate_kms_ecc_public_key(
    ecc_key_arn: &str,
) -> Result<aws_smithy_types::Blob, crate::BoxError> {
    // Create KMS client to get public key
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let kms_client = aws_sdk_kms::Client::new(&sdk_config);

    // This code will call KMS to get the public key for the KMS ECC key.
    // You must have kms:GetPublicKey permissions on the key for this to succeed.
    // The public key generated here will be written to the file EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_SENDER
    // or EXAMPLE_KMS_ECC_PUBLIC_KEY_FILENAME_RECIPIENT.
    let kms_response = kms_client
        .get_public_key()
        .key_id(ecc_key_arn)
        .send()
        .await?;

    let public_key = kms_response
        .public_key
        .expect("Error unwrapping public key from KMS response.");

    Ok(public_key)
}

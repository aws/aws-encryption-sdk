#![allow(clippy::if_same_then_else)]

use crate::test_vectors::types::*;
use anyhow::Result;
use aws_config::Region;
use crate::test_vectors::parse_keys::decode_base64;
use crate::{DecryptInput, decrypt, mpl};
use aws_mpl_legacy::aws_cryptography_primitives::types::EcdhCurveSpec;
use aws_mpl_legacy::client::Client as mpl_client;
use aws_mpl_legacy::types::DiscoveryFilter;
use aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef as CmmRef;
use aws_mpl_legacy::types::keyring::KeyringRef;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

type KmsMap = HashMap<String, aws_sdk_kms::Client>;
const DFLT_REGION: &str = "us-west-2";
pub(crate) async fn make_kms_map() -> KmsMap {
    let mut kms_map = KmsMap::new();
    for region in [DFLT_REGION, "us-east-1"] {
        let client = create_kms_client(region).await;
        kms_map.insert(region.to_string(), client);
    }
    kms_map
}

pub(crate) async fn create_kms_client(region: &str) -> aws_sdk_kms::Client {
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let sdk_config = sdk_config
        .to_builder()
        .region(Region::new(region.to_string()))
        .build();
    aws_sdk_kms::Client::new(&sdk_config)
}

// arn:aws:kms:us-west-2:658956600833:key/mrk-80bd8ecdcd4342aebd84b7dc9da498a7
pub(crate) fn kms_from_arn<'a>(kms: &'a KmsMap, arn: &str) -> &'a aws_sdk_kms::Client {
    let region = region_from_arn(arn);
    &kms[region]
}
pub(crate) fn region_from_arn(arn: &str) -> &str {
    arn.split(':').nth(3).unwrap()
}

pub(crate) fn trim_filename(name: &str) -> &str {
    if name.len() > 7 && name[..7] == "file://"[..] {
        &name[7..]
    } else {
        name
    }
}

pub(crate) fn read_file(name: &str, dir: &str) -> Result<Vec<u8>> {
    let name = trim_filename(name);
    let result = std::fs::read(format!("{dir}/{name}"))?;
    Ok(result)
}

pub(crate) fn read_json(name: &str, dir: &str) -> Result<JsonValue> {
    let name = trim_filename(name);
    let result = std::fs::read_to_string(format!("{dir}/{name}"))?;
    let result: JsonValue = serde_json::from_str(&result)?;
    Ok(result)
}

pub(crate) fn write_json(data: &JsonValue, name: &str) -> Result<()> {
    let text = serde_json::to_string(data)?;
    std::fs::write(name, text.as_bytes())?;
    Ok(())
}

pub(crate) async fn run_decrypt_test(
    test: &EncryptTest,
    cmm: CmmRef,
    dir: &str,
) -> Result<()> {
    let ciphertext = read_file(&test.ciphertext, dir)?;
    let plaintext = read_file(&test.result, dir)?;

    let decrypt_input = DecryptInput {
        ciphertext :&ciphertext,
        materials_manager: Some(cmm),
        encryption_context: test.reproduced_encryption_context.clone(),
        commitment_policy : aws_mpl_rs::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        ..Default::default()
    };
    let decrypt_output = decrypt(&decrypt_input).await?;

    if decrypt_output.plaintext.as_ref() != plaintext {
        anyhow::bail!("Decrypted ciphertext did not match expected plaintext.");
    }

    Ok(())
}

fn get_curve(s: &str) -> Result<EcdhCurveSpec> {
    match s {
        "ecc-256" => Ok(EcdhCurveSpec::EccNistP256),
        "ecc-384" => Ok(EcdhCurveSpec::EccNistP384),
        "ecc-521" => Ok(EcdhCurveSpec::EccNistP521),
        "sm2" => Ok(EcdhCurveSpec::Sm2),
        _ => anyhow::bail!("Unknown curve: {s}"),
    }
}
pub(crate) async fn get_raw_ecdh_keyring_static(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<KeyringRef> {
    let key = &keys[&keydesc.sender];
    let pub_key = decode_base64(&key.recipient_material_public_key)?;
    let raw_ecdh_static_configuration_input =
        aws_mpl_legacy::types::RawPrivateKeyToStaticPublicKeyInput::builder()
            .sender_static_private_key(key.sender_material.as_bytes())
            .recipient_public_key(pub_key)
            .build()?;

    let raw_ecdh_static_configuration =
        aws_mpl_legacy::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(
            raw_ecdh_static_configuration_input,
        );

    let raw_ecdh_keyring = mpl
        .create_raw_ecdh_keyring()
        .curve_spec(get_curve(&keydesc.ecc_curve)?)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;
    Ok(raw_ecdh_keyring)
}
pub(crate) async fn get_raw_ecdh_keyring_ephemeral(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<KeyringRef> {
    let key = &keys[&keydesc.recipient];
    let pub_key = decode_base64(&key.recipient_material_public_key)?;
    let raw_ecdh_static_configuration_input =
        aws_mpl_legacy::types::EphemeralPrivateKeyToStaticPublicKeyInput::builder()
            .recipient_public_key(pub_key)
            .build()?;

    let raw_ecdh_static_configuration =
        aws_mpl_legacy::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(
            raw_ecdh_static_configuration_input,
        );

    let raw_ecdh_keyring = mpl
        .create_raw_ecdh_keyring()
        .curve_spec(get_curve(&keydesc.ecc_curve)?)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;

    Ok(raw_ecdh_keyring)
}
pub(crate) async fn get_raw_ecdh_keyring_discovery(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<KeyringRef> {
    let key = &keys[&keydesc.recipient];
    let raw_ecdh_static_configuration_input = aws_mpl_legacy::types::PublicKeyDiscoveryInput::builder()
        .recipient_static_private_key(key.recipient_material.as_bytes())
        .build()?;

    let raw_ecdh_static_configuration =
        aws_mpl_legacy::types::RawEcdhStaticConfigurations::PublicKeyDiscovery(
            raw_ecdh_static_configuration_input,
        );

    let raw_ecdh_keyring = mpl
        .create_raw_ecdh_keyring()
        .curve_spec(get_curve(&keydesc.ecc_curve)?)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;

    Ok(raw_ecdh_keyring)
}

pub(crate) async fn get_raw_ecdh_keyring(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<KeyringRef> {
    match &keydesc.schema[..] {
        "static" => get_raw_ecdh_keyring_static(keydesc, keys, mpl).await,
        "ephemeral" => get_raw_ecdh_keyring_ephemeral(keydesc, keys, mpl).await,
        "discovery" => get_raw_ecdh_keyring_discovery(keydesc, keys, mpl).await,
        _ => anyhow::bail!("Unknown ecdh schema: {}", keydesc.schema),
    }
}

fn get_kms_enc_alg(s: &str) -> aws_sdk_kms::types::EncryptionAlgorithmSpec {
    match s {
        "RSAES_OAEP_SHA_256" => aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha256,
        "RSAES_OAEP_SHA_1" => aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha1,
        _ => panic!("Unknown KMS encryption algorithm: {s}"),
    }
}
pub(crate) async fn get_aws_kms_rsa_keyring(
    keydesc: &KeyDescription,
    key: &Key,
    mpl: &mpl_client,
    kms: &aws_sdk_kms::Client,
) -> Result<KeyringRef> {
    let keyring = mpl
        .create_aws_kms_rsa_keyring()
        .kms_key_id(key.key_id.clone())
        .public_key(key.material.clone())
        .encryption_algorithm(get_kms_enc_alg(&keydesc.encryption_algorithm))
        .kms_client(kms.clone())
        .send()
        .await?;
    Ok(keyring)
}

pub(crate) async fn get_aws_kms_keyring(
    key: &Key,
    mpl: &mpl_client,
    kms: &aws_sdk_kms::Client,
) -> Result<KeyringRef> {
    let keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(key.key_id.clone())
        .kms_client(kms.clone())
        .send()
        .await?;
    Ok(keyring)
}

pub(crate) async fn get_aws_kms_mrk_keyring(
    key: &Key,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<KeyringRef> {
    let kms = kms_from_arn(kms, &key.key_id);
    let keyring = mpl
        .create_aws_kms_mrk_keyring()
        .kms_key_id(key.key_id.clone())
        .kms_client(kms.clone())
        .send()
        .await?;
    Ok(keyring)
}

pub(crate) async fn get_aws_kms_mrk_discovery_keyring(
    keydesc: &KeyDescription,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<KeyringRef> {
    if keydesc.discovery_filter.partition.is_empty() {
        let keyring: KeyringRef = mpl
            .create_aws_kms_mrk_discovery_keyring()
            .kms_client(kms[&keydesc.default_mrk_region].clone())
            .region(keydesc.default_mrk_region.clone())
            .send()
            .await?;
        Ok(keyring)
    } else {
        let filter = DiscoveryFilter::builder()
            .partition(keydesc.discovery_filter.partition.clone())
            .account_ids(keydesc.discovery_filter.account_ids.clone())
            .build()?;

        let keyring: KeyringRef = mpl
            .create_aws_kms_mrk_discovery_keyring()
            .kms_client(kms[&keydesc.default_mrk_region].clone())
            .region(keydesc.default_mrk_region.clone())
            .discovery_filter(filter)
            .send()
            .await?;
        Ok(keyring)
    }
}

fn get_aes_alg(len: usize) -> Result<aws_mpl_legacy::types::AesWrappingAlg> {
    match len {
        16 => Ok(aws_mpl_legacy::types::AesWrappingAlg::AlgAes128GcmIv12Tag16),
        24 => Ok(aws_mpl_legacy::types::AesWrappingAlg::AlgAes192GcmIv12Tag16),
        32 => Ok(aws_mpl_legacy::types::AesWrappingAlg::AlgAes256GcmIv12Tag16),
        _ => anyhow::bail!("Unknown aes key length: {len}"),
    }
}

async fn get_raw_keyring(
    keydesc: &KeyDescription,
    key: &Key,
    mpl: &mpl_client,
) -> Result<KeyringRef> {
    let hash = &keydesc.padding_hash;
    let e_alg = &keydesc.encryption_algorithm;
    let p_alg = &keydesc.padding_algorithm;
    let is_aes = e_alg == "aes";
    let is_rsa = e_alg == "rsa";
    let key_namespace = &keydesc.provider_id;
    let key_name = &key.key_id;
    if is_aes {
        let keyring: KeyringRef = mpl
            .create_raw_aes_keyring()
            .key_namespace(key_namespace.clone())
            .key_name(key_name.clone())
            .wrapping_key(key.material.clone())
            .wrapping_alg(get_aes_alg(key.material.len())?)
            .send()
            .await?;
        Ok(keyring)
    } else if is_rsa {
        let mode: aws_mpl_legacy::types::PaddingScheme;
        if hash == "sha1" && p_alg == "pkcs1" {
            mode = aws_mpl_legacy::types::PaddingScheme::Pkcs1;
        } else if hash == "sha1" && p_alg == "oaep-mgf1" {
            mode = aws_mpl_legacy::types::PaddingScheme::OaepSha1Mgf1;
        } else if hash == "sha256" && p_alg == "oaep-mgf1" {
            mode = aws_mpl_legacy::types::PaddingScheme::OaepSha256Mgf1;
        } else if hash == "sha384" && p_alg == "oaep-mgf1" {
            mode = aws_mpl_legacy::types::PaddingScheme::OaepSha384Mgf1;
        } else if hash == "sha512" && p_alg == "oaep-mgf1" {
            mode = aws_mpl_legacy::types::PaddingScheme::OaepSha512Mgf1;
        } else {
            anyhow::bail!("Unknown rsa padding combo : {hash} {p_alg}");
        }
        let mut keyring_builder = mpl
            .create_raw_rsa_keyring()
            .key_namespace(key_namespace.clone())
            .key_name(key_name.clone())
            .padding_scheme(mode);

        if key.material[..21] == b"-----BEGIN PUBLIC KEY"[..] {
            keyring_builder = keyring_builder.public_key(key.material.clone());
        } else {
            keyring_builder = keyring_builder.private_key(key.material.clone());
        }
        let keyring = keyring_builder.send().await?;
        Ok(keyring)
    } else {
        anyhow::bail!("Invalid raw type : {e_alg}");
    }
}

pub(crate) async fn get_cmm(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<CmmRef> {
    if keydesc.kind == "required-encryption-context-cmm" {
        let keyring = get_keyring(&keydesc.underlying[0], keys, mpl, kms).await?;
        let under_cmm = mpl
            .create_default_cryptographic_materials_manager()
            .keyring(keyring.clone())
            .send()
            .await?;

        let cmm = mpl
            .create_required_encryption_context_cmm()
            .underlying_cmm(under_cmm)
            .required_encryption_context_keys(keydesc.required_encryption_context_keys.clone())
            .send()
            .await?;

        Ok(cmm)
    } else {
        let keyring = get_keyring(keydesc, keys, mpl, kms).await?;
        let cmm = mpl
            .create_default_cryptographic_materials_manager()
            .keyring(keyring.clone())
            .send()
            .await?;
        Ok(cmm)
    }
}

pub(crate) async fn get_keyring(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<KeyringRef> {
    let key = get_key(keydesc, keys);

    match keydesc.kind.as_str() {
        "aws-kms" => get_aws_kms_keyring(key, mpl, &kms[DFLT_REGION]).await,
        "aws-kms-rsa" => get_aws_kms_rsa_keyring(keydesc, key, mpl, &kms[DFLT_REGION]).await,
        "aws-kms-mrk-aware" => get_aws_kms_mrk_keyring(key, mpl, kms).await,
        "aws-kms-mrk-aware-discovery" => get_aws_kms_mrk_discovery_keyring(keydesc, mpl, kms).await,
        "raw" => get_raw_keyring(keydesc, key, mpl).await,
        "raw-ecdh" => get_raw_ecdh_keyring(keydesc, keys, mpl).await,
        "multi-keyring" => Box::pin(get_multi_keyring(keydesc, keys, mpl, kms)).await,
        _ => anyhow::bail!("Unknown keyring type: {} in {keydesc:?}", keydesc.kind),
    }
}

fn get_key<'a>(keydesc: &KeyDescription, keys: &'a KeyMap) -> &'a Key {
    keys.get(&keydesc.key)
        .unwrap_or_else(|| &keys[super::parse_keys::DEFAULT_KEY])
}

async fn get_multi_keyring(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<KeyringRef> {
    if keydesc.generator.is_empty() {
        anyhow::bail!("Multi keyring has no generator");
    }
    let mut children: Vec<KeyringRef> = Vec::new();
    for child in &keydesc.child_keyrings {
        let keyring = get_keyring(child, keys, mpl, kms).await?;
        children.push(keyring);
    }
    let generator = get_keyring(&keydesc.generator[0], keys, mpl, kms).await?;
    let multi_keyring = mpl
        .create_multi_keyring()
        .generator(generator)
        .child_keyrings(children)
        .send()
        .await?;

    Ok(multi_keyring)
}

pub(crate) async fn run_decrypt_tests(
    tests: &EncryptTests,
    keys: &KeyMap,
    dir: &str,
) -> Result<TestResults> {
    let mpl = mpl();
    let kms = make_kms_map().await;
    let mut res = TestResults::default();
    let mut num_non = 0;
    for test in tests {
        res.total += 1;
        if test.decrypt_key_description.kind == "aws-kms-hierarchy" {
            res.skipped += 1;
        } else if test.decrypt_key_description.kind == "aws-kms-ecdh" {
            res.skipped += 1;
        } else if test.decrypt_key_description.kind == "unknown" {
            res.skipped += 1;
        } else if test.decrypt_key_description.kind != "raw" && num_non > 5 {
            res.skipped += 1;
        } else {
            if test.decrypt_key_description.kind != "raw" {
                num_non += 1;
            }
            let cmm = get_cmm(&test.decrypt_key_description, keys, &mpl, &kms).await?;
            match run_decrypt_test(test, cmm, dir).await {
                Ok(()) => {
                    // println!(
                    //     "Test Passed {} {} {}",
                    //     test.name,
                    //     test.decrypt_key_description.kind,
                    //     test.decrypt_key_description.encryption_algorithm
                    // );
                    res.passed += 1;
                }
                Err(e) => {
                    res.failed += 1;
                    println!(
                        "Failed Test {} {} {} {e:?}",
                        test.name,
                        test.decrypt_key_description.kind,
                        test.decrypt_key_description.encryption_algorithm
                    );
                }
            }
        }
    }
    Ok(res)
}

pub(crate) fn print_test_results(results: &TestResults) {
    println!();
    println!("{} tests total", results.total);
    println!("{} tests passed", results.passed);
    println!("{} tests failed", results.failed);
    println!("{} tests skipped", results.skipped);
}

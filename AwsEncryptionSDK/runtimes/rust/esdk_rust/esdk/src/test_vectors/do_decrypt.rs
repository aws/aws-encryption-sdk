use crate::test_vectors::parse_keys::decode_base64;
use crate::test_vectors::run_tests::is_not_implemented;
use crate::test_vectors::types::*;
use crate::{DecryptInput, MaterialSource, decrypt};
use anyhow::Result;
#[cfg(feature = "kms")]
use aws_config::Region;
#[cfg(feature = "ddb")]
use aws_mpl_rs::Secret;
use aws_mpl_rs::agreement;
use aws_mpl_rs::agreement::RawEcdhStaticConfigurations;
use aws_mpl_rs::keyring::*;
#[cfg(feature = "kms")]
use aws_mpl_rs::kms_keyring::*;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[cfg(feature = "kms")]
use aws_sdk_kms::Client as KmsClient;

#[cfg(not(feature = "kms"))]
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct KmsClient {
    x: u32,
}

#[cfg(feature = "legacy")]
use aws_mpl_legacy::aws_cryptography_primitives::types::EcdhCurveSpec as LegacyEcdhCurveSpec;
#[cfg(feature = "legacy")]
use aws_mpl_legacy::client::Client as mpl_client;
#[cfg(feature = "legacy")]
use aws_mpl_legacy::types::DiscoveryFilter as LegacyDiscoveryFilter;
#[cfg(feature = "legacy")]
use aws_mpl_legacy::types::keyring::KeyringRef as LegacyKeyring;
use aws_mpl_primitives::EcdhCurveSpec;

#[cfg(feature = "legacy")]
use crate::mpl;

type KmsMap = HashMap<String, KmsClient>;
const DFLT_REGION: &str = "us-west-2";
pub(crate) async fn make_kms_map() -> KmsMap {
    let mut kms_map = KmsMap::new();
    for region in [DFLT_REGION, "us-east-1"] {
        let client = create_kms_client(region).await;
        kms_map.insert(region.to_string(), client);
    }
    kms_map
}

#[cfg(feature = "kms")]
pub(crate) async fn create_kms_client(region: &str) -> KmsClient {
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let sdk_config = sdk_config
        .to_builder()
        .region(Region::new(region.to_string()))
        .build();
    KmsClient::new(&sdk_config)
}
#[cfg(not(feature = "kms"))]
#[allow(clippy::unused_async)]
pub(crate) async fn create_kms_client(_region: &str) -> KmsClient {
    KmsClient { x: 0 }
}

#[cfg(feature = "kms")]
pub(crate) fn kms_from_arn<'a>(kms: &'a KmsMap, arn: &str) -> &'a KmsClient {
    let region = region_from_arn(arn);
    &kms[region]
}

#[cfg(feature = "kms")]
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
    ms: SourceStatus,
    dir: &str,
) -> Result<TestStatus> {
    match ms {
        SourceStatus::Ok(ms) => {
            do_run_decrypt_test(test, ms, dir).await?;
            Ok(TestStatus::Ok)
        }
        SourceStatus::NotImplemented => Ok(TestStatus::NotImplemented),
        SourceStatus::NoKmsFeature => Ok(TestStatus::NoKmsFeature),
        SourceStatus::NoDdbFeature => Ok(TestStatus::NoDdbFeature),
    }
}

pub(crate) async fn do_run_decrypt_test(
    test: &EncryptTest,
    ms: MaterialSource,
    dir: &str,
) -> Result<()> {
    let ciphertext = read_file(&test.ciphertext, dir)?;
    let plaintext = read_file(&test.result, dir)?;

    let decrypt_input = DecryptInput {
        ciphertext: &ciphertext,
        source: Some(ms),
        encryption_context: test.reproduced_encryption_context.clone(),
        commitment_policy: aws_mpl_rs::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        ..Default::default()
    };
    let decrypt_output = decrypt(&decrypt_input).await?;

    if decrypt_output.plaintext.as_ref() != plaintext {
        anyhow::bail!("Decrypted ciphertext did not match expected plaintext.");
    }

    Ok(())
}

#[cfg(feature = "legacy")]
fn get_curve_legacy(s: &str) -> Result<LegacyEcdhCurveSpec> {
    match s {
        "ecc-256" => Ok(LegacyEcdhCurveSpec::EccNistP256),
        "ecc-384" => Ok(LegacyEcdhCurveSpec::EccNistP384),
        "ecc-521" => Ok(LegacyEcdhCurveSpec::EccNistP521),
        "sm2" => Ok(LegacyEcdhCurveSpec::Sm2),
        _ => anyhow::bail!("Unknown curve: {s}"),
    }
}

fn get_curve(s: &str) -> Result<EcdhCurveSpec> {
    match s {
        "ecc-256" => Ok(EcdhCurveSpec::EccNistP256),
        "ecc-384" => Ok(EcdhCurveSpec::EccNistP384),
        "ecc-521" => Ok(EcdhCurveSpec::EccNistP521),
        _ => anyhow::bail!("Unknown curve: {s}"),
    }
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_raw_ecdh_keyring_static_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<LegacyKeyring> {
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
        .curve_spec(get_curve_legacy(&keydesc.ecc_curve)?)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;
    Ok(raw_ecdh_keyring)
}

pub(crate) fn get_raw_ecdh_keyring_static(
    keydesc: &KeyDescription,
    keys: &KeyMap,
) -> Result<KeyringStatus> {
    let key = &keys[&keydesc.sender];
    let pub_key = decode_base64(&key.recipient_material_public_key)?;
    let config =
        agreement::RawPrivateKeyToStaticPublicKey::new(key.sender_material.as_bytes(), pub_key);
    let config = RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(config);
    let keyring = CreateRawEcdhKeyringInput::new(config, get_curve(&keydesc.ecc_curve)?).go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_raw_ecdh_keyring_ephemeral_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<LegacyKeyring> {
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
        .curve_spec(get_curve_legacy(&keydesc.ecc_curve)?)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;

    Ok(raw_ecdh_keyring)
}

pub(crate) fn get_raw_ecdh_keyring_ephemeral(
    keydesc: &KeyDescription,
    keys: &KeyMap,
) -> Result<KeyringStatus> {
    let key = &keys[&keydesc.recipient];
    let pub_key = decode_base64(&key.recipient_material_public_key)?;
    let config = agreement::EphemeralPrivateKeyToStaticPublicKey::new(pub_key);
    let config = RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(config);
    let keyring = CreateRawEcdhKeyringInput::new(config, get_curve(&keydesc.ecc_curve)?).go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_raw_ecdh_keyring_discovery_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<LegacyKeyring> {
    let key = &keys[&keydesc.recipient];
    let raw_ecdh_static_configuration_input =
        aws_mpl_legacy::types::PublicKeyDiscoveryInput::builder()
            .recipient_static_private_key(key.recipient_material.as_bytes())
            .build()?;

    let raw_ecdh_static_configuration =
        aws_mpl_legacy::types::RawEcdhStaticConfigurations::PublicKeyDiscovery(
            raw_ecdh_static_configuration_input,
        );

    let raw_ecdh_keyring = mpl
        .create_raw_ecdh_keyring()
        .curve_spec(get_curve_legacy(&keydesc.ecc_curve)?)
        .key_agreement_scheme(raw_ecdh_static_configuration)
        .send()
        .await?;

    Ok(raw_ecdh_keyring)
}

pub(crate) fn get_raw_ecdh_keyring_discovery(
    keydesc: &KeyDescription,
    keys: &KeyMap,
) -> Result<KeyringStatus> {
    let key = &keys[&keydesc.recipient];
    let config = agreement::PublicKeyDiscovery::new(key.recipient_material.as_bytes());
    let config = RawEcdhStaticConfigurations::PublicKeyDiscovery(config);
    let keyring = CreateRawEcdhKeyringInput::new(config, get_curve(&keydesc.ecc_curve)?).go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_raw_ecdh_keyring_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
) -> Result<LegacyKeyring> {
    match &keydesc.schema[..] {
        "static" => get_raw_ecdh_keyring_static_legacy(keydesc, keys, mpl).await,
        "ephemeral" => get_raw_ecdh_keyring_ephemeral_legacy(keydesc, keys, mpl).await,
        "discovery" => get_raw_ecdh_keyring_discovery_legacy(keydesc, keys, mpl).await,
        _ => anyhow::bail!("Unknown ecdh schema: {}", keydesc.schema),
    }
}

pub(crate) fn get_raw_ecdh_keyring(
    keydesc: &KeyDescription,
    keys: &KeyMap,
) -> Result<KeyringStatus> {
    match &keydesc.schema[..] {
        "static" => get_raw_ecdh_keyring_static(keydesc, keys),
        "ephemeral" => get_raw_ecdh_keyring_ephemeral(keydesc, keys),
        "discovery" => get_raw_ecdh_keyring_discovery(keydesc, keys),
        _ => Ok(KeyringStatus::NotImplemented),
    }
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_kms_ecdh_keyring_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<LegacyKeyring> {
    //      = Need(curveSpec in KeyDescription.KmsKey2EccAlgorithmSpec, KeyVectorException(message = "Unknown curve spec"));
    match &keydesc.schema[..] {
        "static" => get_kms_ecdh_keyring_static_legacy(keydesc, keys, mpl, kms).await,
        "discovery" => get_kms_ecdh_keyring_discovery_legacy(keydesc, keys, mpl, kms).await,
        _ => anyhow::bail!("Unknown ecdh schema: {}", keydesc.schema),
    }
}

#[cfg(not(feature = "kms"))]
#[allow(clippy::unnecessary_wraps)]
pub(crate) const fn get_kms_ecdh_keyring(
    _keydesc: &KeyDescription,
    _keys: &KeyMap,
    _kms: &KmsMap,
) -> Result<KeyringStatus> {
    Ok(KeyringStatus::NoKmsFeature)
}

#[cfg(feature = "kms")]
pub(crate) fn get_kms_ecdh_keyring(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    kms: &KmsMap,
) -> Result<KeyringStatus> {
    match &keydesc.schema[..] {
        "static" => get_kms_ecdh_keyring_static(keydesc, keys, kms),
        "discovery" => get_kms_ecdh_keyring_discovery(keydesc, keys, kms),
        _ => Ok(KeyringStatus::NotImplemented),
    }
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_kms_ecdh_keyring_static_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<LegacyKeyring> {
    // = Need(
    //   && material.Some?
    //   && (material.value.KMSEcdh?),
    //   KeyVectorException( message = "Not type: KmsEcdh" ));
    let key = &keys[&keydesc.recipient];
    let sender_kms_key = key.sender_material.clone();
    // = Need(
    //   ComAmazonawsKmsTypes.IsValid_KeyIdType(sender_kms_key) &&
    //   ComAmazonawsKmsTypes.IsValid_KeyIdType(recipientKmsKey),
    //   KeyVectorException(message = "Not a valid Kms Key Id"));
    let kms_client = kms_from_arn(kms, &sender_kms_key);

    let sender_key = decode_base64(&key.sender_material_public_key)?;
    let recipient_key = decode_base64(&key.recipient_material_public_key)?;
    let input = aws_mpl_legacy::types::KmsPrivateKeyToStaticPublicKeyInput::builder()
        .sender_public_key(sender_key)
        .recipient_public_key(recipient_key)
        .sender_kms_identifier(sender_kms_key)
        .build()?;
    let keyring = mpl
        .create_aws_kms_ecdh_keyring()
        .curve_spec(get_curve_legacy(&key.key_id)?)
        .key_agreement_scheme(
            aws_mpl_legacy::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(
                input,
            ),
        )
        .kms_client(kms_client.clone())
        .send()
        .await?;

    Ok(keyring)
}

#[cfg(feature = "kms")]
pub(crate) fn get_kms_ecdh_keyring_static(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    kms: &KmsMap,
) -> Result<KeyringStatus> {
    let key = &keys[&keydesc.recipient];
    let sender_kms_key = key.sender_material.clone();
    let kms_client = kms_from_arn(kms, &sender_kms_key);

    let sender_key = decode_base64(&key.sender_material_public_key)?;
    let recipient_key = decode_base64(&key.recipient_material_public_key)?;
    let schema =
        agreement::KmsPrivateKeyToStaticPublicKey::new(recipient_key, sender_key, sender_kms_key);
    let schema = agreement::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(schema);
    let curve = get_curve(&key.key_id)?;
    let keyring = CreateAwsKmsEcdhKeyringInput::new(schema, curve, kms_client.clone()).go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_kms_ecdh_keyring_discovery_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<LegacyKeyring> {
    let key = keys[&keydesc.recipient].clone();
    let kms_client = kms_from_arn(kms, &key.recipient_material);
    //     = Need(
    //       && recipientMaterial?.Some?
    //       && (recipientMaterial?.value.KMSEcdh?),
    //       KeyVectorException( message = "Not type: KmsEcdh" ));

    let schema = aws_mpl_legacy::types::KmsPublicKeyDiscoveryInput::builder()
        .recipient_kms_identifier(&key.recipient_material)
        .build()?;

    let keyring = mpl
        .create_aws_kms_ecdh_keyring()
        .curve_spec(get_curve_legacy(&key.key_id)?)
        .key_agreement_scheme(
            aws_mpl_legacy::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery(schema),
        )
        .kms_client(kms_client.clone())
        .send()
        .await?;
    Ok(keyring)
}

#[cfg(feature = "kms")]
pub(crate) fn get_kms_ecdh_keyring_discovery(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    kms: &KmsMap,
) -> Result<KeyringStatus> {
    let key = keys[&keydesc.recipient].clone();
    let kms_client = kms_from_arn(kms, &key.recipient_material);
    let schema = agreement::KmsPublicKeyDiscovery::new(&key.recipient_material);
    let schema = agreement::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery(schema);
    let curve = get_curve(&key.key_id)?;
    let keyring = CreateAwsKmsEcdhKeyringInput::new(schema, curve, kms_client.clone()).go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "kms")]
fn get_kms_enc_alg(s: &str) -> aws_sdk_kms::types::EncryptionAlgorithmSpec {
    match s {
        "RSAES_OAEP_SHA_256" => aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha256,
        "RSAES_OAEP_SHA_1" => aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha1,
        _ => panic!("Unknown KMS encryption algorithm: {s}"),
    }
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_aws_kms_rsa_keyring_legacy(
    keydesc: &KeyDescription,
    key: &Key,
    mpl: &mpl_client,
    kms: &KmsClient,
) -> Result<LegacyKeyring> {
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

#[cfg(not(feature = "kms"))]
#[allow(clippy::unnecessary_wraps)]
pub(crate) const fn get_aws_kms_rsa_keyring(
    _keydesc: &KeyDescription,
    _key: &Key,
    _kms: &KmsClient,
) -> Result<KeyringStatus> {
    Ok(KeyringStatus::NoKmsFeature)
}

#[cfg(feature = "kms")]
pub(crate) fn get_aws_kms_rsa_keyring(
    keydesc: &KeyDescription,
    key: &Key,
    kms: &KmsClient,
) -> Result<KeyringStatus> {
    let keyring = CreateAwsKmsRsaKeyringInput::new(
        key.material.clone(),
        key.key_id.clone(),
        get_kms_enc_alg(&keydesc.encryption_algorithm),
        kms.clone(),
    )
    .go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_aws_kms_keyring_legacy(
    key: &Key,
    mpl: &mpl_client,
    kms: &KmsClient,
) -> Result<LegacyKeyring> {
    let keyring = mpl
        .create_aws_kms_keyring()
        .kms_key_id(key.key_id.clone())
        .kms_client(kms.clone())
        .send()
        .await?;
    Ok(keyring)
}

#[cfg(feature = "kms")]
pub(crate) fn get_aws_kms_keyring(key: &Key, kms: &KmsClient) -> Result<KeyringStatus> {
    let keyring = CreateAwsKmsKeyringInput::new(key.key_id.clone(), kms.clone()).go()?;
    Ok(KeyringStatus::Ok(keyring))
}
#[cfg(not(feature = "kms"))]
#[allow(clippy::unnecessary_wraps)]
pub(crate) const fn get_aws_kms_keyring(_key: &Key, _kms: &KmsClient) -> Result<KeyringStatus> {
    Ok(KeyringStatus::NotImplemented)
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_aws_kms_mrk_keyring_legacy(
    key: &Key,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<LegacyKeyring> {
    let kms = kms_from_arn(kms, &key.key_id);
    let keyring = mpl
        .create_aws_kms_mrk_keyring()
        .kms_key_id(key.key_id.clone())
        .kms_client(kms.clone())
        .send()
        .await?;
    Ok(keyring)
}

#[cfg(not(feature = "kms"))]
#[allow(clippy::unnecessary_wraps)]
pub(crate) const fn get_aws_kms_mrk_keyring(_key: &Key, _kms: &KmsMap) -> Result<KeyringStatus> {
    Ok(KeyringStatus::NoKmsFeature)
}

#[cfg(feature = "kms")]
pub(crate) fn get_aws_kms_mrk_keyring(key: &Key, kms: &KmsMap) -> Result<KeyringStatus> {
    let kms = kms_from_arn(kms, &key.key_id);
    let keyring = CreateAwsKmsMrkKeyringInput::new(key.key_id.clone(), kms.clone()).go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_aws_kms_mrk_discovery_keyring_legacy(
    keydesc: &KeyDescription,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<LegacyKeyring> {
    if keydesc.discovery_filter.partition.is_empty() {
        let keyring = mpl
            .create_aws_kms_mrk_discovery_keyring()
            .kms_client(kms[&keydesc.default_mrk_region].clone())
            .region(keydesc.default_mrk_region.clone())
            .send()
            .await?;
        Ok(keyring)
    } else {
        let filter = LegacyDiscoveryFilter::builder()
            .partition(keydesc.discovery_filter.partition.clone())
            .account_ids(keydesc.discovery_filter.account_ids.clone())
            .build()?;

        let keyring = mpl
            .create_aws_kms_mrk_discovery_keyring()
            .kms_client(kms[&keydesc.default_mrk_region].clone())
            .region(keydesc.default_mrk_region.clone())
            .discovery_filter(filter)
            .send()
            .await?;
        Ok(keyring)
    }
}

#[cfg(not(feature = "kms"))]
#[allow(clippy::unnecessary_wraps)]
pub(crate) const fn get_aws_kms_mrk_discovery_keyring(
    _keydesc: &KeyDescription,
    _kms: &KmsMap,
) -> Result<KeyringStatus> {
    Ok(KeyringStatus::NoKmsFeature)
}

#[cfg(feature = "kms")]
pub(crate) fn get_aws_kms_mrk_discovery_keyring(
    keydesc: &KeyDescription,
    kms: &KmsMap,
) -> Result<KeyringStatus> {
    let region = keydesc.default_mrk_region.clone();
    let mut input = CreateAwsKmsMrkDiscoveryKeyringInput::new(kms[&region].clone(), region);
    if !keydesc.discovery_filter.partition.is_empty() {
        let filter = DiscoveryFilter::new(
            keydesc.discovery_filter.partition.clone(),
            keydesc.discovery_filter.account_ids.clone(),
        );
        input.discovery_filter = filter;
    }
    let keyring = input.go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(feature = "legacy")]
fn get_aes_alg_legacy(len: usize) -> Result<aws_mpl_legacy::types::AesWrappingAlg> {
    match len {
        16 => Ok(aws_mpl_legacy::types::AesWrappingAlg::AlgAes128GcmIv12Tag16),
        24 => Ok(aws_mpl_legacy::types::AesWrappingAlg::AlgAes192GcmIv12Tag16),
        32 => Ok(aws_mpl_legacy::types::AesWrappingAlg::AlgAes256GcmIv12Tag16),
        _ => anyhow::bail!("Unknown aes key length: {len}"),
    }
}

fn get_aes_alg(len: usize) -> Result<AesWrappingAlg> {
    match len {
        16 => Ok(AesWrappingAlg::AlgAes128GcmIv12Tag16),
        24 => Ok(AesWrappingAlg::AlgAes192GcmIv12Tag16),
        32 => Ok(AesWrappingAlg::AlgAes256GcmIv12Tag16),
        _ => anyhow::bail!("Unknown aes key length: {len}"),
    }
}

#[cfg(feature = "legacy")]
async fn get_raw_keyring_legacy(
    keydesc: &KeyDescription,
    key: &Key,
    mpl: &mpl_client,
) -> Result<LegacyKeyring> {
    let hash = &keydesc.padding_hash;
    let e_alg = &keydesc.encryption_algorithm;
    let p_alg = &keydesc.padding_algorithm;
    let is_aes = e_alg == "aes";
    let is_rsa = e_alg == "rsa";
    let key_namespace = &keydesc.provider_id;
    let key_name = &key.key_id;
    if is_aes {
        let keyring = mpl
            .create_raw_aes_keyring()
            .key_namespace(key_namespace.clone())
            .key_name(key_name.clone())
            .wrapping_key(key.material.clone())
            .wrapping_alg(get_aes_alg_legacy(key.material.len())?)
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

fn get_raw_keyring(keydesc: &KeyDescription, key: &Key) -> Result<KeyringStatus> {
    let hash = &keydesc.padding_hash;
    let e_alg = &keydesc.encryption_algorithm;
    let p_alg = &keydesc.padding_algorithm;
    let is_aes = e_alg == "aes";
    let is_rsa = e_alg == "rsa";
    let key_namespace = &keydesc.provider_id;
    let key_name = &key.key_id;
    if is_aes {
        let mut input = CreateRawAesKeyringInput::default();
        input.key_namespace.clone_from(key_namespace);
        input.key_name.clone_from(key_name);
        input.wrapping_key.clone_from(&key.material);
        input.wrapping_alg = get_aes_alg(key.material.len())?;
        let keyring = input.go()?;
        Ok(KeyringStatus::Ok(keyring))
    } else if is_rsa {
        let mode: PaddingScheme;
        if hash == "sha1" && p_alg == "pkcs1" {
            mode = PaddingScheme::Pkcs1;
        } else if hash == "sha1" && p_alg == "oaep-mgf1" {
            mode = PaddingScheme::OaepSha1Mgf1;
        } else if hash == "sha256" && p_alg == "oaep-mgf1" {
            mode = PaddingScheme::OaepSha256Mgf1;
        } else if hash == "sha384" && p_alg == "oaep-mgf1" {
            mode = PaddingScheme::OaepSha384Mgf1;
        } else if hash == "sha512" && p_alg == "oaep-mgf1" {
            mode = PaddingScheme::OaepSha512Mgf1;
        } else {
            anyhow::bail!("Unknown rsa padding combo : {hash} {p_alg}");
        }
        let mut input =
            CreateRawRsaKeyringInput::new(key_namespace.clone(), key_name.clone(), mode);
        if key.material[..21] == b"-----BEGIN PUBLIC KEY"[..] {
            input.public_key.clone_from(&key.material);
        } else {
            input.private_key.clone_from(&key.material);
        }
        let keyring = input.go()?;
        Ok(KeyringStatus::Ok(keyring))
    } else {
        Ok(KeyringStatus::NotImplemented)
    }
}

#[allow(dead_code)]
pub(crate) enum KeyringStatus {
    Ok(KeyringRef),
    NotImplemented,
    NoKmsFeature,
    NoDdbFeature,
}

pub(crate) enum TestStatus {
    Ok,
    NotImplemented,
    NoKmsFeature,
    NoDdbFeature,
}

#[allow(dead_code)]
pub(crate) enum SourceStatus {
    Ok(MaterialSource),
    NotImplemented,
    NoKmsFeature,
    NoDdbFeature,
}

#[cfg(all(feature = "legacy", not(feature = "quick_vectors")))]
fn legacy_not_implemented(x: &str) -> bool {
    x == "aws-kms-hierarchy" || x == "unknown"
}

#[cfg(feature = "quick_vectors")]
fn legacy_not_implemented(x: &str) -> bool {
    x != "raw"
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_legacy_cmm(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<SourceStatus> {
    if legacy_not_implemented(&keydesc.kind) {
        Ok(SourceStatus::NotImplemented)
    } else if keydesc.kind == "required-encryption-context-cmm" {
        let keyring = get_keyring_legacy(&keydesc.underlying[0], keys, mpl, kms).await?;
        let under_cmm = mpl
            .create_default_cryptographic_materials_manager()
            .keyring(keyring.clone())
            .send()
            .await?;

        let cmm = mpl
            .create_required_encryption_context_cmm()
            .underlying_cmm(under_cmm)
            .required_encryption_context_keys(keydesc.required_keys.clone())
            .send()
            .await?;

        Ok(SourceStatus::Ok(MaterialSource::LegacyCmm(cmm)))
    } else {
        let keyring = get_keyring_legacy(keydesc, keys, mpl, kms).await?;
        Ok(SourceStatus::Ok(MaterialSource::LegacyKeyring(keyring)))
    }
}

pub(crate) fn get_cmm(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    kms: &KmsMap,
) -> Result<SourceStatus> {
    match do_get_cmm(keydesc, keys, kms) {
        Ok(x) => Ok(x),
        Err(e) => match is_not_implemented(&e) {
            Some(_s) => Ok(SourceStatus::NotImplemented),
            None => Err(e),
        },
    }
}

fn do_get_cmm(keydesc: &KeyDescription, keys: &KeyMap, kms: &KmsMap) -> Result<SourceStatus> {
    // TODO -- move this into get_keyring
    if keydesc.kind == "unknown" {
        Ok(SourceStatus::NotImplemented) // not recognized??
    } else if keydesc.kind == "required-encryption-context-cmm" {
        match get_keyring(&keydesc.underlying[0], keys, kms)? {
            KeyringStatus::Ok(keyring) => {
                let under_cmm =
                    aws_mpl_rs::cmm::create_default_cryptographic_materials_manager(keyring)?;
                let input = aws_mpl_rs::cmm::CreateRequiredEncryptionContextCMMInput::with_cmm(
                    under_cmm,
                    &keydesc.required_keys,
                );
                let cmm = input.go()?;
                Ok(SourceStatus::Ok(MaterialSource::Cmm(cmm)))
            }
            KeyringStatus::NotImplemented => Ok(SourceStatus::NotImplemented),
            KeyringStatus::NoKmsFeature => Ok(SourceStatus::NoKmsFeature),
            KeyringStatus::NoDdbFeature => Ok(SourceStatus::NoDdbFeature),
        }
    } else {
        match get_keyring(keydesc, keys, kms)? {
            KeyringStatus::Ok(keyring) => Ok(SourceStatus::Ok(MaterialSource::Keyring(keyring))),
            KeyringStatus::NotImplemented => Ok(SourceStatus::NotImplemented),
            KeyringStatus::NoKmsFeature => Ok(SourceStatus::NoKmsFeature),
            KeyringStatus::NoDdbFeature => Ok(SourceStatus::NoDdbFeature),
        }
    }
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_keyring_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<LegacyKeyring> {
    let key = get_key(keydesc, keys);

    match keydesc.kind.as_str() {
        "aws-kms" => get_aws_kms_keyring_legacy(key, mpl, &kms[DFLT_REGION]).await,
        "aws-kms-rsa" => get_aws_kms_rsa_keyring_legacy(keydesc, key, mpl, &kms[DFLT_REGION]).await,
        "aws-kms-mrk-aware" => get_aws_kms_mrk_keyring_legacy(key, mpl, kms).await,
        "aws-kms-mrk-aware-discovery" => {
            get_aws_kms_mrk_discovery_keyring_legacy(keydesc, mpl, kms).await
        }
        "raw" => get_raw_keyring_legacy(keydesc, key, mpl).await,
        "raw-ecdh" => get_raw_ecdh_keyring_legacy(keydesc, keys, mpl).await,
        "aws-kms-ecdh" => get_kms_ecdh_keyring_legacy(keydesc, keys, mpl, kms).await,
        "multi-keyring" => Box::pin(get_multi_keyring_legacy(keydesc, keys, mpl, kms)).await,
        _ => anyhow::bail!("Unknown keyring type: {} in {keydesc:?}", keydesc.kind),
    }
}

#[cfg(feature = "ddb")]
fn get_hierarchy_keyring(keydesc: &KeyDescription, keys: &KeyMap) -> Result<KeyringStatus> {
    let key = &keys[&keydesc.key];
    let store = crate::test_vectors::static_keystore::StaticKeyStoreInformation {
        branch_key_version: key.branch_key_version.clone(),
        // key_identifier: key.key_id.clone(),
        beacon_key: Secret(key.beacon_key.clone()),
        branch_key: Secret(key.branch_key.clone()),
    };
    let keyring = CreateAwsKmsHierarchicalKeyringInput::new(
        key.key_id.clone(),
        std::sync::Arc::new(store),
        std::time::Duration::new(11, 0),
    )
    .go()?;
    Ok(KeyringStatus::Ok(keyring))
}

#[cfg(not(feature = "ddb"))]
#[allow(clippy::unnecessary_wraps)]
const fn get_hierarchy_keyring(_keydesc: &KeyDescription, _keys: &KeyMap) -> Result<KeyringStatus> {
    Ok(KeyringStatus::NoDdbFeature)
}

pub(crate) fn get_keyring(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    kms: &KmsMap,
) -> Result<KeyringStatus> {
    let key = get_key(keydesc, keys);
    match keydesc.kind.as_str() {
        "aws-kms" => get_aws_kms_keyring(key, &kms[DFLT_REGION]),
        "aws-kms-rsa" => get_aws_kms_rsa_keyring(keydesc, key, &kms[DFLT_REGION]),
        "aws-kms-mrk-aware" => get_aws_kms_mrk_keyring(key, kms),
        "aws-kms-mrk-aware-discovery" => get_aws_kms_mrk_discovery_keyring(keydesc, kms),
        "raw" => get_raw_keyring(keydesc, key),
        "raw-ecdh" => get_raw_ecdh_keyring(keydesc, keys),
        "aws-kms-ecdh" => get_kms_ecdh_keyring(keydesc, keys, kms),
        "aws-kms-hierarchy" => get_hierarchy_keyring(keydesc, keys),
        "multi-keyring" => get_multi_keyring(keydesc, keys, kms),
        _ => Ok(KeyringStatus::NotImplemented),
    }
}

fn get_key<'a>(keydesc: &KeyDescription, keys: &'a KeyMap) -> &'a Key {
    keys.get(&keydesc.key)
        .unwrap_or_else(|| &keys[super::parse_keys::DEFAULT_KEY])
}

#[cfg(feature = "legacy")]
async fn get_multi_keyring_legacy(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    mpl: &mpl_client,
    kms: &KmsMap,
) -> Result<LegacyKeyring> {
    if keydesc.generator.is_empty() {
        anyhow::bail!("Multi keyring has no generator");
    }
    let mut children: Vec<LegacyKeyring> = Vec::new();
    for child in &keydesc.child_keyrings {
        let keyring = get_keyring_legacy(child, keys, mpl, kms).await?;
        children.push(keyring);
    }
    let generator = get_keyring_legacy(&keydesc.generator[0], keys, mpl, kms).await?;
    let multi_keyring = mpl
        .create_multi_keyring()
        .generator(generator)
        .child_keyrings(children)
        .send()
        .await?;

    Ok(multi_keyring)
}

fn get_multi_keyring(
    keydesc: &KeyDescription,
    keys: &KeyMap,
    kms: &KmsMap,
) -> Result<KeyringStatus> {
    if keydesc.generator.is_empty() {
        anyhow::bail!("Multi keyring has no generator");
    }
    let mut children: Vec<KeyringRef> = Vec::new();
    for child in &keydesc.child_keyrings {
        let keyring = get_keyring(child, keys, kms)?;
        if let KeyringStatus::Ok(keyring) = keyring {
            children.push(keyring);
        } else {
            return Ok(keyring);
        }
    }
    let maybe_generator = get_keyring(&keydesc.generator[0], keys, kms)?;
    if let KeyringStatus::Ok(generator) = maybe_generator {
        let multi_keyring = CreateMultiKeyringInput::new(generator, children).go()?;
        Ok(KeyringStatus::Ok(multi_keyring))
    } else {
        Ok(maybe_generator)
    }
}

pub(crate) async fn run_decrypt_tests(
    tests: &EncryptTests,
    keys: &KeyMap,
    dir: &str,
) -> Result<TestResults> {
    #[cfg(feature = "legacy")]
    let mpl = mpl();
    let kms = make_kms_map().await;
    let mut res = TestResults::default();
    for test in tests {
        res.total += 1;
        let cmm = get_cmm(&test.decrypt_key_description, keys, &kms)?;
        match run_decrypt_test(test, cmm, dir).await {
            Ok(x) => match x {
                TestStatus::Ok => res.passed += 1,
                TestStatus::NotImplemented => res.not_implemented += 1,
                TestStatus::NoKmsFeature => res.no_kms_feature += 1,
                TestStatus::NoDdbFeature => res.no_ddb_feature += 1,
            },
            Err(e) => {
                res.fail(test, &e);
            }
        }
        #[cfg(feature = "legacy")]
        let cmm = get_legacy_cmm(&test.decrypt_key_description, keys, &mpl, &kms).await?;
        #[cfg(feature = "legacy")]
        match run_decrypt_test(test, cmm, dir).await {
            Ok(x) => match x {
                TestStatus::Ok => res.legacy_passed += 1,
                _ => res.legacy_skipped += 1,
            },
            Err(e) => {
                res.fail_legacy(test, &e);
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
    println!("{} tests not implemented", results.not_implemented);
    println!("{} tests no ddb feature", results.no_ddb_feature);
    println!("{} tests no kms feature", results.no_kms_feature);
    #[cfg(feature = "legacy")]
    println!("{} tests passed legacy", results.legacy_passed);
    #[cfg(feature = "legacy")]
    println!("{} tests failed legacy", results.legacy_failed);
    #[cfg(feature = "legacy")]
    println!("{} tests skipped legacy", results.legacy_skipped);
}

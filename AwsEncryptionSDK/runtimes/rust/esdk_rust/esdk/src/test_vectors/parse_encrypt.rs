use super::types::*;
use crate::test_vectors::parse_keys::parse_encryption_context;
use crate::test_vectors::parse_keys::parse_string_list;
use anyhow::Result;
use aws_mpl_legacy::types::EsdkAlgorithmSuiteId;
use serde_json::Value as JsonValue;

pub(crate) fn parse_encrypt_tests(data: &JsonValue, version: i64) -> Result<EncryptTests> {
    let mut tests = EncryptTests::new();
    for (key, value) in data.as_object().unwrap() {
        tests.push(parse_encrypt_test(key, value, version)?);
    }
    Ok(tests)
}

pub(crate) fn get_alg_id(alg: &str) -> Result<EsdkAlgorithmSuiteId> {
    match alg {
        "0578" => Ok(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384),
        "0478" => Ok(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey),
        "0378" => Ok(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384),
        "0346" => Ok(EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384),
        "0214" => Ok(EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256),
        "0178" => Ok(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256),
        "0146" => Ok(EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256),
        "0114" => Ok(EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256),
        "0078" => Ok(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf),
        "0046" => Ok(EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf),
        "0014" => Ok(EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf),
        _ => anyhow::bail!("Unknown Alg Id '{alg}'"),
    }
}

pub(crate) fn parse_encrypt_test(
    name: &str,
    data: &JsonValue,
    version: i64,
) -> Result<EncryptTest> {
    match version {
        1 => parse_encrypt_test_v1(name, data),
        5 => parse_encrypt_test_v5(name, data),
        _ => anyhow::bail!("Unknown version {version}"),
    }
}

pub(crate) fn parse_encrypt_test_v1(name: &str, data: &JsonValue) -> Result<EncryptTest> {
    let mut test = EncryptTest::default();
    for (key, value) in data.as_object().unwrap() {
        match key.as_str() {
            "plaintext" => test.result = value.as_str().unwrap().to_string(),
            "ciphertext" => test.ciphertext = value.as_str().unwrap().to_string(),
            "master-keys" => {
                let keys = value.as_array().unwrap();
                let mut kd = KeyDescription::default();
                if keys.len() == 1 {
                    for (k, v) in keys[0].as_object().unwrap() {
                        match k.as_str() {
                            "type" => kd.kind = v.as_str().unwrap().to_string(),
                            "key" => kd.key = v.as_str().unwrap().to_string(),
                            _ => anyhow::bail!("Unexpected master key element : {k}"),
                        }
                    }
                } else {
                    kd.kind = "unknown".to_string();
                }
                test.key_description = kd.clone();
                test.decrypt_key_description = kd;
            }
            _ => anyhow::bail!("Unexpected element of Encrypt Test {name} : {key}"),
        }
    }
    Ok(test)
}

pub(crate) fn parse_encrypt_test_v5(name: &str, data: &JsonValue) -> Result<EncryptTest> {
    let mut test = EncryptTest::default();
    let mut saw = false;
    for (key, value) in data.as_object().unwrap() {
        match key.as_str() {
            "encryption-scenario" | "decryption-scenario" => {
                if saw {
                    anyhow::bail!("Unexpected extra {key} in {name}");
                }
                test = parse_scenario(value, name)?;
                saw = true;
            }
            _ => {
                anyhow::bail!("Unexpected element of Encrypt Test : {key}");
            }
        }
    }
    Ok(test)
}

pub(crate) fn parse_scenario(data: &JsonValue, name: &str) -> Result<EncryptTest> {
    let mut scenario = EncryptTest {
        name: name.to_string(),
        ..Default::default()
    };

    for (key, value) in data.as_object().unwrap() {
        match key.as_str() {
            "type" => scenario.kind = value.as_str().unwrap().to_string(),
            "algorithmSuiteId" => {
                scenario.alg_suite_id = value.as_str().unwrap().to_string();
                scenario.alg_id = get_alg_id(&scenario.alg_suite_id)?;
            }
            "description" => scenario.description = value.as_str().unwrap().to_string(),
            "errorDescription" => scenario.error_description = value.as_str().unwrap().to_string(),
            "decryptErrorDescription" => {
                scenario.decrypt_error_description = value.as_str().unwrap().to_string();
            }
            "plaintext" => scenario.plaintext = value.as_str().unwrap().to_string(),
            "ciphertext" => scenario.ciphertext = value.as_str().unwrap().to_string(),
            "result" => scenario.result = value.as_str().unwrap().to_string(),
            "frame-size" => scenario.frame_size = value.as_u64().unwrap() as u32,
            "encryption-context" => scenario.encryption_context = parse_encryption_context(value),
            "reproduced-encryption-context" => {
                scenario.reproduced_encryption_context = parse_encryption_context(value);
                scenario.reproduced_json = value.clone();
            }
            "decryptKeyDescription" => {
                scenario.decrypt_key_description = parse_key_description(value)?;
                scenario.decrypt_json = value.clone();
            }
            "encryptKeyDescription" => {
                scenario.encrypt_key_description = parse_key_description(value)?;
            }
            "keyDescription" => scenario.key_description = parse_key_description(value)?,
            _ => anyhow::bail!("Unexpected element of Scenario : {key} {value:?}"),
        }
    }
    Ok(scenario)
}

pub(crate) fn parse_discovery_filter(data: &JsonValue) -> Result<DiscoveryFilter> {
    let mut filter = DiscoveryFilter::default();
    for (key, value) in data.as_object().unwrap() {
        match key.as_str() {
            "partition" => filter.partition = value.as_str().unwrap().to_string(),
            "account-ids" => filter.account_ids = parse_string_list(value),
            _ => anyhow::bail!("Unexpected element of DiscoveryFilter : {key}"),
        }
    }
    Ok(filter)
}

pub(crate) fn parse_key_list(data: &JsonValue) -> Result<Vec<KeyDescription>> {
    let mut keys = Vec::new();
    for value in data.as_array().unwrap() {
        keys.push(parse_key_description(value)?);
    }
    Ok(keys)
}

pub(crate) fn parse_key_description(data: &JsonValue) -> Result<KeyDescription> {
    let mut key = KeyDescription::default();
    for (key_, value) in data.as_object().unwrap() {
        match key_.as_str() {
            "type" => key.kind = value.as_str().unwrap().to_string(),
            "sender" => key.sender = value.as_str().unwrap().to_string(),
            "recipient" => key.recipient = value.as_str().unwrap().to_string(),
            "sender-public-key" => key.sender_public_key = value.as_str().unwrap().to_string(),
            "recipient-public-key" => {
                key.recipient_public_key = value.as_str().unwrap().to_string();
            }
            "provider-id" => key.provider_id = value.as_str().unwrap().to_string(),
            "ecc-curve" => key.ecc_curve = value.as_str().unwrap().to_string(),
            "schema" => key.schema = value.as_str().unwrap().to_string(),
            "key" => key.key = value.as_str().unwrap().to_string(),
            "encryption-algorithm" => {
                key.encryption_algorithm = value.as_str().unwrap().to_string();
            }
            "padding-algorithm" => key.padding_algorithm = value.as_str().unwrap().to_string(),
            "padding-hash" => key.padding_hash = value.as_str().unwrap().to_string(),
            "default-mrk-region" => key.default_mrk_region = value.as_str().unwrap().to_string(),
            "aws-kms-discovery-filter" => key.discovery_filter = parse_discovery_filter(value)?,
            "childKeyrings" => key.child_keyrings = parse_key_list(value)?,
            "generator" => key.generator.push(parse_key_description(value)?),
            "underlying" => key.underlying.push(parse_key_description(value)?),
            "requiredEncryptionContextKeys" => {
                key.required_encryption_context_keys = parse_string_list(value);
            }
            _ => anyhow::bail!("Unexpected element of KeyDescription : {key_}"),
        }
    }
    Ok(key)
}

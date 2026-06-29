#![allow(dead_code)]

use super::do_decrypt::get_cmm;
use super::do_decrypt::make_kms_map;
use super::do_decrypt::{SourceStatus, trim_filename};
use crate::test_vectors::types::*;
use crate::{EncryptInput, MaterialSource, encrypt};
use anyhow::Result;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::primitives::generate_random_bytes;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use serde_json::Value as JsonValue;

#[cfg(feature = "legacy")]
use super::do_decrypt::get_legacy_cmm;
#[cfg(feature = "legacy")]
use crate::mpl;

pub(crate) fn write_file(filename: &str, data: &[u8], dir: &str) -> Result<()> {
    let filename = trim_filename(filename);
    let name = format!("{dir}/{filename}");
    std::fs::write(name, data)?;
    Ok(())
}

pub(crate) fn make_plain_texts(
    plaintexts: &serde_json::Map<String, serde_json::Value>,
    dir: &str,
) -> Result<PlainTexts> {
    let dir = format!("{dir}/plaintexts");
    std::fs::create_dir_all(&dir)?;

    let mut p = PlainTexts::with_capacity(plaintexts.len());
    for (key, value) in plaintexts {
        let Some(size_u64) = value.as_u64() else {
            return Err(anyhow::anyhow!("plaintext size for {key:?} is not a non-negative integer"));
        };
        let size = usize::try_from(size_u64)
            .map_err(|_| anyhow::anyhow!("plaintext size {size_u64} does not fit in usize"))?;
        let mut bytes = vec![0; size];
        generate_random_bytes(&mut bytes).map_err(anyhow::Error::msg)?;
        write_file(key, &bytes, &dir)?;
        p.insert(key.clone(), bytes);
    }
    Ok(p)
}

fn make_decrypt_json(test: &EncryptTest, ciphertext_result: &[u8], dir: &str) -> Result<JsonValue> {
    let outname = format!("/ciphertexts/{}", test.name);
    write_file(&outname, ciphertext_result, dir)?;
    let mut inner_obj = serde_json::Map::new();
    inner_obj.insert("type".into(), JsonValue::from(test.kind.as_str()));
    inner_obj.insert(
        "result".into(),
        JsonValue::from(format!("file://plaintexts/{}", test.plaintext)),
    );
    inner_obj.insert(
        "ciphertext".into(),
        JsonValue::from(format!("file://{outname}")),
    );
    inner_obj.insert(
        "algorithmSuiteId".into(),
        JsonValue::from(test.alg_suite_id.as_str()),
    );
    inner_obj.insert("frame-size".into(), JsonValue::from(test.frame_size));
    inner_obj.insert(
        "decryptKeyDescription".into(),
        test.decrypt_json.clone(),
    );
    inner_obj.insert(
        "reproduced-encryption-context".into(),
        test.reproduced_json.clone(),
    );
    inner_obj.insert(
        "description".into(),
        JsonValue::from(test.description.as_str()),
    );
    inner_obj.retain(|_key, value| !value.is_null());
    let outer = serde_json::json!({
        "decryption-scenario": JsonValue::Object(inner_obj)
    });
    Ok(outer)
}

pub(crate) enum TestStatus {
    Ok(JsonValue),
    NotImplemented,
    NoKmsFeature,
    NoDdbFeature,
}

pub(crate) async fn run_encrypt_test(
    test: &EncryptTest,
    ms: SourceStatus,
    plaintexts: &PlainTexts,
    dir: &str,
) -> Result<TestStatus> {
    match ms {
        SourceStatus::Ok(ms) => {
            let result = do_run_encrypt_test(test, ms, plaintexts, dir).await?;
            Ok(TestStatus::Ok(result))
        }
        SourceStatus::NotImplemented => Ok(TestStatus::NotImplemented),
        SourceStatus::NoKmsFeature => Ok(TestStatus::NoKmsFeature),
        SourceStatus::NoDdbFeature => Ok(TestStatus::NoDdbFeature),
    }
}

pub(crate) async fn do_run_encrypt_test(
    test: &EncryptTest,
    source: MaterialSource,
    plaintexts: &PlainTexts,
    dir: &str,
) -> Result<JsonValue> {
    let plaintext = &plaintexts[&test.plaintext];
    let encrypt_input = EncryptInput {
        plaintext,
        source: Some(source),
        algorithm_suite_id: Some(test.alg_id),
        encryption_context: test.encryption_context.clone(),
        commitment_policy: policy(test.alg_id),
        ..Default::default()
    };

    let encrypt_output = encrypt(&encrypt_input).await?;

    make_decrypt_json(test, encrypt_output.ciphertext.as_ref(), dir)
}

pub(crate) const fn is_committing(id: EsdkAlgorithmSuiteId) -> bool {
    matches!(
        id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey
            | EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
    )
}

pub(crate) const fn policy(id: EsdkAlgorithmSuiteId) -> EsdkCommitmentPolicy {
    if is_committing(id) {
        EsdkCommitmentPolicy::RequireEncryptAllowDecrypt
    } else {
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt
    }
}

#[allow(clippy::if_same_then_else)]
pub(crate) async fn run_encrypt_tests(
    tests: &EncryptTests,
    keys: &KeyMap,
    plaintexts: &PlainTexts,
    res: &mut TestResults,
    dir: &str,
) -> Result<JsonValue> {
    #[cfg(feature = "legacy")]
    let mpl = mpl();
    let kms = make_kms_map().await;

    std::fs::create_dir_all(format!("{dir}/ciphertexts"))?;
    let manifest = serde_json::json!({
        "type": "awses-decrypt",
        "version": 5
    });
    let client_json = serde_json::json!({
        "name": "aws-encryption-sdk-rust",
        "version": "2.4.1"
    });
    let mut out_tests = serde_json::json!({});

    for test in tests {
        res.total += 1;
        let cmm = get_cmm(&test.encrypt_key_description, keys, &kms)?;
        match run_encrypt_test(test, cmm, plaintexts, dir).await {
            Ok(x) => match x {
                TestStatus::Ok(j) => {
                    res.passed += 1;
                    out_tests[test.name.clone()] = j;
                }
                TestStatus::NotImplemented => res.not_implemented += 1,
                TestStatus::NoDdbFeature => res.no_ddb_feature += 1,
                TestStatus::NoKmsFeature => res.no_kms_feature += 1,
            },
            Err(e) => {
                res.fail(test, &e);
            }
        }
        #[cfg(feature = "legacy")]
        {
            let cmm = get_legacy_cmm(&test.encrypt_key_description, keys, &mpl, &kms).await?;
            match run_encrypt_test(test, cmm, plaintexts, dir).await {
                Ok(x) => match x {
                    TestStatus::Ok(j) => {
                        res.legacy_passed += 1;
                        out_tests[test.name.clone()] = j;
                    }
                    _ => res.legacy_skipped += 1,
                },
                Err(e) => {
                    res.fail_legacy(test, &e);
                }
            }
        }
    }

    let result = serde_json::json!({
        "manifest": manifest,
        "client": client_json,
        "keys" :  "file://keys.json",
        "tests" : out_tests
    });

    Ok(result)
}

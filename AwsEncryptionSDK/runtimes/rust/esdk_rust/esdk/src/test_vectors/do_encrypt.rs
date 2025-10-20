use super::do_decrypt::trim_filename;
use super::do_decrypt::{get_cmm, make_kms_map};
use crate::test_vectors::types::*;
use crate::types::EncryptInputBuilder;
use anyhow::Result;
use aws_mpl_primitives::generate_random_bytes;
use aws_mpl_rs::client::Client as mpl_client;
use aws_mpl_rs::types::EsdkAlgorithmSuiteId;
use aws_mpl_rs::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef as CmmRef;
use serde_json::Value as JsonValue;

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
        let size = value.as_u64().unwrap() as usize;
        let mut bytes = vec![0; size];
        generate_random_bytes(&mut bytes).map_err(anyhow::Error::msg)?;
        write_file(key, &bytes, &dir).unwrap();
        p.insert(key.clone(), bytes);
    }
    Ok(p)
}

fn make_decrypt_json(test: &EncryptTest, ciphertext_result: &[u8], dir: &str) -> Result<JsonValue> {
    let outname = format!("/ciphertexts/{}", test.name);
    write_file(&outname, ciphertext_result, dir)?;
    let mut inner = serde_json::json!({
        "type": test.kind,
        "result": format!("file://plaintexts/{}", test.plaintext),
        "ciphertext": format!("file://{}", outname),
        "algorithmSuiteId": test.alg_suite_id,
        "frame-size": test.frame_size,
        "decryptKeyDescription": test.decrypt_json,
        "reproduced-encryption-context": test.reproduced_json,
        "description": test.description
    });
    let inner_obj = inner.as_object_mut().unwrap();
    inner_obj.retain(|_key, value| !value.is_null());
    let outer = serde_json::json!({
        "decryption-scenario": inner
    });
    Ok(outer)
}

pub(crate) async fn run_encrypt_test(
    client: &crate::client::Client,
    test: &EncryptTest,
    cmm: CmmRef,
    plaintexts: &PlainTexts,
    dir: &str,
) -> Result<JsonValue> {
    let plaintext = &plaintexts[&test.plaintext];
    let encrypt_input = EncryptInputBuilder::default()
        .plaintext(plaintext)
        .materials_manager(cmm)
        .algorithm_suite_id(test.alg_id)
        .encryption_context(&test.encryption_context)
        .build()
        .unwrap();

    let encrypt_output = client.encrypt(&encrypt_input).await?;

    make_decrypt_json(test, encrypt_output.ciphertext.as_ref(), dir)
}

pub(crate) const fn is_committing(id: EsdkAlgorithmSuiteId) -> bool {
    matches!(
        id,
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey
            | EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
    )
}

pub(crate) const fn client<'a>(
    id: EsdkAlgorithmSuiteId,
    require: &'a crate::client::Client,
    forbid: &'a crate::client::Client,
) -> &'a crate::client::Client {
    if is_committing(id) { require } else { forbid }
}

#[allow(clippy::if_same_then_else)]
pub(crate) async fn run_encrypt_tests(
    tests: &EncryptTests,
    keys: &KeyMap,
    plaintexts: &PlainTexts,
    res: &mut TestResults,
    dir: &str,
) -> Result<JsonValue> {
    let mpl_config = aws_mpl_rs::types::MaterialProvidersConfig::builder().build()?;
    let mpl = mpl_client::from_conf(mpl_config)?;
    let esdk_config = crate::types::AwsEncryptionSdkConfigBuilder::default()
        .commitment_policy(aws_mpl_rs::types::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt)
        .build()?;
    let forbid = crate::client::Client::from_conf(esdk_config)?;
    let esdk_config = crate::types::AwsEncryptionSdkConfigBuilder::default()
        .commitment_policy(aws_mpl_rs::types::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt)
        .build()?;
    let require = crate::client::Client::from_conf(esdk_config)?;
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
        if "aws-kms-hierarchy" == test.encrypt_key_description.kind {
            res.skipped += 1;
        } else if "aws-kms-ecdh" == test.encrypt_key_description.kind {
            res.skipped += 1;
        } else if "raw" != test.encrypt_key_description.kind {
            res.skipped += 1;
        } else {
            let cmm = get_cmm(&test.encrypt_key_description, keys, &mpl, &kms).await?;
            match run_encrypt_test(
                client(test.alg_id, &require, &forbid),
                test,
                cmm,
                plaintexts,
                dir,
            )
            .await
            {
                Ok(j) => {
                    // println!(
                    //     "Test Passed {} {} {}",
                    //     test.name,
                    //     test.decrypt_key_description.kind,
                    //     test.decrypt_key_description.encryption_algorithm
                    // );
                    res.passed += 1;
                    out_tests[test.name.clone()] = j;
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

    let result = serde_json::json!({
        "manifest": manifest,
        "client": client_json,
        "keys" :  "file://keys.json",
        "tests" : out_tests
    });

    Ok(result)
}

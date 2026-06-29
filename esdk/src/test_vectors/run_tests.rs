use crate::test_vectors::do_decrypt::read_json;
use anyhow::{Error, Result};

pub(crate) fn is_not_implemented(e: &Error) -> Option<String> {
    if let Some(mpl) = e.downcast_ref::<aws_mpl_legacy::error::Error>()
        && matches!(&mpl.kind, aws_mpl_legacy::error::ErrorKind::NotImplemented)
    {
        return Some(mpl.message.clone());
    }
    let msg = format!("{e}");
    if msg.contains("Not Implemented") {
        println!("***\n{msg}\n***\n");
        return Some(msg);
    }
    None
}

/// Decrypt and verify the decrypt manifest. Enabled with `features = "test_vectors"`.
pub async fn decrypt_test_vectors(
    manifest_path: &str,
    manifest_name: &str,
    _test_name: &str,
) -> Result<()> {
    let json_data_base = read_json(manifest_name, manifest_path)?;
    let Some(json_data) = json_data_base.as_object() else {
        anyhow::bail!("Decrypt manifest '{manifest_name}' is not a JSON object");
    };

    let Some(manifest) = json_data["manifest"].as_object() else {
        anyhow::bail!("Decrypt manifest '{manifest_name}' missing 'manifest' object");
    };
    let Some(manifest_type) = manifest["type"].as_str() else {
        anyhow::bail!("Decrypt manifest '{manifest_name}' missing string 'manifest.type'");
    };
    if manifest_type != "awses-decrypt" {
        anyhow::bail!("Decrypt manifest type was '{manifest_type}' instead of 'awses-decrypt'");
    }
    let Some(manifest_version) = manifest["version"].as_i64() else {
        anyhow::bail!("Decrypt manifest '{manifest_name}' missing integer 'manifest.version'");
    };
    if manifest_version != 5 && manifest_version != 1 {
        anyhow::bail!("Decrypt manifest version was {manifest_version} instead of 5");
    }

    let Some(keys_file) = json_data["keys"].as_str() else {
        anyhow::bail!("Decrypt manifest '{manifest_name}' missing string 'keys'");
    };
    let keys_data = read_json(keys_file, manifest_path)?;
    let Some(manifest) = keys_data["manifest"].as_object() else {
        anyhow::bail!("Keys file '{keys_file}' missing 'manifest' object");
    };
    let Some(manifest_type) = manifest["type"].as_str() else {
        anyhow::bail!("Keys file '{keys_file}' missing string 'manifest.type'");
    };
    if manifest_type != "keys" {
        anyhow::bail!("Keys manifest type was '{manifest_type}' instead of 'keys'");
    }
    let Some(keys_version) = manifest["version"].as_i64() else {
        anyhow::bail!("Keys file '{keys_file}' missing integer 'manifest.version'");
    };
    let json_keys = &keys_data["keys"];
    let keys = super::parse_keys::parse_keys(json_keys, keys_version)?;

    let json_tests = &json_data["tests"];
    let tests = super::parse_encrypt::parse_encrypt_tests(json_tests, manifest_version)?;

    let results: super::types::TestResults =
        super::do_decrypt::run_decrypt_tests(&tests, &keys, manifest_path).await?;
    super::do_decrypt::print_test_results(&results);
    if results.failed != 0 {
        anyhow::bail!("Some Tests Failed!");
    }
    Ok(())
}

/// Encrypt an encrypt manifest, creating a decrypt manifest. Enabled with `features = "test_vectors"`
pub async fn encrypt_test_vectors(
    encrypt_path: &str,
    decrypt_path: &str,
    _test_name: &str,
) -> Result<()> {
    drop(std::fs::remove_dir_all(format!("{encrypt_path}/plaintexts")));
    drop(std::fs::remove_dir_all(format!("{encrypt_path}/ciphertexts")));
    let decrypt_manifest = format!("{encrypt_path}/decrypt-manifest.json");
    let json_data_base = read_json("encrypt-manifest.json", encrypt_path)?;
    let Some(json_data) = json_data_base.as_object() else {
        anyhow::bail!("Encrypt manifest at '{encrypt_path}' is not a JSON object");
    };

    let Some(manifest) = json_data["manifest"].as_object() else {
        anyhow::bail!("Encrypt manifest at '{encrypt_path}' missing 'manifest' object");
    };
    let Some(manifest_type) = manifest["type"].as_str() else {
        anyhow::bail!("Encrypt manifest at '{encrypt_path}' missing string 'manifest.type'");
    };
    if manifest_type != "awses-encrypt" {
        anyhow::bail!("Encrypt manifest type was '{manifest_type}' instead of 'awses-encrypt'");
    }
    let Some(manifest_version) = manifest["version"].as_i64() else {
        anyhow::bail!("Encrypt manifest at '{encrypt_path}' missing integer 'manifest.version'");
    };
    if manifest_version != 5 {
        anyhow::bail!("Encrypt manifest version was {manifest_version} instead of 5");
    }
    let Some(plaintexts) = json_data["plaintexts"].as_object() else {
        anyhow::bail!("Encrypt manifest at '{encrypt_path}' missing 'plaintexts' object");
    };
    let plaintext_data = super::do_encrypt::make_plain_texts(plaintexts, decrypt_path)?;

    let Some(keys_file) = json_data["keys"].as_str() else {
        anyhow::bail!("Encrypt manifest at '{encrypt_path}' missing string 'keys'");
    };
    let keys_data = read_json(keys_file, encrypt_path)?;
    let Some(manifest) = keys_data["manifest"].as_object() else {
        anyhow::bail!("Keys file '{keys_file}' missing 'manifest' object");
    };
    let Some(manifest_type) = manifest["type"].as_str() else {
        anyhow::bail!("Keys file '{keys_file}' missing string 'manifest.type'");
    };
    if manifest_type != "keys" {
        anyhow::bail!("Keys manifest type was '{manifest_type}' instead of 'keys'");
    }
    let Some(keys_version) = manifest["version"].as_i64() else {
        anyhow::bail!("Keys file '{keys_file}' missing integer 'manifest.version'");
    };
    let json_keys = &keys_data["keys"];
    let keys = super::parse_keys::parse_keys(json_keys, keys_version)?;

    let json_tests = &json_data["tests"];
    let tests = super::parse_encrypt::parse_encrypt_tests(json_tests, manifest_version)?;

    let mut results = super::types::TestResults::default();
    let decrypt_vectors = super::do_encrypt::run_encrypt_tests(
        &tests,
        &keys,
        &plaintext_data,
        &mut results,
        encrypt_path,
    )
    .await?;

    super::do_decrypt::write_json(&decrypt_vectors, &decrypt_manifest)?;
    super::do_decrypt::print_test_results(&results);
    if results.failed != 0 {
        anyhow::bail!("Some Tests Failed!");
    }

    Ok(())
}

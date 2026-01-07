use crate::test_vectors::do_decrypt::read_json;
use anyhow::{Error, Result};

pub(crate) fn is_not_implemented(e: &Error) -> Option<String> {
    if let Some(mpl) = e.downcast_ref::<aws_mpl_rs::error::Error>()
        && let aws_mpl_rs::error::ErrorKind::NotImplemented(s) = &mpl.kind
    {
        return Some(s.clone());
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
    let json_data = json_data_base.as_object().unwrap();

    let manifest = json_data["manifest"].as_object().unwrap();
    let manifest_type = manifest["type"].as_str().unwrap();
    if manifest_type != "awses-decrypt" {
        anyhow::bail!("Decrypt manifest type was '{manifest_type}' instead of 'awses-decrypt'",);
    }
    let manifest_version = manifest["version"].as_i64().unwrap();
    if manifest_version != 5 && manifest_version != 1 {
        anyhow::bail!("Decrypt manifest version was {manifest_version} instead of 5");
    }

    let keys_file = json_data["keys"].as_str().unwrap();
    let keys_data = read_json(keys_file, manifest_path)?;
    let manifest = keys_data["manifest"].as_object().unwrap();
    let manifest_type = manifest["type"].as_str().unwrap();
    if manifest_type != "keys" {
        anyhow::bail!("Keys manifest type was '{manifest_type}' instead of 'keys'",);
    }
    let keys_version = manifest["version"].as_i64().unwrap();
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

    let results: super::types::TestResults =
        super::do_decrypt::run_decrypt_tests(&tests, &keys, manifest_path).await?;

    super::do_decrypt::print_test_results(&results);
    if results.failed != 0 {
        anyhow::bail!("Some Decrypt Tests Failed!");
    }

    Ok(())
}

/// Encrypt an encrypt manifest, creating a decrypt manifest. Enabled with `features = "test_vectors"`
pub async fn encrypt_test_vectors(
    encrypt_path: &str,
    decrypt_path: &str,
    _test_name: &str,
) -> Result<()> {
    drop(std::fs::remove_dir_all(format!(
        "{encrypt_path}/plaintexts"
    )));
    drop(std::fs::remove_dir_all(format!(
        "{encrypt_path}/ciphertexts"
    )));
    let decrypt_manifest = format!("{encrypt_path}/decrypt-manifest.json");
    let json_data_base = read_json("encrypt-manifest.json", encrypt_path)?;
    let json_data = json_data_base.as_object().unwrap();

    let manifest = json_data["manifest"].as_object().unwrap();
    let manifest_type = manifest["type"].as_str().unwrap();
    if manifest_type != "awses-encrypt" {
        anyhow::bail!("Encrypt manifest type was '{manifest_type}' instead of 'awses-decrypt'",);
    }
    let manifest_version = manifest["version"].as_i64().unwrap();
    if manifest_version != 5 {
        anyhow::bail!("Encrypt manifest version was {manifest_version} instead of 5");
    }
    let plaintexts = json_data["plaintexts"].as_object().unwrap();
    let plaintext_data = super::do_encrypt::make_plain_texts(plaintexts, decrypt_path)?;

    let keys_file = json_data["keys"].as_str().unwrap();
    let keys_data = read_json(keys_file, encrypt_path)?;
    let manifest = keys_data["manifest"].as_object().unwrap();
    let manifest_type = manifest["type"].as_str().unwrap();
    if manifest_type != "keys" {
        anyhow::bail!("Keys manifest type was '{manifest_type}' instead of 'keys'",);
    }
    let keys_version = manifest["version"].as_i64().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test(flavor = "multi_thread")]
    async fn test_do_decrypt() {
        let manifest_path = "test_vectors_java";
        let manifest_name = "decrypt-manifest.json";
        let result = decrypt_test_vectors(manifest_path, manifest_name, "").await;
        println!("{result:?}");
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_python_decrypt() {
        let manifest_path = "test_vectors_python";
        let manifest_name = "decrypt_message.json";
        let result = decrypt_test_vectors(manifest_path, manifest_name, "").await;
        println!("{result:?}");
        assert!(result.is_ok());
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn test_do_encrypt() {
        let manifest_path = "test_vectors_rust";
        let manifest_name = "decrypt-manifest.json";

        let result = encrypt_test_vectors(manifest_path, manifest_path, "").await;
        assert!(result.is_ok());

        let result = decrypt_test_vectors(manifest_path, manifest_name, "").await;
        assert!(result.is_ok());
    }
}

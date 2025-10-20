use super::types::*;
use crate::test_vectors::parse_encrypt::get_alg_id;
use crate::types::*;
use anyhow::Result;
use base64::prelude::*;
use serde_json::Value as JsonValue;

pub(crate) const DEFAULT_KEY: &str = "_dflt";

pub(crate) fn decode_base64(input: &str) -> Result<Bytes> {
    let val = BASE64_STANDARD.decode(input)?;
    Ok(val)
}

pub(crate) fn parse_key_v1(data: &JsonValue, name: &str) -> Result<Key> {
    let mut key = Key::default();
    let mut material = String::new();
    let mut sep = String::new();
    for (key_str, value) in data.as_object().unwrap() {
        match key_str.as_str() {
            "key-id" => key.key_id = value.as_str().unwrap().to_string(),
            "encrypt" => key.encrypt = value.as_bool().unwrap(),
            "decrypt" => key.decrypt = value.as_bool().unwrap(),
            "algorithm" => key.alg = value.as_str().unwrap().to_string(),
            "type" => key.kind = value.as_str().unwrap().to_string(),
            "bits" => key.bits = value.as_u64().unwrap() as u32,
            "encoding" => key.encoding = value.as_str().unwrap().to_string(),
            "line-separator" => sep = value.as_str().unwrap().to_string(),
            "material" => {
                let materials = value.as_array().unwrap();
                for m in materials {
                    let mm = m.as_str().unwrap();
                    material.push_str(mm);
                    material.push_str(&sep);
                }
            }
            _ => anyhow::bail!("Unknown field in JSON for version 1 key {name}: {key_str}"),
        }
    }
    if !material.is_empty() {
        if key.encoding.is_empty() {
            println!("------------ EMPTY ENCODING ---------");
            key.material = material.as_bytes().into();
        } else if key.encoding == "base64" {
            key.material = decode_base64(&material)?;
        } else if key.encoding == "pem" {
            key.material = material.as_bytes().into();
        } else {
            anyhow::bail!("Unknown key encoding : {}", key.encoding);
        }
    }
    Ok(key)
}

pub(crate) fn parse_key_v2(data: &JsonValue, _name: &str) -> Result<Key> {
    let mut key = Key::default();
    let mut material = String::new();
    for (key_str, value) in data.as_object().unwrap() {
        match key_str.as_str() {
            "type" => key.kind = value.as_str().unwrap().to_string(),
            "key-id" => key.key_id = value.as_str().unwrap().to_string(),
            "algorithm" => key.alg = value.as_str().unwrap().to_string(),
            "algorithmSuiteId" => {
                key.alg_suite_id = value.as_str().unwrap().to_string();
                key.alg_id = get_alg_id(&key.alg_suite_id)?;
            }
            "encrypt" => key.encrypt = value.as_bool().unwrap(),
            "decrypt" => key.decrypt = value.as_bool().unwrap(),
            "bits" => key.bits = value.as_u64().unwrap() as u32,
            "material" => material = value.as_str().unwrap().to_string(),
            "encoding" => key.encoding = value.as_str().unwrap().to_string(),
            "public-key-encoding" => key.public_key_encoding = value.as_str().unwrap().to_string(),
            "plaintextDataKey" => {
                key.plaintext_data_key = decode_base64(value.as_str().unwrap())?;
            }
            "beaconKey" => key.beacon_key = decode_base64(value.as_str().unwrap())?,
            "branchKey" => key.branch_key = decode_base64(value.as_str().unwrap())?,
            "encryptedDataKeys" => key.encrypted_data_keys = parse_edks(value),
            "recipient-material" => key.recipient_material = value.as_str().unwrap().to_string(),
            "sender-material" => key.sender_material = value.as_str().unwrap().to_string(),
            "recipient-material-public-key" => {
                key.recipient_material_public_key = value.as_str().unwrap().to_string();
            }

            "sender-material-public-key" => {
                key.sender_material_public_key = value.as_str().unwrap().to_string();
            }

            "branchKeyVersion" => key.branch_key_version = value.as_str().unwrap().to_string(),
            "encryptionContext" => key.encryption_context = parse_encryption_context(value),
            "requiredEncryptionContextKeys" => {
                key.required_encryption_context_keys = parse_string_list(value);
            }
            _ => anyhow::bail!("Unknown field in JSON for version 2 key: {key_str}"),
        }
    }
    if !material.is_empty() {
        if key.encoding.is_empty() || key.encoding == "base64" {
            key.material = decode_base64(&material)?;
        } else if key.encoding == "pem" {
            key.material = material.as_bytes().into();
        } else {
            anyhow::bail!("Unknown key encoding : {}", key.encoding);
        }
    }
    Ok(key)
}

pub(crate) fn parse_keys(data: &JsonValue, version: i64) -> Result<KeyMap> {
    let mut keymap = KeyMap::default();
    if version == 1 {
        for (key_str, value) in data.as_object().unwrap() {
            keymap.insert(key_str.to_string(), parse_key_v1(value, key_str)?);
        }
    } else if version == 2 {
        for (key_str, value) in data.as_object().unwrap() {
            keymap.insert(key_str.to_string(), parse_key_v2(value, key_str)?);
        }
    } else {
        anyhow::bail!("Unknown key version : {version}");
    }
    keymap.insert(DEFAULT_KEY.to_string(), Key::default());
    Ok(keymap)
}

pub(crate) fn parse_edk(data: &JsonValue) -> Result<Edk> {
    let mut edk = Edk::default();
    for (key_str, value) in data.as_object().unwrap() {
        match key_str.as_str() {
            "keyProviderId" => edk.key_provider_id = value.as_str().unwrap().to_string(),
            "ciphertext" => edk.ciphertext = decode_base64(value.as_str().unwrap()).unwrap(),
            "keyProviderInfo" => {
                edk.key_provider_info = decode_base64(value.as_str().unwrap()).unwrap();
            }
            _ => anyhow::bail!("Unknown field in JSON for EDK: {key_str}"),
        }
    }
    Ok(edk)
}

pub(crate) fn parse_edks(data: &JsonValue) -> EDKs {
    let mut edks = EDKs::default();
    for value in data.as_array().unwrap() {
        edks.push(parse_edk(value).unwrap());
    }
    edks
}

pub(crate) fn parse_encryption_context(data: &JsonValue) -> EncryptionContext {
    let mut ec = EncryptionContext::default();
    for (key_str, value) in data.as_object().unwrap() {
        ec.insert(key_str.to_string(), value.as_str().unwrap().to_string());
    }
    ec
}

pub(crate) fn parse_string_list(data: &JsonValue) -> RequiredKeys {
    let mut keys = RequiredKeys::default();
    for value in data.as_array().unwrap() {
        keys.push(value.as_str().unwrap().to_string());
    }
    keys
}

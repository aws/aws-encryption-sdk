use crate::types::*;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub(crate) struct Edk {
    pub(crate) key_provider_id: String,
    pub(crate) key_provider_info: Bytes,
    pub(crate) ciphertext: Bytes,
}
pub(crate) type EDKs = Vec<Edk>;
pub(crate) type RequiredKeys = Vec<String>;
pub(crate) type Bytes = Vec<u8>;

//https://github.com/awslaspecification/blob/master/framework/test-vectors/keys-manifest.md
#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[allow(
    clippy::struct_field_names,
    reason = "because field names contain 'key'"
)]
pub(crate) struct Key {
    pub(crate) kind: String,
    pub(crate) key_id: String,
    pub(crate) alg_suite_id: String,
    pub(crate) alg: String,
    pub(crate) alg_id: EsdkAlgorithmSuiteId,
    pub(crate) encoding: String,
    pub(crate) public_key_encoding: String,
    pub(crate) recipient_material: String,
    pub(crate) sender_material: String,
    pub(crate) recipient_material_public_key: String,
    pub(crate) sender_material_public_key: String,
    pub(crate) branch_key_version: String,

    pub(crate) material: Bytes,
    pub(crate) plaintext_data_key: Bytes,
    pub(crate) beacon_key: Bytes,
    pub(crate) branch_key: Bytes,
    pub(crate) encrypt: bool,
    pub(crate) decrypt: bool,
    pub(crate) bits: u32,
    pub(crate) encryption_context: EncryptionContext,
    pub(crate) encrypted_data_keys: EDKs,
    pub(crate) required_keys: RequiredKeys,
}

pub(crate) type KeyMap = HashMap<String, Key>;
pub(crate) type AccountIDs = Vec<String>;

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub(crate) struct DiscoveryFilterConfig {
    pub(crate) partition: String,
    pub(crate) account_ids: AccountIDs,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub(crate) struct KeyDescription {
    pub(crate) kind: String,
    pub(crate) sender: String,
    pub(crate) recipient: String,
    pub(crate) sender_public_key: String,
    pub(crate) recipient_public_key: String,
    pub(crate) provider_id: String,
    pub(crate) ecc_curve: String,
    pub(crate) schema: String,
    pub(crate) key: String,
    pub(crate) encryption_algorithm: String,
    pub(crate) padding_algorithm: String,
    pub(crate) padding_hash: String,
    pub(crate) default_mrk_region: String,
    pub(crate) child_keyrings: Vec<Self>,
    pub(crate) discovery_filter: DiscoveryFilterConfig,
    pub(crate) required_keys: RequiredKeys,
    pub(crate) generator: Vec<Self>,
    pub(crate) underlying: Vec<Self>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub(crate) struct EncryptTest {
    pub(crate) name: String,
    pub(crate) kind: String,
    pub(crate) description: String,
    pub(crate) alg_suite_id: String,
    pub(crate) alg_id: EsdkAlgorithmSuiteId,
    pub(crate) decrypt_error_description: String,
    pub(crate) error_description: String,
    pub(crate) plaintext: String,
    pub(crate) ciphertext: String,
    pub(crate) result: String,

    pub(crate) frame_size: u32,

    pub(crate) encryption_context: EncryptionContext,
    pub(crate) reproduced_encryption_context: EncryptionContext,
    pub(crate) reproduced_json: JsonValue,
    pub(crate) decrypt_key_description: KeyDescription,
    pub(crate) decrypt_json: JsonValue,
    pub(crate) encrypt_key_description: KeyDescription,
    pub(crate) key_description: KeyDescription,
}
pub(crate) type EncryptTests = Vec<EncryptTest>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default, Hash)]
pub(crate) struct TestResults {
    pub(crate) total: u32,
    pub(crate) passed: u32,
    pub(crate) failed: u32,
    pub(crate) not_implemented: u32,
    pub(crate) no_kms_feature: u32,
    pub(crate) no_ddb_feature: u32,
    pub(crate) legacy_passed: u32,
    pub(crate) legacy_failed: u32,
    pub(crate) legacy_skipped: u32,
}

impl TestResults {
    pub(crate) fn fail_legacy(&mut self, test: &EncryptTest, e: &anyhow::Error) {
        self.legacy_failed += 1;
        println!(
            "Failed Test Legacy {} {} {} {e:?}",
            test.name,
            test.decrypt_key_description.kind,
            test.decrypt_key_description.encryption_algorithm
        );
    }
    pub(crate) fn fail(&mut self, test: &EncryptTest, e: &anyhow::Error) {
        self.failed += 1;
        println!(
            "Failed Test {} {} {} {e:?}",
            test.name,
            test.decrypt_key_description.kind,
            test.decrypt_key_description.encryption_algorithm
        );
    }
}

pub(crate) type PlainTexts = HashMap<String, Bytes>;

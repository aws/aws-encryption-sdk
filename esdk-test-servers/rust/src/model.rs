// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Rust mirror of the ESDK TestServer Smithy model shapes, with serde renames
//! matching each model member name and `serde_bytes` on blob members so they
//! travel as CBOR byte strings (rpcv2Cbor), not arrays.
//!
//! The polymorphic `CryptographicMaterialsManager` and `Keyring` are modeled in
//! Smithy as structures whose variant members are all optional ("tagged union
//! via optional members"); they are mirrored here as structs of `Option`s and
//! the exactly-one-set invariant is enforced at runtime in `keyring.rs`.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Operation request / response shapes

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClientRequest {
    pub config: EsdkClientConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClientResponse {
    #[serde(rename = "clientId")]
    pub client_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptRequest {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(with = "serde_bytes")]
    pub plaintext: Vec<u8>,
    #[serde(
        rename = "encryptionContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_context: Option<BTreeMap<String, String>>,
    #[serde(
        rename = "algorithmSuiteId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
    #[serde(
        rename = "frameLength",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frame_length: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptResponse {
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptRequest {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
    #[serde(
        rename = "encryptionContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_context: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptResponse {
    #[serde(with = "serde_bytes")]
    pub plaintext: Vec<u8>,
    #[serde(
        rename = "encryptionContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_context: Option<BTreeMap<String, String>>,
    #[serde(
        rename = "algorithmSuiteId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub algorithm_suite_id: Option<EsdkAlgorithmSuiteId>,
}

// Client configuration

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsdkClientConfig {
    #[serde(rename = "commitmentPolicy")]
    pub commitment_policy: EsdkCommitmentPolicy,
    #[serde(
        rename = "maxEncryptedDataKeys",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_encrypted_data_keys: Option<i64>,
    pub cmm: CryptographicMaterialsManager,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EsdkCommitmentPolicy {
    #[serde(rename = "FORBID_ENCRYPT_ALLOW_DECRYPT")]
    ForbidEncryptAllowDecrypt,
    #[serde(rename = "REQUIRE_ENCRYPT_ALLOW_DECRYPT")]
    RequireEncryptAllowDecrypt,
    #[serde(rename = "REQUIRE_ENCRYPT_REQUIRE_DECRYPT")]
    RequireEncryptRequireDecrypt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EsdkAlgorithmSuiteId {
    #[serde(rename = "ALG_AES_128_GCM_IV12_TAG16_NO_KDF")]
    Alg128GcmNoKdf,
    #[serde(rename = "ALG_AES_192_GCM_IV12_TAG16_NO_KDF")]
    Alg192GcmNoKdf,
    #[serde(rename = "ALG_AES_256_GCM_IV12_TAG16_NO_KDF")]
    Alg256GcmNoKdf,
    #[serde(rename = "ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256")]
    Alg128GcmHkdf,
    #[serde(rename = "ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256")]
    Alg192GcmHkdf,
    #[serde(rename = "ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256")]
    Alg256GcmHkdf,
    #[serde(rename = "ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256")]
    Alg128GcmHkdfEcdsa,
    #[serde(rename = "ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384")]
    Alg192GcmHkdfEcdsa,
    #[serde(rename = "ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384")]
    Alg256GcmHkdfEcdsa,
    #[serde(rename = "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY")]
    Alg256GcmCommit,
    #[serde(rename = "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384")]
    Alg256GcmCommitEcdsa,
}

// CMMs (tagged union via optional members)

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CryptographicMaterialsManager {
    #[serde(rename = "Default", default, skip_serializing_if = "Option::is_none")]
    pub default_cmm: Option<DefaultCmmConfig>,
    #[serde(
        rename = "RequiredEncryptionContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_encryption_context: Option<RequiredEncryptionContextCmmConfig>,
    #[serde(rename = "Caching", default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<CachingCmmConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultCmmConfig {
    pub keyring: Keyring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredEncryptionContextCmmConfig {
    #[serde(rename = "underlyingCMM")]
    pub underlying_cmm: Box<CryptographicMaterialsManager>,
    #[serde(rename = "requiredEncryptionContextKeys")]
    pub required_encryption_context_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingCmmConfig {
    #[serde(rename = "underlyingCMM")]
    pub underlying_cmm: Box<CryptographicMaterialsManager>,
    #[serde(rename = "cacheLimitTtlSeconds")]
    pub cache_limit_ttl_seconds: i32,
    #[serde(
        rename = "partitionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_id: Option<String>,
    #[serde(
        rename = "limitBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_bytes: Option<i64>,
    #[serde(
        rename = "limitMessages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_messages: Option<i64>,
}

// Keyrings (tagged union via optional members)

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Keyring {
    #[serde(rename = "AwsKms", default, skip_serializing_if = "Option::is_none")]
    pub aws_kms: Option<AwsKmsKeyringConfig>,
    #[serde(rename = "AwsKmsMrk", default, skip_serializing_if = "Option::is_none")]
    pub aws_kms_mrk: Option<AwsKmsMrkKeyringConfig>,
    #[serde(
        rename = "AwsKmsMultiKeyring",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_kms_multi: Option<AwsKmsMultiKeyringConfig>,
    #[serde(
        rename = "AwsKmsMrkMultiKeyring",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_kms_mrk_multi: Option<AwsKmsMrkMultiKeyringConfig>,
    #[serde(
        rename = "AwsKmsDiscovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_kms_discovery: Option<AwsKmsDiscoveryKeyringConfig>,
    #[serde(
        rename = "AwsKmsMrkDiscovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_kms_mrk_discovery: Option<AwsKmsMrkDiscoveryKeyringConfig>,
    #[serde(rename = "AwsKmsRsa", default, skip_serializing_if = "Option::is_none")]
    pub aws_kms_rsa: Option<AwsKmsRsaKeyringConfig>,
    #[serde(rename = "RawAes", default, skip_serializing_if = "Option::is_none")]
    pub raw_aes: Option<RawAesKeyringConfig>,
    #[serde(rename = "RawRsa", default, skip_serializing_if = "Option::is_none")]
    pub raw_rsa: Option<RawRsaKeyringConfig>,
    #[serde(
        rename = "AwsKmsHierarchical",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_kms_hierarchical: Option<AwsKmsHierarchicalKeyringConfig>,
    #[serde(rename = "Multi", default, skip_serializing_if = "Option::is_none")]
    pub multi: Option<MultiKeyringConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAesKeyringConfig {
    #[serde(rename = "keyNamespace")]
    pub key_namespace: String,
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "wrappingKey", with = "serde_bytes")]
    pub wrapping_key: Vec<u8>,
    #[serde(rename = "wrappingAlg")]
    pub wrapping_alg: AesWrappingAlg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AesWrappingAlg {
    #[serde(rename = "ALG_AES128_GCM_IV12_TAG16")]
    Alg128,
    #[serde(rename = "ALG_AES192_GCM_IV12_TAG16")]
    Alg192,
    #[serde(rename = "ALG_AES256_GCM_IV12_TAG16")]
    Alg256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRsaKeyringConfig {
    #[serde(rename = "keyNamespace")]
    pub key_namespace: String,
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[serde(rename = "paddingScheme")]
    pub padding_scheme: PaddingScheme,
    #[serde(
        rename = "publicKey",
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_bytes_opt"
    )]
    pub public_key: Option<Vec<u8>>,
    #[serde(
        rename = "privateKey",
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_bytes_opt"
    )]
    pub private_key: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaddingScheme {
    #[serde(rename = "PKCS1")]
    Pkcs1,
    #[serde(rename = "OAEP_SHA1_MGF1")]
    OaepSha1Mgf1,
    #[serde(rename = "OAEP_SHA256_MGF1")]
    OaepSha256Mgf1,
    #[serde(rename = "OAEP_SHA384_MGF1")]
    OaepSha384Mgf1,
    #[serde(rename = "OAEP_SHA512_MGF1")]
    OaepSha512Mgf1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsKeyringConfig {
    #[serde(rename = "kmsKeyId")]
    pub kms_key_id: String,
    #[serde(
        rename = "grantTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsMrkKeyringConfig {
    #[serde(rename = "kmsKeyId")]
    pub kms_key_id: String,
    #[serde(
        rename = "grantTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsMultiKeyringConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    #[serde(rename = "kmsKeyIds", default, skip_serializing_if = "Option::is_none")]
    pub kms_key_ids: Option<Vec<String>>,
    #[serde(
        rename = "grantTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsMrkMultiKeyringConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    #[serde(rename = "kmsKeyIds", default, skip_serializing_if = "Option::is_none")]
    pub kms_key_ids: Option<Vec<String>>,
    #[serde(
        rename = "grantTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsDiscoveryKeyringConfig {
    #[serde(
        rename = "discoveryFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discovery_filter: Option<DiscoveryFilter>,
    #[serde(
        rename = "grantTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsMrkDiscoveryKeyringConfig {
    pub region: String,
    #[serde(
        rename = "discoveryFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discovery_filter: Option<DiscoveryFilter>,
    #[serde(
        rename = "grantTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryFilter {
    pub partition: String,
    #[serde(rename = "accountIds")]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsRsaKeyringConfig {
    #[serde(rename = "kmsKeyId")]
    pub kms_key_id: String,
    #[serde(
        rename = "publicKey",
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_bytes_opt"
    )]
    pub public_key: Option<Vec<u8>>,
    #[serde(
        rename = "encryptionAlgorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_algorithm: Option<KmsRsaEncryptionAlgorithm>,
    #[serde(
        rename = "grantTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KmsRsaEncryptionAlgorithm {
    #[serde(rename = "RSAES_OAEP_SHA_1")]
    OaepSha1,
    #[serde(rename = "RSAES_OAEP_SHA_256")]
    OaepSha256,
}

/// Hierarchical keyring config: branch keys live in a DynamoDB key store and
/// are wrapped by a KMS key. The DynamoDB and KMS clients are constructed
/// server-side from the ambient AWS configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsKmsHierarchicalKeyringConfig {
    #[serde(rename = "branchKeyId")]
    pub branch_key_id: String,
    #[serde(rename = "keyStoreTableName")]
    pub key_store_table_name: String,
    #[serde(rename = "logicalKeyStoreName")]
    pub logical_key_store_name: String,
    #[serde(rename = "kmsKeyArn")]
    pub kms_key_arn: String,
    #[serde(rename = "ttlSeconds")]
    pub ttl_seconds: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiKeyringConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<Box<Keyring>>,
    #[serde(rename = "childKeyrings")]
    pub child_keyrings: Vec<Keyring>,
}

/// `serde_bytes` for `Option<Vec<u8>>` blob members.
mod serde_bytes_opt {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(bytes) => s.serialize_bytes(bytes),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u8>>, D::Error> {
        let opt: Option<serde_bytes::ByteBuf> = Option::deserialize(d)?;
        Ok(opt.map(serde_bytes::ByteBuf::into_vec))
    }
}

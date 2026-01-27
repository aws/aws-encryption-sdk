// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]
use crate::serialize::*;
use crate::utils::*;
use crate::*;
use aws_mpl_rs::EncryptedDataKey;
use aws_mpl_rs::suites::AlgorithmSuite;

// Key Value Pair to Bytes
fn serialize_one_kv_pair(data: &mut Vec<u8>, key: &str, value: &str)
//= specification/structured-encryption/header.md#key-value-pair-entries
//= type=implication
//# Each Key Value Pair MUST be serialized as follows
// | Field | Length (bytes) | Interpreted as |
// | ----- | -------------- | -------------- |
// | Key Length | 2 | big endian u16 |
// | Key | Variable. Equal to the value specified in the previous 2 bytes (Key Length). | UTF-8 encoded bytes |
// | Value Length | 2 | big endian u16 |
// | Value | Variable. Equal to the value specified in the previous 2 bytes (Value Length). | UTF-8 encoded bytes |
{
    write_u16(data, key.len() as u16);
    data.extend_from_slice(key.as_bytes());
    write_u16(data, value.len() as u16);
    data.extend_from_slice(value.as_bytes());
}

// Encryption Context to Bytes
pub(crate) fn serialize_context(data: &mut Vec<u8>, context: &EncryptionContext)
//= specification/structured-encryption/header.md#encryption-context
//= type=implication
//# The Encryption Context MUST be serialized as follows
// | Field | Length (bytes) | Interpreted as |
// | ----- | -------------- | -------------- |
// | Key Value Pair Count | 2 | big endian u16 |
// | Key Value Pair Entries | Variable. Determined by the count and length of each key-value pair. | Key Value Pair Entries |
{
    //= specification/structured-encryption/header.md#key-value-pair-entries
    //# These entries MUST have entries sorted, by key,
    //# in ascending order according to the UTF-8 encoded binary value.
    let mut items: Vec<(&String, &String)> = context.iter().collect();
    items.sort_by(|a, b| a.0.cmp(b.0));
    write_u16(data, items.len() as u16);
    for item in items {
        serialize_one_kv_pair(data, item.0, item.1);
    }
}

//   ghost const VERSION_LEN = 1
//   ghost const FLAVOR_LEN = 1
//   ghost const COMMITMENT_LEN = 32
//   ghost const PREFIX_LEN = VERSION_LEN + FLAVOR_LEN + MSGID_LEN
const VERSION_LEN: usize = 1;
const FLAVOR_LEN: usize = 1;
const COMMITMENT_LEN: usize = 32;
const PREFIX_LEN: usize = VERSION_LEN + FLAVOR_LEN + MSGID_LEN;
const U16_LIMIT: usize = u16::MAX as usize;
const U8_LIMIT: usize = u8::MAX as usize;
//   ghost const u8_LIMIT = 256
const U8_LIMIT64: u64 = 256;
pub(crate) const ENCRYPT_AND_SIGN_LEGEND: u8 = 0x65;
pub(crate) const SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT_LEGEND: u8 = 0x63;
pub(crate) const SIGN_ONLY_LEGEND: u8 = 0x73;

//= specification/structured-encryption/header.md#format-version
//= type=implication
//# The Version MUST be `0x01` or `0x02`.
type Version = u8;
const fn valid_version(x: u8) -> bool {
    x == 1 || x == 2
}

type Flavor = u8;
//= specification/structured-encryption/header.md#format-flavor
//= type=implication
//# The algorithm suite indicated by the flavor MUST be a
//# [DBE supported algorithm suite](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum).
// | Value | Algorithm Suite ID | Algorithm Suite Enum |
// |---|---|---|
// | 0x00 | 0x67 0x00 | ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384 |
// | 0x01 | 0x67 0x01 | ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384 |
const fn valid_flavor(x: u8) -> bool {
    x == 0 || x == 1
}

pub(crate) type LegendByte = u8;

const fn valid_legend_byte(x: u8) -> bool {
    x == ENCRYPT_AND_SIGN_LEGEND
        || x == SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT_LEGEND
        || x == SIGN_ONLY_LEGEND
}

/*

  //= specification/structured-encryption/header.md#encrypted-data-key-count
  //= type=implication
  //# This value MUST be greater than 0.
  type CMPEncryptedDataKeyList = x : seq<CMPEncryptedDataKey> | 0 < |x| < u8_LIMIT witness *

  type Commitment = x: Bytes | |x| == COMMITMENT_LEN witness *
  type CMPEncryptedDataKey = x : CMP.EncryptedDataKey | ValidEncryptedDataKey(x) witness *
  type CMPEncryptionContext  = x : CMP.EncryptionContext | ValidEncryptionContext(x) witness *
  type CMPEncryptedDataKeyListEmptyOK = x : seq<CMPEncryptedDataKey> | |x| < u8_LIMIT
  type LegendByte = x : u8 | ValidLegendByte(x) witness SIGN_ONLY_LEGEND
  type Legend = x : seq<LegendByte> | |x| < U16_LIMIT
  type CMPUtf8Bytes = x : CMP.Utf8Bytes | |x| < U16_LIMIT
*/
fn is_version2_schema(data: &[CanonCryptoItem]) -> bool {
    for item in data {
        if item.action == CryptoAction::SignAndIncludeInEncryptionContext {
            return true;
        }
    }
    false
}
fn version_from_schema(data: &[CanonCryptoItem]) -> Version {
    if is_version2_schema(data) { 2 } else { 1 }
}

fn valid_encryption_context(x: &EncryptionContext) -> bool {
    if x.len() > U16_LIMIT {
        return false;
    }

    for (key, value) in x {
        if key.len() > U16_LIMIT || value.len() > U16_LIMIT {
            return false;
        }
    }
    true
}

const fn valid_encrypted_data_key(x: &EncryptedDataKey) -> bool {
    x.key_provider_id.len() < U16_LIMIT
        && x.key_provider_info.len() < U16_LIMIT
        && x.ciphertext.len() < U16_LIMIT
}

// header without commitment
pub(crate) struct PartialHeader {
    pub(crate) version: Version,
    pub(crate) flavor: Flavor,
    pub(crate) msg_id: MessageID,
    pub(crate) legend: Vec<LegendByte>,
    pub(crate) enc_context: EncryptionContext,
    pub(crate) data_keys: Vec<EncryptedDataKey>,
}

impl PartialHeader {
    // PartialHeader to Bytes
    fn serialize(&self, data: &mut Vec<u8>)
    //= specification/structured-encryption/header.md#partial-header
    //= type=implication
    //# The Partial Header MUST be
    // | Length (bytes) | Meaning |
    // |---|---|
    // | 1 | [Format Version](#format-version) |
    // | 1 | [Format Flavor](#format-flavor) |
    // | 32 | [Message ID](#message-id) |
    // | Variable | [Encrypt Legend](#encrypt-legend) |
    // | Variable | [Encryption Context](#encryption-context) |
    // | Variable | [Encrypted Data Keys](#encrypted-data-keys) |
    {
        data.push(self.version);
        data.push(self.flavor);
        data.extend_from_slice(&self.msg_id);
        serialize_legend(data, &self.legend);
        serialize_context(data, &self.enc_context);
        serialize_data_keys(data, &self.data_keys);
    }

    fn get_algorithm_suite(&self) -> Result<&'static AlgorithmSuite, Error> {
        let suite =
            aws_mpl_rs::suites::get_algorithm_suite_info([DBE_ALGORITHM_FAMILY, self.flavor])?;
        Ok(suite)
    }
}

// Legend to Bytes
fn serialize_legend(data: &mut Vec<u8>, x: &[LegendByte])
//= specification/structured-encryption/header.md#encrypt-legend
//= type=implication
//# The Encrypt Legend MUST be serialized as
// | Field | Length (bytes) | Interpreted as |
// | ----- | -------------- | -------------- |
// | Encrypt Legend Length | 2 | big endian u16 |
// | Encrypt Legend Bytes | Variable. Equal to the value specified in the previous 2 bytes | Bytes |
{
    write_u16(data, x.len() as u16);
    data.extend_from_slice(x);
}

// Data Key to Bytes
fn serialize_one_data_key(data: &mut Vec<u8>, k: &EncryptedDataKey)
//= specification/structured-encryption/header.md#encrypted-data-key-entries
//= type=implication
//# Each Encrypted Data Key MUST be serialized as follows
// | Field | Length (bytes) | Interpreted as |
// | ----- | -------------- | -------------- |
// | Key Provider ID Length  | 2 | big endian u16 |
// | Key Provider ID | Variable. Equal to the value specified in the previous 2 bytes (Key Provider ID Length). | UTF-8 encoded bytes |
// | Key Provider Information Length | 2 | big endian u16 |
// | Key Provider Information | Variable. Equal to the value specified in the previous 2 bytes (Key Provider Information Length). | Bytes |
// | Encrypted Data Key Length | 2 | big endian u16 |
// | Encrypted Data Key | Variable. Equal to the value specified in the previous 2 bytes (Encrypted Data Key Length). | Bytes |
{
    write_u16(data, k.key_provider_id.len() as u16);
    data.extend_from_slice(k.key_provider_id.as_bytes());
    write_u16(data, k.key_provider_info.len() as u16);
    data.extend_from_slice(&k.key_provider_info);
    write_u16(data, k.ciphertext.len() as u16);
    data.extend_from_slice(&k.ciphertext);
}

// Data Key List to Bytes
fn serialize_data_keys(data: &mut Vec<u8>, x: &[EncryptedDataKey])
//= specification/structured-encryption/header.md#encrypted-data-keys
//= type=implication
//# The Encrypted Data Keys MUST be serialized as follows
// | Field | Length (bytes) | Interpreted as |
// | ----- | -------------- | -------------- |
// | Encrypted Data Key Count | 1 | big endian u16 |
// | [Encrypted Data Key Entries | Variable. Determined by the count and length of each key-value pair. | Encrypted Data Key Entries |
{
    data.push(x.len() as u8);
    for key in x {
        serialize_one_data_key(data, key);
    }
}

// Calculate key commitment. Fail if it doesn't match the stored one.
fn verify_commitment(alg: &AlgorithmSuite, commit_key: &[u8], data: &[u8]) -> Result<(), Error>
//= specification/structured-encryption/header.md#commitment-verification
//= type=implication
//# Header commitment comparisons MUST be constant time operations.
{
    let stored_commitment = &data[data.len() - COMMITMENT_LEN..];
    let calc_commitment =
        calculate_header_commitment(alg, commit_key, &data[..data.len() - COMMITMENT_LEN]);
    need(
        aws_mpl_primitives::constant_time_equal(stored_commitment, &calc_commitment),
        "Key commitment mismatch.",
    )?;
    Ok(())
}

// serialize and add commitment
pub(crate) fn serialize(
    alg: &AlgorithmSuite,
    commit_key: &[u8],
    partial_header: &PartialHeader,
) -> Vec<u8>
//= specification/structured-encryption/header.md#full-header-value
//= type=implication
//# The value of the header MUST be
// | Length (bytes) | Meaning |
// |---|---|
// | Variable | [Partial Header](#partial-header) |
// | 32 | [Header Commitment](#header-commitment) |
{
    let mut body = Vec::new();
    partial_header.serialize(&mut body);
    let commitment = calculate_header_commitment(alg, commit_key, &body);
    body.extend_from_slice(&commitment);
    body
}

// config to PartialHeader
fn create(
    schema: &[CanonCryptoItem],
    msg_id: &MessageID,
    mat: &aws_mpl_rs::EncryptionMaterials,
) -> Result<PartialHeader, Error>
//= specification/structured-encryption/header.md#format-version
    //= type=implication
    //# If any [Crypto Action](./structures.md#crypto-action) is configured as
    //# [SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT Crypto Action](./structures.md#sign_and_include_in_encryption_context)
    //# the Version MUST be 0x02; otherwise, Version MUST be 0x01.
{
    need(
        valid_encryption_context(&mat.encryption_context),
        "Invalid Encryption Context",
    )?;
    need(
        !mat.encrypted_data_keys.is_empty(),
        "There must be at least one data key",
    )?;
    need(
        mat.encrypted_data_keys.len() < U8_LIMIT,
        "Too many data keys.",
    )?;
    need(
        mat.encrypted_data_keys.iter().all(valid_encrypted_data_key),
        "Invalid Data Key",
    )?;
    need(
        mat.algorithm_suite.binary_id[0] == DBE_ALGORITHM_FAMILY,
        "Algorithm Suite not suitable for structured encryption.",
    )?;
    need(
        valid_flavor(mat.algorithm_suite.binary_id[1]),
        "Algorithm Suite has unexpected flavor.",
    )?;
    let legend = make_legend(schema);

    //= specification/structured-encryption/encrypt-path-structure.md#header-field
    //# The encryption context field serialized in the header MUST contain all key-value
    //# pairs of the encryption context in the [encryption materials](#retrieve-encryption-materials)
    //# that are not included in the
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys) list.
    let stored_ec: EncryptionContext = mat
        .encryption_context
        .iter()
        .filter(|(key, _)| !mat.required_encryption_context_keys.contains(key))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    Ok(PartialHeader {
        version: version_from_schema(schema),
        flavor: mat.algorithm_suite.binary_id[1],
        msg_id: *msg_id,
        legend,
        enc_context: stored_ec,
        data_keys: mat.encrypted_data_keys.clone(),
    })
}

// bytes to PartialHeader, i.e. does not look at commitment -- Deserialize does that
pub(crate) fn partial_deserialize(data: &mut dyn SafeRead) -> Result<PartialHeader, Error> {
    let version = read_u8(data)?;
    need(valid_version(version), "Invalid Version Number")?;
    let flavor = read_u8(data)?;
    need(valid_flavor(flavor), "Invalid Flavor")?;
    let msg_id = read_array::<MSGID_LEN>(data)?;
    let legend = get_legend(data)?;
    let enc_context = get_context(data)?;
    let data_keys = get_data_keys(data)?;

    Ok(PartialHeader {
        version,
        flavor,
        msg_id,
        legend,
        enc_context,
        data_keys,
    })
}

fn get_hmac_alg(x: aws_mpl_rs::suites::DerivationAlgorithm) -> aws_mpl_primitives::DigestAlg {
    match x {
        aws_mpl_rs::suites::DerivationAlgorithm::Hkdf(x) => x.hmac,
        _ => panic!(),
    }
}

// calculate Hmac384 for header commitment
fn calculate_header_commitment(
    alg: &AlgorithmSuite,
    commit_key: &[u8],
    data: &[u8],
) -> [u8; COMMITMENT_LEN]
//= specification/structured-encryption/header.md#commitment-calculation
//= type=implication
//# The Header Commitment MUST be calculated as a the first 32 bytes of an HmacSha384,
//# with the serialized partial header as the message, and the Commit Key as the key.
{
    let hash = aws_mpl_primitives::hmac(get_hmac_alg(alg.commitment), commit_key, data);
    hash.try_into().unwrap()
}

// Create a Legend from the Schema
fn make_legend(data: &[CanonCryptoItem]) -> Vec<LegendByte>
//= specification/structured-encryption/header.md#encrypt-legend-bytes
              //= type=implication
              //# The length of this serialized value (in bytes) MUST equal the number of authenticated fields indicated
              //# by the caller's [Authenticate Schema](./structures.md#authenticate-schema).
{
    let mut serialized = Vec::new();
    for item in data {
        if let Some(legend) = get_action_legend(item.action) {
            serialized.push(legend);
        }
    }
    serialized
}

// CryptoAction to bytes. One byte for signed, zero bytes for unsigned
const fn get_action_legend(x: CryptoAction) -> Option<LegendByte>
//= specification/structured-encryption/header.md#encrypt-legend-bytes
    //= type=implication
    //# Each Crypto Action MUST be encoded as follows
    // - `0x65` (`e` in UTF-8, for "Encrypt and Sign") means that a particular field was encrypted
    //   and included in the signature calculation.
    //   This indicates that this field will be attempted to be decrypted during decryption.
    // - `0x73` (`s` in UTF-8, for "Sign Only") means that a particular field was not encrypted,
    //   but still included in the signature calculation.
    //   This indicates that this field will not be attempted to be decrypted during decryption.
    // - `0x63` (`c` in UTF-8, for "Context") means that a particular field was not encrypted,
    //   but still included in the signature calculation,
    //   as well as being included in the encryption context.
    //   This indicates that this field MUST NOT be attempted to be decrypted during decryption.    
    // - no entry if the attribute is not signed
{
    match x {
        CryptoAction::EncryptAndSign => Some(ENCRYPT_AND_SIGN_LEGEND),
        CryptoAction::SignAndIncludeInEncryptionContext => {
            Some(SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT_LEGEND)
        }
        CryptoAction::SignOnly => Some(SIGN_ONLY_LEGEND),
        CryptoAction::DoNothing => None,
    }
}

// Bytes to Legend
fn get_legend(data: &mut dyn SafeRead) -> Result<Vec<LegendByte>, Error> {
    let legend = read_seq_u16(data)?;
    need(
        legend.iter().all(|&x| valid_legend_byte(x)),
        "Invalid byte in stored legend",
    )?;
    Ok(legend)
}

// Bytes to Encryption Context
fn get_context(data: &mut dyn SafeRead) -> Result<EncryptionContext, Error> {
    let count = read_u16(data)?;
    let mut ret = EncryptionContext::new();
    let mut prev_key = String::new();
    for _ in 0..count {
        let key = read_str_u16(data)?;
        let value = read_str_u16(data)?;

        //= specification/structured-encryption/header.md#key-value-pair-entries
        //# This sequence MUST NOT contain duplicate entries.
        // if the previous key is always less than the current key, there can be no duplicates

        //= specification/structured-encryption/header.md#key-value-pair-entries
        //# These entries MUST have entries sorted, by key,
        //# in ascending order according to the UTF-8 encoded binary value.
        need(prev_key < key, "Context keys out of order")?;

        prev_key.clone_from(&key);
        ret.insert(key, value);
    }
    Ok(ret)
}

// Bytes to Data Key
fn get_one_data_key(data: &mut dyn SafeRead) -> Result<EncryptedDataKey, Error> {
    let key_provider_id = read_str_u16(data)?;
    let key_provider_info = read_seq_u16(data)?;
    let ciphertext = read_seq_u16(data)?;
    Ok(EncryptedDataKey::new(
        key_provider_id,
        key_provider_info,
        ciphertext,
    ))
}

// Bytes to Data Key List
fn get_data_keys(data: &mut dyn SafeRead) -> Result<Vec<EncryptedDataKey>, Error> {
    let count = read_u8(data)?;
    let mut keys: Vec<EncryptedDataKey> = Vec::with_capacity(count as usize);
    for _ in 0..count {
        keys.push(get_one_data_key(data)?);
    }
    Ok(keys)
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]

mod fixtures;

use aws_esdk::*;
use fixtures::*;

pub const IV_LEN: usize = 12;
pub const TAG_LEN: usize = 16;
pub const ENDFRAME_MARKER: [u8; 4] = 0xFFFF_FFFFu32.to_be_bytes();

#[derive(Clone, Copy, Debug)]
pub enum Version {
    V1,
    V2,
}

pub const VERSIONS: [Version; 2] = [Version::V1, Version::V2];

use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::dafny::types::keyring::KeyringRef;
use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId;

/// Create a raw AES keyring for testing (no KMS needed).
pub async fn test_keyring() -> KeyringRef {
    let (ns, name) = namespace_and_name(0);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([0u8; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

/// Create a raw AES keyring with key material derived from `n`.
pub async fn aes_keyring(n: u8) -> KeyringRef {
    let (ns, name) = namespace_and_name(n);
    mpl()
        .create_raw_aes_keyring()
        .key_namespace(ns)
        .key_name(name)
        .wrapping_key(aws_smithy_types::Blob::new([n; 32]))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap()
}

/// Create a KMS keyring backed by the shared `KEY_ARN` test key in us-west-2.
///
/// Requires AWS credentials with permission to call `kms:GenerateDataKey` and
/// `kms:Decrypt` against the test key. Tests that only need a round-trip
/// keyring without specifically exercising KMS should prefer `test_keyring()`
/// (raw AES) instead.
pub async fn kms_keyring() -> KeyringRef {
    let mpl = mpl();
    let client_supplier = mpl.create_default_client_supplier().send().await.unwrap();
    let kms_client = client_supplier
        .get_client()
        .region("us-west-2")
        .send()
        .await
        .unwrap();
    mpl.create_aws_kms_keyring()
        .kms_key_id(KEY_ARN)
        .kms_client(kms_client)
        .send()
        .await
        .unwrap()
}

/// Create a multi-keyring from a generator and child keyrings.
pub async fn multi_keyring(generator_kr: KeyringRef, children: Vec<KeyringRef>) -> KeyringRef {
    mpl()
        .create_multi_keyring()
        .generator(generator_kr)
        .child_keyrings(children)
        .send()
        .await
        .unwrap()
}

/// Encrypt with defaults, return full EncryptOutput.
pub async fn encrypt_default(plaintext: &[u8]) -> EncryptOutput {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap()
}

/// Encrypt with a V1 (non-committing) algorithm suite, return ciphertext bytes.
pub async fn encrypt_v1(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a V1 (non-committing) algorithm suite and encryption context, return ciphertext bytes.
pub async fn encrypt_v1_with_ec(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with default (V2) algorithm suite, return ciphertext bytes.
pub async fn encrypt_v2(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt plaintext with a given frame length, return ciphertext bytes.
pub async fn encrypt_with_frame_length(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.frame_length = FrameLength::new(frame_length).unwrap();
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt plaintext with a given frame length under a V1 (non-committing) algorithm
/// suite, return ciphertext bytes.
pub async fn encrypt_v1_with_frame_length(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    input.frame_length = FrameLength::new(frame_length).unwrap();
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a signing algorithm suite, return ciphertext bytes.
pub async fn encrypt_with_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a V1 signing algorithm suite, return ciphertext bytes.
pub async fn encrypt_with_v1_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384);
    input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a non-signing algorithm suite, return ciphertext bytes.
pub async fn encrypt_without_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a specific suite, policy, and keyring, return ciphertext bytes.
pub async fn encrypt_with_suite(
    plaintext: &[u8],
    suite: EsdkAlgorithmSuiteId,
    policy: EsdkCommitmentPolicy,
    keyring: &KeyringRef,
) -> Vec<u8> {
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id = Some(suite);
    enc_input.commitment_policy = policy;
    encrypt(&enc_input).await.unwrap().ciphertext
}

/// Encrypt with a specific version and keyring, return ciphertext bytes.
pub async fn encrypt_with_version(
    plaintext: &[u8],
    version: Version,
    keyring: KeyringRef,
) -> Vec<u8> {
    let mut input = EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    if let Version::V1 = version {
        input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
        input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    }
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a non-signing algorithm suite for the given version, with a
/// caller-supplied encryption context. Returns ciphertext bytes.
///
/// Non-signing suites are used so the serialized encryption context matches
/// exactly what the caller provided; signing suites add an
/// `aws-crypto-public-key` entry that alters the EC seen in the header.
pub async fn encrypt_no_sign_with_ec(
    plaintext: &[u8],
    ec: EncryptionContext,
    version: Version,
) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring);
    match version {
        Version::V1 => {
            input.algorithm_suite_id =
                Some(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256);
            input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        Version::V2 => {
            input.algorithm_suite_id = Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
        }
    }
    encrypt(&input).await.unwrap().ciphertext
}

/// Decrypt ciphertext with the default test keyring, return full DecryptOutput.
pub async fn decrypt_ciphertext(ciphertext: &[u8]) -> DecryptOutput {
    let keyring = test_keyring().await;
    let dec_input =
        DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap()
}

/// Decrypt ciphertext with the default test keyring, returning the Result
/// (for negative/tamper tests that expect failure).
pub async fn decrypt_ciphertext_result(ciphertext: &[u8]) -> Result<DecryptOutput, Error> {
    let keyring = test_keyring().await;
    let dec_input =
        DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await
}

/// Decrypt with a specific policy and keyring, return full DecryptOutput.
pub async fn decrypt_with(
    ciphertext: &[u8],
    policy: EsdkCommitmentPolicy,
    keyring: &KeyringRef,
) -> DecryptOutput {
    let mut dec_input =
        DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring.clone());
    dec_input.commitment_policy = policy;
    decrypt(&dec_input).await.unwrap()
}

/// Decrypt a ciphertext that was produced by `encrypt_no_sign_with_ec` (or
/// otherwise uses a V1 non-committing suite for V1). Returns the full
/// DecryptOutput so callers can assert on both plaintext and
/// `encryption_context`.
pub async fn decrypt_with_version(ciphertext: &[u8], version: Version) -> DecryptOutput {
    let keyring = test_keyring().await;
    let mut dec_input =
        DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring);
    if let Version::V1 = version {
        dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    }
    decrypt(&dec_input).await.unwrap()
}

/// Encrypt then decrypt round-trip, return decrypted plaintext.
pub async fn round_trip(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt then decrypt with a given frame length, return decrypted plaintext.
pub async fn round_trip_framed(plaintext: &[u8], frame_length: u32) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.frame_length = FrameLength::new(frame_length).unwrap();
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt V1 then decrypt, returning decrypted plaintext.
pub async fn round_trip_v1(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let ct = encrypt_v1_with_ec(plaintext, ec).await;
    let mut dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt V2 then decrypt, returning decrypted plaintext.
pub async fn round_trip_v2(plaintext: &[u8], ec: EncryptionContext) -> Vec<u8> {
    let keyring = test_keyring().await;
    let input = EncryptInput::with_legacy_keyring(plaintext, ec, keyring.clone());
    let ct = encrypt(&input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt then decrypt with a signing algorithm suite.
pub async fn round_trip_signing(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut enc_input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring.clone());
    enc_input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap().plaintext
}

/// Encrypt then decrypt with encryption context, returning full DecryptOutput.
pub async fn round_trip_with_ec(plaintext: &[u8], ec: EncryptionContext) -> DecryptOutput {
    let keyring = test_keyring().await;
    let enc_input = EncryptInput::with_legacy_keyring(plaintext, ec.clone(), keyring.clone());
    let ct = encrypt(&enc_input).await.unwrap().ciphertext;
    let dec_input = DecryptInput::with_legacy_keyring(&ct, ec, keyring);
    decrypt(&dec_input).await.unwrap()
}

/// Find the start of the message body by scanning for the first frame.
/// Returns the byte offset where the first frame begins.
pub fn find_body_start(ct: &[u8], frame_length: u32) -> Option<usize> {
    let seq_one = 1u32.to_be_bytes();
    // The first frame is either:
    //   - A regular frame starting with SeqNum=1
    //   - A final frame starting with 0xFFFFFFFF followed by SeqNum=1
    // Try to find ENDFRAME+SeqNum=1 (final frame as first frame)
    // or SeqNum=1 followed by valid frame data (regular frame)
    for i in 0..ct.len().saturating_sub(4) {
        // Check if this is the start of a final frame (ENDFRAME + seq=1)
        if i + 8 <= ct.len() && ct[i..i + 4] == ENDFRAME_MARKER && ct[i + 4..i + 8] == seq_one {
            return Some(i);
        }
        // Check if this is the start of a regular frame (seq=1)
        // Validate by checking that walking through regular frames leads to ENDFRAME
        if ct[i..i + 4] == seq_one && validate_frame_walk(ct, i, frame_length) {
            return Some(i);
        }
    }
    None
}

/// Validate that starting at `offset` and walking regular frames leads to an ENDFRAME marker.
pub fn validate_frame_walk(ct: &[u8], offset: usize, frame_length: u32) -> bool {
    let regular_frame_size = 4 + IV_LEN + frame_length as usize + TAG_LEN;
    let mut pos = offset;
    loop {
        if pos + 4 > ct.len() {
            return false;
        }
        if ct[pos..pos + 4] == ENDFRAME_MARKER {
            return true;
        }
        // Try to advance past a regular frame
        let next = pos + regular_frame_size;
        if next > ct.len() {
            return false;
        }
        pos = next;
    }
}

/// Count regular and final frames in the ciphertext.
/// Returns `(regular_frame_count, final_frame_count)`.
pub fn count_frames(ct: &[u8], frame_length: u32) -> (usize, usize) {
    let body_start =
        find_body_start(ct, frame_length).expect("could not find body start in ciphertext");
    let regular_frame_size = 4 + IV_LEN + frame_length as usize + TAG_LEN;
    let mut pos = body_start;
    let mut regular = 0usize;
    let mut final_count = 0usize;

    loop {
        if pos + 4 > ct.len() {
            break;
        }
        if ct[pos..pos + 4] == ENDFRAME_MARKER {
            // Final frame: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLength(4) + Content(N) + Tag(16)
            if pos + 24 > ct.len() {
                break;
            }
            let _content_len =
                u32::from_be_bytes([ct[pos + 20], ct[pos + 21], ct[pos + 22], ct[pos + 23]])
                    as usize;
            final_count += 1;
            break; // Final frame is always last
        } else {
            // Regular frame: SeqNum(4) + IV(12) + Content(frame_length) + Tag(16)
            regular += 1;
            pos += regular_frame_size;
        }
    }

    (regular, final_count)
}

/// Extract the content length from the final frame in the ciphertext.
pub fn final_frame_content_length(ct: &[u8]) -> Option<u32> {
    for i in 0..ct.len().saturating_sub(24) {
        if ct[i..i + 4] == ENDFRAME_MARKER {
            return Some(u32::from_be_bytes([
                ct[i + 20],
                ct[i + 21],
                ct[i + 22],
                ct[i + 23],
            ]));
        }
    }
    None
}

/// Parse frames from ciphertext. Returns vec of (seq_num, iv, encrypted_content, auth_tag, is_final).
pub type ParsedFrame = (u32, Vec<u8>, Vec<u8>, Vec<u8>, bool);

pub fn parse_frames(ct: &[u8], frame_length: u32) -> Vec<ParsedFrame> {
    let body_start = find_body_start(ct, frame_length).expect("could not find body start");
    let mut pos = body_start;
    let mut frames = Vec::new();

    loop {
        if pos + 4 > ct.len() {
            break;
        }
        let first4 = u32::from_be_bytes([ct[pos], ct[pos + 1], ct[pos + 2], ct[pos + 3]]);
        if first4 == 0xFFFF_FFFF {
            // Final frame: ENDFRAME(4) + SeqNum(4) + IV(12) + ContentLen(4) + Content(N) + Tag(16)
            pos += 4;
            let seq = u32::from_be_bytes([ct[pos], ct[pos + 1], ct[pos + 2], ct[pos + 3]]);
            pos += 4;
            let iv = ct[pos..pos + IV_LEN].to_vec();
            pos += IV_LEN;
            let content_len =
                u32::from_be_bytes([ct[pos], ct[pos + 1], ct[pos + 2], ct[pos + 3]]) as usize;
            pos += 4;
            let enc_content = ct[pos..pos + content_len].to_vec();
            pos += content_len;
            let tag = ct[pos..pos + TAG_LEN].to_vec();
            frames.push((seq, iv, enc_content, tag, true));
            break;
        } else {
            // Regular frame: SeqNum(4) + IV(12) + Content(frame_length) + Tag(16)
            let seq = first4;
            pos += 4;
            let iv = ct[pos..pos + IV_LEN].to_vec();
            pos += IV_LEN;
            let enc_content = ct[pos..pos + frame_length as usize].to_vec();
            pos += frame_length as usize;
            let tag = ct[pos..pos + TAG_LEN].to_vec();
            pos += TAG_LEN;
            frames.push((seq, iv, enc_content, tag, false));
        }
    }
    frames
}

/// A single parsed message-body frame with on-wire offsets and raw byte slices
/// for every field. Designed for byte-level wire-format tests so each test can
/// assert on individual bytes, decoded values, and field offsets without
/// reinventing the walk.
///
/// Field layouts per spec/data-format/message-body.md:
/// - Regular frame: `SeqNum(4) || IV(IV_LEN) || EncryptedContent(frame_length) || AuthTag(TAG_LEN)`
/// - Final frame:   `SeqNumEnd(4) || SeqNum(4) || IV(IV_LEN) || EncContentLen(4) || EncryptedContent(N) || AuthTag(TAG_LEN)`
#[derive(Debug)]
pub struct ParsedFrameFields<'a> {
    /// Offset where this frame begins in the ciphertext.
    pub frame_offset: usize,
    /// Offset immediately after this frame ends.
    pub end_offset: usize,
    /// True if this is a final frame.
    pub is_final: bool,

    /// For final frames: offset of the 4-byte 0xFFFFFFFF sequence-number-end marker.
    pub endframe_marker_offset: Option<usize>,
    /// For final frames: on-wire bytes of the sequence-number-end marker (always 0xFF FF FF FF).
    pub endframe_marker_bytes: Option<&'a [u8]>,

    /// Offset of the 4-byte sequence number field.
    pub seq_num_offset: usize,
    /// On-wire bytes of the sequence number (4 bytes, big-endian UInt32).
    pub seq_num_bytes: &'a [u8],
    /// Decoded sequence number value.
    pub seq_num: u32,

    /// Offset of the IV field.
    pub iv_offset: usize,
    /// On-wire IV bytes (IV_LEN bytes).
    pub iv: &'a [u8],

    /// For final frames: offset of the 4-byte encrypted-content-length field.
    pub content_length_offset: Option<usize>,
    /// For final frames: on-wire bytes of the encrypted-content-length field (4 bytes, big-endian UInt32).
    pub content_length_bytes: Option<&'a [u8]>,
    /// Decoded encrypted-content length (= frame_length for regular frames).
    pub content_length: u32,

    /// Offset of the encrypted content.
    pub content_offset: usize,
    /// On-wire encrypted-content bytes (content_length bytes).
    pub content: &'a [u8],

    /// Offset of the auth tag.
    pub tag_offset: usize,
    /// On-wire auth-tag bytes (TAG_LEN bytes).
    pub tag: &'a [u8],
}

/// Parse one frame starting at `offset` in `ct`. The frame's type (regular vs
/// final) is determined by whether the first 4 bytes match the ENDFRAME marker.
/// Panics if the ciphertext is malformed at `offset`.
pub fn parse_frame_at(ct: &[u8], offset: usize, frame_length: u32) -> ParsedFrameFields<'_> {
    assert!(
        offset + 4 <= ct.len(),
        "parse_frame_at: not enough bytes at offset {} for frame header",
        offset
    );
    let first4 = u32::from_be_bytes([ct[offset], ct[offset + 1], ct[offset + 2], ct[offset + 3]]);
    if first4 == 0xFFFF_FFFF {
        // Final frame: SeqNumEnd(4) + SeqNum(4) + IV(IV_LEN) + ContentLen(4) + Content(N) + Tag(TAG_LEN)
        let endframe_off = offset;
        let seq_num_off = endframe_off + 4;
        let iv_off = seq_num_off + 4;
        let content_len_off = iv_off + IV_LEN;
        let content_off = content_len_off + 4;
        let seq_num_bytes = &ct[seq_num_off..seq_num_off + 4];
        let seq_num = u32::from_be_bytes([
            seq_num_bytes[0],
            seq_num_bytes[1],
            seq_num_bytes[2],
            seq_num_bytes[3],
        ]);
        let content_length_bytes = &ct[content_len_off..content_len_off + 4];
        let content_length = u32::from_be_bytes([
            content_length_bytes[0],
            content_length_bytes[1],
            content_length_bytes[2],
            content_length_bytes[3],
        ]);
        let tag_off = content_off + content_length as usize;
        let end = tag_off + TAG_LEN;
        assert!(
            end <= ct.len(),
            "parse_frame_at: final-frame end ({}) exceeds ciphertext length ({})",
            end,
            ct.len()
        );
        ParsedFrameFields {
            frame_offset: offset,
            end_offset: end,
            is_final: true,
            endframe_marker_offset: Some(endframe_off),
            endframe_marker_bytes: Some(&ct[endframe_off..endframe_off + 4]),
            seq_num_offset: seq_num_off,
            seq_num_bytes,
            seq_num,
            iv_offset: iv_off,
            iv: &ct[iv_off..iv_off + IV_LEN],
            content_length_offset: Some(content_len_off),
            content_length_bytes: Some(content_length_bytes),
            content_length,
            content_offset: content_off,
            content: &ct[content_off..content_off + content_length as usize],
            tag_offset: tag_off,
            tag: &ct[tag_off..tag_off + TAG_LEN],
        }
    } else {
        // Regular frame: SeqNum(4) + IV(IV_LEN) + Content(frame_length) + Tag(TAG_LEN)
        let seq_num_off = offset;
        let iv_off = seq_num_off + 4;
        let content_off = iv_off + IV_LEN;
        let tag_off = content_off + frame_length as usize;
        let end = tag_off + TAG_LEN;
        assert!(
            end <= ct.len(),
            "parse_frame_at: regular-frame end ({}) exceeds ciphertext length ({})",
            end,
            ct.len()
        );
        let seq_num_bytes = &ct[seq_num_off..seq_num_off + 4];
        let seq_num = u32::from_be_bytes([
            seq_num_bytes[0],
            seq_num_bytes[1],
            seq_num_bytes[2],
            seq_num_bytes[3],
        ]);
        ParsedFrameFields {
            frame_offset: offset,
            end_offset: end,
            is_final: false,
            endframe_marker_offset: None,
            endframe_marker_bytes: None,
            seq_num_offset: seq_num_off,
            seq_num_bytes,
            seq_num,
            iv_offset: iv_off,
            iv: &ct[iv_off..iv_off + IV_LEN],
            content_length_offset: None,
            content_length_bytes: None,
            content_length: frame_length,
            content_offset: content_off,
            content: &ct[content_off..content_off + frame_length as usize],
            tag_offset: tag_off,
            tag: &ct[tag_off..tag_off + TAG_LEN],
        }
    }
}

/// Parse all frames in the message body starting from `find_body_start`.
/// Returns one `ParsedFrameFields` per frame, in order, ending at the final frame.
pub fn parse_all_frames(ct: &[u8], frame_length: u32) -> Vec<ParsedFrameFields<'_>> {
    let mut pos = find_body_start(ct, frame_length).expect("could not find body start");
    let mut frames = Vec::new();
    loop {
        let f = parse_frame_at(ct, pos, frame_length);
        let is_final = f.is_final;
        pos = f.end_offset;
        frames.push(f);
        if is_final {
            break;
        }
    }
    frames
}

/// Locate the final frame in `ct` and return its parsed field record.
pub fn parse_final_frame(ct: &[u8], frame_length: u32) -> ParsedFrameFields<'_> {
    parse_all_frames(ct, frame_length)
        .pop()
        .expect("ciphertext must contain at least a final frame")
}

/// Parse V1 header trailing field offsets from ciphertext.
/// Returns (content_type_offset, reserved_offset, iv_length_offset, frame_length_offset).
pub fn parse_v1_trailing_offsets(ct: &[u8]) -> (usize, usize, usize, usize) {
    // V1 header: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) = 20 fixed bytes
    let mut pos: usize = 20;

    // AAD: 2-byte length, then if non-zero: aad_byte_len bytes (which include the 2-byte kv_count)
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += aad_byte_len;
    }

    // EDKs: 2-byte count, then for each: provider_id(2+len) + provider_info(2+len) + ciphertext(2+len)
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }

    let content_type_offset = pos;
    let reserved_offset = pos + 1;
    let iv_length_offset = pos + 1 + 4;
    let frame_length_offset = pos + 1 + 4 + 1;
    (
        content_type_offset,
        reserved_offset,
        iv_length_offset,
        frame_length_offset,
    )
}

/// Parse the V2 header body fields from ciphertext bytes, returning the byte offset
/// after each field boundary in order. Panics if the header is not well-formed.
/// Returns a Vec of (field_name, start_offset, end_offset) tuples.
pub fn parse_v2_header_field_offsets(ct: &[u8]) -> Vec<(&'static str, usize, usize)> {
    let mut fields = Vec::new();
    let mut pos = 0;

    // Version: 1 byte (must be 0x02 for V2)
    assert!(pos < ct.len(), "not enough bytes for Version");
    assert_eq!(ct[pos], 0x02, "expected V2 version byte");
    fields.push(("Version", pos, pos + 1));
    pos += 1;

    // Algorithm Suite ID: 2 bytes
    assert!(
        pos + 2 <= ct.len(),
        "not enough bytes for Algorithm Suite ID"
    );
    fields.push(("Algorithm Suite ID", pos, pos + 2));
    pos += 2;

    // Message ID: 32 bytes (V2 uses 32-byte message IDs)
    assert!(pos + 32 <= ct.len(), "not enough bytes for Message ID");
    fields.push(("Message ID", pos, pos + 32));
    pos += 32;

    // AAD: variable length, self-describing
    assert!(pos + 2 <= ct.len(), "not enough bytes for AAD length");
    let aad_start = pos;
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += aad_byte_len;
    }
    fields.push(("AAD", aad_start, pos));

    // Encrypted Data Keys: variable length
    assert!(pos + 2 <= ct.len(), "not enough bytes for EDK count");
    let edk_start = pos;
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for i in 0..edk_count {
        assert!(
            pos + 2 <= ct.len(),
            "not enough bytes for provider ID length (EDK {})",
            i
        );
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        assert!(
            pos + 2 <= ct.len(),
            "not enough bytes for provider info length (EDK {})",
            i
        );
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        assert!(
            pos + 2 <= ct.len(),
            "not enough bytes for EDK ciphertext length (EDK {})",
            i
        );
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }
    fields.push(("Encrypted Data Keys", edk_start, pos));

    // Content Type: 1 byte
    assert!(pos < ct.len(), "not enough bytes for Content Type");
    fields.push(("Content Type", pos, pos + 1));
    pos += 1;

    // Frame Length: 4 bytes
    assert!(pos + 4 <= ct.len(), "not enough bytes for Frame Length");
    fields.push(("Frame Length", pos, pos + 4));
    pos += 4;

    // Algorithm Suite Data: 32 bytes (commit key for committing suites)
    assert!(
        pos + 32 <= ct.len(),
        "not enough bytes for Algorithm Suite Data"
    );
    fields.push(("Algorithm Suite Data", pos, pos + 32));

    fields
}

/// Parse a V2 ciphertext header and return (edk_count_offset, content_type_offset, frame_length_offset).
/// V2 header layout: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKs(variable) + ContentType(1) + FrameLength(4) + SuiteData(32).
pub fn parse_header_offsets(ct: &[u8]) -> (usize, usize, usize) {
    let mut pos: usize = 1 + 2 + 32; // skip Version, AlgSuiteID, MessageID

    // AAD: 2-byte length, then if non-zero: aad_byte_len bytes (which include the 2-byte kv_count)
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += aad_byte_len;
    }

    // EDK count offset
    let edk_count_offset = pos;
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }

    let content_type_offset = pos;
    pos += 1; // Content Type: 1 byte

    let frame_length_offset = pos;

    (edk_count_offset, content_type_offset, frame_length_offset)
}

/// Find the content type byte offset in a V2 ciphertext.
/// V2 header: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKs(variable) + ContentType(1).
pub fn content_type_offset_v2(ct: &[u8]) -> usize {
    let mut pos: usize = 1 + 2 + 32; // skip Version, AlgSuiteID, MessageID

    // AAD: 2-byte length, then if non-zero: aad_byte_len bytes (which include the 2-byte kv_count)
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += aad_byte_len;
    }

    // EDKs: 2-byte count, then for each: provider_id(2+len) + provider_info(2+len) + ciphertext(2+len)
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        let ct_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }

    pos
}

/// Read the signature length from the end of a signing-suite ciphertext.
/// The footer is: [sig_len: 2 bytes] [signature: sig_len bytes] at the end.
/// For ECDSA P384, the DER-encoded signature is typically 102-104 bytes.
/// Returns (footer_offset, signature_length).
pub fn find_footer_offset(ct: &[u8]) -> (usize, u16) {
    // Try signature lengths in the expected range for ECDSA P384 DER signatures
    for candidate_len in 90..=110 {
        let offset = ct.len() - 2 - candidate_len;
        let sig_len = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        if sig_len as usize == candidate_len {
            return (offset, sig_len);
        }
    }
    panic!("Could not find footer in ciphertext");
}

/// Find the footer offset in a signing-suite ciphertext, returning only the offset.
/// Footer format: [sig_len: 2 bytes big-endian] [signature: sig_len bytes] at the end.
pub fn find_footer_offset_only(ct: &[u8]) -> usize {
    for candidate_len in 90..=110 {
        let offset = ct.len() - 2 - candidate_len;
        let sig_len = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        if sig_len as usize == candidate_len {
            return offset;
        }
    }
    panic!("Could not find footer in ciphertext");
}

/// Returns `true` if the trailing bytes of `ct` look like a valid ECDSA P-384
/// footer (2-byte length prefix whose value falls in the expected DER signature
/// range and matches the remaining byte count).
pub fn has_footer(ct: &[u8]) -> bool {
    // ECDSA P-384 DER signatures are typically 64..=104 bytes.
    // The footer is: 2-byte big-endian length + that many signature bytes.
    for candidate_len in 64..=110 {
        if ct.len() < 2 + candidate_len {
            continue;
        }
        let offset = ct.len() - 2 - candidate_len;
        let sig_len = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        if sig_len as usize == candidate_len {
            return true;
        }
    }
    false
}

/// Find the byte offset of the EDK count field in a ciphertext header.
/// V1: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) + AAD(variable)
/// V2: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable)
pub fn edk_count_offset(ct: &[u8], version: Version) -> usize {
    match version {
        Version::V1 => {
            // Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) = 20
            let pos = 20;
            let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
            if aad_byte_len > 0 {
                // 2 (aad_byte_len field) + aad_byte_len (already includes kv_count)
                pos + 2 + aad_byte_len
            } else {
                pos + 2
            }
        }
        Version::V2 => {
            let mut pos: usize = 1 + 2 + 32; // Version + AlgSuiteID + MessageID
            let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
            pos += 2;
            if aad_byte_len > 0 {
                pos += aad_byte_len;
            }
            pos
        }
    }
}

/// Parse an EDK entry starting at `offset` in `ct`.
/// Returns (key_provider_id, key_provider_info, edk_ciphertext, end_offset).
pub fn parse_edk_at(ct: &[u8], offset: usize) -> (&str, &[u8], &[u8], usize) {
    let mut pos = offset;

    let kp_id_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    let kp_id = std::str::from_utf8(&ct[pos..pos + kp_id_len]).unwrap();
    pos += kp_id_len;

    let kp_info_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    let kp_info = &ct[pos..pos + kp_info_len];
    pos += kp_info_len;

    let edk_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    let edk_data = &ct[pos..pos + edk_len];
    pos += edk_len;

    (kp_id, kp_info, edk_data, pos)
}

/// Parse the raw EDK fields at `offset` returning all 6 fields with their lengths.
/// Returns (kp_id_len, kp_id, kp_info_len, kp_info, edk_len, edk_data).
pub fn parse_edk_raw_at(
    ct: &[u8],
    offset: usize,
) -> (u16, &[u8], u16, &[u8], u16, &[u8]) {
    let mut pos = offset;

    let kp_id_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
    pos += 2;
    let kp_id = &ct[pos..pos + kp_id_len as usize];
    pos += kp_id_len as usize;

    let kp_info_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
    pos += 2;
    let kp_info = &ct[pos..pos + kp_info_len as usize];
    pos += kp_info_len as usize;

    let edk_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
    pos += 2;
    let edk_data = &ct[pos..pos + edk_len as usize];

    (kp_id_len, kp_id, kp_info_len, kp_info, edk_len, edk_data)
}

/// Parsed EDK entry from raw ciphertext bytes.
pub struct ParsedEdk {
    pub provider_id_len: u16,
    pub provider_id: Vec<u8>,
    pub provider_info_len: u16,
    pub provider_info: Vec<u8>,
    pub edk_len: u16,
    pub edk: Vec<u8>,
}

/// Result of parsing the EDK section from a ciphertext header.
pub struct ParsedEdkSection {
    pub edk_count_offset: usize,
    pub edk_count: u16,
    pub edks: Vec<ParsedEdk>,
    /// Byte offset immediately after the last EDK entry.
    pub end_offset: usize,
}

/// Skip to the EDK count field in a ciphertext header.
/// V1 layout: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) + AAD(variable) + EDKs...
/// V2 layout: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKs...
pub fn skip_to_edk_section(ct: &[u8], version: Version) -> usize {
    let mut pos = match version {
        Version::V1 => 1 + 1 + 2 + 16, // 20
        Version::V2 => 1 + 2 + 32,     // 35
    };
    // AAD: 2-byte length, then if non-zero: aad_byte_len bytes (which include the 2-byte kv_count)
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += aad_byte_len;
    }
    pos
}

pub fn parse_edk_section(ct: &[u8], version: Version) -> ParsedEdkSection {
    let edk_count_offset = skip_to_edk_section(ct, version);
    let edk_count = u16::from_be_bytes([ct[edk_count_offset], ct[edk_count_offset + 1]]);
    let mut pos = edk_count_offset + 2;
    let mut edks = Vec::new();
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        let pid = ct[pos..pos + pid_len as usize].to_vec();
        pos += pid_len as usize;
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        let pinfo = ct[pos..pos + pinfo_len as usize].to_vec();
        pos += pinfo_len as usize;
        let edk_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        let edk_data = ct[pos..pos + edk_len as usize].to_vec();
        pos += edk_len as usize;
        edks.push(ParsedEdk {
            provider_id_len: pid_len,
            provider_id: pid,
            provider_info_len: pinfo_len,
            provider_info: pinfo,
            edk_len,
            edk: edk_data,
        });
    }
    ParsedEdkSection {
        edk_count_offset,
        edk_count,
        edks,
        end_offset: pos,
    }
}

/// Parsed fields from a nonframed message body.
pub struct NonframedBody {
    pub body_start: usize,
    pub iv: Vec<u8>,
    pub encrypted_content_length: u64,
    pub encrypted_content_length_bytes: Vec<u8>,
    pub encrypted_content: Vec<u8>,
    pub auth_tag: Vec<u8>,
}

/// Parse the nonframed body from a complete nonframed message.
/// V2 header: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(var) + EDKs(var) + ContentType(1) + FrameLength(4) + SuiteData(32) + HeaderAuth(16)
pub fn parse_nonframed_body(msg: &[u8]) -> NonframedBody {
    // Skip V2 header
    let mut pos: usize = 1 + 2 + 32; // Version + AlgSuiteID + MessageID
    let aad_byte_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += aad_byte_len;
    }
    // EDKs
    let edk_count = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
        pos += 2 + pid_len;
        let pinfo_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        let ct_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
        pos += 2 + ct_len;
    }
    pos += 1; // Content Type
    pos += 4; // Frame Length
    pos += 32; // Algorithm Suite Data (commitment key)
    pos += TAG_LEN; // Header Auth Tag (V2: no IV, just 16-byte tag)

    let body_start = pos;

    // IV: 12 bytes
    let iv = msg[pos..pos + IV_LEN].to_vec();
    pos += IV_LEN;

    // Encrypted Content Length: 8 bytes
    let encrypted_content_length_bytes = msg[pos..pos + 8].to_vec();
    let encrypted_content_length = u64::from_be_bytes([
        msg[pos],
        msg[pos + 1],
        msg[pos + 2],
        msg[pos + 3],
        msg[pos + 4],
        msg[pos + 5],
        msg[pos + 6],
        msg[pos + 7],
    ]);
    pos += 8;

    // Encrypted Content
    let enc_len = encrypted_content_length as usize;
    let encrypted_content = msg[pos..pos + enc_len].to_vec();
    pos += enc_len;

    // Authentication Tag: 16 bytes
    let auth_tag = msg[pos..pos + TAG_LEN].to_vec();

    NonframedBody {
        body_start,
        iv,
        encrypted_content_length,
        encrypted_content_length_bytes,
        encrypted_content,
        auth_tag,
    }
}

// -----------------------------------------------------------------------------
// External nonframed vectors from aws-encryption-sdk-test-vectors.
//
// The Rust SDK's encrypt path cannot produce nonframed messages (the ESDK
// spec forbids new nonframed encryption), so to exercise the nonframed
// decrypt path end-to-end we anchor on externally produced vectors.
//
// Both vectors come from the public
// `awslabs/aws-encryption-sdk-test-vectors` GitHub repository and use the
// same raw-AES-256 wrapping key (provider-id `aws-raw-vectors-persistant`,
// key-id `aes-256`) — a 32-byte static pattern
// [0x00,0x01,0x02,...,0x28,0x29,0x30,0x31].
//
// V1 vector:
//   Archive:            vectors/awses-decrypt/python-1.3.5.zip
//   Producer:           aws-encryption-sdk-python, version 1.3.5
//                       (predates the "MUST NOT encrypt nonframed" rule)
//   Test ID:            9b86a9ce-e251-4d71-ba7b-cb83e0766aae
//   Version byte:       0x01  (V1)
//   Algorithm Suite ID: 0x0178  (AlgAes256GcmIv12Tag16HkdfSha256,
//                                 non-committing)
//   Message ID length:  16 bytes
//   Plaintext size:     10240 bytes
//   Ciphertext size:    10445 bytes
//
// V2 vector:
//   Archive:            vectors/awses-decrypt/python-2.0.0.zip
//   Producer:           aws-encryption-sdk-python, version 2.0.0
//   Test ID:            24cfe457-2c2b-42c6-8bb5-5300e736b18a
//   Version byte:       0x02  (V2)
//   Algorithm Suite ID: 0x0478  (AlgAes256GcmHkdfSha512CommitKey,
//                                 committing, non-signing)
//   Message ID length:  32 bytes
//   Plaintext size:     10240 bytes
//   Ciphertext size:    10475 bytes
// -----------------------------------------------------------------------------

pub const EXTERNAL_V1_NONFRAMED_CT: &[u8] =
    include_bytes!("fixtures_binary/v1_nonframed_aes256_0178.bin");
pub const EXTERNAL_V1_NONFRAMED_PT: &[u8] =
    include_bytes!("fixtures_binary/v1_nonframed_plaintext_small.bin");

pub const EXTERNAL_V2_NONFRAMED_CT: &[u8] =
    include_bytes!("fixtures_binary/v2_nonframed_aes256_0478.bin");
pub const EXTERNAL_V2_NONFRAMED_PT: &[u8] =
    include_bytes!("fixtures_binary/v2_nonframed_plaintext_small.bin");

/// The `aes-256` static test key from aws-encryption-sdk-test-vectors'
/// `keys.json` (base64 `AAECAwQFBgcICRAREhMUFRYXGBkgISIjJCUmJygpMDE=`).
/// Shared between the V1 and V2 external nonframed vectors above.
pub const EXTERNAL_AES_256_WRAPPING_KEY: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
    0x16, 0x17, 0x18, 0x19, 0x20, 0x21, 0x22, 0x23,
    0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x30, 0x31,
];

/// Return the external nonframed ciphertext bytes for the given message version.
pub fn external_nonframed_ct(version: Version) -> &'static [u8] {
    match version {
        Version::V1 => EXTERNAL_V1_NONFRAMED_CT,
        Version::V2 => EXTERNAL_V2_NONFRAMED_CT,
    }
}

/// Return the expected plaintext bytes for the given external nonframed vector.
pub fn external_nonframed_pt(version: Version) -> &'static [u8] {
    match version {
        Version::V1 => EXTERNAL_V1_NONFRAMED_PT,
        Version::V2 => EXTERNAL_V2_NONFRAMED_PT,
    }
}

/// Decrypt the external nonframed vector for the given message version through
/// the real SDK path. Returns the decrypted plaintext. Panics on failure —
/// decryption succeeding IS the positive evidence callers rely on.
pub async fn decrypt_external_nonframed_vector(version: Version) -> Vec<u8> {
    try_decrypt_external_nonframed(version, external_nonframed_ct(version))
        .await
        .expect("external nonframed vector must decrypt successfully")
}

/// Attempt to decrypt a (possibly tampered) nonframed message using the
/// external-vector keyring appropriate for `version`. Used for negative
/// tampering tests that flip bytes in an external authority vector and expect
/// AES-GCM authentication to fail.
pub async fn try_decrypt_external_nonframed(
    version: Version,
    msg: &[u8],
) -> Result<Vec<u8>, Error> {
    let keyring = mpl()
        .create_raw_aes_keyring()
        .key_namespace("aws-raw-vectors-persistant")
        .key_name("aes-256")
        .wrapping_key(aws_smithy_types::Blob::new(EXTERNAL_AES_256_WRAPPING_KEY))
        .wrapping_alg(aws_mpl_legacy::dafny::types::AesWrappingAlg::AlgAes256GcmIv12Tag16)
        .send()
        .await
        .unwrap();

    let mut dec_input =
        DecryptInput::with_legacy_keyring(msg, EncryptionContext::new(), keyring);
    // V1 suite 0x0178 is non-committing; V2 suite 0x0478 is committing.
    dec_input.commitment_policy = match version {
        Version::V1 => EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
        Version::V2 => EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
    };
    decrypt(&dec_input).await.map(|o| o.plaintext)
}

/// Parsed body fields from an external nonframed message. The nonframed body
/// layout is identical between V1 and V2 (IV(12) + ContentLength(8) +
/// Content(N) + Tag(16)); only the preceding header differs.
pub struct ExternalNonframedBody {
    /// Byte offset in the full message where the body's IV begins.
    pub body_start: usize,
    pub iv: [u8; 12],
    pub encrypted_content_length: u64,
}

/// Parse the body of an external nonframed message for the given version.
///
/// V1 header layout (up to body):
///   Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) + AAD(var)
///   + EDKs(var) + ContentType(1) + Reserved(4) + IVLength(1)
///   + FrameLength(4) + HeaderAuthIV(12) + HeaderAuthTag(16)
///
/// V2 header layout (up to body):
///   Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(var) + EDKs(var)
///   + ContentType(1) + FrameLength(4) + SuiteData(32) + HeaderAuthTag(16)
///
/// Nonframed body layout (both versions):
///   IV(12) + EncryptedContentLength(8) + EncryptedContent(N) + AuthTag(16)
pub fn parse_external_nonframed_body(msg: &[u8], version: Version) -> ExternalNonframedBody {
    let mut pos: usize = match version {
        Version::V1 => 1 + 1 + 2 + 16, // Version + Type + AlgSuiteID + MessageID
        Version::V2 => 1 + 2 + 32,     // Version + AlgSuiteID + MessageID
    };
    let aad_byte_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += aad_byte_len;
    }
    let edk_count = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
    pos += 2;
    for _ in 0..edk_count {
        let pid_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
        pos += 2 + pid_len;
        let pinfo_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        let edk_len = u16::from_be_bytes([msg[pos], msg[pos + 1]]) as usize;
        pos += 2 + edk_len;
    }
    pos += 1; // Content Type
    match version {
        Version::V1 => {
            pos += 4; // Reserved
            pos += 1; // IV length
            pos += 4; // Frame Length
            pos += IV_LEN; // Header Auth IV (V1 only)
        }
        Version::V2 => {
            pos += 4; // Frame Length
            pos += 32; // Algorithm Suite Data (commit key)
        }
    }
    pos += TAG_LEN; // Header Auth Tag

    // Body
    let body_start = pos;
    let mut iv = [0u8; 12];
    iv.copy_from_slice(&msg[pos..pos + IV_LEN]);
    pos += IV_LEN;
    let encrypted_content_length = u64::from_be_bytes([
        msg[pos], msg[pos + 1], msg[pos + 2], msg[pos + 3],
        msg[pos + 4], msg[pos + 5], msg[pos + 6], msg[pos + 7],
    ]);
    ExternalNonframedBody {
        body_start,
        iv,
        encrypted_content_length,
    }
}

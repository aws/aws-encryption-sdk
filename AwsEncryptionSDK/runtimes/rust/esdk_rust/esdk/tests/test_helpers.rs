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

// ── Keyring helpers ──

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

/// Create a multi-keyring from a generator and child keyrings.
pub async fn multi_keyring(
    generator_kr: KeyringRef,
    children: Vec<KeyringRef>,
) -> KeyringRef {
    mpl()
        .create_multi_keyring()
        .generator(generator_kr)
        .child_keyrings(children)
        .send()
        .await
        .unwrap()
}

// ── Encrypt helpers ──

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

/// Encrypt with a signing algorithm suite, return ciphertext bytes.
pub async fn encrypt_with_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384);
    encrypt(&input).await.unwrap().ciphertext
}

/// Encrypt with a non-signing algorithm suite, return ciphertext bytes.
pub async fn encrypt_without_signing_suite(plaintext: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut input =
        EncryptInput::with_legacy_keyring(plaintext, EncryptionContext::new(), keyring);
    input.algorithm_suite_id =
        Some(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey);
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

// ── Decrypt helpers ──

/// Decrypt ciphertext with the default test keyring, return full DecryptOutput.
pub async fn decrypt_ciphertext(ciphertext: &[u8]) -> DecryptOutput {
    let keyring = test_keyring().await;
    let dec_input = DecryptInput::with_legacy_keyring(ciphertext, EncryptionContext::new(), keyring);
    decrypt(&dec_input).await.unwrap()
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

// ── Round-trip helpers ──

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
    let mut dec_input =
        DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring);
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

// --- Frame Parsing Utilities ---

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
    let body_start = find_body_start(ct, frame_length)
        .expect("could not find body start in ciphertext");
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

// --- Header Parsing Utilities ---

/// Parse V1 header trailing field offsets from ciphertext.
/// Returns (content_type_offset, reserved_offset, iv_length_offset, frame_length_offset).
pub fn parse_v1_trailing_offsets(ct: &[u8]) -> (usize, usize, usize, usize) {
    // V1 header: Version(1) + Type(1) + AlgSuiteID(2) + MessageID(16) = 20 fixed bytes
    let mut pos: usize = 20;

    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
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
    (content_type_offset, reserved_offset, iv_length_offset, frame_length_offset)
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
    assert!(pos + 2 <= ct.len(), "not enough bytes for Algorithm Suite ID");
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
        pos += 2 + aad_byte_len;
    }
    fields.push(("AAD", aad_start, pos));

    // Encrypted Data Keys: variable length
    assert!(pos + 2 <= ct.len(), "not enough bytes for EDK count");
    let edk_start = pos;
    let edk_count = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    for i in 0..edk_count {
        assert!(pos + 2 <= ct.len(), "not enough bytes for provider ID length (EDK {})", i);
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pid_len;
        assert!(pos + 2 <= ct.len(), "not enough bytes for provider info length (EDK {})", i);
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
        pos += 2 + pinfo_len;
        assert!(pos + 2 <= ct.len(), "not enough bytes for EDK ciphertext length (EDK {})", i);
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
    assert!(pos + 32 <= ct.len(), "not enough bytes for Algorithm Suite Data");
    fields.push(("Algorithm Suite Data", pos, pos + 32));

    fields
}

/// Parse a V2 ciphertext header and return (edk_count_offset, content_type_offset, frame_length_offset).
/// V2 header layout: Version(1) + AlgSuiteID(2) + MessageID(32) + AAD(variable) + EDKs(variable) + ContentType(1) + FrameLength(4) + SuiteData(32).
pub fn parse_header_offsets(ct: &[u8]) -> (usize, usize, usize) {
    let mut pos: usize = 1 + 2 + 32; // skip Version, AlgSuiteID, MessageID

    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
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

    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
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

// --- Footer Parsing Utilities ---

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

// --- EDK Parsing Utilities ---

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
                // 2 (aad_byte_len field) + 2 (kv_count) + aad_byte_len
                pos + 2 + 2 + aad_byte_len
            } else {
                pos + 2
            }
        }
        Version::V2 => {
            let mut pos: usize = 1 + 2 + 32; // Version + AlgSuiteID + MessageID
            let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
            pos += 2;
            if aad_byte_len > 0 {
                pos += 2 + aad_byte_len;
            }
            pos
        }
    }
}

/// Parse an EDK entry starting at `offset` in `ct`.
/// Returns (key_provider_id, key_provider_info, edk_ciphertext, end_offset).
pub fn parse_edk_at<'a>(ct: &'a [u8], offset: usize) -> (&'a str, &'a [u8], &'a [u8], usize) {
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
pub fn parse_edk_raw_at<'a>(ct: &'a [u8], offset: usize) -> (u16, &'a [u8], u16, &'a [u8], u16, &'a [u8]) {
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
        Version::V2 => 1 + 2 + 32,      // 35
    };
    // AAD: 2-byte length, then if non-zero: 2-byte kv_count + aad_byte_len bytes
    let aad_byte_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]) as usize;
    pos += 2;
    if aad_byte_len > 0 {
        pos += 2 + aad_byte_len;
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
    ParsedEdkSection { edk_count_offset, edk_count, edks, end_offset: pos }
}

// --- Non-framed Message Utilities ---

/// Build a complete nonframed encrypted message from scratch.
///
/// Uses AlgAes256GcmHkdfSha512CommitKey (0x0478), V2 header, NonFramed content type.
/// The wrapping key is `[0u8; 32]` matching the test_keyring() configuration.
pub fn build_nonframed_message(plaintext: &[u8]) -> Vec<u8> {
    use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
    use aws_lc_rs::hkdf::{Salt, HKDF_SHA512};

    let wrapping_key = [0u8; 32];
    let plaintext_data_key = [0x42u8; 32];
    let message_id = [0xAAu8; 32];
    let edk_iv: [u8; 12] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C];
    let alg_suite_id: [u8; 2] = [0x04, 0x78];

    // Wrap the data key (raw AES keyring format)
    let key = UnboundKey::new(&AES_256_GCM, &wrapping_key).unwrap();
    let key = LessSafeKey::new(key);
    let nonce = Nonce::try_assume_unique_for_key(&edk_iv).unwrap();
    let mut edk_ct = plaintext_data_key.to_vec();
    let edk_tag = key.seal_in_place_separate_tag(nonce, Aad::from(&[] as &[u8]), &mut edk_ct).unwrap();
    let mut edk_ciphertext = edk_ct;
    edk_ciphertext.extend_from_slice(edk_tag.as_ref());

    let key_name = b"child0 Name";
    let key_namespace = b"child0 Namespace";
    let mut provider_info = Vec::new();
    provider_info.extend_from_slice(key_name);
    provider_info.extend_from_slice(&128u32.to_be_bytes());
    provider_info.extend_from_slice(&12u32.to_be_bytes());
    provider_info.extend_from_slice(&edk_iv);

    // Derive keys (V2 HKDF-SHA512 with commitment)
    let salt = Salt::new(HKDF_SHA512, &message_id);
    let prk = salt.extract(&plaintext_data_key);
    let mut data_key = [0u8; 32];
    let info_key: &[&[u8]] = &[&alg_suite_id, b"DERIVEKEY"];
    let okm = prk.expand(info_key, &AES_256_GCM).unwrap();
    okm.fill(&mut data_key).unwrap();
    let mut commit_key = [0u8; 32];
    let info_commit: &[&[u8]] = &[b"COMMITKEY"];
    let okm2 = prk.expand(info_commit, &AES_256_GCM).unwrap();
    okm2.fill(&mut commit_key).unwrap();

    // Build V2 header body
    let mut header_body = Vec::new();
    header_body.push(0x02);
    header_body.extend_from_slice(&alg_suite_id);
    header_body.extend_from_slice(&message_id);
    header_body.extend_from_slice(&0u16.to_be_bytes());
    header_body.extend_from_slice(&1u16.to_be_bytes());
    header_body.extend_from_slice(&(key_namespace.len() as u16).to_be_bytes());
    header_body.extend_from_slice(key_namespace);
    header_body.extend_from_slice(&(provider_info.len() as u16).to_be_bytes());
    header_body.extend_from_slice(&provider_info);
    header_body.extend_from_slice(&(edk_ciphertext.len() as u16).to_be_bytes());
    header_body.extend_from_slice(&edk_ciphertext);
    header_body.push(0x01); // Content Type: NonFramed
    header_body.extend_from_slice(&0u32.to_be_bytes()); // Frame Length: 0 for nonframed
    header_body.extend_from_slice(&commit_key);

    // Compute header auth
    let header_auth_iv = [0u8; 12];
    let key = UnboundKey::new(&AES_256_GCM, &data_key).unwrap();
    let key = LessSafeKey::new(key);
    let nonce = Nonce::try_assume_unique_for_key(&header_auth_iv).unwrap();
    let mut empty = Vec::new();
    let header_auth_tag = key.seal_in_place_separate_tag(nonce, Aad::from(&header_body[..]), &mut empty).unwrap();

    // Build nonframed body
    let mut body_aad = Vec::new();
    body_aad.extend_from_slice(&message_id);
    body_aad.extend_from_slice(b"AWSKMSEncryptionClient Single Block");
    body_aad.extend_from_slice(&1u32.to_be_bytes());
    body_aad.extend_from_slice(&(plaintext.len() as u64).to_be_bytes());

    let mut body_iv = [0u8; 12];
    body_iv[8..].copy_from_slice(&1u32.to_be_bytes());

    let key = UnboundKey::new(&AES_256_GCM, &data_key).unwrap();
    let key = LessSafeKey::new(key);
    let nonce = Nonce::try_assume_unique_for_key(&body_iv).unwrap();
    let mut body_ct = plaintext.to_vec();
    let body_tag = key.seal_in_place_separate_tag(nonce, Aad::from(&body_aad[..]), &mut body_ct).unwrap();

    // Assemble the full message
    let mut message = Vec::new();
    message.extend_from_slice(&header_body);
    message.extend_from_slice(header_auth_tag.as_ref());
    // nonframed body: IV(12) + content_length(8) + encrypted_content(N) + auth_tag(16)
    message.extend_from_slice(&body_iv);
    message.extend_from_slice(&(body_ct.len() as u64).to_be_bytes());
    message.extend_from_slice(&body_ct);
    message.extend_from_slice(body_tag.as_ref());

    message
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
        pos += 2 + aad_byte_len;
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
        msg[pos], msg[pos + 1], msg[pos + 2], msg[pos + 3],
        msg[pos + 4], msg[pos + 5], msg[pos + 6], msg[pos + 7],
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

/// Decrypt a nonframed message and return the plaintext.
pub async fn decrypt_nonframed(msg: &[u8]) -> Vec<u8> {
    let keyring = test_keyring().await;
    let mut dec_input = DecryptInput::with_legacy_keyring(msg, EncryptionContext::new(), keyring);
    dec_input.commitment_policy = EsdkCommitmentPolicy::RequireEncryptRequireDecrypt;
    decrypt(&dec_input).await.unwrap().plaintext
}

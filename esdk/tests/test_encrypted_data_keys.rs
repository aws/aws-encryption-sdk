// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Tests for the Encrypted Data Keys sections of specification/data-format/message-header.md

mod fixtures;
mod test_helpers;

use aws_esdk::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use fixtures::*;
use test_helpers::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_encrypted_data_keys_ordering() {
    let single = aes_keyring(0).await;
    let generator = aes_keyring(0).await;
    let c1 = aes_keyring(1).await;
    let c2 = aes_keyring(2).await;
    let triple = multi_keyring(generator, vec![c1, c2]).await;

    // Cover both the single-EDK case and the multi-EDK case so that the
    // "Count, then Entries" structure is exercised with the count field set
    // to both 1 and 3.
    for (label, keyring, expected_count) in [
        ("single", single, 1u16),
        ("triple", triple, 3u16),
    ] {
        for version in VERSIONS {
            //= specification/data-format/message-header.md#encrypted-data-keys
            //= type=test
            //# The Encrypted Data Keys MUST consist of, in order,
            //# Encrypted Data Key Count,
            //# and Encrypted Data Key Entries.
            let ct = encrypt_with_version(b"ordering test", version, keyring.clone()).await;
            let edk_section_start = skip_to_edk_section(&ct, version);
            let parsed = parse_edk_section(&ct, version);

            // 1. Encrypted Data Key Count (2 bytes)
            let count = u16::from_be_bytes([ct[edk_section_start], ct[edk_section_start + 1]]);
            assert_eq!(count, expected_count, "{label} {version:?}: count field value");
            assert_eq!(parsed.edk_count, count);

            // 2. Encrypted Data Key Entries (immediately after the count)
            let entries_start = edk_section_start + 2;
            let first_pid_len = u16::from_be_bytes([ct[entries_start], ct[entries_start + 1]]);
            assert_eq!(
                first_pid_len, parsed.edks[0].provider_id_len,
                "{label} {version:?}: EDK entries must immediately follow the count field"
            );
            assert_eq!(
                parsed.edks.len(),
                count as usize,
                "{label} {version:?}: parsed entries must match count"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_section_length_fields_are_big_endian_uint16() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;
    let (expected_ns, _) = namespace_and_name(0);
    let expected_pid_len = expected_ns.len() as u16;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"length fields uint16", version, mk.clone()).await;
        let parsed = parse_edk_section(&ct, version);
        let edk = &parsed.edks[0];

        // Decode each length field directly from the wire as a big-endian UInt16.
        let entries_start = parsed.edk_count_offset + 2;
        let pid_len_offset = entries_start;
        let pinfo_len_offset = entries_start + 2 + edk.provider_id_len as usize;
        let edk_len_offset = pinfo_len_offset + 2 + edk.provider_info_len as usize;

        let count_wire = u16::from_be_bytes([ct[parsed.edk_count_offset], ct[parsed.edk_count_offset + 1]]);
        let pid_len_wire = u16::from_be_bytes([ct[pid_len_offset], ct[pid_len_offset + 1]]);
        let pinfo_len_wire = u16::from_be_bytes([ct[pinfo_len_offset], ct[pinfo_len_offset + 1]]);
        let edk_len_wire = u16::from_be_bytes([ct[edk_len_offset], ct[edk_len_offset + 1]]);

        // EDK count: 2 keyrings → UInt16 value 2 ([0x00, 0x02]).
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# The length of the serialized encrypted data key count MUST be 2 bytes.
        //
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# The encrypted data key count MUST be interpreted as a UInt16.
        assert_eq!(count_wire, 2, "{version:?}: EDK count UInt16 value");
        assert_eq!(ct[parsed.edk_count_offset], 0x00, "{version:?}: EDK count high byte");
        assert_eq!(ct[parsed.edk_count_offset + 1], 0x02, "{version:?}: EDK count low byte");

        // Key provider ID length: the UInt16 at this offset equals the known keyring namespace byte length.
        //= specification/data-format/message-header.md#key-provider-id-length
        //= type=test
        //# The length of the serialized key provider ID length field MUST be 2 bytes.
        //
        //= specification/data-format/message-header.md#key-provider-id-length
        //= type=test
        //# The key provider ID length MUST be interpreted as a UInt16.
        assert_eq!(pid_len_wire, expected_pid_len, "{version:?}: provider ID length UInt16 value");

        // Key provider information length: the UInt16 at this offset must be positive for a raw AES keyring
        // (which packs key name + bit length + IV length + IV into provider info).
        //= specification/data-format/message-header.md#key-provider-information-length
        //= type=test
        //# The length of the serialized key provider information length field MUST be 2 bytes.
        //
        //= specification/data-format/message-header.md#key-provider-information-length
        //= type=test
        //# The key provider information length MUST be interpreted as a UInt16.
        assert!(pinfo_len_wire > 0, "{version:?}: provider info length UInt16 must be positive");

        // Encrypted data key length: raw AES keyring stores IV in provider_info; the ciphertext field is
        // wrapped data key (32 bytes) + GCM tag (16 bytes) = 48.
        //= specification/data-format/message-header.md#encrypted-data-key-length
        //= type=test
        //# The length of the serialized encrypted data key length field MUST be 2 bytes.
        //
        //= specification/data-format/message-header.md#encrypted-data-key-length
        //= type=test
        //# The encrypted data key length MUST be interpreted as a UInt16.
        assert_eq!(edk_len_wire, 48, "{version:?}: EDK ciphertext length UInt16 value (wrapped 32B key + 16B tag)");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_zero_rejected_on_decrypt() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let mut ct = encrypt_with_version(b"zero count", version, keyring.clone()).await;
        let offset = skip_to_edk_section(&ct, version);
        // Tamper: set count to 0.
        ct[offset] = 0x00;
        ct[offset + 1] = 0x00;
        let mut dec =
            DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        if let Version::V1 = version {
            dec.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }

        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //= reason=Tampering the count to 0 and verifying decrypt rejects it proves the >0 constraint is enforced on the deserialization path.
        //# This value MUST be greater than 0.
        let err = decrypt(&dec)
            .await
            .expect_err(&format!("{version:?}: decrypt must reject EDK count of 0"));
        assert!(
            matches!(err.kind, aws_esdk::ErrorKind::SerializationError),
            "{version:?}: expected SerializationError, got: {} ({:?})",
            err.message, err.kind
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_max_enforcement() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    let expect_exceed = |err: &aws_esdk::Error| {
        assert!(
            err.message.contains("exceed") && err.message.contains("maximum"),
            "error must indicate EDK count exceeds maximum, got: {} ({:?})",
            err.message, err.kind
        );
    };

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //# This value MUST be less than or equal to the [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys) if the maximum number is configured.

    // Encrypt with 2 EDKs and max=1 → error.
    let mut enc_over = EncryptInput::with_legacy_keyring(
        b"max edk encrypt",
        EncryptionContext::new(),
        mk.clone(),
    );
    enc_over.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
    expect_exceed(&encrypt(&enc_over).await.expect_err("encrypt must fail when EDK count exceeds max"));

    // Decrypt a 2-EDK message with max=1 → error.
    let ct = encrypt_with_version(b"max edk decrypt", Version::V2, mk.clone()).await;
    let mut dec_over = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), mk.clone());
    dec_over.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
    expect_exceed(&decrypt(&dec_over).await.expect_err("decrypt must fail when EDK count exceeds max"));

    // Encrypt with 2 EDKs and max=2 → ok (the "equal to" side of ≤).
    let mut enc_at = EncryptInput::with_legacy_keyring(b"at max", EncryptionContext::new(), mk);
    enc_at.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(2).unwrap());
    assert!(
        encrypt(&enc_at).await.is_ok(),
        "encrypt must succeed when EDK count equals max"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_entry_field_order() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"entry order", version, keyring.clone()).await;
        let edk_start = skip_to_edk_section(&ct, version) + 2; // skip count
        let mut pos = edk_start;

        //= specification/data-format/message-header.md#encrypted-data-key-entries
        //= type=test
        //# Each Encrypted Data Key Entry MUST consist of, in order,
        //# Key Provider ID Length,
        //# Key Provider ID,
        //# Key Provider Information Length,
        //# Key Provider Information,
        //# Encrypted Data Key Length,
        //# and Encrypted Data Key.

        // 1. Key Provider ID Length (2 bytes)
        let pid_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        assert!(pid_len > 0, "{version:?}: provider ID length must be positive");

        // 2. Key Provider ID (pid_len bytes)
        let pid = &ct[pos..pos + pid_len as usize];
        let pid_str = std::str::from_utf8(pid).expect("provider ID must be valid UTF-8");
        let (expected_ns, _) = namespace_and_name(0);
        assert_eq!(
            pid_str, expected_ns,
            "{version:?}: provider ID must match keyring namespace"
        );
        pos += pid_len as usize;

        // 3. Key Provider Information Length (2 bytes)
        let pinfo_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;

        // 4. Key Provider Information (pinfo_len bytes)
        let _pinfo = &ct[pos..pos + pinfo_len as usize];
        pos += pinfo_len as usize;

        // 5. Encrypted Data Key Length (2 bytes)
        let edk_len = u16::from_be_bytes([ct[pos], ct[pos + 1]]);
        pos += 2;
        assert!(
            edk_len > 0,
            "{version:?}: encrypted data key length must be positive"
        );

        // 6. Encrypted Data Key (edk_len bytes)
        let _edk = &ct[pos..pos + edk_len as usize];
        pos += edk_len as usize;

        // Verify we consumed exactly one entry and the position matches the parser.
        let parsed = parse_edk_section(&ct, version);
        assert_eq!(
            pos, parsed.end_offset,
            "{version:?}: manual walk must match parser end offset"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_entries_preserve_keyring_order() {
    let generator = aes_keyring(0).await;
    let c1 = aes_keyring(1).await;
    let c2 = aes_keyring(2).await;
    let mk = multi_keyring(generator, vec![c1, c2]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"order check", version, mk.clone()).await;
        let parsed = parse_edk_section(&ct, version);

        //= specification/data-format/message-header.md#encrypted-data-keys
        //= type=test
        //= reason=Verifying that EDK provider IDs appear in generator-then-children order proves entries are serialized in the order they appear in the encryption materials, exercising the "Entries" component of the Count+Entries structure.
        //# The Encrypted Data Keys MUST consist of, in order,
        //# Encrypted Data Key Count,
        //# and Encrypted Data Key Entries.
        for (i, edk) in parsed.edks.iter().enumerate() {
            let pid_str = std::str::from_utf8(&edk.provider_id).unwrap();
            let (expected_ns, _) = namespace_and_name(i as u8);
            assert_eq!(
                pid_str, expected_ns,
                "{version:?}: EDK {i} provider ID must match keyring {i} namespace"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_entry_lengths_match_fields() {
    // Multi-keyring so multiple entries are checked per run.
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"entry lengths match", version, mk.clone()).await;
        let parsed = parse_edk_section(&ct, version);

        for (i, edk) in parsed.edks.iter().enumerate() {
            //= specification/data-format/message-header.md#key-provider-id
            //= type=test
            //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
            assert_eq!(
                edk.provider_id.len(), edk.provider_id_len as usize,
                "{version:?}: EDK {i}: provider ID byte length must equal the provider ID length field"
            );

            //= specification/data-format/message-header.md#key-provider-information
            //= type=test
            //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
            assert_eq!(
                edk.provider_info.len(), edk.provider_info_len as usize,
                "{version:?}: EDK {i}: provider info byte length must equal the provider info length field"
            );

            //= specification/data-format/message-header.md#encrypted-data-key
            //= type=test
            //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
            assert_eq!(
                edk.edk.len(), edk.edk_len as usize,
                "{version:?}: EDK {i}: encrypted data key byte length must equal the EDK length field"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_is_utf8() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pid utf8", version, mk.clone()).await;
        let parsed = parse_edk_section(&ct, version);

        //= specification/data-format/message-header.md#key-provider-id
        //= type=test
        //# The key provider ID MUST be interpreted as UTF-8 encoded bytes.
        for (i, edk) in parsed.edks.iter().enumerate() {
            let pid_str =
                std::str::from_utf8(&edk.provider_id).expect("provider ID must be valid UTF-8");
            let (expected_ns, _) = namespace_and_name(i as u8);
            assert_eq!(
                pid_str, expected_ns,
                "{version:?}: provider ID must be the keyring namespace as UTF-8"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_info_interpreted_as_bytes() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pinfo bytes", version, keyring.clone()).await;
        let parsed = parse_edk_section(&ct, version);
        let edk = &parsed.edks[0];
        // Provider info for raw AES keyring starts with the key name.
        let (_, expected_name) = namespace_and_name(0);

        //= specification/data-format/message-header.md#key-provider-information
        //= type=test
        //# The key provider information MUST be interpreted as bytes.
        assert!(
            edk.provider_info.starts_with(expected_name.as_bytes()),
            "{version:?}: provider info must start with the known key name"
        );
    }
}

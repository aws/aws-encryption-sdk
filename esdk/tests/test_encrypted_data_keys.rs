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
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        //= specification/data-format/message-header.md#encrypted-data-keys
        //= type=test
        //# The Encrypted Data Keys MUST consist of, in order,
        //# Encrypted Data Key Count,
        //# and Encrypted Data Key Entries.
        let ct = encrypt_with_version(b"ordering test", version, keyring.clone()).await;
        let parsed = parse_edk_section(&ct, version);
        // Count field comes first, then entries follow immediately
        assert_eq!(parsed.edk_count, 1);
        assert_eq!(parsed.edks.len(), 1);
        // The entries start at edk_count_offset + 2 (right after the 2-byte count)
        let entries_start = parsed.edk_count_offset + 2;
        // First entry's provider_id_len is at entries_start
        let first_pid_len = u16::from_be_bytes([ct[entries_start], ct[entries_start + 1]]);
        assert_eq!(
            first_pid_len, parsed.edks[0].provider_id_len,
            "{version:?}: EDK entries must immediately follow the count field"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_field_is_2_bytes() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"count 2 bytes", version, keyring.clone()).await;
        let offset = skip_to_edk_section(&ct, version);

        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# The length of the serialized encrypted data key count MUST be 2 bytes.
        // The count occupies exactly bytes [offset] and [offset+1]
        let count = u16::from_be_bytes([ct[offset], ct[offset + 1]]);
        assert_eq!(
            count, 1,
            "{version:?}: single keyring produces exactly 1 EDK"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_interpreted_as_uint16() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"uint16 count", version, mk.clone()).await;
        let offset = skip_to_edk_section(&ct, version);

        //= specification/data-format/message-header.md#encrypted-data-key-count
        //= type=test
        //# The encrypted data key count MUST be interpreted as a UInt16.
        // Big-endian UInt16: high byte should be 0, low byte should be 2
        assert_eq!(
            ct[offset], 0x00,
            "{version:?}: high byte of UInt16 count must be 0 for small counts"
        );
        assert_eq!(
            ct[offset + 1],
            0x02,
            "{version:?}: low byte of UInt16 count must be 2 for two keyrings"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_zero_rejected_on_decrypt() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let mut ct = encrypt_with_version(b"zero count", version, keyring.clone()).await;
        let offset = skip_to_edk_section(&ct, version);
        // Tamper: set count to 0
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
        // Setting count=0 corrupts the header structure: either the count=0 check fires, or
        // subsequent bytes are misinterpreted by the parser. Both outcomes are valid structural
        // rejections — assert this is a SerializationError (parse/structural failure), not a
        // generic unrelated failure.
        assert!(
            matches!(err.kind, aws_esdk::ErrorKind::SerializationError)
                || err.message.to_lowercase().contains("empty")
                || err.message.to_lowercase().contains("encrypted data key"),
            "{version:?}: error must indicate a structural/EDK failure, got: {} ({:?})",
            err.message, err.kind
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_max_enforcement_encrypt() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //= reason=max_encrypted_data_keys on encrypt enforces the upper bound on EDK count before serialization.
    //# This value MUST be less than or equal to the [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys) if the maximum number is configured.
    let mut input =
        EncryptInput::with_legacy_keyring(b"max edk encrypt", EncryptionContext::new(), mk);
    input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());
    let err = encrypt(&input).await.expect_err("encrypt must fail when EDK count exceeds max");
    assert!(
        err.message.contains("exceed") && err.message.contains("maximum"),
        "error must indicate EDK count exceeds maximum, got: {} ({:?})",
        err.message, err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_max_enforcement_decrypt() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    let ct = encrypt_with_version(b"max edk decrypt", Version::V2, mk.clone()).await;
    let mut dec = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), mk);
    dec.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(1).unwrap());

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //= reason=max_encrypted_data_keys on decrypt enforces the upper bound when deserializing the header.
    //# This value MUST be less than or equal to the [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys) if the maximum number is configured.
    let err = decrypt(&dec).await.expect_err("decrypt must fail when EDK count exceeds max");
    assert!(
        err.message.contains("exceed") && err.message.contains("maximum"),
        "error must indicate EDK count exceeds maximum, got: {} ({:?})",
        err.message, err.kind
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_at_max_succeeds() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    //= specification/data-format/message-header.md#encrypted-data-key-count
    //= type=test
    //= reason=Setting max equal to actual count verifies the less-than-or-equal semantics.
    //# This value MUST be less than or equal to the [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys) if the maximum number is configured.
    let mut input =
        EncryptInput::with_legacy_keyring(b"at max", EncryptionContext::new(), mk);
    input.max_encrypted_data_keys = Some(std::num::NonZeroUsize::new(2).unwrap());
    assert!(
        encrypt(&input).await.is_ok(),
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

        // Verify we consumed exactly one entry and the position matches the parser
        let parsed = parse_edk_section(&ct, version);
        assert_eq!(
            pos, parsed.end_offset,
            "{version:?}: manual walk must match parser end offset"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_count_matches_entries() {
    let generator = aes_keyring(0).await;
    let c1 = aes_keyring(1).await;
    let c2 = aes_keyring(2).await;
    let mk = multi_keyring(generator, vec![c1, c2]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"3 edks", version, mk.clone()).await;
        let parsed = parse_edk_section(&ct, version);

        //= specification/data-format/message-header.md#encrypted-data-keys
        //= type=test
        //= reason=Using a multi-keyring with 3 keyrings verifies the serialized count matches the number of entries that follow, covering the "Count, then Entries" structure.
        //# The Encrypted Data Keys MUST consist of, in order,
        //# Encrypted Data Key Count,
        //# and Encrypted Data Key Entries.
        assert_eq!(
            parsed.edk_count, 3,
            "{version:?}: multi-keyring with 3 keyrings must produce 3 EDKs"
        );
        assert_eq!(
            parsed.edks.len(),
            3,
            "{version:?}: parsed entries must match count"
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
async fn test_key_provider_id_length_is_2_bytes() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pid len 2 bytes", version, keyring.clone()).await;
        let edk_start = skip_to_edk_section(&ct, version) + 2;

        //= specification/data-format/message-header.md#key-provider-id-length
        //= type=test
        //# The length of the serialized key provider ID length field MUST be 2 bytes.
        // The first 2 bytes of the entry are the provider ID length field
        let pid_len_bytes = &ct[edk_start..edk_start + 2];
        assert_eq!(
            pid_len_bytes.len(),
            2,
            "{version:?}: key provider ID length field must be exactly 2 bytes"
        );
        let pid_len = u16::from_be_bytes([pid_len_bytes[0], pid_len_bytes[1]]);
        let (expected_ns, _) = namespace_and_name(0);
        assert_eq!(
            pid_len as usize,
            expected_ns.len(),
            "{version:?}: provider ID length must equal the namespace string length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_length_interpreted_as_uint16() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pid len uint16", version, keyring.clone()).await;
        let edk_start = skip_to_edk_section(&ct, version) + 2;
        let (expected_ns, _) = namespace_and_name(0);
        let expected_len = expected_ns.len() as u16;

        //= specification/data-format/message-header.md#key-provider-id-length
        //= type=test
        //# The key provider ID length MUST be interpreted as a UInt16.
        // Verify big-endian UInt16 encoding
        assert_eq!(
            ct[edk_start],
            (expected_len >> 8) as u8,
            "{version:?}: high byte of UInt16 provider ID length"
        );
        assert_eq!(
            ct[edk_start + 1],
            (expected_len & 0xFF) as u8,
            "{version:?}: low byte of UInt16 provider ID length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_id_length_matches_field() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pid len match", version, keyring.clone()).await;
        let parsed = parse_edk_section(&ct, version);

        //= specification/data-format/message-header.md#key-provider-id
        //= type=test
        //# The length of the serialized key provider ID MUST be equal to the value of the [Key Provider ID Length](#key-provider-id-length) field.
        for edk in &parsed.edks {
            assert_eq!(
                edk.provider_id.len(),
                edk.provider_id_len as usize,
                "{version:?}: provider ID byte length must equal the provider ID length field"
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
async fn test_key_provider_info_length_is_2_bytes() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pinfo len 2 bytes", version, keyring.clone()).await;
        let parsed = parse_edk_section(&ct, version);
        // The provider info length was parsed as 2 bytes by our parser.
        // Verify it's consistent by checking the raw bytes at the expected offset.
        let edk_start = parsed.edk_count_offset + 2;
        let pid_len = parsed.edks[0].provider_id_len as usize;
        let pinfo_len_offset = edk_start + 2 + pid_len; // skip pid_len field + pid bytes

        //= specification/data-format/message-header.md#key-provider-information-length
        //= type=test
        //# The length of the serialized key provider information length field MUST be 2 bytes.
        let pinfo_len = u16::from_be_bytes([ct[pinfo_len_offset], ct[pinfo_len_offset + 1]]);
        assert_eq!(
            pinfo_len, parsed.edks[0].provider_info_len,
            "{version:?}: provider info length field must be 2 bytes wide"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_info_length_interpreted_as_uint16() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pinfo len uint16", version, keyring.clone()).await;
        let edk_start = skip_to_edk_section(&ct, version) + 2;
        // Walk the wire bytes: provider_id_len (2) + provider_id (pid_len) → then provider_info_len at that offset
        let pid_len = u16::from_be_bytes([ct[edk_start], ct[edk_start + 1]]);
        let pinfo_len_offset = edk_start + 2 + pid_len as usize;
        let wire_pinfo_len = u16::from_be_bytes([ct[pinfo_len_offset], ct[pinfo_len_offset + 1]]);
        let parsed = parse_edk_section(&ct, version);
        let edk = &parsed.edks[0];

        //= specification/data-format/message-header.md#key-provider-information-length
        //= type=test
        //# The key provider information length MUST be interpreted as a UInt16.
        // Decoding the length directly from the wire bytes as big-endian UInt16 must match
        // the parser's interpretation and the actual provider info byte length.
        assert_eq!(
            wire_pinfo_len as usize,
            edk.provider_info.len(),
            "{version:?}: big-endian UInt16 read from wire bytes must equal actual provider info length"
        );
        assert_eq!(
            wire_pinfo_len, edk.provider_info_len,
            "{version:?}: wire UInt16 must match parser-interpreted UInt16"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_key_provider_info_length_matches_field() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"pinfo len match", version, mk.clone()).await;
        let parsed = parse_edk_section(&ct, version);

        //= specification/data-format/message-header.md#key-provider-information
        //= type=test
        //# The length of the serialized key provider information MUST be equal to the value of the [Key Provider Information Length](#key-provider-information-length) field.
        for edk in &parsed.edks {
            assert_eq!(
                edk.provider_info.len(),
                edk.provider_info_len as usize,
                "{version:?}: provider info byte length must equal the provider info length field"
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
        // Provider info for raw AES keyring starts with the key name
        let (_, expected_name) = namespace_and_name(0);

        //= specification/data-format/message-header.md#key-provider-information
        //= type=test
        //# The key provider information MUST be interpreted as bytes.
        assert!(
            edk.provider_info.starts_with(expected_name.as_bytes()),
            "{version:?}: provider info must start with the known key name"
        );
        // Round-trip proves the bytes are correctly interpreted
        let mut dec =
            DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        if let Version::V1 = version {
            dec.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }
        let result = decrypt(&dec).await.unwrap();
        assert_eq!(result.plaintext, b"pinfo bytes");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_length_field_is_2_bytes() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk len 2 bytes", version, keyring.clone()).await;
        let parsed = parse_edk_section(&ct, version);
        let edk = &parsed.edks[0];
        // Walk to the EDK length field offset manually
        let edk_start = parsed.edk_count_offset + 2;
        let edk_len_offset = edk_start
            + 2 + edk.provider_id_len as usize   // pid_len field + pid bytes
            + 2 + edk.provider_info_len as usize; // pinfo_len field + pinfo bytes

        //= specification/data-format/message-header.md#encrypted-data-key-length
        //= type=test
        //# The length of the serialized encrypted data key length field MUST be 2 bytes.
        let edk_len = u16::from_be_bytes([ct[edk_len_offset], ct[edk_len_offset + 1]]);
        assert_eq!(
            edk_len, edk.edk_len,
            "{version:?}: encrypted data key length field must be exactly 2 bytes"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_length_interpreted_as_uint16() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk len uint16", version, keyring.clone()).await;
        let parsed = parse_edk_section(&ct, version);
        let edk = &parsed.edks[0];

        //= specification/data-format/message-header.md#encrypted-data-key-length
        //= type=test
        //# The encrypted data key length MUST be interpreted as a UInt16.
        // The UInt16 value must match the actual encrypted data key byte length
        assert_eq!(
            edk.edk_len as usize,
            edk.edk.len(),
            "{version:?}: UInt16 EDK length must match actual EDK bytes"
        );
        // For AES-GCM wrapping, the EDK is non-trivially sized (IV + ciphertext + tag)
        assert!(
            edk.edk_len > 0,
            "{version:?}: encrypted data key must have positive length"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_length_matches_field() {
    let generator = aes_keyring(0).await;
    let child = aes_keyring(1).await;
    let mk = multi_keyring(generator, vec![child]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk len match", version, mk.clone()).await;
        let parsed = parse_edk_section(&ct, version);

        //= specification/data-format/message-header.md#encrypted-data-key
        //= type=test
        //# The length of the serialized encrypted data key MUST be equal to the value of the [Encrypted Data Key Length](#encrypted-data-key-length) field.
        for (i, edk) in parsed.edks.iter().enumerate() {
            assert_eq!(
                edk.edk.len(),
                edk.edk_len as usize,
                "{version:?}: EDK {i}: encrypted data key byte length must equal the EDK length field"
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_interpreted_as_bytes() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk as bytes", version, keyring.clone()).await;
        let mut dec =
            DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), keyring.clone());
        if let Version::V1 = version {
            dec.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
        }

        //= specification/data-format/message-header.md#encrypted-data-key
        //= type=test
        //# The encrypted data key MUST be interpreted as bytes.
        let result = decrypt(&dec).await.unwrap();
        assert_eq!(
            result.plaintext, b"edk as bytes",
            "{version:?}: round-trip proves EDK bytes are correctly interpreted"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_edk_bytes_are_nonempty_ciphertext() {
    let keyring = aes_keyring(0).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"edk nonempty", version, keyring.clone()).await;
        let parsed = parse_edk_section(&ct, version);
        let edk = &parsed.edks[0];

        //= specification/data-format/message-header.md#encrypted-data-key
        //= type=test
        //= reason=Verifying the EDK contains non-zero bytes proves it holds actual encrypted key material, not a placeholder.
        //# The encrypted data key MUST be interpreted as bytes.
        assert!(
            !edk.edk.is_empty(),
            "{version:?}: encrypted data key must not be empty"
        );
        // AES-GCM wrapping produces at minimum IV (12 bytes) + tag (16 bytes) = 28 bytes
        assert!(
            edk.edk.len() >= 28,
            "{version:?}: EDK must be at least 28 bytes (AES-GCM IV + tag), got {}",
            edk.edk.len()
        );
        // The EDK should contain actual ciphertext (not all zeros)
        assert!(
            edk.edk.iter().any(|&b| b != 0),
            "{version:?}: encrypted data key must contain non-zero bytes (actual ciphertext)"
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_multi_keyring_round_trip_each_child() {
    let generator = aes_keyring(0).await;
    let c1 = aes_keyring(1).await;
    let c2 = aes_keyring(2).await;
    let mk = multi_keyring(generator.clone(), vec![c1.clone(), c2.clone()]).await;

    for version in VERSIONS {
        let ct = encrypt_with_version(b"multi rt", version, mk.clone()).await;

        // Each individual keyring should be able to decrypt
        for kr in [generator.clone(), c1.clone(), c2.clone()] {
            let mut dec = DecryptInput::with_legacy_keyring(&ct, EncryptionContext::new(), kr);
            if let Version::V1 = version {
                dec.commitment_policy = EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt;
            }
            let result = decrypt(&dec).await.unwrap();
            assert_eq!(result.plaintext, b"multi rt");
        }
    }
}

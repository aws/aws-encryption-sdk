// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! V1 message header body serialization and deserialization.

use super::encrypted_data_keys::{read_edks, write_edks};
use super::encryption_context::{read_canonical_ec, write_aad_section};
use super::shared_header_functions::{read_esdk_suite_id, read_message_id_v1, write_esdk_suite_id, write_message_id};
use super::{Error, ser_err};
use super::header_types::{MessageFormatVersion, V1HeaderBody, read_content_type, read_msg_type, write_content_type, write_msg_format_version, write_msg_type};
use super::serializable_types::get_iv_length;
use super::serialize_functions::{read_bytes, read_u32, read_u8, write_bytes, write_u32, write_u8};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::AlgorithmSuite;

//= specification/data-format/message-header.md#reserved
//# A reserved sequence of 4 bytes
//# that MUST have the value (hex) of `00 00 00 00`.

//= specification/data-format/message-header.md#reserved
//# The length of the serialized reserved field MUST be 4 bytes.
const RESERVED_BYTES: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

pub(crate) fn write_v1_header_body(
    w: &mut dyn SafeWrite,
    body: &V1HeaderBody,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#v1-header
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification:

    //= specification/client-apis/encrypt.md#v1-header
    //# The serialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification.

    //= specification/data-format/message-header.md#header-body-version-1-0
    //# The V1 Header Body MUST consist of, in order,
    //# Version,
    //# Type,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Reserved,
    //# IV Length,
    //# and Frame Length.

    // Version
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Version](../data-format/message-header.md#version): MUST be serialized according to the
    //# [Version](../data-format/message-header.md#version) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
    write_msg_format_version(
        w,
        //= specification/data-format/message-header.md#header-body-version-1-0
        //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
        MessageFormatVersion::V1,
    )?;

    // Type
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Type](../data-format/message-header.md#type): MUST be serialized according to the
    //# [Type](../data-format/message-header.md#type) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
    write_msg_type(w, body.message_type)?;

    // Algorithm Suite ID
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
    //# [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    write_esdk_suite_id(w, &body.algorithm_suite)?;

    // Message ID
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
    //# [Message ID](../data-format/message-header.md#message-id) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    write_message_id(w, &body.message_id)?;

    // AAD
    //= specification/client-apis/encrypt.md#v1-header
    //# - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
    //# [AAD](../data-format/message-header.md#aad) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    write_aad_section(w, &body.encryption_context)?;

    // Encrypted Data Keys
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
    //# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    write_edks(w, &body.encrypted_data_keys)?;

    // Content Type
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
    //# [Content Type](../data-format/message-header.md#content-type) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    write_content_type(w, body.content_type)?;

    // Reserved
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Reserved](../data-format/message-header.md#reserved): MUST be serialized according to the
    //# [Reserved](../data-format/message-header.md#reserved) specification.
    write_bytes(w, &RESERVED_BYTES)?;

    // IV Length
    //= specification/client-apis/encrypt.md#v1-header
    //# - [IV Length](../data-format/message-header.md#iv-length): MUST be serialized according to the
    //# [IV Length](../data-format/message-header.md#iv-length) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST match the [IV length](../framework/algorithm-suites.md#iv-length)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md).
    let iv_length = get_iv_length(&body.algorithm_suite);
    //= specification/data-format/message-header.md#iv-length
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    //= specification/data-format/message-header.md#iv-length
    //# The length of the serialized IV length field MUST be 1 byte.
    write_u8(
        w,
        //= specification/data-format/message-header.md#iv-length
        //# The IV length MUST be interpreted as a UInt8.
        iv_length,
    )?;

    // Frame Length
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
    //# [Frame Length](../data-format/message-header.md#frame-length) specification.
    //= specification/client-apis/encrypt.md#v1-header
    //# The value MUST be the value of the frame size determined above.
    let frame_len = body.frame_length;
    //= specification/data-format/message-header.md#frame-length
    //# The length of the serialized frame length field MUST be 4 bytes.
    write_u32(
        w,
        //= specification/data-format/message-header.md#frame-length
        //# The frame length MUST be interpreted as a UInt32.
        frame_len,
    )
}

pub(crate) fn read_v1_reserved_bytes(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    let mut result = [0; RESERVED_BYTES.len()];
    read_bytes(r, &mut result, raw)?;
    if result == RESERVED_BYTES {
        Ok(())
    } else {
        ser_err("Incorrect reserved bytes")
    }
}

pub(crate) fn read_v1_header_iv_length(
    r: &mut dyn SafeRead,
    suite: &AlgorithmSuite,
    raw: &mut dyn SafeWrite,
) -> Result<u8, Error> {
    //= specification/data-format/message-header.md#iv-length
    //# The length of the serialized IV length field MUST be 1 byte.
    //= specification/data-format/message-header.md#iv-length
    //# The IV length MUST be interpreted as a UInt8.
    let raw = read_u8(r, raw)?;
    //= specification/data-format/message-header.md#iv-length
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    if raw == get_iv_length(suite) {
        Ok(raw)
    } else {
        ser_err("Header IV length does not match algorithm suite")
    }
}

pub(crate) fn read_v1_header_body(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<V1HeaderBody, Error> {
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [Type](../data-format/message-header.md#type): MUST be deserialized according to the
    //# [Type](../data-format/message-header.md#type) specification.
    let message_type = read_msg_type(r, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be deserialized according to the
    //# [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
    let algorithm_suite = read_esdk_suite_id(r, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [Message ID](../data-format/message-header.md#message-id): MUST be deserialized according to the
    //# [Message ID](../data-format/message-header.md#message-id) specification.
    let message_id = read_message_id_v1(r, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [AAD](../data-format/message-header.md#aad): MUST be deserialized according to the
    //# [AAD](../data-format/message-header.md#aad) specification.
    let encryption_context: Vec<(String, String)> = read_canonical_ec(r, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be deserialized according to the
    //# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
    let encrypted_data_keys = read_edks(r, max_edks, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [Content Type](../data-format/message-header.md#content-type): MUST be deserialized according to the
    //# [Content Type](../data-format/message-header.md#content-type) specification.
    let content_type = read_content_type(r, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [Reserved](../data-format/message-header.md#reserved): MUST be deserialized according to the
    //# [Reserved](../data-format/message-header.md#reserved) specification.
    read_v1_reserved_bytes(r, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [IV Length](../data-format/message-header.md#iv-length): MUST be deserialized according to the
    //# [IV Length](../data-format/message-header.md#iv-length) specification.
    let header_iv_length = read_v1_header_iv_length(r, algorithm_suite, raw)?;
    //= specification/client-apis/decrypt.md#v1-header-deserialization
    //# - [Frame Length](../data-format/message-header.md#frame-length): MUST be deserialized according to the
    //# [Frame Length](../data-format/message-header.md#frame-length) specification.
    //= specification/data-format/message-header.md#frame-length
    //# The length of the serialized frame length field MUST be 4 bytes.
    let frame_length_raw = read_u32(r, raw)?;
    //= specification/data-format/message-header.md#frame-length
    //# The frame length MUST be interpreted as a UInt32.
    let frame_length = frame_length_raw;

    Ok(V1HeaderBody {
        message_type,
        algorithm_suite: algorithm_suite.clone(),
        message_id,
        encryption_context,
        encrypted_data_keys,
        content_type,
        header_iv_length: u64::from(header_iv_length),
        frame_length,
    })
}

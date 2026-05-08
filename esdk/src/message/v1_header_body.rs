// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! V1 message header body serialization and deserialization.

use super::encrypted_data_keys::{read_edks, write_edks};
use super::encryption_context::{read_canonical_ec, write_aad_section};
use super::header_types::{
    MessageFormatVersion, V1HeaderBody, read_content_type, read_msg_type, write_content_type,
    write_msg_format_version, write_msg_type,
};
use super::serializable_types::get_iv_length;
use super::serialize_functions::{read_bytes, read_u8, read_u32, write_bytes, write_u8, write_u32};
use super::shared_header_functions::{
    read_esdk_suite_id, read_message_id_v1, write_esdk_suite_id, write_message_id,
};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::AlgorithmSuite;

//= spec/data-format/message-header.md#reserved
//# A reserved sequence of 4 bytes
//# that MUST have the value (hex) of `00 00 00 00`.
const RESERVED_BYTES: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

pub(crate) fn write_v1_header_body(
    w: &mut dyn SafeWrite,
    body: &V1HeaderBody,
) -> Result<(), Error> {
    //= spec/data-format/message-header.md#header-body-version-1-0
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
    //
    //= spec/client-apis/encrypt.md#v1-header
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0,
    //# the remaining header fields MUST be serialized according to the
    //# [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification:
    //
    //= spec/client-apis/encrypt.md#v1-header
    //# The serialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification.

    // Version
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Version](../data-format/message-header.md#version).
    //# The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
    write_msg_format_version(
        w,
        //= spec/data-format/message-header.md#header-body-version-1-0
        //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
        MessageFormatVersion::V1,
    )?;

    // Type
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Type](../data-format/message-header.md#type).
    //# The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
    write_msg_type(w, body.message_type)?;

    // Algorithm Suite ID
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    write_esdk_suite_id(w, &body.algorithm_suite)?;

    // Message ID
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Message ID](../data-format/message-header.md#message-id).
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    write_message_id(w, &body.message_id)?;

    // AAD
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [AAD](../data-format/message-header.md#aad).
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    write_aad_section(w, &body.encryption_context)?;

    // Encrypted Data Keys
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    write_edks(w, &body.encrypted_data_keys)?;

    // Content Type
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Content Type](../data-format/message-header.md#content-type).
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    write_content_type(w, body.content_type)?;

    // Reserved
    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Reserved](../data-format/message-header.md#reserved).
    //
    //= spec/data-format/message-header.md#reserved
    //# The length of the serialized reserved field MUST be 4 bytes.
    write_bytes(w, &RESERVED_BYTES)?;

    // IV Length
    let iv_length = get_iv_length(&body.algorithm_suite);

    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [IV Length](../data-format/message-header.md#iv-length).
    //# The value MUST match the [IV length](../framework/algorithm-suites.md#iv-length)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md).
    //
    //= spec/data-format/message-header.md#iv-length
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    //
    //= spec/data-format/message-header.md#iv-length
    //# The length of the serialized IV length field MUST be 1 byte.
    //
    //= spec/data-format/message-header.md#iv-length
    //# The IV length MUST be interpreted as a UInt8.
    write_u8(w, iv_length)?;

    // Frame Length
    let frame_len = body.frame_length;

    //= spec/client-apis/encrypt.md#v1-header
    //# - MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).
    //# The value MUST be the value of the frame size determined above.
    //
    //= spec/data-format/message-header.md#frame-length
    //# The length of the serialized frame length field MUST be 4 bytes.
    write_u32(w, frame_len)
}

pub(crate) fn read_v1_reserved_bytes(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    let mut result = [0; RESERVED_BYTES.len()];
    read_bytes(r, &mut result, raw)?;
    //= spec/data-format/message-header.md#reserved
    //# A reserved sequence of 4 bytes
    //# that MUST have the value (hex) of `00 00 00 00`.
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
    let iv_len = read_u8(r, raw)?;

    //= spec/data-format/message-header.md#iv-length
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    if iv_len == get_iv_length(suite) {
        Ok(iv_len)
    } else {
        ser_err("Header IV length does not match algorithm suite")
    }
}

pub(crate) fn read_v1_header_body(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<V1HeaderBody, Error> {
    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# If the value of the deserialized version field is [1.0](../data-format/message-header.md#supported-versions),
    //# the remaining header fields MUST be deserialized according to the
    //# [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification:

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [Type](../data-format/message-header.md#type).
    let message_type = read_msg_type(r, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    let algorithm_suite = read_esdk_suite_id(r, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [Message ID](../data-format/message-header.md#message-id).
    let message_id = read_message_id_v1(r, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [AAD](../data-format/message-header.md#aad).
    let encryption_context: Vec<(String, String)> = read_canonical_ec(r, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    let encrypted_data_keys = read_edks(r, max_edks, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [Content Type](../data-format/message-header.md#content-type).
    let content_type = read_content_type(r, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [Reserved](../data-format/message-header.md#reserved).
    read_v1_reserved_bytes(r, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [IV Length](../data-format/message-header.md#iv-length).
    let header_iv_length = read_v1_header_iv_length(r, algorithm_suite, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# - MUST deserialize the [Frame Length](../data-format/message-header.md#frame-length).
    let frame_length = read_u32(r, raw)?;

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

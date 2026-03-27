// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::encrypted_data_keys::*;
use super::encryption_context::*;
use super::shared_header_functions::*;
use super::*;
use crate::message::header_types::*;
use crate::message::serializable_types::*;
use crate::message::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::AlgorithmSuite;

//= specification/data-format/message-header.md#reserved
//# A reserved sequence of 4 bytes
//# that MUST have the value (hex) of `00 00 00 00`.
const RESERVED_BYTES: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

pub(crate) fn write_v1_header_body(
    w: &mut dyn SafeWrite,
    body: &V1HeaderBody,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#v1-header
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# then the [message header body](../data-format/message-header.md#header-body-version-10) MUST be serialized with the following specifics:

    //= specification/client-apis/encrypt.md#v1-header
    //= type=implication
    //= reason=The sequential write calls in this function body enforce the serialization order structurally.
    //# The serialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification.

    //= specification/data-format/message-header.md#header-body-version-1-0
    //= type=implication
    //= reason=The sequential write calls below serialize each field in the specified order.
    //# The V1 Header Body MUST be serialized as, in order,
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

    //= specification/client-apis/encrypt.md#v1-header
    //# - [Version](../data-format/message-header.md#version): MUST be serialized according to the
    //# [Version](../data-format/message-header.md#version) specification.
    //# The value MUST correspond to [1.0](../data-format/message-header.md#supported-versions).
    //= specification/data-format/message-header.md#header-body-version-1-0
    //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
    write_msg_format_version(w, MessageFormatVersion::V1)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Type](../data-format/message-header.md#type): MUST be serialized according to the
    //# [Type](../data-format/message-header.md#type) specification.
    //# The value MUST correspond to [Customer Authenticated Encrypted Data](../data-format/message-header.md#supported-types).
    write_msg_type(w, body.message_type)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
    //# [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    write_esdk_suite_id(w, &body.algorithm_suite)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
    //# [Message ID](../data-format/message-header.md#message-id) specification.
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    write_message_id(w, &body.message_id)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
    //# [AAD](../data-format/message-header.md#aad) specification.
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    write_aad_section(w, &body.encryption_context)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
    //# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    write_edks(w, &body.encrypted_data_keys)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
    //# [Content Type](../data-format/message-header.md#content-type) specification.
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    write_content_type(w, body.content_type)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Reserved](../data-format/message-header.md#reserved): MUST be serialized according to the
    //# [Reserved](../data-format/message-header.md#reserved) specification.
    write_bytes(w, &RESERVED_BYTES)?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [IV Length](../data-format/message-header.md#iv-length): MUST be serialized according to the
    //# [IV Length](../data-format/message-header.md#iv-length) specification.
    //# The value MUST match the [IV length](../framework/algorithm-suites.md#iv-length)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md).
    //= specification/data-format/message-header.md#iv-length
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    write_u8(w, get_iv_length(&body.algorithm_suite))?;
    //= specification/client-apis/encrypt.md#v1-header
    //# - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
    //# [Frame Length](../data-format/message-header.md#frame-length) specification.
    //# The value MUST be the value of the frame size determined above.

    //= specification/data-format/message-header.md#frame-length
    //= type=implication
    //= reason=write_u32 writes exactly 4 bytes (big-endian u32)
    //# The length of the serialized frame length field MUST be 4 bytes.

    //= specification/data-format/message-header.md#frame-length
    //= type=implication
    //= reason=write_u32 serializes a u32 in big-endian format, which is UInt32
    //# The frame length MUST be serialized as a UInt32.
    write_u32(w, body.frame_length)
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
        ser_err("Incorrect reserved bytes.")
    }
}

pub(crate) fn read_v1_header_iv_length(
    r: &mut dyn SafeRead,
    suite: &AlgorithmSuite,
    raw: &mut dyn SafeWrite,
) -> Result<u8, Error> {
    let raw = read_u8(r, raw)?;
    if raw == get_iv_length(suite) {
        Ok(raw)
    } else {
        ser_err("HeaderIv Length does not match Algorithm Suite.")
    }
}

pub(crate) fn read_v1_header_body(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<V1HeaderBody, Error> {
    let message_type = read_msg_type(r, raw)?;
    let algorithm_suite = read_esdk_suite_id(r, raw)?;
    let message_id = read_message_id_v1(r, raw)?;
    let encryption_context: Vec<(String, String)> = read_canonical_ec(r, raw)?;
    let encrypted_data_keys = read_edks(r, max_edks, raw)?;
    //= specification/client-apis/decrypt.md#parse-the-header
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
    //# deserialized from the [message header](../data-format/message-header.md)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# then as soon as that can be determined during deserializing
    //# decrypt MUST process no more bytes and yield an error.
    if let Some(max) = max_edks {
        if encrypted_data_keys.len() > max.get() {
            return ser_err("Number of encrypted data keys exceeds the maximum allowed.");
        }
    }
    let content_type = read_content_type(r, raw)?;
    read_v1_reserved_bytes(r, raw)?;
    let header_iv_length = read_v1_header_iv_length(r, algorithm_suite, raw)?;
    //= specification/data-format/message-header.md#frame-length
    //= type=implication
    //= reason=read_u32 reads exactly 4 bytes (big-endian u32)
    //# The length of the serialized frame length field MUST be 4 bytes.

    //= specification/data-format/message-header.md#frame-length
    //= type=implication
    //= reason=read_u32 deserializes a big-endian u32, which is UInt32
    //# The frame length MUST be serialized as a UInt32.
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

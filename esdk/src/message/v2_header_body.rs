// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! V2 message header body serialization and deserialization.

use super::encrypted_data_keys::{read_edks, write_edks};
use super::encryption_context::{read_canonical_ec, write_aad_section};
use super::header_types::{
    MessageFormatVersion, V2HeaderBody, read_content_type, write_content_type,
    write_msg_format_version,
};
use super::serialize_functions::{read_u32, read_vec, write_bytes, write_u32};
use super::shared_header_functions::{
    read_esdk_suite_id, read_message_id_v2, write_esdk_suite_id, write_message_id,
};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::DerivationAlgorithm;

pub(crate) fn write_v2_header_body(
    w: &mut dyn SafeWrite,
    body: &V2HeaderBody,
) -> Result<(), Error> {
    //= spec/client-apis/encrypt.md#v2-header
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0
    //# then the [message header body](../data-format/message-header.md#header-body-version-1-0) MUST be serialized with the following specifics:

    // Version
    //= spec/client-apis/encrypt.md#v2-header
    //# - [Version](../data-format/message-header.md#version-1): MUST have a value corresponding to
    //# [2.0](../data-format/message-header.md#supported-versions)
    write_msg_format_version(
        w,
        //= spec/data-format/message-header.md#header-body-version-2-0
        //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
        MessageFormatVersion::V2,
    )?;

    // Algorithm Suite ID
    //= spec/client-apis/encrypt.md#v2-header
    //# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST correspond to
    //# the [algorithm suite](../framework/algorithm-suites.md) used in this behavior
    write_esdk_suite_id(w, &body.algorithm_suite)?;

    // Message ID
    //= spec/client-apis/encrypt.md#v2-header
    //# - [Message ID](../data-format/message-header.md#message-id): The process used to generate
    //# this identifier MUST use a good source of randomness to make the chance of duplicate identifiers negligible.
    write_message_id(w, &body.message_id)?;

    // AAD
    //= spec/client-apis/encrypt.md#v2-header
    //# - [AAD](../data-format/message-header.md#aad): MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    write_aad_section(w, &body.encryption_context)?;

    // Encrypted Data Keys
    //= spec/client-apis/encrypt.md#v2-header
    //# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-key-entries): MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials)
    write_edks(w, &body.encrypted_data_keys)?;

    // Content Type
    //= spec/client-apis/encrypt.md#v2-header
    //# - [Content Type](../data-format/message-header.md#content-type): MUST be [02](../data-format/message-header.md#supported-content-types)
    write_content_type(w, body.content_type)?;

    // Frame Length
    //= spec/client-apis/encrypt.md#v2-header
    //# - [Frame Length](../data-format/message-header.md#frame-length): MUST be the value of the frame size determined above.
    write_u32(w, body.frame_length)?;

    // Algorithm Suite Data
    //= spec/client-apis/encrypt.md#v2-header
    //# - [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data): MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    write_bytes(w, &body.suite_data)
}

pub(crate) fn get_hkdf(x: &DerivationAlgorithm) -> Result<&aws_mpl_legacy::suites::Hkdf, Error> {
    if let DerivationAlgorithm::Hkdf(x) = x {
        Ok(x)
    } else {
        ser_err("DerivationAlgorithm must be HKDF")
    }
}
pub(crate) const fn has_hkdf(x: &DerivationAlgorithm) -> bool {
    matches!(x, DerivationAlgorithm::Hkdf(_))
}

pub(crate) fn read_v2_header_body(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<V2HeaderBody, Error> {
    //= spec/client-apis/decrypt.md#parse-the-header
    //# Given encrypted message bytes, this operation MUST process those bytes sequentially,
    //# deserializing those bytes according to the [message format](../data-format/message.md).

    let algorithm_suite = read_esdk_suite_id(r, raw)?;
    if !has_hkdf(&algorithm_suite.commitment) {
        return ser_err("Algorithm suite must support commitment");
    }

    let message_id = read_message_id_v2(r, raw)?;

    let encryption_context: Vec<(String, String)> = read_canonical_ec(r, raw)?;

    let encrypted_data_keys = read_edks(r, max_edks, raw)?;

    let content_type = read_content_type(r, raw)?;

    let frame_length = read_u32(r, raw)?;

    let len = get_hkdf(&algorithm_suite.commitment)?.output_key_length;
    let Ok(len_usize) = usize::try_from(len) else {
        return ser_err("Algorithm suite data length exceeds platform capacity");
    };
    let suite_data = read_vec(r, len_usize, raw)?;

    Ok(V2HeaderBody {
        algorithm_suite: algorithm_suite.clone(),
        message_id,
        encryption_context,
        encrypted_data_keys,
        content_type,
        frame_length,
        suite_data,
    })
}

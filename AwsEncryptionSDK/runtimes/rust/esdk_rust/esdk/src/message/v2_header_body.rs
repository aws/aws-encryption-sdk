// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::encrypted_data_keys::*;
use super::encryption_context::*;
use super::shared_header_functions::*;
use super::*;
use crate::message::header_types::*;
use crate::message::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::DerivationAlgorithm;

pub(crate) fn write_v2_header_body(
    w: &mut dyn SafeWrite,
    body: &V2HeaderBody,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#v2-header
    //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0
    //# then the [message header body](../data-format/message-header.md#header-body-version-1-0) MUST be serialized with the following specifics:

    //= specification/client-apis/encrypt.md#v2-header
    //# - [Version](../data-format/message-header.md#version-1): MUST have a value corresponding to
    //# [2.0](../data-format/message-header.md#supported-versions)
    //= specification/data-format/message-header.md#header-body-version-2-0
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    write_msg_format_version(w, MessageFormatVersion::V2)?;
    //= specification/client-apis/encrypt.md#v2-header
    //# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST correspond to
    //# the [algorithm suite](../framework/algorithm-suites.md) used in this behavior
    write_esdk_suite_id(w, &body.algorithm_suite)?;
    //= specification/client-apis/encrypt.md#v2-header
    //# - [Message ID](../data-format/message-header.md#message-id): The process used to generate
    //# this identifier MUST use a good source of randomness to make the chance of duplicate identifiers negligible.
    write_message_id(w, &body.message_id)?;
    //= specification/client-apis/encrypt.md#v2-header
    //# - [AAD](../data-format/message-header.md#aad): MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    write_aad_section(w, &body.encryption_context)?;
    //= specification/client-apis/encrypt.md#v2-header
    //# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-key-entries): MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials)
    write_edks(w, &body.encrypted_data_keys)?;
    //= specification/client-apis/encrypt.md#v2-header
    //# - [Content Type](../data-format/message-header.md#content-type): MUST be [02](../data-format/message-header.md#supported-content-types)
    write_content_type(w, body.content_type)?;
    //= specification/client-apis/encrypt.md#v2-header
    //# - [Frame Length](../data-format/message-header.md#frame-length): MUST be the value of the frame size determined above.
    write_u32(w, body.frame_length)?;
    //= specification/client-apis/encrypt.md#v2-header
    //# - [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data): MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    write_bytes(w, &body.suite_data)
}

pub(crate) const fn get_hkdf(x: &DerivationAlgorithm) -> &aws_mpl_legacy::suites::Hkdf {
    if let DerivationAlgorithm::Hkdf(x) = x {
        x
    } else {
        panic!()
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
    let algorithm_suite = read_esdk_suite_id(r, raw)?;
    if !has_hkdf(&algorithm_suite.commitment) {
        return ser_err("Algorithm suite must support commitment.");
    }

    let message_id = read_message_id_v2(r, raw)?;
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
    let frame_length = read_u32(r, raw)?;
    let len = get_hkdf(&algorithm_suite.commitment).output_key_length;
    let suite_data = read_vec(r, len as usize, raw)?;

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

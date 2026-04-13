// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::encrypted_data_keys::{read_edks, write_edks};
use super::encryption_context::{read_canonical_ec, write_aad_section};
use super::shared_header_functions::{read_esdk_suite_id, read_message_id_v2, write_esdk_suite_id, write_message_id};
use super::{Error, ser_err};
use crate::message::header_types::{MessageFormatVersion, V2HeaderBody, read_content_type, write_content_type, write_msg_format_version};
use crate::message::serialize_functions::{read_u32, read_vec, write_bytes, write_u32};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::DerivationAlgorithm;

//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
//# the remaining header fields MUST be serialized according to the
//# [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification:
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Version](../data-format/message-header.md#version): MUST be serialized according to the
//# [Version](../data-format/message-header.md#version) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be serialized according to the
//# [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Message ID](../data-format/message-header.md#message-id): MUST be serialized according to the
//# [Message ID](../data-format/message-header.md#message-id) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The process used to generate this identifier MUST use a good source of randomness
//# to make the chance of duplicate identifiers negligible.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [AAD](../data-format/message-header.md#aad): MUST be serialized according to the
//# [AAD](../data-format/message-header.md#aad) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
//# in the [encryption materials](../framework/structures.md#encryption-materials),
//# and this serialization MUST NOT contain any key value pairs listed in
//# the [encryption material's](../framework/structures.md#encryption-materials)
//# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be serialized according to the
//# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST be the serialization of the
//# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Content Type](../data-format/message-header.md#content-type): MUST be serialized according to the
//# [Content Type](../data-format/message-header.md#content-type) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST be [02](../data-format/message-header.md#supported-content-types).
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Frame Length](../data-format/message-header.md#frame-length): MUST be serialized according to the
//# [Frame Length](../data-format/message-header.md#frame-length) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST be the value of the frame size determined above.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data): MUST be serialized according to the
//# [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data) specification.
//= aws-encryption-sdk-specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
//# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
pub(crate) fn write_v2_header_body(
    w: &mut dyn SafeWrite,
    body: &V2HeaderBody,
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#header-body-version-2-0
    //# The V2 Header Body MUST consist of, in order,
    //# Version,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Frame Length,
    //# and Algorithm Suite Data.

    // Version
    //= specification/data-format/message-header.md#header-body-version-2-0
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    write_msg_format_version(w, MessageFormatVersion::V2)?;

    // Algorithm Suite ID
    write_esdk_suite_id(w, &body.algorithm_suite)?;

    // Message ID
    write_message_id(w, &body.message_id)?;

    // AAD
    write_aad_section(w, &body.encryption_context)?;

    // Encrypted Data Keys
    write_edks(w, &body.encrypted_data_keys)?;

    // Content Type
    write_content_type(w, body.content_type)?;

    // Frame Length
    //= specification/data-format/message-header.md#frame-length
    //# The length of the serialized frame length field MUST be 4 bytes.
    //= specification/data-format/message-header.md#frame-length
    //# The frame length MUST be interpreted as a UInt32.
    write_u32(w, body.frame_length)?;

    // Algorithm Suite Data
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

//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id): MUST be deserialized according to the
//# [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id) specification.
//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [Message ID](../data-format/message-header.md#message-id): MUST be deserialized according to the
//# [Message ID](../data-format/message-header.md#message-id) specification.
//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [AAD](../data-format/message-header.md#aad): MUST be deserialized according to the
//# [AAD](../data-format/message-header.md#aad) specification.
//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys): MUST be deserialized according to the
//# [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys) specification.
//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [Content Type](../data-format/message-header.md#content-type): MUST be deserialized according to the
//# [Content Type](../data-format/message-header.md#content-type) specification.
//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [Frame Length](../data-format/message-header.md#frame-length): MUST be deserialized according to the
//# [Frame Length](../data-format/message-header.md#frame-length) specification.
//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//# - [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data): MUST be deserialized according to the
//# [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data) specification.
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
    let content_type = read_content_type(r, raw)?;
    //= specification/data-format/message-header.md#frame-length
    //# The length of the serialized frame length field MUST be 4 bytes.
    //= specification/data-format/message-header.md#frame-length
    //# The frame length MUST be interpreted as a UInt32.
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

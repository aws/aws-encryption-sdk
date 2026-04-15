// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! V2 message header body serialization and deserialization.

use super::encrypted_data_keys::{read_edks, write_edks};
use super::encryption_context::{read_canonical_ec, write_aad_section};
use super::shared_header_functions::{read_esdk_suite_id, read_message_id_v2, write_esdk_suite_id, write_message_id};
use super::{Error, ser_err};
use super::header_types::{MessageFormatVersion, V2HeaderBody, read_content_type, write_content_type, write_msg_format_version};
use super::serialize_functions::{read_u32, read_vec, write_bytes, write_u32};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::DerivationAlgorithm;

//= specification/client-apis/encrypt.md#v2-header
//# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0,
//# the remaining header fields MUST be serialized according to the
//# [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification:

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The serialization order MUST follow the [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification.

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [Version](../data-format/message-header.md#version).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [Message ID](../data-format/message-header.md#message-id).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The process used to generate this identifier MUST use a good source of randomness
//# to make the chance of duplicate identifiers negligible.

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [AAD](../data-format/message-header.md#aad).

//= specification/client-apis/encrypt.md#v2-header
//# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
//# in the [encryption materials](../framework/structures.md#encryption-materials),
//# and this serialization MUST NOT contain any key value pairs listed in
//# the [encryption material's](../framework/structures.md#encryption-materials)
//# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).

//= specification/client-apis/encrypt.md#v2-header
//# The value MUST be the serialization of the
//# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [Content Type](../data-format/message-header.md#content-type).

//= specification/client-apis/encrypt.md#v2-header
//# The value MUST be [02](../data-format/message-header.md#supported-content-types).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# The value MUST be the value of the frame size determined above.

//= specification/client-apis/encrypt.md#v2-header
//= type=implication
//# - The Encrypt operation MUST serialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).

//= specification/client-apis/encrypt.md#v2-header
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
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# - The Decrypt operation MUST deserialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    let algorithm_suite = read_esdk_suite_id(r, raw)?;
    if !has_hkdf(&algorithm_suite.commitment) {
        return ser_err("Algorithm suite must support commitment");
    }

    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# - The Decrypt operation MUST deserialize the [Message ID](../data-format/message-header.md#message-id).
    let message_id = read_message_id_v2(r, raw)?;
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# - The Decrypt operation MUST deserialize the [AAD](../data-format/message-header.md#aad).
    let encryption_context: Vec<(String, String)> = read_canonical_ec(r, raw)?;
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# - The Decrypt operation MUST deserialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    let encrypted_data_keys = read_edks(r, max_edks, raw)?;
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# - The Decrypt operation MUST deserialize the [Content Type](../data-format/message-header.md#content-type).
    let content_type = read_content_type(r, raw)?;
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# - The Decrypt operation MUST deserialize the [Frame Length](../data-format/message-header.md#frame-length).

    //= specification/data-format/message-header.md#frame-length
    //# The length of the serialized frame length field MUST be 4 bytes.

    //= specification/data-format/message-header.md#frame-length
    //# The frame length MUST be interpreted as a UInt32.
    let frame_length = read_u32(r, raw)?;
    //= specification/client-apis/decrypt.md#v2-header-deserialization
    //# - The Decrypt operation MUST deserialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    let len = get_hkdf(&algorithm_suite.commitment)?.output_key_length;
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

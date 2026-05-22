// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! V2 message header body serialization and deserialization.

use super::encrypted_data_keys::{read_edks, write_edks};
use super::encryption_context::{read_canonical_ec, write_aad_section};
use super::header_types::{
    MessageFormatVersion, V2HeaderBody, read_content_type, write_content_type,
    write_msg_format_version,
    read_esdk_suite_id, read_message_id_v2, write_esdk_suite_id, write_message_id,
};
use super::serialize_functions::{read_u32, read_vec, write_bytes, write_u32};
use super::{Error, ser_err};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::DerivationAlgorithm;

pub(crate) fn write_v2_header_body(
    w: &mut dyn SafeWrite,
    body: &V2HeaderBody,
) -> Result<(), Error> {
    //= spec/data-format/message-header.md#header-body-version-2-0
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

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [Version](../data-format/message-header.md#version).
    //# The value MUST correspond to [2.0](../data-format/message-header.md#supported-versions).
    write_msg_format_version(
        w,
        //= spec/data-format/message-header.md#header-body-version-2-0
        //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
        MessageFormatVersion::V2,
    )?;

    // Algorithm Suite ID

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    //# The value MUST correspond to the [algorithm suite](../framework/algorithm-suites.md) used in this behavior.
    write_esdk_suite_id(w, &body.algorithm_suite)?;

    // Message ID

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [Message ID](../data-format/message-header.md#message-id).
    //
    //= spec/client-apis/encrypt.md#v2-header
    //= reason=randomness is sourced via aws_mpl_legacy::primitives::generate_random_bytes
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    write_message_id(w, &body.message_id)?;

    // AAD

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [AAD](../data-format/message-header.md#aad).
    //
    //= spec/client-apis/encrypt.md#v2-header
    //= reason=required EC keys are filtered out in shared code before the v2 header-specific code
    //# The value MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    write_aad_section(w, &body.encryption_context)?;

    // Encrypted Data Keys

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    //# The value MUST be the serialization of the
    //# [encrypted data keys](../framework/structures.md#encrypted-data-keys) in the [encryption materials](../framework/structures.md#encryption-materials).
    write_edks(w, &body.encrypted_data_keys)?;

    // Content Type

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [Content Type](../data-format/message-header.md#content-type).
    //# The value MUST be [02](../data-format/message-header.md#supported-content-types).
    write_content_type(w, body.content_type)?;

    // Frame Length

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [Frame Length](../data-format/message-header.md#frame-length).
    //# The value MUST be the value of the frame size determined above.
    write_u32(w, body.frame_length)?;

    // Algorithm Suite Data

    //= spec/client-apis/encrypt.md#v2-header
    //# - MUST serialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    //
    //= spec/client-apis/encrypt.md#v2-header
    //= reason=the commit key is derived during encryption materials generation and stored in suite_data for later use during commitment validation
    //# The value MUST be the value of the [commit key](../framework/algorithm-suites.md#commit-key)
    //# derived according to the [algorithm suites commit key derivation settings](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    //
    //= spec/data-format/message-header.md#algorithm-suite-data
    //= type=implication
    //# The algorithm suite data MUST be interpreted as bytes.
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
    //= spec/data-format/message-header.md#header-body-version-2-0
    //# The V2 Header Body MUST consist of, in order,
    //# Version,
    //# Algorithm Suite ID,
    //# Message ID,
    //# AAD,
    //# Encrypted Data Keys,
    //# Content Type,
    //# Frame Length,
    //# and Algorithm Suite Data.

    // Algorithm Suite ID

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# - MUST deserialize the [Algorithm Suite ID](../data-format/message-header.md#algorithm-suite-id).
    let algorithm_suite = read_esdk_suite_id(r, raw)?;
    if !has_hkdf(&algorithm_suite.commitment) {
        return ser_err("Algorithm suite must support commitment");
    }

    // Message ID

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# - MUST deserialize the [Message ID](../data-format/message-header.md#message-id).
    let message_id = read_message_id_v2(r, raw)?;

    // AAD

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# - MUST deserialize the [AAD](../data-format/message-header.md#aad).
    let encryption_context: Vec<(String, String)> = read_canonical_ec(r, raw)?;

    // Encrypted Data Keys

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# - MUST deserialize the [Encrypted Data Keys](../data-format/message-header.md#encrypted-data-keys).
    let encrypted_data_keys = read_edks(r, max_edks, raw)?;

    // Content Type

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# - MUST deserialize the [Content Type](../data-format/message-header.md#content-type).
    let content_type = read_content_type(r, raw)?;

    // Frame Length

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# - MUST deserialize the [Frame Length](../data-format/message-header.md#frame-length).
    let frame_length = read_u32(r, raw)?;

    // Algorithm Suite Data

    let len = get_hkdf(&algorithm_suite.commitment)?.output_key_length;
    let Ok(len_usize) = usize::try_from(len) else {
        return ser_err("Algorithm suite data length exceeds platform capacity");
    };
    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# - MUST deserialize the [Algorithm Suite Data](../data-format/message-header.md#algorithm-suite-data).
    //
    //= spec/data-format/message-header.md#algorithm-suite-data
    //= type=implication
    //# The algorithm suite data MUST be interpreted as bytes.
    let suite_data = read_vec(r, len_usize, raw)?;

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= type=implication
    //= reason=std::io::Read blocks until bytes are available or EOF; the sequential read calls above inherently wait for consumable bytes
    //# This operation MUST wait if it doesn't have enough consumable encrypted message bytes to
    //# deserialize the next field of the message header until enough input bytes become consumable or
    //# the caller indicates an end to the encrypted message.

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

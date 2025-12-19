// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::encrypted_data_keys::*;
use super::encryption_context::*;
use super::shared_header_functions::*;
use super::*;
use crate::serialize::header_types::*;
use crate::serialize::serializable_types::*;
use crate::serialize::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};

//= compliance/data-format/message-header.txt#2.5.2.1
//= type=implication
//# A reserved sequence of 4 bytes that MUST have the value (hex) of "00
//# 00 00 00".
const RESERVED_BYTES: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

pub(crate) fn write_v1_header_body(
    w: &mut dyn SafeWrite,
    body: &V1HeaderBody,
) -> Result<(), Error> {
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# If the message format version associated with the algorithm suite
    //# (../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
    //# then the message header body (../data-format/message-
    //# header.md#header-body-version-1-0) MUST be serialized with the
    //# following specifics:

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Version (../data-format/message-header.md#version-1): MUST have a
    //# value corresponding to 1.0 (../data-format/message-
    //# header.md#supported-versions)
    write_msg_format_version(w, MessageFormatVersion::V1)?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Type (../data-format/message-header.md#type): MUST have a value
    //# corresponding to Customer Authenticated Encrypted Data (../data-
    //# format/message-header.md#supported-types)
    write_msg_type(w, body.message_type)?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Algorithm Suite ID (../data-format/message-header.md#algorithm-
    //# suite-id): MUST correspond to the algorithm suite (../framework/
    //# algorithm-suites.md) used in this behavior
    write_esdk_suite_id(w, &body.algorithm_suite)?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Message ID (../data-format/message-header.md#message-id): The
    //# process used to generate this identifier MUST use a good source of
    //# randomness to make the chance of duplicate identifiers negligible.
    write_message_id(w, &body.message_id)?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  AAD (../data-format/message-header.md#aad): MUST be the
    //# serialization of the encryption context (../framework/
    //# structures.md#encryption-context) in the encryption materials
    //# (../framework/structures.md#encryption-materials)
    write_aad_section(w, &body.encryption_context)?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Encrypted Data Keys (../data-format/message-header.md#encrypted-
    //# data-key-entries): MUST be the serialization of the encrypted data
    //# keys (../framework/structures.md#encrypted-data-keys) in the
    //# encryption materials (../framework/structures.md#encryption-
    //# materials)
    write_edks(w, &body.encrypted_data_keys)?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Content Type (../data-format/message-header.md#content-type): MUST
    //# be 02 (../data-format/message-header.md#supported-content-types)
    write_content_type(w, body.content_type)?;
    write_bytes(w, &RESERVED_BYTES)?;

    //= compliance/data-format/message-header.txt#2.5.2.2
    //# This value MUST be
    //# equal to the IV length (../framework/algorithm-suites.md#iv-length)
    //# value of the algorithm suite (../framework/algorithm-suites.md)
    //# specified by the Algorithm Suite ID (Section 2.5.1.5) field.
    //
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  IV Length (../data-format/message-header.md#iv-length): MUST match
    //# the IV length (../framework/algorithm-suites.md#iv-length)
    //# specified by the algorithm suite (../framework/algorithm-
    //# suites.md)
    write_u8(w, get_iv_length(&body.algorithm_suite))?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# *  Frame Length (../data-format/message-header.md#frame-length): MUST
    //# be the value of the frame size determined above.
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
    suite: &aws_mpl_legacy::types::AlgorithmSuiteInfo,
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
    mpl: &aws_mpl_legacy::Client,
    raw: &mut dyn SafeWrite,
) -> Result<V1HeaderBody, Error> {
    let message_type = read_msg_type(r, raw)?;
    let algorithm_suite = read_esdk_suite_id(r, mpl, raw)?;
    let message_id = read_message_id_v1(r, raw)?;
    let encryption_context: Vec<(String, String)> = read_canonical_ec(r, raw)?;
    let encrypted_data_keys = read_edks(r, max_edks, raw)?;
    let content_type = read_content_type(r, raw)?;
    read_v1_reserved_bytes(r, raw)?;
    let header_iv_length = read_v1_header_iv_length(r, &algorithm_suite, raw)?;
    let frame_length = read_u32(r, raw)?;

    Ok(V1HeaderBody {
        message_type,
        algorithm_suite,
        message_id,
        encryption_context,
        encrypted_data_keys,
        content_type,
        header_iv_length: u64::from(header_iv_length),
        frame_length,
    })
}

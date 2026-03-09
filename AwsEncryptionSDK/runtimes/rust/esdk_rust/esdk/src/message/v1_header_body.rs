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
    //= specification/data-format/message-header.md#version
    //# The version (hex) of this field MUST be a value that exists in the following table:
    //= specification/data-format/message-header.md#header-body-version-1-0
    //# The value of the `Version` field MUST be `01` in the Version 1.0 header body.
    write_msg_format_version(w, MessageFormatVersion::V1)?;
    //= specification/data-format/message-header.md#type
    //# The type (hex) of this field MUST be a value that exists in the following table:
    write_msg_type(w, body.message_type)?;
    write_esdk_suite_id(w, &body.algorithm_suite)?;
    //= specification/data-format/message-header.md#message-id
    //# A Message ID MUST uniquely identify the [message](message.md).
    write_message_id(w, &body.message_id)?;
    write_aad_section(w, &body.encryption_context)?;
    write_edks(w, &body.encrypted_data_keys)?;
    write_content_type(w, body.content_type)?;
    write_bytes(w, &RESERVED_BYTES)?;
    //= specification/data-format/message-header.md#iv-length
    //# This value MUST be equal to the [IV length](../framework/algorithm-suites.md#iv-length) value of the
    //# [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    write_u8(w, get_iv_length(&body.algorithm_suite))?;
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
    let content_type = read_content_type(r, raw)?;
    read_v1_reserved_bytes(r, raw)?;
    let header_iv_length = read_v1_header_iv_length(r, algorithm_suite, raw)?;
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

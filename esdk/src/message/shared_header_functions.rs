// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Shared header field read/write functions for V1 and V2 formats.

use super::Error;
use super::header_types::{MESSAGE_ID_LEN_V1, MESSAGE_ID_LEN_V2, MessageId};
use super::serialize_functions::{read_bytes, read_vec, write_bytes};
use crate::error::val_err_with_cause;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) fn read_esdk_suite_id(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<&'static AlgorithmSuite, Error> {
    //= spec/data-format/message-header.md#algorithm-suite-id
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    let mut esdk_suite_id_bytes = [0; 2];
    read_bytes(r, &mut esdk_suite_id_bytes, raw)?;

    //= spec/data-format/message-header.md#algorithm-suite-id
    //# The value (hex) of this field MUST be a value that exists in the
    //# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.
    //
    //= spec/data-format/message-header.md#algorithm-suite-id
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    let suite = aws_mpl_legacy::suites::get_algorithm_suite_info(esdk_suite_id_bytes)
        .map_err(|e| val_err_with_cause(
            format!("Unrecognized or unsupported algorithm suite ID: 0x{:02X}{:02X}", esdk_suite_id_bytes[0], esdk_suite_id_bytes[1]),
            e,
        ))?;
    Ok(suite)
}

pub(crate) fn read_message_id_v1(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    //= spec/data-format/message-header.md#message-id
    //# The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.
    read_vec(r, MESSAGE_ID_LEN_V1, raw)
}
pub(crate) fn read_message_id_v2(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    //= spec/data-format/message-header.md#message-id
    //# The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
    read_vec(r, MESSAGE_ID_LEN_V2, raw)
}

pub(crate) fn write_esdk_suite_id(
    w: &mut dyn SafeWrite,
    suite: &AlgorithmSuite,
) -> Result<(), Error> {
    //= spec/data-format/message-header.md#algorithm-suite-id
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    write_bytes(w, &suite.binary_id[..])
}

pub(crate) fn write_message_id(w: &mut dyn SafeWrite, message_id: &MessageId) -> Result<(), Error> {
    write_bytes(w, message_id)
}

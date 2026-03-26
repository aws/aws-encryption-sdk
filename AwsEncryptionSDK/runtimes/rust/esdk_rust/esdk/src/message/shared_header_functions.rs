// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header_types::*;
use super::*;
use crate::message::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) fn read_esdk_suite_id(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<&'static AlgorithmSuite, Error> {
    let mut esdk_suite_id_bytes = [0; 2];
    read_bytes(r, &mut esdk_suite_id_bytes, raw)?;

    let suite = aws_mpl_legacy::suites::get_algorithm_suite_info(esdk_suite_id_bytes)?;
    Ok(suite)
}

pub(crate) fn read_message_id_v1(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    //= aws-encryption-sdk-specification/data-format/message-header.md#message-id
    //# The length of the serialized message ID MUST be 16 bytes for [version 1.0](#header-body-version-10) headers.
    read_vec(r, MESSAGE_ID_LEN_V1 as usize, raw)
}
pub(crate) fn read_message_id_v2(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    //= aws-encryption-sdk-specification/data-format/message-header.md#message-id
    //# The length of the serialized message ID MUST be 32 bytes for [version 2.0](#header-body-version-20) headers.
    read_vec(r, MESSAGE_ID_LEN_V2 as usize, raw)
}

//= specification/data-format/message-header.md#algorithm-suite-id
//# The value (hex) of this field MUST be a value that exists in the
//# [Supported Algorithm Suites](../framework/algorithm-suites.md#supported-algorithm-suites) table.

//= specification/data-format/message-header.md#algorithm-suite-id
//# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
pub(crate) fn write_esdk_suite_id(
    w: &mut dyn SafeWrite,
    suite: &AlgorithmSuite,
) -> Result<(), Error> {
    write_bytes(w, &suite.binary_id[..])
}

//= specification/data-format/message-header.md#message-id
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn write_message_id(w: &mut dyn SafeWrite, message_id: &MessageId) -> Result<(), Error> {
    //= aws-encryption-sdk-specification/data-format/message-header.md#message-id
    //= type=implication
    //= reason=MessageId is Vec<u8>; write_bytes treats it as raw bytes
    //# The message ID MUST be interpreted as bytes.
    write_bytes(w, message_id)
}

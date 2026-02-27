// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header_types::*;
use super::*;
use crate::serialize::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_rs::suites::AlgorithmSuite;

pub(crate) fn read_esdk_suite_id(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<&'static AlgorithmSuite, Error> {
    let mut esdk_suite_id_bytes = [0; 2];
    read_bytes(r, &mut esdk_suite_id_bytes, raw)?;

    let suite = aws_mpl_rs::suites::get_algorithm_suite_info(esdk_suite_id_bytes)?;
    Ok(suite)
}

pub(crate) fn read_message_id_v1(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
    read_vec(r, MESSAGE_ID_LEN_V1 as usize, raw)
}
pub(crate) fn read_message_id_v2(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageId, Error> {
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

/*
 * Writes the message id as bytes, which, since the message id is already stored
 * as bytes, simply returns the message id.
 *
 * Though we have different V1 and V2 methods for the read path, since
 * they read different numbers of bytes, a single method on the write path
 * is fine since writing is identical for both.
 */
pub(crate) fn write_message_id(w: &mut dyn SafeWrite, message_id: &MessageId) -> Result<(), Error> {
    write_bytes(w, message_id)
}

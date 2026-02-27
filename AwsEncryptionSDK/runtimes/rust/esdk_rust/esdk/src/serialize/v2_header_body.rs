// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::encrypted_data_keys::*;
use super::encryption_context::*;
use super::shared_header_functions::*;
use super::*;
use crate::serialize::header_types::*;
use crate::serialize::serialize_functions::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_rs::suites::DerivationAlgorithm;

pub(crate) fn write_v2_header_body(
    w: &mut dyn SafeWrite,
    body: &V2HeaderBody,
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#header-body-version-2-0
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    //= specification/data-format/message-header.md#header-body-version-2-0
    //# The value of the `Version` field MUST be `02` in the Version 2.0 header body.
    write_msg_format_version(w, MessageFormatVersion::V2)?;
    write_esdk_suite_id(w, &body.algorithm_suite)?;
    //= specification/data-format/message-header.md#message-id
    //# A Message ID MUST uniquely identify the [message](message.md).
    write_message_id(w, &body.message_id)?;
    write_aad_section(w, &body.encryption_context)?;
    write_edks(w, &body.encrypted_data_keys)?;
    write_content_type(w, body.content_type)?;
    write_u32(w, body.frame_length)?;
    write_bytes(w, &body.suite_data)
}

pub(crate) const fn get_hkdf(x: &DerivationAlgorithm) -> &aws_mpl_rs::suites::Hkdf {
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

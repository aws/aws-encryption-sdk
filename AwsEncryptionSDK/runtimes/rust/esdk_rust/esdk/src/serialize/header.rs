// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header_types::*;
use super::serializable_types::*;
use super::v1_header_body::*;
use super::v2_header_body::*;
use super::*;
use crate::types::{SafeRead, SafeWrite};

pub(crate) const START_SEQUENCE_NUMBER: u32 = 1;
//= ../specification/data-format/message-body.md#sequence-number-end
//= type=implication
//# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
pub(crate) const ENDFRAME_SEQUENCE_NUMBER: u32 = 0xFFFF_FFFF;
pub(crate) const NONFRAMED_SEQUENCE_NUMBER: u32 = 1;
pub(crate) const SAFE_MAX_ENCRYPT: u64 = 0x000F_FFFF_FFE0;
use aws_mpl_rs::suites::AlgorithmSuite;
use aws_mpl_rs::suites::DerivationAlgorithm;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct HeaderInfo {
    pub(crate) body: HeaderBody,
    pub(crate) raw_header: Vec<u8>,
    pub(crate) encryption_context: ESDKEncryptionContext,
    pub(crate) suite: AlgorithmSuite,
    pub(crate) header_auth: HeaderAuth,
}

//= ../specification/data-format/message-header.md#structure
//= type=implication
//# The message header is a sequence of bytes that MUST be in big-endian format.
pub(crate) fn write_header_body(w: &mut dyn SafeWrite, body: &HeaderBody) -> Result<(), Error> {
    match body {
        HeaderBody::V1Body(x) => write_v1_header_body(w, x),
        HeaderBody::V2Body(x) => write_v2_header_body(w, x),
    }
}

pub(crate) fn read_header_body(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw: &mut dyn SafeWrite,
) -> Result<HeaderBody, Error> {
    let version = read_msg_format_version(r, raw)?;

    let result = match version {
        MessageFormatVersion::V1 => {
            let body = read_v1_header_body(r, max_edks, raw)?;
            HeaderBody::V1Body(body)
        }
        MessageFormatVersion::V2 => {
            let body = read_v2_header_body(r, max_edks, raw)?;
            HeaderBody::V2Body(body)
        }
    };

    match result.content_type() {
        ContentType::Framed => {
            if result.frame_length() == 0 {
                return ser_err("Frame length must be positive if content is framed");
            }
        }
        ContentType::NonFramed => {
            if result.frame_length() != 0 {
                return ser_err("Frame length must be zero if content is non-framed");
            }
        }
    }
    Ok(result)
}

pub(crate) const fn header_version_supports_commitment(
    suite: &AlgorithmSuite,
    body: &HeaderBody,
) -> bool {
    match (suite.commitment, body) {
        (DerivationAlgorithm::Hkdf(header), HeaderBody::V2Body(body)) => {
            body.suite_data.len() == header.output_key_length as usize
        }
        (_, HeaderBody::V1Body(_)) => true,
        _ => false,
    }
}

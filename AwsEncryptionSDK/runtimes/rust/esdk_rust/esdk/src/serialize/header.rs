// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header_types::*;
use super::serializable_types::*;
use super::v1_header_body::*;
use super::v2_header_body::*;
use super::*;
use crate::types::{SafeRead, SafeWrite};

//= compliance/client-apis/encrypt.txt#2.7.1
//= type=implication
//# If this is the first frame sequentially, this
//# value MUST be 1.
pub(crate) const START_SEQUENCE_NUMBER: u32 = 1;
pub(crate) const ENDFRAME_SEQUENCE_NUMBER: u32 = 0xFFFF_FFFF;
pub(crate) const NONFRAMED_SEQUENCE_NUMBER: u32 = 1;
pub(crate) const SAFE_MAX_ENCRYPT: u64 = 0x000F_FFFF_FFE0;

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct HeaderInfo {
    pub(crate) body: HeaderBody,
    pub(crate) raw_header: Vec<u8>,
    pub(crate) encryption_context: ESDKEncryptionContext,
    pub(crate) suite: aws_mpl_legacy::types::AlgorithmSuiteInfo,
    pub(crate) header_auth: HeaderAuth,
}

pub(crate) fn write_header_body(w: &mut dyn SafeWrite, body: &HeaderBody) -> Result<(), Error> {
    match body {
        HeaderBody::V1Body(x) => write_v1_header_body(w, x),
        HeaderBody::V2Body(x) => write_v2_header_body(w, x),
    }
}

// ReadHeaderBody does not support streaming at this time
//= compliance/client-apis/decrypt.txt#2.7.1
//= type=exception
//# This operation MUST wait if it doesn't have enough consumable
//# encrypted message bytes to deserialize the next field of the message
//# header until enough input bytes become consumable or the caller
//# indicates an end to the encrypted message.

//= compliance/client-apis/decrypt.txt#2.7.1
//# This operation MUST attempt to deserialize all consumable encrypted
//# message bytes until it has successfully deserialized a valid message
//# header (../data-format/message-header.md).
pub(crate) fn read_header_body(
    r: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    mpl: &aws_mpl_legacy::Client,
    raw: &mut dyn SafeWrite,
) -> Result<HeaderBody, Error> {
    let version = read_msg_format_version(r, raw)?;

    let result = match version {
        MessageFormatVersion::V1 => {
            let body = read_v1_header_body(r, max_edks, mpl, raw)?;
            HeaderBody::V1Body(body)
        }
        MessageFormatVersion::V2 => {
            let body = read_v2_header_body(r, max_edks, mpl, raw)?;
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
    suite: &aws_mpl_legacy::types::AlgorithmSuiteInfo,
    body: &HeaderBody,
) -> bool {
    match (suite.commitment.as_ref().unwrap(), body) {
        (aws_mpl_legacy::types::DerivationAlgorithm::Hkdf(header), HeaderBody::V2Body(body)) => {
            body.suite_data.len() == header.output_key_length.unwrap() as usize
        }
        (_, HeaderBody::V1Body(_)) => true,
        _ => false,
    }
}

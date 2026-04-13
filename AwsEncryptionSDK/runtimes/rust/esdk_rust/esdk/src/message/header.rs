// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header_types::*;
use super::serializable_types::*;
use super::v1_header_body::*;
use super::v2_header_body::*;
use super::*;
use crate::error::val_err;
use crate::types::{SafeRead, SafeWrite};

pub(crate) const START_SEQUENCE_NUMBER: u32 = 1;
//= specification/data-format/message-body.md#sequence-number-end
//# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
pub(crate) const ENDFRAME_SEQUENCE_NUMBER: u32 = 0xFFFF_FFFF;
pub(crate) const NONFRAMED_SEQUENCE_NUMBER: u32 = 1;
pub(crate) const SAFE_MAX_ENCRYPT: u64 = 0x000F_FFFF_FFE0;
use aws_mpl_legacy::suites::AlgorithmSuite;
use aws_mpl_legacy::suites::DerivationAlgorithm;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct HeaderInfo {
    pub(crate) body: HeaderBody,
    pub(crate) raw_header: Vec<u8>,
    pub(crate) encryption_context: ESDKEncryptionContext,
    pub(crate) suite: AlgorithmSuite,
    pub(crate) header_auth: HeaderAuth,
}

//= specification/data-format/message-header.md#structure
//# The message header is a sequence of bytes that MUST be in big-endian format.
pub(crate) fn write_header_body(w: &mut dyn SafeWrite, body: &HeaderBody) -> Result<(), Error> {
    match body {
        HeaderBody::V1Body(x) => write_v1_header_body(w, x),
        HeaderBody::V2Body(x) => write_v2_header_body(w, x),
    }
}

//= aws-encryption-sdk-specification/client-apis/decrypt.md#parse-the-header
//= type=implication
//= reason=SafeRead (std::io::Read) only supports sequential consumption with no skip/seek,
//= reason=so reading from it inherently processes all consumable bytes until a valid header is formed.
//# This operation MUST attempt to deserialize all consumable encrypted message bytes
//# until it has successfully deserialized a valid [message header](../data-format/message-header.md).
//= aws-encryption-sdk-specification/client-apis/decrypt.md#v2-header-deserialization
//= type=implication
//= reason=SafeRead (std::io::Read) only supports sequential consumption with no skip/seek,
//= reason=so reading from it inherently processes all consumable bytes until a valid header is formed.
//# This operation MUST wait if it doesn't have enough consumable encrypted message bytes
//# to deserialize the next field of the message header until enough input bytes become consumable
//# or the caller indicates an end to the encrypted message.
pub(crate) fn read_header_body(
    ciphertext: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw_header: &mut dyn SafeWrite,
) -> Result<HeaderBody, Error> {
    //= specification/client-apis/decrypt.md#parse-the-header
    //= reason=Every read method reads the next available bytes and does not jump out of sequence
    //# Given encrypted message bytes, this operation MUST process those bytes sequentially,
    //# deserializing those bytes according to the [message format](../data-format/message.md).
    //= specification/client-apis/decrypt.md#parse-the-header
    //# Each header field MUST be deserialized according to its specification in the [message header](../data-format/message-header.md):
    //= specification/client-apis/decrypt.md#parse-the-header
    //# The [Version](../data-format/message-header.md#version) field MUST be deserialized first.
    let version = read_msg_format_version(ciphertext, raw_header)?;

    //= specification/client-apis/decrypt.md#parse-the-header
    //# The header deserialization order MUST follow the [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10)
    //# or [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification,
    //# depending on the [Version](../data-format/message-header.md#version) field.
    let result = match version {
        //= specification/client-apis/decrypt.md#v1-header-deserialization
        //# If the version is [1.0](../data-format/message-header.md#supported-versions),
        //# the remaining header fields MUST be deserialized according to the
        //# [Header Body Version 1.0](../data-format/message-header.md#header-body-version-10) specification:
        MessageFormatVersion::V1 => {
            let body = read_v1_header_body(ciphertext, max_edks, raw_header)?;
            HeaderBody::V1Body(body)
        }
        //= specification/client-apis/decrypt.md#v2-header-deserialization
        //# If the version is [2.0](../data-format/message-header.md#supported-versions),
        //# the remaining header fields MUST be deserialized according to the
        //# [Header Body Version 2.0](../data-format/message-header.md#header-body-version-20) specification:
        MessageFormatVersion::V2 => {
            let body = read_v2_header_body(ciphertext, max_edks, raw_header)?;
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
            //= specification/data-format/message-header.md#frame-length
            //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
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

pub(crate) fn validate_max_encrypted_data_keys(
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    edks: &[aws_mpl_legacy::EncryptedDataKey],
) -> Result<(), Error> {
    if let Some(max) = max_encrypted_data_keys {
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //# This value MUST be less than or equal to the
        //# [maximum number of encrypted data keys](../client-apis/client.md#maximum-number-of-encrypted-data-keys)
        //# if the maximum number is configured.
        if edks.len() > max.get() {
            return Err(val_err("Encrypted data keys exceed maxEncryptedDataKeys"));
        }
        //= specification/data-format/message-header.md#encrypted-data-key-count
        //# This value MUST be greater than 0.
        if edks.is_empty() {
            return Err(val_err("Encrypted data keys is empty."));
        }
    }
    Ok(())
}

pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {
    let length = if suite.message_version == 1 {
        MESSAGE_ID_LEN_V1
    } else {
        MESSAGE_ID_LEN_V2
    };
    let mut rand_bytes: Vec<u8> = vec![0; length as usize];
    //= specification/data-format/message-header.md#message-id
    //= reason=Assuming the MPL uses a good source of randomness
    //# While implementations cannot guarantee complete uniqueness,
    //# implementations MUST use a good source of randomness when generating messages IDs in order to make
    //# the chance of duplicate IDs negligible.
    aws_mpl_legacy::primitives::generate_random_bytes(&mut rand_bytes)?;
    Ok(rand_bytes)
}

pub(crate) fn validate_suite_data(
    suite: &AlgorithmSuite,
    header_body: &HeaderBody,
    expected_suite_data: &[u8],
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#algorithm-suite-data
    //= reason=Check against expected_suite_data (a &[u8] type) implies interpreting as bytes
    //# The algorithm suite data MUST be interpreted as bytes.
    if header_body.suite_data() != expected_suite_data {
        return Err(val_err("Commitment key does not match"));
    }
    //= specification/data-format/message-header.md#algorithm-suite-data
    //# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
    //# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    if get_hkdf(&suite.commitment).output_key_length != expected_suite_data.len() as u32 {
        return Err(val_err("Commitment key is invalid"));
    }
    Ok(())
}

/// Serialize the message header (body + auth tag) to the output stream.
pub(crate) fn serialize_header(
    header: &HeaderInfo,
    ciphertext: &mut dyn SafeWrite,
    sig_digest: &mut DigestWriter,
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#structure
    //# The header MUST consist of, in order,
    //# Header Body,
    //# and Header Authentication.
    let mut header_buf = Vec::new();
    // Header body
    serialize_functions::write_bytes(&mut header_buf, &header.raw_header)?;
    // Header Authentication
    header_auth::write_header_auth_tag(&mut header_buf, &header.header_auth, &header.suite)?;
    serialize_functions::write_bytes(ciphertext, &header_buf)?;
    //= specification/client-apis/encrypt.md#authentication-tag
    //# If the algorithm suite contains a signature algorithm and
    //# this operation is [streaming](streaming.md) the encrypted message output to the caller,
    //# this operation MUST input the serialized header to the signature algorithm as soon as it is serialized,
    //# such that the serialized header isn't required to remain in memory to [construct the signature](#construct-the-signature).
    serialize_functions::write_bytes(sig_digest, &header_buf)?;
    Ok(())
}

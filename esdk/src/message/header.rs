// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Top-level message header construction and parsing.

use super::header_types::{
    ContentType, HeaderAuth, HeaderBody, MESSAGE_ID_LEN_V1, MESSAGE_ID_LEN_V2,
    MessageFormatVersion, MessageId, read_msg_format_version,
};
use super::serializable_types::ESDKEncryptionContext;
use super::v1_header_body::{read_v1_header_body, write_v1_header_body};
use super::v2_header_body::{get_hkdf, read_v2_header_body, write_v2_header_body};
use super::{DigestWriter, Error, header_auth, ser_err, serialize_functions};
use crate::error::val_err;
use crate::types::{SafeRead, SafeWrite};
use aws_lc_rs::constant_time;
use aws_mpl_legacy::suites::{AlgorithmSuite, DerivationAlgorithm};

pub(crate) const START_SEQUENCE_NUMBER: u32 = 1;

//= spec/data-format/message-body.md#sequence-number-end
//# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
pub(crate) const ENDFRAME_SEQUENCE_NUMBER: u32 = 0xFFFF_FFFF;
pub(crate) const NONFRAMED_SEQUENCE_NUMBER: u32 = 1;
pub(crate) const SAFE_MAX_ENCRYPT: u64 = 0x000F_FFFF_FFE0;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct HeaderInfo {
    pub(crate) body: HeaderBody,
    pub(crate) raw_header: Vec<u8>,
    pub(crate) encryption_context: ESDKEncryptionContext,
    pub(crate) suite: AlgorithmSuite,
    pub(crate) header_auth: HeaderAuth,
}

//= spec/data-format/message-header.md#structure
//# The message header is a sequence of bytes that MUST be in big-endian format.
pub(crate) fn write_header_body(w: &mut dyn SafeWrite, body: &HeaderBody) -> Result<(), Error> {
    match body {
        HeaderBody::V1Body(x) => write_v1_header_body(w, x),
        HeaderBody::V2Body(x) => write_v2_header_body(w, x),
    }
}

//= spec/client-apis/decrypt.md#parse-the-header
//# This operation MUST attempt to deserialize all consumable encrypted message bytes until it has
//# successfully deserialized a valid [message header](../data-format/message-header.md).
//
//= spec/client-apis/decrypt.md#parse-the-header
//= reason=SafeRead (std::io::Read) only supports sequential consumption with no skip/seek, so reading from it inherently processes all consumable bytes until a valid header is formed.
//# This operation MUST wait if it doesn't have enough consumable encrypted message bytes to
//# deserialize the next field of the message header until enough input bytes become consumable or
//# the caller indicates an end to the encrypted message.
pub(crate) fn read_header_body(
    ciphertext: &mut dyn SafeRead,
    max_edks: Option<std::num::NonZeroUsize>,
    raw_header: &mut dyn SafeWrite,
) -> Result<HeaderBody, Error> {
    //= spec/client-apis/decrypt.md#parse-the-header
    //= reason=Every read method reads the next available bytes and does not jump out of sequence
    //# Given encrypted message bytes, this operation MUST process those bytes sequentially,
    //# deserializing those bytes according to the [message format](../data-format/message.md).
    let version = read_msg_format_version(ciphertext, raw_header)?;

    let result = match version {
        MessageFormatVersion::V1 => {
            let body = read_v1_header_body(ciphertext, max_edks, raw_header)?;
            HeaderBody::V1Body(body)
        }

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
            if result.frame_length() != 0 {
                //= spec/data-format/message-header.md#frame-length
                //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
                return ser_err("Frame length must be zero if content is nonframed");
            }
        }
    }
    Ok(result)
}

pub(crate) fn header_version_supports_commitment(
    suite: &AlgorithmSuite,
    body: &HeaderBody,
) -> bool {
    match (suite.commitment, body) {
        (DerivationAlgorithm::Hkdf(header), HeaderBody::V2Body(body)) => {
            usize::try_from(header.output_key_length)
                .map(|n| body.suite_data.len() == n)
                .unwrap_or(false)
        }
        (_, HeaderBody::V1Body(_)) => true,
        _ => false,
    }
}

pub(crate) fn validate_max_encrypted_data_keys(
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    edks: &[aws_mpl_legacy::EncryptedDataKey],
) -> Result<(), Error> {
    if edks.is_empty() {
        //= spec/data-format/message-header.md#encrypted-data-key-count
        //# This value MUST be greater than 0.
        return Err(val_err("Encrypted data key count must be greater than 0"));
    }

    if let Some(max) = max_encrypted_data_keys {
        if edks.len() > max.get() {
            //= spec/client-apis/decrypt.md#parse-the-header
            //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
            //# deserialized from the [message header](../data-format/message-header.md)
            //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
            //# then as soon as that can be determined during deserializing
            //# decrypt MUST process no more bytes and yield an error.
            return Err(val_err(format!(
                "Encrypted data key count {} exceeds configured maximum {}",
                edks.len(),
                max.get(),
            )));
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
    let mut rand_bytes: Vec<u8> = vec![0; length];

    //= spec/data-format/message-header.md#message-id
    //= reason=Uniqueness follows from drawing sufficient randomness; the same randomness call satisfies both sub-items.
    //# A Message ID MUST uniquely identify the [message](message.md).
    //
    //= spec/data-format/message-header.md#message-id
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
    if constant_time::verify_slices_are_equal(header_body.suite_data(), expected_suite_data)
        .is_err()
    {
        return Err(val_err("Algorithm suite data does not match expected value"));
    }

    let Ok(suite_data_len) = u32::try_from(expected_suite_data.len()) else {
        return Err(val_err("Algorithm suite data length exceeds u32::MAX"));
    };
    //= spec/data-format/message-header.md#algorithm-suite-data
    //# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
    //# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    if get_hkdf(&suite.commitment)?.output_key_length != suite_data_len {
        return Err(val_err(
            "Algorithm suite data length does not match the algorithm suite's declared suite-data length",
        ));
    }
    Ok(())
}

/// Write the message header (body + auth tag) to the output stream.
pub(crate) fn write_header(
    header: &HeaderInfo,
    ciphertext: &mut dyn SafeWrite,
    sig_digest: &mut DigestWriter,
) -> Result<(), Error> {
    let mut header_buf = Vec::new();
    // Header body
    serialize_functions::write_bytes(&mut header_buf, &header.raw_header)?;
    // Header Authentication
    header_auth::write_header_auth_tag(&mut header_buf, &header.header_auth, &header.suite)?;
    serialize_functions::write_bytes(ciphertext, &header_buf)?;

    //= spec/client-apis/encrypt.md#authentication-tag
    //# If the algorithm suite contains a signature algorithm and
    //# this operation is [streaming](streaming.md) the encrypted message output to the caller,
    //# this operation MUST input the serialized header to the signature algorithm as soon as it is serialized,
    //# such that the serialized header isn't required to remain in memory to [construct the signature](#construct-the-signature).
    serialize_functions::write_bytes(sig_digest, &header_buf)?;
    Ok(())
}

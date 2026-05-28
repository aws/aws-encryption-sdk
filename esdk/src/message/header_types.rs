// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Type definitions for message header fields.

use super::serializable_types::ESDKCanonicalEncryptionContext;
use super::serialize_functions::{read_bytes, read_u8, read_vec, write_bytes, write_u8};
use super::{Error, ser_err};
use crate::error::val_err_with_cause;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::EncryptedDataKey;
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) type MessageId = Vec<u8>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum MessageFormatVersion {
    V1 = 1,
    V2 = 2,
}

pub(crate) fn write_msg_format_version(
    w: &mut dyn SafeWrite,
    data: MessageFormatVersion,
) -> Result<(), Error> {
    //= spec/data-format/message-header.md#version
    //# The length of the serialized version field MUST be 1 byte.
    write_u8(w, data as u8)
}

pub(crate) fn read_msg_format_version(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageFormatVersion, Error> {
    let version = read_u8(r, raw)?;
    //= spec/data-format/message-header.md#supported-versions
    //# The supported versions MUST be:
    //
    //= spec/client-apis/decrypt.md#parse-the-header
    //# The value MUST be a [supported version](../data-format/message-header.md#supported-versions).
    match version {
        //= spec/data-format/message-header.md#supported-versions
        //# - Hex value `01` MUST be version 1.0
        val if val == MessageFormatVersion::V1 as u8 => Ok(MessageFormatVersion::V1),
        //= spec/data-format/message-header.md#supported-versions
        //# - Hex value `02` MUST be version 2.0
        val if val == MessageFormatVersion::V2 as u8 => Ok(MessageFormatVersion::V2),
        //= spec/client-apis/decrypt.md#encrypted-message-format
        //# To make diagnosing this mistake easier, implementations SHOULD detect the first two bytes of the Base64 encoding of any supported message [versions](../data-format/message-header.md#version)
        //# and [types](../data-format/message-header.md#type)
        //# and fail with a more specific error message.
        0x41 => ser_err(
            "Input appears to be Base64-encoded. The ESDK expects raw binary message format, not Base64",
        ),
        _ => ser_err(&format!("unsupported message format version: {version:#04x}")),
    }
}

//= spec/data-format/message-header.md#supported-types
//# The supported types MUST be:
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum MessageType {
    //= spec/data-format/message-header.md#supported-types
    //# - `80` MUST be Customer Authenticated Encrypted Data
    #[default]
    TypeCustomerAed = 0x80,
}

pub(crate) fn write_msg_type(w: &mut dyn SafeWrite, data: MessageType) -> Result<(), Error> {
    //= spec/data-format/message-header.md#type
    //# The length of the serialized type field MUST be 1 byte.
    write_u8(w, data as u8)
}

pub(crate) fn read_msg_type(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageType, Error> {
    //= spec/data-format/message-header.md#type
    //# The length of the serialized type field MUST be 1 byte.
    let msg_type = read_u8(r, raw)?;

    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# The value MUST be a [supported type](../data-format/message-header.md#supported-types).
    match msg_type {
        val if val == MessageType::TypeCustomerAed as u8 => Ok(MessageType::TypeCustomerAed),
        _ => ser_err(&format!("unsupported message type: {msg_type:#04x}")),
    }
}

//= spec/data-format/message-header.md#supported-content-types
//# The supported content types MUST be:
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum ContentType {
    NonFramed = 1,
    #[default]
    Framed = 2,
}

pub(crate) fn write_content_type(w: &mut dyn SafeWrite, data: ContentType) -> Result<(), Error> {
    //= spec/data-format/message-header.md#content-type
    //# The length of the serialized content type field MUST be 1 byte.
    write_u8(w, data as u8)
}

pub(crate) fn read_content_type(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<ContentType, Error> {
    let content_type = read_u8(r, raw)?;
    //= spec/data-format/message-header.md#content-type
    //# The length of the serialized content type field MUST be 1 byte.
    //
    //= spec/client-apis/decrypt.md#v1-header-deserialization
    //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
    //
    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
    match content_type {
        val if val == ContentType::NonFramed as u8 => Ok(ContentType::NonFramed),
        val if val == ContentType::Framed as u8 => Ok(ContentType::Framed),
        _ => ser_err(&format!("unsupported content type: {content_type:#04x}")),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct V1HeaderBody {
    pub(crate) message_type: MessageType,
    pub(crate) algorithm_suite: AlgorithmSuite,
    pub(crate) message_id: MessageId,
    pub(crate) encryption_context: ESDKCanonicalEncryptionContext,
    pub(crate) encrypted_data_keys: Vec<EncryptedDataKey>,
    pub(crate) content_type: ContentType,
    pub(crate) header_iv_length: u8,
    pub(crate) frame_length: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct V2HeaderBody {
    pub(crate) algorithm_suite: AlgorithmSuite,
    pub(crate) message_id: MessageId,
    pub(crate) encryption_context: ESDKCanonicalEncryptionContext,
    pub(crate) encrypted_data_keys: Vec<EncryptedDataKey>,
    pub(crate) content_type: ContentType,
    pub(crate) frame_length: u32,
    pub(crate) suite_data: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HeaderBody {
    V1Body(V1HeaderBody),
    V2Body(V2HeaderBody),
}

impl HeaderBody {
    pub(crate) const fn frame_length(&self) -> u32 {
        match self {
            Self::V1Body(body) => body.frame_length,
            Self::V2Body(body) => body.frame_length,
        }
    }
    pub(crate) const fn content_type(&self) -> ContentType {
        match self {
            Self::V1Body(body) => body.content_type,
            Self::V2Body(body) => body.content_type,
        }
    }
    pub const fn message_id(&self) -> &[u8] {
        match self {
            Self::V1Body(body) => body.message_id.as_slice(),
            Self::V2Body(body) => body.message_id.as_slice(),
        }
    }
    pub(crate) const fn encryption_context(&self) -> &ESDKCanonicalEncryptionContext {
        match self {
            Self::V1Body(body) => &body.encryption_context,
            Self::V2Body(body) => &body.encryption_context,
        }
    }
    pub(crate) const fn encrypted_data_keys(&self) -> &[EncryptedDataKey] {
        match self {
            Self::V1Body(body) => body.encrypted_data_keys.as_slice(),
            Self::V2Body(body) => body.encrypted_data_keys.as_slice(),
        }
    }
    pub const fn algorithm_suite(&self) -> &AlgorithmSuite {
        match self {
            Self::V1Body(body) => &body.algorithm_suite,
            Self::V2Body(body) => &body.algorithm_suite,
        }
    }
    pub(crate) const fn suite_data(&self) -> &[u8] {
        match self {
            Self::V1Body(_) => &[],
            Self::V2Body(body) => body.suite_data.as_slice(),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum HeaderAuth {
    AESMac {
        header_iv: Vec<u8>,
        header_auth_tag: Vec<u8>,
    },
}

impl HeaderAuth {
    pub(crate) fn header_iv(&self) -> &[u8] {
        match self {
            Self::AESMac { header_iv, .. } => header_iv,
        }
    }
    pub(crate) fn header_auth_tag(&self) -> &[u8] {
        match self {
            Self::AESMac {
                header_auth_tag, ..
            } => header_auth_tag,
        }
    }
}

pub(crate) const MESSAGE_ID_LEN_V1: usize = 16;
pub(crate) const MESSAGE_ID_LEN_V2: usize = 32;

// --- Shared header read/write functions (suite ID, message ID) ---

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

pub(crate) fn write_esdk_suite_id(
    w: &mut dyn SafeWrite,
    suite: &AlgorithmSuite,
) -> Result<(), Error> {
    //= spec/data-format/message-header.md#algorithm-suite-id
    //# The length of the serialized algorithm suite ID field MUST be 2 bytes.
    debug_assert_eq!(suite.binary_id.len(), 2);
    write_bytes(w, &suite.binary_id[..])
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

pub(crate) fn write_message_id(w: &mut dyn SafeWrite, message_id: &MessageId) -> Result<(), Error> {
    write_bytes(w, message_id)
}

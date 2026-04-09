// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::serializable_types::*;
use super::serialize_functions::*;
use super::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::EncryptedDataKey;
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) type MessageId = Vec<u8>;

//= specification/data-format/message-header.md#supported-versions
//# The supported versions MUST be:
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum MessageFormatVersion {
    //= specification/data-format/message-header.md#supported-versions
    //# - Hex value `01` MUST be version 1.0
    V1 = 1,
    //= specification/data-format/message-header.md#supported-versions
    //# - Hex value `02` MUST be version 2.0
    V2 = 2,
}

pub(crate) fn write_msg_format_version(
    w: &mut dyn SafeWrite,
    data: MessageFormatVersion,
) -> Result<(), Error> {
    //= specification/data-format/message-header.md#version
    //# The length of the serialized version field MUST be 1 byte.
    write_u8(w, data as u8)
}

pub(crate) fn read_msg_format_version(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageFormatVersion, Error> {
    //= specification/data-format/message-header.md#version
    //# The length of the serialized version field MUST be 1 byte.
    let version = read_u8(r, raw)?;
    match version {
        val if val == MessageFormatVersion::V1 as u8 => Ok(MessageFormatVersion::V1),
        val if val == MessageFormatVersion::V2 as u8 => Ok(MessageFormatVersion::V2),
        //= specification/client-apis/decrypt.md#encrypted-message-format
        //# To make diagnosing this mistake easier, implementations SHOULD detect the first two bytes of the Base64 encoding of any supported message [versions](../data-format/message-header.md#version-1)
        //# and [types](../data-format/message-header.md#type)
        //# and fail with a more specific error message.
        0x41 => ser_err("Input appears to be Base64-encoded. The ESDK expects raw binary message format, not Base64."),
        //= specification/client-apis/decrypt.md#parse-the-header
        //# The value MUST be a [supported version](../data-format/message-header.md#supported-versions).
        _ => ser_err("Unsupported Version."),
    }
}

//= specification/data-format/message-header.md#supported-types
//# The supported types MUST be:
//= specification/data-format/message-header.md#type
//# The type (hex) of this field MUST be a value that exists in the following table:
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum MessageType {
    //= specification/data-format/message-header.md#supported-types
    //# - `80` MUST be Customer Authenticated Encrypted Data
    #[default]
    TypeCustomerAed = 0x80,
}

pub(crate) fn write_msg_type(w: &mut dyn SafeWrite, data: MessageType) -> Result<(), Error> {
    //= specification/data-format/message-header.md#type
    //# The length of the serialized type field MUST be 1 byte.
    write_u8(w, data as u8)
}

pub(crate) fn read_msg_type(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<MessageType, Error> {
    //= specification/data-format/message-header.md#type
    //# The length of the serialized type field MUST be 1 byte.
    let msg_type = read_u8(r, raw)?;
    match msg_type {
        val if val == MessageType::TypeCustomerAed as u8 => Ok(MessageType::TypeCustomerAed),
        //= specification/client-apis/decrypt.md#v1-header-deserialization
        //# The value MUST be a [supported type](../data-format/message-header.md#supported-types).
        _ => ser_err("Unsupported Message Type."),
    }
}

//= specification/data-format/message-header.md#supported-content-types
//# The supported content types MUST be:
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) enum ContentType {
    //= specification/data-format/message-header.md#supported-content-types
    //# - `01` for [Non-Framed](message-body.md#non-framed-data)
    NonFramed = 1,
    //= specification/data-format/message-header.md#supported-content-types
    //# - `02` for [Framed](message-body.md#framed-data)
    #[default]
    Framed = 2,
}

pub(crate) fn write_content_type(w: &mut dyn SafeWrite, data: ContentType) -> Result<(), Error> {
    //= specification/data-format/message-header.md#content-type
    //# The length of the serialized content type field MUST be 1 byte.
    write_u8(w, data as u8)
}

pub(crate) fn read_content_type(
    r: &mut dyn SafeRead,
    raw: &mut dyn SafeWrite,
) -> Result<ContentType, Error> {
    //= specification/data-format/message-header.md#content-type
    //# The length of the serialized content type field MUST be 1 byte.
    let content_type = read_u8(r, raw)?;
    match content_type {
        val if val == ContentType::NonFramed as u8 => Ok(ContentType::NonFramed),
        val if val == ContentType::Framed as u8 => Ok(ContentType::Framed),
        //= specification/client-apis/decrypt.md#v2-header-deserialization
        //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
        //= specification/client-apis/decrypt.md#v1-header-deserialization
        //# The value MUST be a [supported content type](../data-format/message-header.md#supported-content-types).
        _ => ser_err("Unsupported Content Type."),
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct V1HeaderBody {
    pub(crate) message_type: MessageType,
    pub(crate) algorithm_suite: AlgorithmSuite,
    pub(crate) message_id: MessageId,
    pub(crate) encryption_context: ESDKCanonicalEncryptionContext,
    pub(crate) encrypted_data_keys: Vec<EncryptedDataKey>,
    pub(crate) content_type: ContentType,
    pub(crate) header_iv_length: u64,
    pub(crate) frame_length: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
pub(crate) enum HeaderBody {
    V1Body(V1HeaderBody),
    V2Body(V2HeaderBody),
}
impl Default for HeaderBody {
    fn default() -> Self {
        Self::V2Body(V2HeaderBody::default())
    }
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
    pub(crate) const fn message_id(&self) -> &MessageId {
        match self {
            Self::V1Body(body) => &body.message_id,
            Self::V2Body(body) => &body.message_id,
        }
    }
    pub(crate) const fn encryption_context(&self) -> &ESDKCanonicalEncryptionContext {
        match self {
            Self::V1Body(body) => &body.encryption_context,
            Self::V2Body(body) => &body.encryption_context,
        }
    }
    pub(crate) fn encrypted_data_keys(&self) -> &[EncryptedDataKey] {
        match self {
            Self::V1Body(body) => &body.encrypted_data_keys,
            Self::V2Body(body) => &body.encrypted_data_keys,
        }
    }
    pub(crate) const fn algorithm_suite(&self) -> &AlgorithmSuite {
        match self {
            Self::V1Body(body) => &body.algorithm_suite,
            Self::V2Body(body) => &body.algorithm_suite,
        }
    }
    pub(crate) fn suite_data(&self) -> &[u8] {
        match self {
            Self::V1Body(_) => &[],
            Self::V2Body(body) => &body.suite_data,
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

impl Default for HeaderAuth {
    fn default() -> Self {
        // This is a dummy value. It should never be used.
        Self::AESMac {
            header_iv: vec![0u8; 12],
            header_auth_tag: vec![0u8; 16],
        }
    }
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

pub(crate) const MESSAGE_ID_LEN_V1: u32 = 16;
pub(crate) const MESSAGE_ID_LEN_V2: u32 = 32;

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Message body AAD (additional authenticated data) construction.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum BodyAADContent {
    RegularFrame,
    FinalFrame,
    SingleBlock,
}

//= specification/data-format/message-body-aad.md#body-aad-content
//# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data)
//# MUST use the value `AWSKMSEncryptionClient Frame`.
const BODY_AAD_CONTENT_REGULAR_FRAME: &str = "AWSKMSEncryptionClient Frame";
//= specification/data-format/message-body-aad.md#body-aad-content
//# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data)
//# MUST use the value `AWSKMSEncryptionClient Final Frame`.
const BODY_AAD_CONTENT_FINAL_FRAME: &str = "AWSKMSEncryptionClient Final Frame";
//= specification/data-format/message-body-aad.md#body-aad-content
//# - [nonframed data](message-body.md#nonframed-data)
//# MUST use the value `AWSKMSEncryptionClient Single Block`.
const BODY_AAD_CONTENT_SINGLE_BLOCK: &str = "AWSKMSEncryptionClient Single Block";

const fn body_aad_content_type_string(bc: BodyAADContent) -> &'static str {
    match bc {
        BodyAADContent::RegularFrame => BODY_AAD_CONTENT_REGULAR_FRAME,
        BodyAADContent::FinalFrame => BODY_AAD_CONTENT_FINAL_FRAME,
        BodyAADContent::SingleBlock => BODY_AAD_CONTENT_SINGLE_BLOCK,
    }
}

pub(crate) fn iv_seq(sequence_number: u32, result: &mut [u8]) {
    let pivot = result.len() - 4;
    result[pivot..].copy_from_slice(&sequence_number.to_be_bytes());
}

// Serializes the Message Body AAD

pub(crate) fn body_aad(
    message_id: &[u8],
    bc: BodyAADContent,
    //= specification/data-format/message-body-aad.md#sequence-number
    //= reason=sequence_number parameter is u32, serialized via to_be_bytes()
    //# The sequence number field MUST be interpreted as a UInt32.
    sequence_number: u32,
    //= specification/data-format/message-body-aad.md#content-length
    //= reason=length parameter is u64, serialized via to_be_bytes()
    //# The content length field MUST be interpreted as a UInt64.
    length: u64,
    result: &mut Vec<u8>,
) {
    result.clear();
    //= specification/data-format/message-body-aad.md#structure
    //# The message body AAD MUST consist of, in order,
    //# Message ID,
    //# Body AAD Content,
    //# Sequence Number,
    //# and Content Length.
    // Message ID
    //= specification/data-format/message-body-aad.md#message-id
    //= reason=V1 message IDs are 16 bytes, V2 message IDs are 32 bytes; the debug_assert enforces this
    //# The length of the Message ID field MUST be equal to the length of the [Message ID](message-header.md#message-id) defined by the message header version.
    debug_assert!(
        message_id.len() == 16 || message_id.len() == 32,
        "message ID must be 16 or 32 bytes, got {}",
        message_id.len()
    );
    //= specification/data-format/message-body-aad.md#message-id
    //# This MUST be the [message ID](message-header.md#message-id) stored in the header of the message.
    result.extend_from_slice(message_id);

    // Body AAD Content
    result.extend_from_slice(body_aad_content_type_string(bc).as_bytes());

    // Sequence Number
    //= specification/data-format/message-body-aad.md#sequence-number
    //= reason=u32::to_be_bytes() produces exactly 4 bytes
    //# The length of the sequence number field MUST be 4 bytes.
    let seq_bytes = sequence_number.to_be_bytes();
    debug_assert_eq!(seq_bytes.len(), 4, "sequence number field must be exactly 4 bytes");
    result.extend_from_slice(&seq_bytes);

    // Content Length
    //= specification/data-format/message-body-aad.md#content-length
    //= reason=u64::to_be_bytes() produces exactly 8 bytes
    //# The length of the content length field MUST be 8 bytes.
    let len_bytes = length.to_be_bytes();
    debug_assert_eq!(len_bytes.len(), 8, "content length field must be exactly 8 bytes");
    result.extend_from_slice(&len_bytes);
}

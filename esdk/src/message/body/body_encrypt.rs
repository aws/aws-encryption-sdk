// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Frame encryption and body serialization.

use super::{BodyAADContent, body_aad, get_encrypt, iv_seq};
use crate::error::val_err;
use crate::message::header::{ENDFRAME_SEQUENCE_NUMBER, HeaderInfo, START_SEQUENCE_NUMBER};
use crate::message::serializable_types::{get_iv_length, get_tag_length};
use crate::message::serialize_functions::{read_opt_u8, read_up_to_peek, write_bytes, write_u32};
use crate::message::{DigestWriter, Error, ser_err};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::primitives::{AesGcm, aes_encrypt};

const MAX_DATA: usize = (1usize << 36) - 32;

/// Input for constructing a single frame (regular or final).
pub(crate) struct ConstructFrameInput<'a> {
    pub(crate) alg: AesGcm,
    pub(crate) key: &'a [u8],
    pub(crate) plaintext: &'a [u8],
    pub(crate) message_id: &'a [u8],
    pub(crate) aad_content: BodyAADContent,
    pub(crate) sequence_number: u32,
    pub(crate) is_final: bool,
}

/// Construct and serialize a single frame (regular or final).
pub(crate) fn construct_frame(
    input: &ConstructFrameInput<'_>,
    iv: &mut [u8],
    aad: &mut Vec<u8>,
    frame_buf: &mut Vec<u8>,
    ciphertext: &mut dyn SafeWrite,
    sig_digest: &mut DigestWriter,
) -> Result<(), Error> {
    frame_buf.clear();

    // AAD

    //= spec/client-apis/encrypt.md#construct-a-frame
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed according to the [Message Body AAD](../data-format/message-body-aad.md) specification, as follows:
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
    //# equal to the length of the plaintext being encrypted.
    //
    //= spec/data-format/message-body-aad.md#content-length
    //= reason=plaintext.len() is the length of the plaintext being encrypted in this frame
    //# - For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes,
    //# of the plaintext being encrypted in this frame.
    let Ok(plaintext_len_u64) = u64::try_from(input.plaintext.len()) else {
        return ser_err(&format!(
            "Plaintext length {} exceeds u64::MAX",
            input.plaintext.len()
        ));
    };
    body_aad(
        //= spec/client-apis/encrypt.md#construct-a-frame
        //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
        //# [message ID](../data-format/message-header.md#message-id) serialized in the header of this message.
        //
        //= spec/data-format/message-header.md#message-id
        //= reason=input.message_id is the message ID from the header, interpreted as raw bytes for AAD construction
        //# The message ID MUST be interpreted as bytes.
        input.message_id,
        //= spec/client-apis/encrypt.md#construct-a-frame
        //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be the structure defined in
        //# [Message Body AAD](../data-format/message-body-aad.md).
        input.aad_content,
        //= spec/client-apis/encrypt.md#construct-a-frame
        //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
        //# number of the frame being encrypted.
        //
        //= spec/data-format/message-body-aad.md#sequence-number
        //= reason=The sequence_number parameter is the frame sequence number passed from encrypt_and_serialize_body
        //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
        input.sequence_number,
        plaintext_len_u64,
        aad,
    );

    // IV (computed; serialized below)

    //= spec/client-apis/encrypt.md#construct-a-frame
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD for this frame,
    //# padded to the [IV length](../data-format/message-header.md#iv-length).
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# The Encrypt operation MUST serialize a regular frame or final frame with the following specifics:
    //
    //= spec/data-format/message-body.md#regular-frame-iv
    //# Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
    //
    //= spec/data-format/message-body.md#final-frame-iv
    //# A generated IV MUST be a unique IV within the message.
    iv_seq(input.sequence_number, iv);

    //= spec/data-format/message-body.md#final-frame
    //= reason=The two `if input.is_final` blocks below add Sequence Number End and Encrypted Content Length to the shared frame serialization path
    //# A final frame MUST only differ from a regular frame by the addition of the
    //# Sequence Number End
    //# and Encrypted Content Length.
    if input.is_final {
        // Sequence Number End

        //= spec/client-apis/encrypt.md#construct-a-frame
        //= reason=The following lines serialize SeqNumEnd, SeqNum, IV, EncContentLen, EncContent, and AuthTag in order per the Final Frame spec
        //# For a final frame, each field MUST be serialized according to its specification:
        //
        //= spec/data-format/message-body.md#final-frame
        //# A final frame MUST consist of, in order,
        //# Sequence Number End,
        //# Sequence Number,
        //# IV,
        //# Encrypted Content Length,
        //# Encrypted Content,
        //# and Authentication Tag.
        //
        //= spec/client-apis/encrypt.md#construct-a-frame
        //# - MUST serialize the [Sequence Number End](../data-format/message-body.md#sequence-number-end).
        //
        //= spec/data-format/message-body.md#sequence-number-end
        //# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
        write_u32(frame_buf, ENDFRAME_SEQUENCE_NUMBER)?;
    }

    // Sequence Number

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=The following lines serialize SeqNum, IV, EncContent, and AuthTag in order per the Regular Frame spec
    //# For a regular frame, each field MUST be serialized according to its specification:
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# - MUST serialize the [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number).
    //# The value MUST be the sequence number of this frame.
    //
    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //# The sequence number MUST be interpreted as a UInt32.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=write_u32 above serializes the sequence number for both regular and final frames in this shared code path
    //# - MUST serialize the [Sequence Number](../data-format/message-body.md#final-frame-sequence-number).
    //
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= reason=write_u32 serializes the sequence number as a UInt32, same type as the regular frame sequence number
    //# The Final Frame Sequence Number MUST be interpreted as the same type as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    //
    //= spec/data-format/message-body.md#final-frame-sequence-number
    //= reason=write_u32 writes exactly 4 bytes, the same length as the regular frame sequence number field
    //# The length of the Final Frame Sequence number field MUST be the same as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the sequence number of this frame.
    write_u32(frame_buf, input.sequence_number)?;

    // IV

    //= spec/client-apis/encrypt.md#construct-a-frame
    //# - MUST serialize the [IV](../data-format/message-body.md#regular-frame-iv).
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the IV used when calculating the encrypted content for this frame.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=write_bytes above serializes the IV for both regular and final frames in this shared code path
    //# - MUST serialize the [IV](../data-format/message-body.md#final-frame-iv).
    //
    //= spec/data-format/message-body.md#final-frame-iv
    //= reason=iv is &[u8], interpreted as raw bytes
    //# The IV MUST be interpreted as bytes.
    write_bytes(frame_buf, iv)?;

    if input.is_final {
        // Encrypted Content Length

        //= spec/client-apis/encrypt.md#construct-a-frame
        //# - MUST serialize the [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length).
        //
        //= spec/data-format/message-body.md#final-frame-encrypted-content-length
        //# The encrypted content length MUST be a UInt32.
        let Ok(enc_content_len) = u32::try_from(input.plaintext.len()) else {
            return ser_err("Plaintext length exceeds u32");
        };

        //= spec/data-format/message-body.md#final-frame-encrypted-content-length
        //# The length of the serialized encrypted content length field MUST be 4 bytes.
        //
        //= spec/data-format/message-header.md#frame-length
        //= reason=enc_content_len is bounded by the frame length from the header, interpreted as a UInt32
        //# The frame length MUST be interpreted as a UInt32.
        write_u32(frame_buf, enc_content_len)?;
    }

    // Encrypted Content + Authentication Tag

    //= spec/client-apis/encrypt.md#construct-a-frame
    //# To construct a regular or final frame that represents the next frame in the encrypted message's body,
    //# the Encrypt operation MUST calculate the encrypted content and an authentication tag using the
    //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md),
    //# with the following inputs:
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# - MUST serialize the [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content).
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=aes_encrypt writes encrypted content to frame_buf for both regular and final frames in this shared code path
    //# - MUST serialize the [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content).
    //
    //= spec/data-format/message-body.md#final-frame-encrypted-content
    //= reason=aes_encrypt output bytes are written directly to frame_buf, interpreted as raw bytes
    //# The encrypted content MUST be interpreted as bytes.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the encrypted content calculated for this frame.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# - MUST serialize the [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag).
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=aes_encrypt writes the authentication tag to frame_buf for both regular and final frames in this shared code path
    //# - MUST serialize the [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag).
    //
    //= spec/data-format/message-body.md#final-frame-authentication-tag
    //= reason=aes_encrypt output tag bytes are written directly to frame_buf, interpreted as raw bytes
    //# The authentication tag MUST be interpreted as bytes.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the authentication tag output when calculating the encrypted content for this frame.
    aes_encrypt(
        input.alg,
        iv,
        //= spec/client-apis/encrypt.md#construct-a-frame
        //= reason=input.key is set from the derived data key in encrypt_and_serialize_body
        //# - The cipherkey MUST be the derived data key
        input.key,
        //= spec/client-apis/encrypt.md#construct-a-frame
        //# - The plaintext MUST be the next subsequence of consumable plaintext bytes that have not yet been encrypted.
        input.plaintext,
        aad,
        frame_buf,
    )?;

    // Frame release

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=Frame is fully serialized into frame_buf before being written to ciphertext; frame_buf.clear() at function start and write_bytes(ciphertext, frame_buf) here ensure atomic release
    //# The serialized frame bytes MUST NOT be released until the entire frame has been serialized.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# If the Encrypt operation is streaming the encrypted message and
    //# the entire frame has been serialized,
    //# the serialized frame MUST be released.
    write_bytes(ciphertext, frame_buf)?;

    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=DigestWriter feeds the serialized frame bytes to the signature algorithm
    //# If the algorithm suite contains a signature algorithm and
    //# the Encrypt operation is [streaming](streaming.md) the encrypted message output to the caller,
    //# the Encrypt operation MUST input the serialized frame to the signature algorithm as soon as it is serialized,
    //# such that the serialized frame isn't required to remain in memory to [construct the signature](#construct-the-signature).
    write_bytes(sig_digest, frame_buf)?;
    Ok(())
}

/// Encrypt plaintext and serialize the message body (framed) to the output stream.
pub(crate) fn encrypt_and_serialize_body(
    plaintext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    ciphertext: &mut dyn SafeWrite,
    sig_digest: &mut DigestWriter,
    max_plaintext_length: Option<usize>,
) -> Result<(), Error> {
    let mut total_data_size: usize = 0;
    let Ok(frame_length) = usize::try_from(header.body.frame_length()) else {
        return ser_err(&format!(
            "Frame length {} exceeds usize::MAX",
            header.body.frame_length()
        ));
    };
    let iv_len = usize::from(get_iv_length(&header.suite));
    let auth_len = usize::from(get_tag_length(&header.suite));
    let frame_len = frame_length + iv_len + auth_len + 4;
    let mut frame_buf = Vec::with_capacity(frame_len);

    //= spec/data-format/message-body.md#regular-frame-sequence-number
    //= reason=START_SEQUENCE_NUMBER is defined as 1
    //# Framed Data MUST start at Sequence Number 1.
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //= reason=START_SEQUENCE_NUMBER is defined as 1
    //# If this is the first frame sequentially, the sequence number value MUST be 1.
    let mut sequence_number = START_SEQUENCE_NUMBER;
    let alg = get_encrypt(&header.suite)?;

    let mut iv = vec![0; iv_len];
    let mut plaintext_frame = vec![0; frame_length];
    let mut aad = Vec::new();
    let mut in_size: usize;
    let mut next_char: Option<u8> = None;

    //= spec/client-apis/encrypt.md#construct-the-body
    //# Before the end of the input is indicated,
    //# this operation MUST process as much of the consumable bytes as possible
    //# by [constructing regular frames](#construct-a-frame).
    loop {
        in_size = read_up_to_peek(plaintext, &mut plaintext_frame, next_char)?;

        //= spec/client-apis/encrypt.md#construct-the-body
        //# - If there are not enough input consumable plaintext bytes to create a new regular frame,
        //# then this operation MUST [construct a final frame](#construct-a-frame)
        //
        //= spec/data-format/message-body.md#final-frame
        //= reason=When in_size < frame_length on the first iteration, the loop breaks immediately to construct a single final frame
        //# - When the length of the Plaintext is less than the Frame Length,
        //# the body MUST contain exactly one frame and that frame MUST be a Final Frame.
        if in_size != frame_length {
            break;
        }
        next_char = read_opt_u8(plaintext)?;

        //= spec/client-apis/encrypt.md#construct-the-body
        //# - If there are exactly enough consumable plaintext bytes to create one regular frame,
        //# such that creating a regular frame processes all consumable bytes,
        //# then this operation MUST [construct either a final frame or regular frame](#construct-a-frame)
        //# with the remaining plaintext.
        //
        //= spec/data-format/message-body.md#final-frame
        //# - When the length of the Plaintext is an exact multiple of the Frame Length
        //# (including if it is equal to the frame length),
        //# the Final Frame encrypted content length SHOULD be equal to the frame length but MAY be 0.
        if next_char.is_none() {
            break;
        }

        //= spec/client-apis/encrypt.md#construct-the-body
        //# - If there are enough input plaintext bytes consumable to create a new regular frame,
        //# such that creating a regular frame does not processes all consumable bytes,
        //# then this operation MUST [construct a regular frame](#construct-a-frame)
        //# using the consumable plaintext bytes.
        if sequence_number == ENDFRAME_SEQUENCE_NUMBER {
            //= spec/data-format/message-body.md#framed-data
            //# - The number of frames in a single message MUST be less than or equal to `2^32 - 1`.
            return Err(val_err("Too many frames"));
        }

        total_data_size += frame_length;
        if total_data_size > MAX_DATA {
            return Err(val_err("Plaintext too large"));
        }
        if let Some(max_plaintext_len) = max_plaintext_length {
            if total_data_size > max_plaintext_len {
                //= spec/client-apis/encrypt.md#plaintext-length-bound
                //# If this input is provided, this operation MUST NOT encrypt a plaintext with length
                //# greater than this value.
                //
                //= spec/client-apis/encrypt.md#construct-the-body
                //# If [Plaintext Length Bound](#plaintext-length-bound) was specified on input
                //# and this operation determines at any time that the plaintext being encrypted
                //# has a length greater than this value,
                //# this operation MUST immediately fail.
                return Err(val_err(
                    "Plaintext length exceeds specified Plaintext Length Bound",
                ));
            }
        }

        //= spec/client-apis/encrypt.md#construct-a-frame
        //# Regular frame serialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
        //
        //= spec/data-format/message-body.md#regular-frame
        //# A regular frame MUST consist of, in order,
        //# Sequence Number,
        //# IV,
        //# Encrypted Content,
        //# and Authentication Tag.
        construct_frame(
            &ConstructFrameInput {
                alg,
                key,
                //= spec/client-apis/encrypt.md#construct-a-frame
                //= reason=plaintext_frame is exactly frame_length bytes (the full buffer read from input)
                //# - For a regular frame the length of this plaintext MUST equal the frame length.
                //
                //= spec/client-apis/encrypt.md#construct-a-frame
                //= reason=plaintext_frame is exactly frame_length bytes for a regular frame
                //# - For a regular frame the length of this plaintext subsequence MUST equal the frame length.
                //
                //= spec/data-format/message-body.md#regular-frame-encrypted-content
                //# The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
                plaintext: &plaintext_frame,
                message_id: header.body.message_id(),
                aad_content: BodyAADContent::RegularFrame,
                sequence_number,
                is_final: false,
            },
            &mut iv,
            &mut aad,
            &mut frame_buf,
            ciphertext,
            sig_digest,
        )?;

        //= spec/data-format/message-body.md#regular-frame-sequence-number
        //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
        //
        //= spec/client-apis/encrypt.md#construct-a-frame
        //# Otherwise, the sequence number value MUST be 1 greater than the value of the sequence number
        //# of the previous frame.
        sequence_number += 1;
    }

    //= spec/client-apis/encrypt.md#construct-the-body
    //# When the end of the input is indicated,
    //# this operation MUST perform the following until all consumable plaintext bytes are processed:
    // Final frame
    total_data_size += in_size;
    if total_data_size > MAX_DATA {
        return Err(val_err("Plaintext too large"));
    }
    if let Some(max_len) = max_plaintext_length {
        if total_data_size > max_len {
            return Err(val_err(
                "Plaintext length exceeds specified Plaintext Length Bound",
            ));
        }
    }

    //= spec/data-format/message-body.md#final-frame
    //= reason=in_size is the plaintext length for the final frame; this asserts it is <= frame_length
    //# The length of the plaintext to be encrypted in the Final Frame MUST be
    //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).
    debug_assert!(in_size <= frame_length);
    debug_assert!(
        in_size > 0 || sequence_number == START_SEQUENCE_NUMBER,
        "empty final frame only allowed when entire plaintext is empty"
    );

    //= spec/client-apis/encrypt.md#construct-the-body
    //# If an end to the input has been indicated, there are no more consumable plaintext bytes to process,
    //# and a final frame has not yet been constructed,
    //# this operation MUST [construct an empty final frame](#construct-a-frame).
    //
    //= spec/client-apis/encrypt.md#construct-a-frame
    //# Final frame serialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    construct_frame(
        &ConstructFrameInput {
            alg,
            key,
            //= spec/client-apis/encrypt.md#construct-a-frame
            //= reason=plaintext_frame[0..in_size] is the remaining unencrypted bytes, where in_size <= frame_length
            //# - For a final frame this MUST be the length of the remaining plaintext bytes
            //# which have not yet been encrypted,
            //# whose length MUST be equal to or less than the frame length.
            //
            //= spec/client-apis/encrypt.md#construct-a-frame
            //= reason=plaintext_frame[0..in_size] is the remaining unencrypted bytes, where in_size <= frame_length
            //# - For a final frame this MUST be the remaining plaintext bytes which have not yet been encrypted,
            //# whose length MUST be equal to or less than the frame length.
            plaintext: &plaintext_frame[0..in_size],
            message_id: header.body.message_id(),
            aad_content: BodyAADContent::FinalFrame,
            //= spec/data-format/message-body.md#final-frame-sequence-number
            //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
            sequence_number,
            is_final: true,
        },
        &mut iv,
        &mut aad,
        &mut frame_buf,
        ciphertext,
        sig_digest,
    )?;

    Ok(())
}

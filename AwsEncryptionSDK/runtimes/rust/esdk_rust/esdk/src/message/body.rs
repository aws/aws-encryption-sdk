// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header::{ENDFRAME_SEQUENCE_NUMBER, HeaderInfo, START_SEQUENCE_NUMBER};
use super::serializable_types::*;
use super::serialize_functions::{read_bytes, read_seq_u32_bounded, read_u32, write_u32, write_bytes, read_up_to_peek, read_opt_u8};
use super::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::primitives::{AesGcm, aes_decrypt, aes_encrypt};
use aws_mpl_legacy::suites::AlgorithmSuite;

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
//# - [Non-framed data](message-body.md#non-framed-data)
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

pub(crate) fn get_encrypt(info: &AlgorithmSuite) -> AesGcm {
    match &info.encrypt {
        aws_mpl_legacy::suites::Encrypt::AesGcm(aes_gcm) => *aes_gcm,
        _ => panic!("not an aes gcm"),
    }
}

/*
 * Serializes the Message Body AAD
 */

pub(crate) fn body_aad(
    message_id: &[u8],
    bc: BodyAADContent,
    sequence_number: u32,
    length: u64,
    result: &mut Vec<u8>,
) {
    // Callers are responsible for passing the correct sequence_number and length
    // per the body-aad spec. See call sites for the annotations that document
    // which values satisfy which requirements.
    result.clear();
    result.extend_from_slice(message_id);
    result.extend_from_slice(body_aad_content_type_string(bc).as_bytes());
    result.extend_from_slice(&sequence_number.to_be_bytes());
    result.extend_from_slice(&length.to_be_bytes());
}

pub(crate) fn read_and_decrypt_framed_message_body(
    r: &mut dyn SafeRead,
    w: &mut dyn SafeWrite,
    header: &HeaderInfo,
    key: &[u8],
    raw: &mut dyn SafeWrite,
    fail_if_multi_frame: bool,
) -> Result<Vec<u8>, Error> {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=implementation
    //# Regular frame deserialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= type=implementation
    //# Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# If this is framed data and the first frame sequentially, this value MUST be 1.
    let mut expected_frame: u32 = START_SEQUENCE_NUMBER;
    //= specification/data-format/message-body.md#regular-frame-iv
    //= type=implication
    //# The IV length MUST be equal to the IV length of the algorithm suite specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let mut iv = vec![0u8; get_iv_length(&header.suite) as usize];
    //= specification/data-format/message-body.md#regular-frame-authentication-tag
    //= type=implication
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let mut auth_tag = vec![0u8; get_tag_length(&header.suite) as usize];
    let alg = get_encrypt(&header.suite);
    let frame_length_u64 = u64::from(header.body.frame_length());
    let frame_length_usize = header.body.frame_length() as usize;
    let mut enc_content = vec![0u8; frame_length_usize];
    let mut result = vec![0; frame_length_usize];
    let mut aad = Vec::new();

    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# If deserializing [framed data](../data-format/message-body.md#framed-data),
    //# the Decrypt operation MUST use the first 4 bytes of a frame to determine
    //# whether the operation will deserialize the frame as a [final frame](../data-format/message-body.md#final-frame)
    //# or [regular frame](../data-format/message-body.md#regular-frame).
    loop {
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //= type=implication
        //# When serializing the sequence number to a message, the length of the serialized sequence number MUST be 4 bytes.
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //= type=implication
        //# When reading the sequence number from a message, the sequence number MUST be interpreted as a UInt32.
        //= specification/data-format/message-body.md#sequence-number-end
        //= type=implication
        //# The length of the serialized sequence number end MUST be 4 bytes.
        //= specification/data-format/message-body.md#sequence-number-end
        //= type=implication
        //# The sequence number end MUST be interpreted as bytes.
        let seq_num = read_u32(r, raw)?;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# If the first 4 bytes have a value of 0xFFFF,
        //# then the Decrypt operation MUST deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
        if seq_num == ENDFRAME_SEQUENCE_NUMBER {
            //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame-sequence-number
            //= type=implication
            //= reason=read_u32 is used for both regular and final frame sequence numbers
            //# The Final Frame Sequence Number MUST be interpreted from a message the same way as the
            //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
            let seq_num: u32 = read_u32(r, raw)?;
            if seq_num != expected_frame {
                return Err("Final sequence number out of order.".into());
            }
            //= specification/data-format/message-body.md#final-frame-iv
            //= type=implication
            //# The IV length MUST be equal to the IV length of the [algorithm suite](../framework/algorithm-suites.md) that generated the message.
            //= specification/data-format/message-body.md#final-frame-iv
            //= type=implication
            //# The IV MUST be interpreted as bytes.
            read_bytes(r, &mut iv, raw)?;
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# If deserializing a [final frame](../data-format/message-body.md#final-frame),
            //# the Decrypt operation MUST ensure that the length of the encrypted content field is
            //# less than or equal to the frame length deserialized in the message header.
            //= specification/data-format/message-body.md#final-frame-encrypted-content-length
            //= type=implication
            //# When serializing the encrypted content length to a message, the length of the serialized encrypted content length field MUST be 4 bytes.
            //= specification/data-format/message-body.md#final-frame-encrypted-content-length
            //= type=implication
            //# When reading the encrypted content length from a message, the encrypted content length MUST be interpreted as a UInt32.
            //= specification/data-format/message-body.md#final-frame-encrypted-content
            //# The length of the serialized encrypted content MUST be equal to the value of the [Encrypted Content Length](#encrypted-content-length-1) field.
            //= specification/data-format/message-body.md#final-frame
            //= reason=read_seq_u32_bounded enforces encrypted content length <= frame_length, which bounds the plaintext length
            //# The length of the plaintext to be encrypted in the Final Frame MUST be
            //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).
            read_seq_u32_bounded(
                r,
                header.body.frame_length(),
                "Content length MUST NOT exceed the frame length.",
                &mut enc_content,
                raw,
            )?;
            //= specification/data-format/message-body.md#final-frame-authentication-tag
            //= type=implication
            //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
            //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
            //= specification/data-format/message-body.md#final-frame-authentication-tag
            //= type=implication
            //# The authentication tag MUST be interpreted as bytes.
            read_bytes(r, &mut auth_tag, raw)?;
            body_aad(
                header.body.message_id(),
                BodyAADContent::FinalFrame,
                seq_num,
                enc_content.len() as u64,
                &mut aad,
            );
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# If this decryption fails, this operation MUST immediately halt and fail.
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# Once at least a single frame is deserialized (or the entire body in the un-framed case),
            //# the Decrypt operation MUST decrypt and authenticate the frame (or body) using the
            //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
            //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
            if enc_content.is_empty() {
                // final frame is empty, so return last full frame
                let mut empty_result = Vec::new();
                aes_decrypt(
                    alg,
                    key,
                    &enc_content,
                    &auth_tag,
                    &iv,
                    &aad,
                    &mut empty_result[..],
                )?;
            } else {
                //= specification/client-apis/decrypt.md#decrypt-the-message-body
                //# Otherwise, the Decrypt operation MUST deserialize the bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
                // write previous frame's data, now that we know we have another frame.
                if expected_frame != START_SEQUENCE_NUMBER {
                    if fail_if_multi_frame {
                        return Err("Streaming Interface can return data before signature has been validated. Set `allow_unsafe_unverified_signature` in the DecryptStreamInput struct if this is ok.".into());
                    }
                    write_bytes(w, &result)?;
                }
                aes_decrypt(
                    alg,
                    key,
                    &enc_content,
                    &auth_tag,
                    &iv,
                    &aad,
                    &mut result[0..enc_content.len()],
                )?;
                result.resize(enc_content.len(), 0);
            }
            //= specification/data-format/message-body.md#final-frame
            //= type=implementation
            //# Framed data MUST contain exactly one final frame.
            //= specification/data-format/message-body.md#final-frame
            //= type=implementation
            //# The final frame MUST be the last frame.
            return Ok(result);
        }
        if seq_num != expected_frame {
            return Err("Sequence number out of order.".into());
        }
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# - If the streamed Decrypt operation is using an algorithm suite with a signature algorithm,
        //# all plaintext decrypted from regular frames SHOULD be released as soon as the above calculation,
        //# including tag verification, succeeds.
        // write previous frame's data, now that we know we have another frame.
        if expected_frame != START_SEQUENCE_NUMBER {
            if fail_if_multi_frame {
                return Err("Streaming Interface can return data before signature has been validated. Set `allow_unsafe_unverified_signature` in the DecryptStreamInput struct if this is ok.".into());
            }
            write_bytes(w, &result)?;
        }
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# Otherwise, this value MUST be 1 greater than the value of the sequence number
        //# of the previous frame.
        expected_frame += 1;
        //= specification/data-format/message-body.md#regular-frame-iv
        //= type=implication
        //# The IV MUST be interpreted as bytes.
        read_bytes(r, &mut iv, raw)?;
        //= specification/data-format/message-body.md#regular-frame-encrypted-content
        //= type=implication
        //# The encrypted content MUST be interpreted as bytes.
        read_bytes(r, &mut enc_content, raw)?;
        //= specification/data-format/message-body.md#regular-frame-authentication-tag
        //= type=implication
        //# The authentication tag MUST be interpreted as bytes.
        read_bytes(r, &mut auth_tag, raw)?;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
        //# equal to the length of the plaintext that was encrypted.
        body_aad(
            header.body.message_id(),
            BodyAADContent::RegularFrame,
            seq_num,
            frame_length_u64,
            &mut aad,
        );
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# Once at least a single frame is deserialized (or the entire body in the un-framed case),
        //# the Decrypt operation MUST decrypt and authenticate the frame (or body) using the
        //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
        //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# If this decryption fails, this operation MUST immediately halt and fail.
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# This operation MUST NOT release any unauthenticated plaintext.
        aes_decrypt(alg, key, &enc_content, &auth_tag, &iv, &aad, &mut result)?;
    }
}

#[allow(clippy::no_effect_underscore_binding)]
pub(crate) fn read_and_decrypt_non_framed_message_body(
    r: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    // Non-framed write-path requirements: ESDK only encrypts framed data
    //= specification/data-format/message-body.md#non-framed-data
    //= type=exception
    //= reason=The ESDK only encrypts framed data per encrypt.md; non-framed serialization is not supported
    //# Non-framed data MUST be serialized (written) as, in order,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.

    //= specification/data-format/message-body.md#non-framed-data-iv
    //= type=exception
    //= reason=The ESDK only encrypts framed data; non-framed write path is not implemented
    //# When writing a message, the IV MUST be a unique IV within the message.

    //= specification/data-format/message-body.md#non-framed-data-iv
    //= type=exception
    //= reason=The ESDK only encrypts framed data; non-framed write path is not implemented
    //# When writing a message, the operation MUST serialize the IV to be [IV Length](message-header.md#iv-length) bytes.

    //= specification/data-format/message-body.md#non-framed-data-encrypted-content-length
    //= type=exception
    //= reason=The ESDK only encrypts framed data; non-framed write path is not implemented
    //# When serializing the encrypted content length to a message, the length of the serialized encrypted content length MUST be 8 bytes.

    //= specification/data-format/message-body.md#non-framed-data-encrypted-content-length
    //= type=exception
    //= reason=The ESDK only encrypts framed data; non-framed write path is not implemented
    //# The encrypted content length MUST be serialized as a Uint64.

    //= specification/data-format/message-header.md#frame-length
    //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
    if header.body.frame_length() != 0 {
        return Err("Non-framed message contains non-zero frame length.".into());
    }

    //= specification/data-format/message-body.md#non-framed-data
    //= type=implication
    //= reason=The fields are read in order: IV, then content length + content (via read_seq_u64_bounded), then auth tag
    //# Non-framed data MUST be deserialized (read) as, in order,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.

    //= specification/data-format/message-body.md#non-framed-data-iv
    //= type=implication
    //= reason=read_vec reads exactly get_iv_length bytes and returns them as the IV
    //# When reading a message, the operation MUST deserialize [IV Length](message-header.md#iv-length) bytes and interpret it as the IV.
    //= specification/data-format/message-body.md#non-framed-data-iv
    //= type=implication
    //= reason=read_vec returns Vec<u8>
    //# When reading a message, the deserialized IV MUST be interpreted as bytes.
    let iv = serialize_functions::read_vec(r, get_iv_length(&header.suite) as usize, raw)?;

    //= specification/data-format/message-body.md#non-framed-data-encrypted-content-length
    //= type=implication
    //= reason=SAFE_MAX_ENCRYPT equals 2^36 - 32; read_seq_u64_bounded rejects values above this limit
    //# The length MUST NOT be greater than `2^36 - 32`, or 64 gibibytes (64 GiB),
    //# due to restrictions imposed by the [implemented algorithms](../framework/algorithm-suites.md).
    //= specification/data-format/message-body.md#non-framed-data-encrypted-content-length
    //= type=implication
    //= reason=read_seq_u64_bounded reads 8 bytes as a u64
    //# When reading the encrypted content length from a message, the encrypted content length MUST be interpreted as a Uint64.
    let enc_content = serialize_functions::read_seq_u64_bounded(
        r,
        header::SAFE_MAX_ENCRYPT,
        "Frame exceeds AES-GCM cryptographic safety for a single key/iv.",
        raw,
    )?;
    //= specification/data-format/message-body.md#non-framed-data-encrypted-content
    //= type=implication
    //= reason=read_seq_u64_bounded reads exactly content_length bytes
    //# The length of the serialized encrypted content MUST be equal to the value of the [Encrypted Content Length](#encrypted-content-length) field.
    //= specification/data-format/message-body.md#non-framed-data-encrypted-content
    //= type=implication
    //= reason=read_seq_u64_bounded returns Vec<u8>
    //# The encrypted content MUST be interpreted as bytes.
    let _enc_content_read = &enc_content;

    //= specification/data-format/message-body.md#non-framed-data-authentication-tag
    //= type=implication
    //= reason=read_vec reads exactly get_tag_length bytes, matching the algorithm suite's authentication tag length
    //# The length of the serialized authentication tag MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    //= specification/data-format/message-body.md#non-framed-data-authentication-tag
    //= type=implication
    //= reason=read_vec returns Vec<u8>
    //# The authentication tag MUST be interpreted as bytes.
    let auth_tag = serialize_functions::read_vec(r, get_tag_length(&header.suite) as usize, raw)?;
    let mut aad = Vec::new();
    body_aad(
        header.body.message_id(),
        //= specification/data-format/message-body-aad.md#body-aad-content
        //# - [Non-framed data](message-body.md#non-framed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
        BodyAADContent::SingleBlock,
        //= specification/data-format/message-body-aad.md#sequence-number
        //# For [non-framed data](message-body.md#non-framed-data), the value of this field MUST be `1`.
        header::NONFRAMED_SEQUENCE_NUMBER,
        //= specification/data-format/message-body-aad.md#content-length
        //# - For [non-framed data](message-body.md#non-framed-data), this value MUST equal the length, in bytes,
        //# of the plaintext data provided to the algorithm for encryption.
        enc_content.len() as u64,
        &mut aad,
    );

    let mut result: Vec<u8> = enc_content.clone();
    aes_decrypt(
        get_encrypt(&header.suite),
        key,
        &enc_content,
        &auth_tag,
        &iv,
        &aad,
        result.as_mut(),
    )?;

    Ok(result)
}

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
#[allow(clippy::no_effect_underscore_binding)]
pub(crate) fn construct_frame(
    input: &ConstructFrameInput<'_>,
    iv: &mut [u8],
    aad: &mut Vec<u8>,
    w: &mut Vec<u8>,
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    w.clear();

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed as follows:
    body_aad(
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
        //# [message ID](../data-frame/message-header.md#message-id) serialized in the header of this message.
        input.message_id,
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be the structure defined in
        //# [Message Body AAD](../data-format/message-body-aad.md).
        input.aad_content,
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
        //# number of the frame being encrypted.
        //= aws-encryption-sdk-specification/data-format/message-body-aad.md#sequence-number
        //= type=implication
        //= reason=The sequence_number parameter is the frame sequence number passed from encrypt_and_serialize_body
        //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#sequence-number).
        input.sequence_number,
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
        //# equal to the length of the plaintext being encrypted.
        //= aws-encryption-sdk-specification/data-format/message-body-aad.md#content-length
        //= type=implication
        //= reason=plaintext.len() is the length of the plaintext being encrypted in this frame
        //# - For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes,
        //# of the plaintext being encrypted in this frame.
        input.plaintext.len() as u64,
        aad,
    );

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=iv_seq pads the sequence number to the IV length by writing it into the last 4 bytes of a zeroed IV buffer
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD for this frame,
    //# padded to the [IV length](../data-format/message-header.md#iv-length).
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=Serialization order is enforced by the sequence of write calls below
    //# The Encrypt operation MUST serialize a regular frame or final frame with the following specifics:
    iv_seq(input.sequence_number, iv);
    //= aws-encryption-sdk-specification/data-format/message-body.md#regular-frame-iv
    //= type=implication
    //= reason=Each frame's IV is derived from its unique sequence number via iv_seq
    //# Each frame in the [Framed Data](#framed-data) MUST include an IV that is unique within the message.
    let _iv_is_unique = &iv;

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=The if input.is_final guard ensures Sequence Number End is only written for the final frame
    //# The Sequence Number End MUST only be serialized for the final frame.
    if input.is_final {
        //= specification/data-format/message-body.md#final-frame
        //= type=implication
        //= reason=Serialization order is enforced by the sequence of write calls: ENDFRAME marker, seq num, IV, content length, encrypted content, auth tag
        //# A final frame MUST be serialized as, in order,
        //# Sequence Number End,
        //# Sequence Number,
        //# IV,
        //# Encrypted Content Length,
        //# Encrypted Content,
        //# and Authentication Tag.
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //= type=implication
        //= reason=ENDFRAME_SEQUENCE_NUMBER is the 0xFFFFFFFF marker required by the spec
        //# - [Sequence Number End](../data-format/message-body.md#sequence-number-end): MUST be serialized according to the
        //# [Sequence Number End](../data-format/message-body.md#sequence-number-end) specification.
        write_u32(w, ENDFRAME_SEQUENCE_NUMBER)?;
        //= aws-encryption-sdk-specification/data-format/message-body.md#sequence-number-end
        //= type=implication
        //= reason=ENDFRAME_SEQUENCE_NUMBER is defined as 0xFFFFFFFF
        //# The value MUST be encoded as the 4 bytes `FF FF FF FF` in hexadecimal notation.
        let _endframe_written = ();
    }

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be serialized according to the
    //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
    //# The value MUST be the sequence number of this frame.
    //= aws-encryption-sdk-specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=implication
    //= reason=write_u32 serializes as a 4-byte big-endian UInt32
    //# The sequence number MUST be serialized as a UInt32.
    write_u32(w, input.sequence_number)?;
    //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame-sequence-number
    //= type=implication
    //= reason=construct_frame uses the same write_u32 for both regular and final frames
    //# The Final Frame Sequence Number MUST be serialized to a message the same way as the
    //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
    let _seq_num_written = &input.sequence_number;

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be serialized according to the
    //# [Regular Frame IV](../data-format/message-body.md#regular-frame-iv) specification.
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //# The value MUST be the IV used when calculating the encrypted content for this frame.
    write_bytes(w, iv)?;

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=The if input.is_final guard ensures Encrypted Content Length is only written for the final frame
    //# The Encrypted Content Length MUST only be serialized for the final frame.
    //= specification/data-format/message-body.md#final-frame
    //= type=implication
    //= reason=The is_final branches add Sequence Number End and Encrypted Content Length to the regular frame fields
    //# This means a final frame MUST be a regular frame with the addition of the serialized
    //# Sequence Number End
    //# and Encrypted Content Length.
    if input.is_final {
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //= type=implication
        //= reason=write_u32 serializes as a 4-byte big-endian UInt32
        //# - [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length): MUST be serialized according to the
        //# [Final Frame Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length) specification.
        //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame-encrypted-content-length
        //= type=implication
        //= reason=write_u32 serializes as a 4-byte big-endian UInt32
        //# The encrypted content length MUST be serialized as a UInt32.
        write_u32(w, input.plaintext.len() as u32)?;
    }

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //# To construct a regular or final frame that represents the next frame in the encrypted message's body,
    //# the Encrypt operation MUST calculate the encrypted content and an authentication tag using the
    //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md),
    //# with the following inputs:
    aes_encrypt(
        input.alg,
        iv,
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //= reason=input.key is set from the derived data key in encrypt_and_serialize_body
        //# - The cipherkey MUST be the derived data key
        input.key,
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //# - The plaintext MUST be the next subsequence of consumable plaintext bytes that have not yet been encrypted.
        input.plaintext,
        aad,
        w,
    )?;
    // aes_encrypt writes encrypted content followed by authentication tag to w
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=aes_encrypt writes the encrypted content directly into the output buffer w
    //# - [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content): MUST be serialized according to the
    //# [Regular Frame Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content) specification.
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=aes_encrypt writes the encrypted content directly into the output buffer w
    //# The value MUST be the encrypted content calculated for this frame.
    let _encrypted_content_written = ();
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=aes_encrypt appends the authentication tag directly after the encrypted content in w
    //# - [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag): MUST be serialized according to the
    //# [Regular Frame Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag) specification.
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=aes_encrypt appends the authentication tag directly after the encrypted content in w
    //# The value MUST be the authentication tag output when calculating the encrypted content for this frame.
    let _authentication_tag_written = ();

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= reason=Frame is fully serialized into w before being written to out; w.clear() at function start and write_bytes(out, w) here ensure atomic release
    //# The serialized frame bytes MUST NOT be released until the entire frame has been serialized.
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //# If the Encrypt operation is streaming the encrypted message and
    //# the entire frame has been serialized,
    //# the serialized frame MUST be released.
    write_bytes(out, w)?;

    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= type=implication
    //= reason=DigestWriter feeds the serialized frame bytes to the signature algorithm
    //# If the algorithm suite contains a signature algorithm and
    //# the Encrypt operation is [streaming](streaming.md) the encrypted message output to the caller,
    //# the Encrypt operation MUST input the serialized frame to the signature algorithm as soon as it is serialized,
    //# such that the serialized frame isn't required to remain in memory to [construct the signature](#construct-the-signature).
    write_bytes(dw, w)?;
    Ok(())
}

/// Encrypt plaintext and serialize the message body (framed) to the output stream.
pub(crate) fn encrypt_and_serialize_body(
    plaintext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=todo
    //# If [Plaintext Length Bound](#plaintext-length-bound) was specified on input
    //# and this operation determines at any time that the plaintext being encrypted
    //# has a length greater than this value,
    //# this operation MUST immediately fail.
    let mut total_data_size: usize = 0;
    let frame_length = header.body.frame_length() as usize;
    let iv_len = get_iv_length(&header.suite) as usize;
    let auth_len = get_tag_length(&header.suite) as usize;
    let frame_len = frame_length + iv_len + auth_len + 4;
    let mut w = Vec::with_capacity(frame_len);

    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=implementation
    //# Framed Data MUST start at Sequence Number 1.
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
    //= reason=START_SEQUENCE_NUMBER is defined as 1
    //# If this is the first frame sequentially, the sequence number value MUST be 1.
    let mut sequence_number = START_SEQUENCE_NUMBER;
    let alg = get_encrypt(&header.suite);

    let mut iv = vec![0; iv_len];
    let mut plaintext_frame = vec![0; frame_length];
    let mut aad = Vec::new();
    let mut in_size: usize;
    let mut next_char: Option<u8> = None;

    //= specification/client-apis/encrypt.md#construct-the-body
    //# Before the end of the input is indicated,
    //# this operation MUST process as much of the consumable bytes as possible
    //# by [constructing regular frames](#construct-a-frame).
    loop {
        in_size = read_up_to_peek(plaintext, &mut plaintext_frame, next_char)?;
        //= specification/client-apis/encrypt.md#construct-the-body
        //# - If there are not enough input consumable plaintext bytes to create a new regular frame,
        //# then this operation MUST [construct a final frame](#construct-a-frame)
        //= specification/data-format/message-body.md#final-frame
        //= reason=When in_size < frame_length on the first iteration, the loop breaks immediately to construct a single final frame
        //# - When the length of the Plaintext is less than the Frame Length,
        //# the body MUST contain exactly one frame and that frame MUST be a Final Frame.
        if in_size != frame_length {
            break;
        }
        next_char = read_opt_u8(plaintext)?;
        //= specification/client-apis/encrypt.md#construct-the-body
        //# - If there are exactly enough consumable plaintext bytes to create one regular frame,
        //# such that creating a regular frame processes all consumable bytes,
        //# then this operation MUST [construct either a final frame or regular frame](#construct-a-frame)
        //# with the remaining plaintext.
        //= specification/data-format/message-body.md#final-frame
        //= type=implication
        //= reason=When plaintext is an exact multiple of frame length, the implementation constructs a final frame with the full frame's data
        //# - When the length of the Plaintext is an exact multiple of the Frame Length
        //# (including if it is equal to the frame length),
        //# the Final Frame encrypted content length SHOULD be equal to the frame length but MAY be 0.
        if next_char.is_none() {
            break;
        }

        //= specification/client-apis/encrypt.md#construct-the-body
        //# - If there are enough input plaintext bytes consumable to create a new regular frame,
        //# such that creating a regular frame does not processes all consumable bytes,
        //# then this operation MUST [construct a regular frame](#construct-a-frame)
        //# using the consumable plaintext bytes.

        //= specification/data-format/message-body.md#framed-data
        //# - The number of frames in a single message MUST be less than or equal to `2^32 - 1`.
        if sequence_number == ENDFRAME_SEQUENCE_NUMBER {
            return Err("too many frames".into());
        }

        total_data_size += frame_length;
        if total_data_size > MAX_DATA {
            return Err("Plain text too large".into());
        }

        //= specification/client-apis/encrypt.md#construct-the-body
        //# Regular frame serialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
        //= specification/data-format/message-body.md#regular-frame
        //= type=implication
        //= reason=Serialization order is enforced by construct_frame's implementation; this annotation traces the structural requirement
        //# A regular frame MUST be serialized as, in order,
        //# Sequence Number,
        //# IV,
        //# Encrypted Content,
        //# and Authentication Tag.
        construct_frame(
            &ConstructFrameInput {
                alg, key,
                //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
                //= reason=plaintext_frame is exactly frame_length bytes (the full buffer read from input)
                //# - For a regular frame the length of this plaintext MUST equal the frame length.
                //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
                //= reason=plaintext_frame is exactly frame_length bytes for a regular frame
                //# - For a regular frame the length of this plaintext subsequence MUST equal the frame length.
                //= aws-encryption-sdk-specification/data-format/message-body.md#regular-frame-encrypted-content
                //= type=implication
                //= reason=plaintext_frame is exactly frame_length bytes, so encrypted content length equals frame length
                //# The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
                plaintext: &plaintext_frame,
                message_id: header.body.message_id(),
                aad_content: BodyAADContent::RegularFrame,
                sequence_number,
                //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
                //= type=implication
                //= reason=is_final=false means this is a regular frame, whose serialization follows the Regular Frame specification
                //# For a regular frame, the serialization MUST follow the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
                is_final: false,
            },
            &mut iv, &mut aad, &mut w, out, dw,
        )?;

        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
        //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
        //# Otherwise, the sequence number value MUST be 1 greater than the value of the sequence number
        //# of the previous frame.
        sequence_number += 1;
    }

    //= specification/client-apis/encrypt.md#construct-the-body
    //# When the end of the input is indicated,
    //# this operation MUST perform the following until all consumable plaintext bytes are processed:

    // Final frame
    total_data_size += in_size;
    if total_data_size > MAX_DATA {
        return Err("Plain text too large".into());
    }

    //= specification/data-format/message-body.md#final-frame
    //= reason=in_size is the plaintext length for the final frame; this asserts it is <= frame_length
    //# The length of the plaintext to be encrypted in the Final Frame MUST be
    //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).
    debug_assert!(in_size <= frame_length);
    debug_assert!(in_size > 0 || sequence_number == START_SEQUENCE_NUMBER,
        "empty final frame only allowed when entire plaintext is empty");

    //= specification/client-apis/encrypt.md#construct-the-body
    //# If an end to the input has been indicated, there are no more consumable plaintext bytes to process,
    //# and a final frame has not yet been constructed,
    //# this operation MUST [construct an empty final frame](#construct-a-frame).
    //= specification/client-apis/encrypt.md#construct-the-body
    //# Final frame serialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
    construct_frame(
        &ConstructFrameInput {
            alg, key,
            //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
            //= reason=plaintext_frame[0..in_size] is the remaining unencrypted bytes, where in_size <= frame_length
            //# - For a final frame this MUST be the length of the remaining plaintext bytes
            //# which have not yet been encrypted,
            //# whose length MUST be equal to or less than the frame length.
            //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
            //= reason=plaintext_frame[0..in_size] is the remaining unencrypted bytes, where in_size <= frame_length
            //# - For a final frame this MUST be the remaining plaintext bytes which have not yet been encrypted,
            //# whose length MUST be equal to or less than the frame length.
            plaintext: &plaintext_frame[0..in_size],
            message_id: header.body.message_id(),
            aad_content: BodyAADContent::FinalFrame,
            //= aws-encryption-sdk-specification/data-format/message-body.md#final-frame-sequence-number
            //= type=implication
            //= reason=sequence_number is incremented for each regular frame and equals the total frame count at the final frame
            //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
            sequence_number,
            //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-a-frame
            //= type=implication
            //= reason=is_final=true means this is a final frame, whose serialization follows the Final Frame specification
            //# For a final frame, the serialization MUST follow the [Final Frame](../data-format/message-body.md#final-frame) specification.
            is_final: true,
        },
        &mut iv, &mut aad, &mut w, out, dw,
    )?;

    Ok(())
}

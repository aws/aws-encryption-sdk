// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::header::{ENDFRAME_SEQUENCE_NUMBER, HeaderInfo, START_SEQUENCE_NUMBER};
use super::serializable_types::*;
use super::serialize_functions::{read_bytes, read_seq_u32_bounded, read_u32};
use super::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::primitives::{AesGcm, aes_decrypt};
use aws_mpl_legacy::suites::AlgorithmSuite;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum BodyAADContent {
    RegularFrame,
    FinalFrame,
    SingleBlock,
}

//= specification/data-format/message-body-aad.md#body-aad-content
//# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
const BODY_AAD_CONTENT_REGULAR_FRAME: &str = "AWSKMSEncryptionClient Frame";
//= specification/data-format/message-body-aad.md#body-aad-content
//# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.
const BODY_AAD_CONTENT_FINAL_FRAME: &str = "AWSKMSEncryptionClient Final Frame";
//= specification/data-format/message-body-aad.md#body-aad-content
//# - [Non-framed data](message-body.md#non-framed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
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
    // per the body-aad spec. See call sites in encrypt_decrypt.rs for the
    // annotations that document which values satisfy which requirements.
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
    //# the Decrypt operation operation MUST use the first 4 bytes of a frame to determine
    //# whether the operation will deserialize the frame as a [final frame](../data-format/message-body.md#final-frame)
    //# or [regular frame](../fata-format/message-body/md#regular-frame).
    loop {
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //= type=implication
        //# The length of the serialized sequence number MUST be 4 bytes.
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //= type=implication
        //# The sequence number MUST be interpreted as a UInt32.
        //= specification/data-format/message-body.md#sequence-number-end
        //= type=implication
        //# The length of the serialized sequence number end MUST be 4 bytes.
        //= specification/data-format/message-body.md#sequence-number-end
        //= type=implication
        //# The sequence number end MUST be interpreted as bytes.
        let seq_num = read_u32(r, raw)?;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# If the first 4 bytes have a value of 0xFFFF,
        //# then the Decrypt operation MUST deserialize this as the [sequence number end](../data-format/message-header.md#sequence-number-end)
        //# and the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
        if seq_num == ENDFRAME_SEQUENCE_NUMBER {
            //= specification/data-format/message-body.md#final-frame-sequence-number
            //= type=implication
            //# The length of the serialized sequence number MUST be 4 bytes.
            //= specification/data-format/message-body.md#final-frame-sequence-number
            //= type=implication
            //# The sequence number MUST be interpreted as a UInt32.
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
            //# The length of the serialized encrypted content length field MUST be 4 bytes.
            //= specification/data-format/message-body.md#final-frame-encrypted-content-length
            //= type=implication
            //# The encrypted content length MUST be interpreted as a UInt32.
            //= specification/data-format/message-body.md#final-frame-encrypted-content
            //# The length of the serialized encrypted content MUST be equal to the value of the [Encrypted Content Length](#encrypted-content-length-1) field.
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
                //# Otherwise, the Decrypt operation MUST deserialize this as the [sequence number](../data-format/message-header.md#sequence-number)
                //# and the following bytes according to the [regular frame spec](../data-format/message-body.md#regular-frame).
                // write previous frame's data, now that we know we have another frame.
                if expected_frame != START_SEQUENCE_NUMBER {
                    if fail_if_multi_frame {
                        return Err("Streaming Interface can return data before signature has been validated. Set `allow_unsafe_unverified_signature` in the DecryptStreamInput struct if this is ok.".into());
                    }
                    serialize_functions::write_bytes(w, &result)?;
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
            serialize_functions::write_bytes(w, &result)?;
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

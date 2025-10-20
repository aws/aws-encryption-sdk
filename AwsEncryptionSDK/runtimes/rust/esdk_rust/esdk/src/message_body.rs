// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::serialize::header::{ENDFRAME_SEQUENCE_NUMBER, HeaderInfo, START_SEQUENCE_NUMBER};
use crate::serialize::serializable_types::*;
use crate::serialize::serialize_functions::{read_bytes, read_seq_u32_bounded, read_u32};
use crate::serialize::*;
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_primitives::{AesGcm, aes_decrypt};
use aws_mpl_rs::types::AlgorithmSuiteInfo;

pub(crate) fn get_aes_alg(suite: &AlgorithmSuiteInfo) -> AesGcm {
    let alg = get_encrypt(suite);
    match alg.key_length() {
        Some(16) => AesGcm::Aes128Gcm,
        Some(24) => AesGcm::Aes192Gcm,
        Some(32) => AesGcm::Aes256Gcm,
        _ => panic!("Only AES-128, AES-192 and AES-256 are supported"),
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub(crate) enum BodyAADContent {
    RegularFrame,
    FinalFrame,
    SingleBlock,
}

const BODY_AAD_CONTENT_REGULAR_FRAME: &str = "AWSKMSEncryptionClient Frame";
const BODY_AAD_CONTENT_FINAL_FRAME: &str = "AWSKMSEncryptionClient Final Frame";
const BODY_AAD_CONTENT_SINGLE_BLOCK: &str = "AWSKMSEncryptionClient Single Block";

const fn body_aad_content_type_string(bc: BodyAADContent) -> &'static str {
    match bc {
        BodyAADContent::RegularFrame => BODY_AAD_CONTENT_REGULAR_FRAME,
        BodyAADContent::FinalFrame => BODY_AAD_CONTENT_FINAL_FRAME,
        BodyAADContent::SingleBlock => BODY_AAD_CONTENT_SINGLE_BLOCK,
    }
}

pub(crate) fn iv_seq(sequence_number: u32, result: &mut [u8])
//= compliance/data-format/message-body.txt#2.5.2.1.2
//= type=implication
//# The IV length MUST be equal to the IV
//# length of the algorithm suite specified by the Algorithm Suite ID
//# (message-header.md#algorithm-suite-id) field.
//
//= compliance/data-format/message-body.txt#2.5.2.2.3
//= type=implication
//# The IV length MUST be equal to the IV length of the algorithm suite
//# (../framework/algorithm-suites.md) that generated the message.

//= compliance/data-format/message-body.txt#2.5.2.1.2
//= type=implication
//# Each frame in the Framed Data (Section 2.5.2) MUST include an IV that
//# is unique within the message.
//
//= compliance/data-format/message-body.txt#2.5.2.2.3
//= type=implication
//# The IV MUST be a unique IV within the message.
{
    let pivot = result.len() - 4;
    result[pivot..].copy_from_slice(&sequence_number.to_be_bytes());
}

pub(crate) fn get_encrypt(
    info: &AlgorithmSuiteInfo,
) -> aws_mpl_rs::deps::aws_cryptography_primitives::types::AesGcm {
    match &info.encrypt.as_ref().unwrap() {
        aws_mpl_rs::types::Encrypt::AesGcm(aes_gcm) => aes_gcm.clone(),
        _ => panic!("not an aes gcm"),
    }
}

/*
 * Serializes the Message Body ADD
 */

pub(crate) fn body_aad2(
    message_id: &[u8],
    bc: BodyAADContent,
    sequence_number: u32,
    length: u64,
    result: &mut Vec<u8>,
) {
    result.clear();
    //= compliance/client-apis/decrypt.txt#2.7.4
    //#*  The AAD is the serialized message body AAD (../data-format/
    //#   message-body-aad.md), constructed as follows:

    //# -  The message ID (../data-format/message-body-aad.md#message-id)
    //#    is the same as the message ID (../data-frame/message-
    //#    header.md#message-id) deserialized from the header of this
    //#    message.
    result.extend_from_slice(message_id);
    //# -  The Body AAD Content (../data-format/message-body-aad.md#body-
    //#    aad-content) depends on whether the thing being decrypted is a
    //#    regular frame, final frame, or un-framed data.  Refer to
    //#    Message Body AAD (../data-format/message-body-aad.md)
    //#    specification for more information.
    result.extend_from_slice(body_aad_content_type_string(bc).as_bytes());
    //# -  The sequence number (../data-format/message-body-
    //#    aad.md#sequence-number) is the sequence number deserialized
    //#    from the frame being decrypted.
    result.extend_from_slice(&sequence_number.to_be_bytes());
    //= compliance/client-apis/decrypt.txt#2.7.4
    //# -  The content length (../data-format/message-body-aad.md#content-
    //# length) MUST have a value equal to the length of the plaintext
    //# that was encrypted.
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
    let mut expected_frame: u32 = START_SEQUENCE_NUMBER;
    let mut iv = vec![0u8; get_iv_length(&header.suite) as usize];
    let mut auth_tag = vec![0u8; get_tag_length(&header.suite) as usize];
    let alg = get_aes_alg(&header.suite);
    let frame_length_u64 = u64::from(header.body.frame_length());
    let frame_length_usize = header.body.frame_length() as usize;
    let mut enc_content = vec![0u8; frame_length_usize];
    let mut result = vec![0; frame_length_usize];
    let mut aad = Vec::new();

    loop {
        let seq_num = read_u32(r, raw)?;
        if seq_num == ENDFRAME_SEQUENCE_NUMBER {
            let seq_num: u32 = read_u32(r, raw)?;
            if seq_num != expected_frame {
                return Err("Final sequence number out of order.".into());
            }
            read_bytes(r, &mut iv, raw)?;
            read_seq_u32_bounded(
                r,
                header.body.frame_length(),
                "Content length MUST NOT exceed the frame length.",
                &mut enc_content,
                raw,
            )?;
            read_bytes(r, &mut auth_tag, raw)?;
            body_aad2(
                header.body.message_id(),
                BodyAADContent::FinalFrame,
                seq_num,
                enc_content.len() as u64,
                &mut aad,
            );
            if enc_content.is_empty() {
                // final frame is empty, to return last full frame
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
                return Ok(result);
            } else {
                // write previous frame's data, now that we know we have another frame.
                if expected_frame != START_SEQUENCE_NUMBER {
                    if fail_if_multi_frame {
                        return Err("Streaming Interface can return data before signature has been validated. Set `i_accept_the_danger` in the DecryptStreamInput struct if this is ok.".into());
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
                return Ok(result);
            }
        }
        if seq_num != expected_frame {
            return Err("Sequence number out of order.".into());
        }
        // write previous frame's data, now that we know we have another frame.
        if expected_frame != START_SEQUENCE_NUMBER {
            if fail_if_multi_frame {
                return Err("Streaming Interface can return data before signature has been validated. Set `i_accept_the_danger` in the DecryptStreamInput struct if this is ok.".into());
            }
            serialize_functions::write_bytes(w, &result)?;
        }
        expected_frame += 1;
        read_bytes(r, &mut iv, raw)?;
        read_bytes(r, &mut enc_content, raw)?;
        read_bytes(r, &mut auth_tag, raw)?;
        body_aad2(
            header.body.message_id(),
            BodyAADContent::RegularFrame,
            seq_num,
            frame_length_u64,
            &mut aad,
        );
        aes_decrypt(alg, key, &enc_content, &auth_tag, &iv, &aad, &mut result)?;
    }
}

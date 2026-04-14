// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Frame decryption and body deserialization.

use super::{body_aad, get_encrypt, BodyAADContent};
use crate::error::esdk_err;
use crate::message::header::{ENDFRAME_SEQUENCE_NUMBER, HeaderInfo, START_SEQUENCE_NUMBER};
use crate::message::serializable_types::{get_iv_length, get_tag_length};
use crate::message::serialize_functions::{read_bytes, read_seq_u32_bounded, read_u32, write_bytes};
use crate::message::{Error, header, ser_err, serialize_functions};
use crate::types::{SafeRead, SafeWrite};
use aws_mpl_legacy::primitives::aes_decrypt;

#[allow(clippy::no_effect_underscore_binding)]
pub(crate) fn read_and_decrypt_framed_message_body(
    ciphertext: &mut dyn SafeRead,
    w: &mut dyn SafeWrite,
    header: &HeaderInfo,
    key: &[u8],
    sig_digest: &mut dyn SafeWrite,
    fail_if_multi_frame: bool,
) -> Result<Vec<u8>, Error> {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# If this is framed data and the first frame sequentially, this value MUST be 1.
    let mut expected_frame: u32 = START_SEQUENCE_NUMBER;
    //= specification/data-format/message-body.md#regular-frame-iv
    //# The IV length MUST be equal to the IV length of the algorithm suite specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let mut iv = vec![0u8; get_iv_length(&header.suite) as usize];
    //= specification/data-format/message-body.md#regular-frame-authentication-tag
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let mut auth_tag = vec![0u8; get_tag_length(&header.suite) as usize];
    let alg = get_encrypt(&header.suite)?;
    let frame_length_u64 = u64::from(header.body.frame_length());
    let frame_length_usize = header.body.frame_length() as usize;
    let mut enc_content = vec![0u8; frame_length_usize];
    let mut result = vec![0; frame_length_usize];
    let mut aad = Vec::new();

    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //= reason=The loop continuously reads and decrypts frames from the ciphertext stream, processing each frame as its bytes become available, until the final frame is encountered
    //# If there could still be message body left to deserialize and decrypt,
    //# this operation MUST either wait for more of the encrypted message bytes to become consumable,
    //# wait for the end to the encrypted message to be indicated,
    //# or deserialize and/or decrypt the consumable bytes.
    loop {
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# If deserializing [framed data](../data-format/message-body.md#framed-data),
        //# the Decrypt operation MUST use the first 4 bytes of a frame to determine
        //# whether the operation will deserialize the frame as a [final frame](../data-format/message-body.md#final-frame)
        //# or [regular frame](../data-format/message-body.md#regular-frame).
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //= reason=read_u32 reads the first 4 bytes which serve as both the Sequence Number End check and the Sequence Number for regular frames
        //# - The [Sequence Number End](../data-format/message-body.md#sequence-number-end): MUST be deserialized according to the
        //# [Sequence Number End](../data-format/message-body.md#sequence-number-end) specification.
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //= reason=read_u32 reads the first 4 bytes as a UInt32, which is the sequence number for regular frames
        //# - [Sequence Number](../data-format/message-body.md#regular-frame-sequence-number): MUST be deserialized according to the
        //# [Regular Frame Sequence Number](../data-format/message-body.md#regular-frame-sequence-number) specification.
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //# The length of the serialized sequence number field MUST be 4 bytes.
        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //# The sequence number MUST be interpreted as a UInt32.
        //= specification/data-format/message-body.md#sequence-number-end
        //# The length of the sequence number end field MUST be 4 bytes.
        let seq_num = read_u32(ciphertext, sig_digest)?;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# If the first 4 bytes have a value of 0xFFFFFFFF,
        //# then the Decrypt operation MUST deserialize the following bytes according to the [final frame spec](../data-format/message-body.md#final-frame).
        //= specification/data-format/message-body.md#sequence-number-end
        //# The sequence number end MUST be interpreted as bytes.
        if seq_num == ENDFRAME_SEQUENCE_NUMBER {
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# Final frame deserialization MUST conform to the [Final Frame](../data-format/message-body.md#final-frame) specification.
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# For a final frame, each field MUST be deserialized according to its specification:
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=read_u32 reads the final frame sequence number after the ENDFRAME marker
            //# - [Sequence Number](../data-format/message-body.md#final-frame-sequence-number): MUST be deserialized according to the
            //# [Final Frame Sequence Number](../data-format/message-body.md#final-frame-sequence-number) specification.
            //= specification/data-format/message-body.md#final-frame-sequence-number
            //# The Final Frame Sequence Number MUST be interpreted as the same type as the
            //# [Regular Frame Sequence Number](#regular-frame-sequence-number).
            //= specification/data-format/message-body.md#regular-frame-sequence-number
            //# The length of the serialized sequence number field MUST be 4 bytes.
            //= specification/data-format/message-body.md#regular-frame-sequence-number
            //# The sequence number MUST be interpreted as a UInt32.
            let seq_num: u32 = read_u32(ciphertext, sig_digest)?;
            if seq_num != expected_frame {
                return ser_err("Final sequence number out of order");
            }
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=read_bytes reads IV bytes from the final frame
            //# - [IV](../data-format/message-body.md#final-frame-iv): MUST be deserialized according to the
            //# [Final Frame IV](../data-format/message-body.md#final-frame-iv) specification.
            //= specification/data-format/message-body.md#final-frame-iv
            //# The length of the IV field MUST be equal to the IV length of the [algorithm suite](../framework/algorithm-suites.md) that generated the message.
            //= specification/data-format/message-body.md#final-frame-iv
            //# The IV MUST be interpreted as bytes.
            read_bytes(ciphertext, &mut iv, sig_digest)?;
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# The Decrypt operation MUST ensure that the length of the encrypted content field is
            //# less than or equal to the frame length deserialized in the message header.
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=read_seq_u32_bounded reads the encrypted content length field from the final frame
            //# - [Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length): MUST be deserialized according to the
            //# [Final Frame Encrypted Content Length](../data-format/message-body.md#final-frame-encrypted-content-length) specification.
            //= specification/data-format/message-body.md#final-frame-encrypted-content-length
            //# The length of the serialized encrypted content length field MUST be 4 bytes.
            //= specification/data-format/message-body.md#final-frame-encrypted-content-length
            //# The encrypted content length MUST be a UInt32.
            //= specification/data-format/message-body.md#final-frame-encrypted-content
            //# The length of the serialized encrypted content field MUST be equal to the value of the [Encrypted Content Length](#encrypted-content-length-1) field.
            //= specification/data-format/message-body.md#final-frame
            //= reason=read_seq_u32_bounded enforces encrypted content length <= frame_length, which bounds the plaintext length
            //# The length of the plaintext to be encrypted in the Final Frame MUST be
            //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).
            read_seq_u32_bounded(
                ciphertext,
                header.body.frame_length(),
                "Content length MUST NOT exceed the frame length",
                &mut enc_content,
                sig_digest,
            )?;
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=enc_content holds the encrypted content bytes deserialized from the final frame by read_seq_u32_bounded
            //# - [Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content): MUST be deserialized according to the
            //# [Final Frame Encrypted Content](../data-format/message-body.md#final-frame-encrypted-content) specification.
            //= specification/data-format/message-body.md#final-frame-encrypted-content
            //# The encrypted content MUST be interpreted as bytes.
            let _enc_content_is_bytes = &enc_content;
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=read_bytes reads the authentication tag bytes from the final frame
            //# - [Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag): MUST be deserialized according to the
            //# [Final Frame Authentication Tag](../data-format/message-body.md#final-frame-authentication-tag) specification.
            //= specification/data-format/message-body.md#final-frame-authentication-tag
            //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
            //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
            //= specification/data-format/message-body.md#final-frame-authentication-tag
            //# The authentication tag MUST be interpreted as bytes.
            read_bytes(ciphertext, &mut auth_tag, sig_digest)?;
            body_aad(
                //= specification/client-apis/decrypt.md#decrypt-the-message-body
                //= reason=header.body.message_id() is the message ID deserialized from the header
                //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
                //# [message ID](../data-format/message-header.md#message-id) deserialized from the header of this message.
                header.body.message_id(),
                //= specification/client-apis/decrypt.md#decrypt-the-message-body
                //= reason=BodyAADContent::FinalFrame selects the correct AAD content type for the final frame
                //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be constructed
                //# according to [Message Body AAD](../data-format/message-body-aad.md) depending on
                //# whether the bytes being decrypted are a regular frame, final frame, or un-framed data.
                //= specification/data-format/message-body-aad.md#body-aad-content
                //= reason=BodyAADContent::FinalFrame maps to "AWSKMSEncryptionClient Final Frame"
                //# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.
                BodyAADContent::FinalFrame,
                //= specification/client-apis/decrypt.md#decrypt-the-message-body
                //= reason=seq_num is the sequence number deserialized from this final frame via read_u32
                //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
                //# number deserialized from the frame being decrypted.
                seq_num,
                //= specification/client-apis/decrypt.md#decrypt-the-message-body
                //# If this is not a regular frame, this SHOULD be determined by using the the [encrypted content length](../data-format/message-body.md#final-frame-encrypted-content-length).
                //= specification/data-format/message-body-aad.md#content-length
                //# - For the [final frame](message-body.md#final-frame), this value MUST be greater than or equal to
                //# 0 and less than or equal to the value of the [frame length](message-header.md#frame-length)
                //# field in the message header.
                {
                    debug_assert!(enc_content.len() <= frame_length_usize);
                    enc_content.len() as u64
                },
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
                    //= specification/data-format/message-body.md#final-frame-authentication-tag
                    //= reason=auth_tag is passed to aes_decrypt which uses it to authenticate the final frame
                    //# It MUST be used to authenticate the final frame.
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
                        return Err(esdk_err("Streaming interface can return data before signature has been validated. Set `allow_unsafe_unverified_signature` in the DecryptStreamInput struct if this is ok"));
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
            //# Framed data MUST contain exactly one final frame.
            //= specification/data-format/message-body.md#final-frame
            //# The final frame MUST be the last frame.
            return Ok(result);
        }
        if seq_num != expected_frame {
            return ser_err("Sequence number out of order");
        }
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# - If the streamed Decrypt operation is using an algorithm suite with a signature algorithm,
        //# all plaintext decrypted from regular frames SHOULD be released as soon as the above calculation,
        //# including tag verification, succeeds.
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# - If the streamed Decrypt operation is using an algorithm suite without a signature algorithm,
        //# plaintext SHOULD be released as soon as the above calculation, including tag verification,
        //# succeeds.
        // write previous frame's data, now that we know we have another frame.
        if expected_frame != START_SEQUENCE_NUMBER {
            if fail_if_multi_frame {
                return Err(esdk_err("Streaming interface can return data before signature has been validated. Set `allow_unsafe_unverified_signature` in the DecryptStreamInput struct if this is ok"));
            }
            write_bytes(w, &result)?;
        }
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# Regular frame deserialization MUST conform to the [Regular Frame](../data-format/message-body.md#regular-frame) specification.
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# For a regular frame, each field MUST be deserialized according to its specification:
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# Otherwise, this value MUST be 1 greater than the value of the sequence number
        //# of the previous frame.
        expected_frame += 1;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //= reason=read_bytes reads IV bytes from the regular frame
        //# - [IV](../data-format/message-body.md#regular-frame-iv): MUST be deserialized according to the
        //# [Regular Frame IV](../data-format/message-body.md#regular-frame-iv) specification.
        //= specification/data-format/message-body.md#regular-frame-iv
        //# The IV MUST be interpreted as bytes.
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# - The streamed Decrypt operation SHOULD input the serialized frame to the signature algorithm as soon as it is deserialized,
        //# such that the serialized frame isn't required to remain in memory to complete
        //# the [signature verification](#verify-the-signature).
        read_bytes(ciphertext, &mut iv, sig_digest)?;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //= reason=read_bytes reads the encrypted content bytes from the regular frame
        //# - [Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content): MUST be deserialized according to the
        //# [Regular Frame Encrypted Content](../data-format/message-body.md#regular-frame-encrypted-content) specification.
        //= specification/data-format/message-body.md#regular-frame-encrypted-content
        //# The encrypted content MUST be interpreted as bytes.
        read_bytes(ciphertext, &mut enc_content, sig_digest)?;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //= reason=read_bytes reads the authentication tag bytes from the regular frame
        //# - [Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag): MUST be deserialized according to the
        //# [Regular Frame Authentication Tag](../data-format/message-body.md#regular-frame-authentication-tag) specification.
        //= specification/data-format/message-body.md#regular-frame-authentication-tag
        //# The authentication tag MUST be interpreted as bytes.
        read_bytes(ciphertext, &mut auth_tag, sig_digest)?;
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
        //# equal to the length of the plaintext that was encrypted.
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
        //# constructed according to the [Message Body AAD](../data-format/message-body-aad.md) specification, as follows:
        body_aad(
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=header.body.message_id() is the message ID deserialized from the header
            //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
            //# [message ID](../data-format/message-header.md#message-id) deserialized from the header of this message.
            header.body.message_id(),
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=BodyAADContent::RegularFrame selects the correct AAD content type for regular frames
            //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be constructed
            //# according to [Message Body AAD](../data-format/message-body-aad.md) depending on
            //# whether the bytes being decrypted are a regular frame, final frame, or un-framed data.
            //= specification/data-format/message-body-aad.md#body-aad-content
            //= reason=BodyAADContent::RegularFrame maps to "AWSKMSEncryptionClient Frame"
            //# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
            BodyAADContent::RegularFrame,
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=seq_num is the sequence number deserialized from this frame via read_u32
            //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
            //# number deserialized from the frame being decrypted.
            //= specification/data-format/message-body-aad.md#sequence-number
            //= reason=seq_num is the frame sequence number read from the regular frame
            //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#regular-frame-sequence-number).
            seq_num,
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# If this is a regular frame, this SHOULD be determined by using the [frame length](../data-format/message-header.md#frame-length)
            //# deserialized from the message header.
            //= specification/data-format/message-body-aad.md#content-length
            //# - For [regular frames](message-body.md#regular-frame), this value MUST equal the value of
            //# the [frame length](message-header.md#frame-length) field in the message header.
            {
                debug_assert_eq!(enc_content.len(), frame_length_usize);
                frame_length_u64
            },
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
        aes_decrypt(
            alg,
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=key is the derived data key passed from step_decrypt_body
            //# - The cipherkey MUST be the derived data key
            key,
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=enc_content is the encrypted content deserialized from the frame
            //# - The ciphertext MUST be the [encrypted content](../data-format/message-body.md#regular-frame-encrypted-content).
            &enc_content,
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //= reason=auth_tag is the authentication tag deserialized from the frame
            //# - The tag MUST be the value serialized in the
            //# [authentication tag field](../data-format/message-body.md#regular-frame-authentication-tag)
            //# in the message body or frame.
            //= specification/data-format/message-body.md#final-frame-authentication-tag
            //= reason=aes_decrypt validates the decryption against this provided auth_tag
            //# It MUST be used to authenticate the final frame.
            &auth_tag,
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
            //# used in the message body AAD above,
            //# padded to the [IV length](../data-format/message-header.md#iv-length) with 0.
            &iv,
            &aad,
            &mut result,
        )?;
    }
}

#[allow(clippy::no_effect_underscore_binding)]
pub(crate) fn read_and_decrypt_non_framed_message_body(
    ciphertext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    sig_digest: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    // Non-framed write-path requirements: ESDK only encrypts framed data
    //= specification/data-format/message-body.md#non-framed-data-iv
    //= type=exception
    //= reason=The ESDK only encrypts framed data; non-framed write path is not implemented
    //# A generated IV MUST be a unique IV within the message.
    //= specification/data-format/message-header.md#frame-length
    //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
    if header.body.frame_length() != 0 {
        return ser_err("Non-framed message contains non-zero frame length");
    }

    //= specification/data-format/message-body.md#non-framed-data
    //= reason=The fields are read in order: IV, then content length + content (via read_seq_u64_bounded), then auth tag
    //# Non-framed data MUST consist of, in order,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.
    //= specification/data-format/message-body.md#non-framed-data-iv
    //# The length of the IV field MUST be [IV Length](message-header.md#iv-length) bytes.
    //= specification/data-format/message-body.md#non-framed-data-iv
    //= reason=read_vec returns Vec<u8>
    //# The IV MUST be interpreted as bytes.
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= reason=iv is deserialized here and passed to aes_decrypt below
    //# - The IV MUST be the [IV](../data-format/message-body.md#non-framed-data-iv) deserialized from the message body.
    let iv = serialize_functions::read_vec(ciphertext, get_iv_length(&header.suite) as usize, sig_digest)?;
    debug_assert_eq!(iv.len(), get_iv_length(&header.suite) as usize);

    //= specification/data-format/message-body.md#non-framed-data-encrypted-content-length
    //= reason=SAFE_MAX_ENCRYPT equals 2^36 - 32; read_seq_u64_bounded rejects values above this limit
    //# The value of this field MUST NOT be greater than `2^36 - 32`, or 64 gibibytes (64 GiB),
    //# due to restrictions imposed by the [implemented algorithms](../framework/algorithm-suites.md).
    //= specification/data-format/message-body.md#non-framed-data-encrypted-content-length
    //= reason=read_seq_u64_bounded reads 8 bytes as a u64
    //# The encrypted content length MUST be interpreted as a UInt64.
    let enc_content = serialize_functions::read_seq_u64_bounded(
        ciphertext,
        header::SAFE_MAX_ENCRYPT,
        "Frame exceeds AES-GCM cryptographic safety for a single key/iv.",
        sig_digest,
    )?;
    //= specification/data-format/message-body.md#non-framed-data-encrypted-content-length
    //= reason=read_seq_u64_bounded reads exactly 8 bytes for the content length field
    //# The length of the encrypted content field MUST be 8 bytes.
    let _enc_content_len_is_8_bytes = &enc_content;
    //= specification/data-format/message-body.md#non-framed-data-encrypted-content
    //# The length of the serialized encrypted content field MUST be equal to the value of the [Encrypted Content Length](#non-framed-data-encrypted-content-length) field.
    //= specification/data-format/message-body.md#non-framed-data-encrypted-content
    //= reason=read_seq_u64_bounded returns Vec<u8>
    //# The encrypted content value MUST be interpreted as bytes.
    // read_seq_u64_bounded reads the u64 length then reads exactly that many bytes,
    // so enc_content.len() is guaranteed to equal the deserialized content length field.
    debug_assert!(enc_content.len() as u64 <= header::SAFE_MAX_ENCRYPT);

    //= specification/data-format/message-body.md#non-framed-data-authentication-tag
    //# The length of the serialized authentication tag field MUST be equal to the [authentication tag length](../framework/algorithm-suites.md#authentication-tag-length) of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    //= specification/data-format/message-body.md#non-framed-data-authentication-tag
    //= reason=read_vec returns Vec<u8>
    //# The authentication tag value MUST be interpreted as bytes.
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= reason=auth_tag is deserialized here and passed to aes_decrypt below
    //# - The tag MUST be the [Authentication Tag](../data-format/message-body.md#non-framed-data-authentication-tag) deserialized from the message body.
    let auth_tag = serialize_functions::read_vec(ciphertext, get_tag_length(&header.suite) as usize, sig_digest)?;
    debug_assert_eq!(auth_tag.len(), get_tag_length(&header.suite) as usize);
    let mut aad = Vec::new();
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= reason=body_aad constructs the AAD per the message-body-aad spec
    //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
    //# constructed with:
    body_aad(
        header.body.message_id(),
        //= specification/data-format/message-body-aad.md#body-aad-content
        //= reason=BodyAADContent::SingleBlock maps to "AWSKMSEncryptionClient Single Block"
        //# - [Non-framed data](message-body.md#non-framed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
        //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
        //= reason=BodyAADContent::SingleBlock maps to the non-framed data value
        //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST use the value for
        //# [non-framed data](../data-format/message-body-aad.md#body-aad-content).
        BodyAADContent::SingleBlock,
        //= specification/data-format/message-body-aad.md#sequence-number
        //# For [non-framed data](message-body.md#non-framed-data), the value of this field MUST be `1`.
        //= specification/client-apis/decrypt.md#decrypt-the-message-body
        //= reason=NONFRAMED_SEQUENCE_NUMBER is defined as 1
        //# If this is un-framed data, this value MUST be 1.
        //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
        //= reason=NONFRAMED_SEQUENCE_NUMBER is the constant 1
        //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be `1`.
        {
            debug_assert_eq!(header::NONFRAMED_SEQUENCE_NUMBER, 1);
            header::NONFRAMED_SEQUENCE_NUMBER
        },
        //= specification/data-format/message-body-aad.md#content-length
        //# - For [non-framed data](message-body.md#non-framed-data), this value MUST equal the length, in bytes,
        //# of the plaintext data provided to the algorithm for encryption.
        //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
        //= reason=enc_content.len() equals the length of the encrypted content
        //# - The [content length](../data-format/message-body-aad.md#content-length) MUST equal the length of the encrypted content.
        {
            let content_len = enc_content.len() as u64;
            debug_assert!(content_len <= header::SAFE_MAX_ENCRYPT);
            content_len
        },
        &mut aad,
    );

    let mut result: Vec<u8> = enc_content.clone();
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= reason=aes_decrypt is called with the algorithm suite's authenticated encryption algorithm
    //# and decrypt it using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= reason=enc_content is passed as the ciphertext input to aes_decrypt
    //# - The ciphertext MUST be the [Encrypted Content](../data-format/message-body.md#non-framed-data-encrypted-content) deserialized from the message body.
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= reason=key parameter is the derived data key from decryption materials
    //# - The cipherkey MUST be the derived data key.
    //= specification/client-apis/decrypt.md#un-framed-message-body-decryption
    //= reason=The ? operator propagates the decryption error, halting the operation immediately
    //# If this decryption fails, this operation MUST immediately halt and fail.
    aes_decrypt(
        get_encrypt(&header.suite)?,
        key,
        &enc_content,
        &auth_tag,
        &iv,
        &aad,
        result.as_mut(),
    )?;

    Ok(result)
}

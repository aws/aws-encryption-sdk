// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::message_body::*;
use crate::serialize::encryption_context::*;
use crate::serialize::header::*;
use crate::serialize::header_auth::*;
use crate::serialize::header_types::*;
use crate::serialize::serializable_types::*;
use crate::serialize::serialize_functions::*;
use crate::serialize::v2_header_body::get_hkdf;
use crate::serialize::*;
use aws_mpl_rs::EncryptedDataKey;
use aws_mpl_rs::suites::AlgorithmSuite;

use aws_mpl_primitives::ecdsa_verify_context;
use aws_mpl_primitives::{EcdsaSignatureAlgorithm, aes_encrypt, generate_random_bytes};

const RESERVED_ENCRYPTION_CONTEXT: &str = "aws-crypto-";
const MAX_DATA: usize = (1usize << 36) - 32;

pub(crate) fn encrypt_and_serialize(
    plaintext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    let mut total_data_size: usize = 0;
    let frame_length = header.body.frame_length() as usize;
    //= ../specification/data-format/message-body.md#iv-1
    //= type=implication
    //# The IV length MUST be equal to the IV length of the algorithm suite specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    let iv_len = get_iv_length(&header.suite) as usize;
    let auth_len = get_tag_length(&header.suite) as usize;
    let frame_len = frame_length + iv_len + auth_len + 4;
    //= ../specification/data-format/message-body.md#encrypted-content-1
    //= type=implication
    //# The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
    let mut w = Vec::with_capacity(frame_len);
    write_bytes(&mut w, &header.raw_header)?;
    write_header_auth_tag(&mut w, &header.header_auth, &header.suite)?;
    write_bytes(out, &w)?;
    write_bytes(dw, &w)?;

    //= ../specification/data-format/message-body.md#sequence-number
    //= type=implication
    //# Framed Data MUST start at Sequence Number 1.
    let mut sequence_number = START_SEQUENCE_NUMBER;
    let alg = get_encrypt(&header.suite);

    let mut iv = vec![0; iv_len];
    let mut plaintext_frame = vec![0; frame_length];
    let mut aad = Vec::new();
    let mut in_size: usize;
    let mut next_char: Option<u8> = None;

    loop {
        w.clear();
        in_size = read_up_to_peek(plaintext, &mut plaintext_frame, next_char)?;
        if in_size != frame_length {
            break;
        }
        next_char = read_opt_u8(plaintext)?;
        if next_char.is_none() {
            break;
        }
        //= ../specification/data-format/message-body.md#framed-data
        //= type=implication
        //# - The number of frames in a single message MUST be less than or equal to `2^32 - 1`.
        if sequence_number == ENDFRAME_SEQUENCE_NUMBER {
            return Err("too many frames".into());
        }

        total_data_size += frame_length;
        //= ../specification/data-format/message-body.md#encrypted-content-length
        //= type=implication
        //# The length MUST NOT be greater than `2^36 - 32`, or 64 gibibytes (64 GiB),
        //# due to restrictions imposed by the [implemented algorithms](../framework/algorithm-suites.md).
        if total_data_size > MAX_DATA {
            return Err("Plain text too large".into());
        }
        iv_seq(sequence_number, &mut iv);

        body_aad(
            header.body.message_id(),
            //= ../specification/data-format/message-body-aad.md#body-aad-content
            //= type=implication
            //# - The [regular frames](message-body.md#regular-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Frame`.
            BodyAADContent::RegularFrame,
            //= ../specification/data-format/message-body-aad.md#sequence-number
            //= type=implication
            //# For [framed data](message-body.md#framed-data), the value of this field MUST be the [frame sequence number](message-body.md#sequence-number).
            sequence_number,
            //= ../specification/data-format/message-body-aad.md#content-length
            //= type=implication
            //# - For [framed data](message-body.md#framed-data), this value MUST equal the length, in bytes,
            //# of the plaintext being encrypted in this frame.

            //= ../specification/data-format/message-body-aad.md#content-length
            //= type=implication
            //# - For [regular frames](message-body.md#regular-frame), this value MUST equal the value of
            //# the [frame length](message-header.md#frame-length) field in the message header.
            frame_length as u64,
            &mut aad,
        );
        write_u32(&mut w, sequence_number)?;
        write_bytes(&mut w, &iv)?;
        //= ../specification/data-format/message-body.md#authentication-tag-1
        //= type=implication
        //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
        //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
        aes_encrypt(alg, &iv, key, &plaintext_frame, &aad, &mut w)?;
        write_bytes(out, &w)?;
        write_bytes(dw, &w)?;

        //= ../specification/data-format/message-body.md#sequence-number
        //= type=implication
        //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
        sequence_number += 1;
    }

    // Now process final frame

    total_data_size += in_size;
    if total_data_size > MAX_DATA {
        return Err("Plain text too large".into());
    }

    //= ../specification/data-format/message-body.md#final-frame
    //= type=implication
    //# The length of the plaintext to be encrypted in the Final Frame MUST be
    //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).

    //= ../specification/data-format/message-body.md#final-frame
    //= type=implication
    //# - When the length of the Plaintext is an exact multiple of the Frame Length
    //# (including if it is equal to the frame length),
    //# the Final Frame encrypted content length SHOULD be equal to the frame length but MAY be 0.

    //= ../specification/data-format/message-body.md#final-frame
    //= type=implication
    //# - When the length of the Plaintext is less than the Frame Length,
    //# the body MUST contain exactly one frame and that frame MUST be a Final Frame.

    // Final frame should not be empty, unless the whole plaintext was empty
    debug_assert!(in_size > 0 || sequence_number == START_SEQUENCE_NUMBER);
    debug_assert!(in_size <= frame_length);
    iv_seq(sequence_number, &mut iv);
    w.clear();

    body_aad(
        header.body.message_id(),
        //= ../specification/data-format/message-body-aad.md#body-aad-content
        //= type=implication
        //# - The [final frame](message-body.md#final-frame) in [framed data](message-body.md#framed-data) MUST use the value `AWSKMSEncryptionClient Final Frame`.
        BodyAADContent::FinalFrame,
        sequence_number,
        //= ../specification/data-format/message-body-aad.md#content-length
        //= type=implication
        //# - For the [final frame](message-body.md#final-frame), this value MUST be greater than or equal to
        //# 0 and less than or equal to the value of the [frame length](message-header.md#frame-length)
        //# field in the message header.
        in_size as u64,
        &mut aad,
    );

    //= ../specification/data-format/message-body.md#final-frame
    //= type=implication
    //# Framed data MUST contain exactly one final frame.

    //= ../specification/data-format/message-body.md#final-frame
    //= type=implication
    //# The final frame MUST be the last frame.
    write_u32(&mut w, ENDFRAME_SEQUENCE_NUMBER)?;
    //= ../specification/data-format/message-body.md#sequence-number-1
    //= type=implication
    //# The Final Frame Sequence number MUST be equal to the total number of frames in the Framed Data.
    write_u32(&mut w, sequence_number)?;
    write_bytes(&mut w, &iv)?;
    write_u32(&mut w, in_size as u32)?;
    //= ../specification/data-format/message-body.md#authentication-tag-2
    //= type=implication
    //# The authentication tag length MUST be equal to the authentication tag length of the algorithm suite
    //# specified by the [Algorithm Suite ID](message-header.md#algorithm-suite-id) field.
    aes_encrypt(alg, &iv, key, &plaintext_frame[0..in_size], &aad, &mut w)?;
    write_bytes(out, &w)?;
    write_bytes(dw, &w)?;

    Ok(())
}

pub(crate) fn get_ecdsa_alg(
    alg: aws_mpl_rs::suites::SignatureAlgorithm,
) -> Result<EcdsaSignatureAlgorithm, Error> {
    match alg {
        aws_mpl_rs::suites::SignatureAlgorithm::Ecdsa(x) => Ok(x),
        _ => Err("UnsupportedAlgorithm".into()),
    }
}

pub(crate) fn verify_signature(
    r: &mut dyn SafeRead,
    context: aws_mpl_primitives::DigestContext,
    dec_mat: aws_mpl_rs::DecryptionMaterials,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    if dec_mat.verification_key.is_none() {
        return Ok(());
    }

    let signature = read_seq_u16(r, raw)?;
    let ecdsa_params = get_ecdsa_alg(dec_mat.algorithm_suite.signature)?;
    let valid = ecdsa_verify_context(
        ecdsa_params,
        &dec_mat.verification_key.unwrap().0,
        context,
        &signature,
    )?;

    if !valid {
        return Err("InvalidSignature".into());
    }
    Ok(())
}

pub(crate) fn validate_encryption_context(ec: &EncryptionContext) -> Result<(), Error> {
    for key in ec.keys() {
        if key.starts_with(RESERVED_ENCRYPTION_CONTEXT) {
            return Err(
                "Encryption context keys cannot contain reserved prefix 'aws-crypto-'".into(),
            );
        }
    }
    Ok(())
}

//= ../specification/data-format/message-header.md#encrypted-data-key-count
//= type=implication
//# This value MUST be greater than 0.
pub(crate) fn validate_max_encrypted_data_keys(
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    edks: &[EncryptedDataKey],
) -> Result<(), Error> {
    if let Some(max) = max_encrypted_data_keys {
        if edks.len() > max.get() {
            return Err("Encrypted data keys exceed maxEncryptedDataKeys".into());
        }
        if edks.is_empty() {
            return Err("Encrypted data keys is empty.".into());
        }
    }

    Ok(())
}
/*
 * Generate a message id of appropriate length for the given algorithm suite.
 */
//= ../specification/data-format/message-header.md#message-id
//= type=implication
//# A Message ID MUST uniquely identify the [message](message.md).

//= ../specification/data-format/message-header.md#message-id
//= type=implication
//# While implementations cannot guarantee complete uniqueness,
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.

pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {
    let length = if suite.message_version == 1 {
        MESSAGE_ID_LEN_V1
    } else {
        MESSAGE_ID_LEN_V2
    };
    let mut rand_bytes: Vec<u8> = vec![0; length as usize];
    generate_random_bytes(&mut rand_bytes)?;
    Ok(rand_bytes)
}

pub(crate) fn build_header_for_encrypt(
    message_id: &MessageId,
    suite: &AlgorithmSuite,
    encryption_context: &EncryptionContext,
    required_encryption_context_keys: &[String],
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: u32,
    derived_data_keys: &key_derivation::ExpandedKeyMaterial,
) -> Result<HeaderInfo, Error> {
    let mut stored_encryption_context = encryption_context.clone();
    let mut required_encryption_context_map: EncryptionContext = EncryptionContext::new();
    for key in required_encryption_context_keys {
        if stored_encryption_context.contains_key(key) {
            required_encryption_context_map
                .insert(key.clone(), stored_encryption_context.remove(key).unwrap());
        }
    }
    let canonical_stored_encryption_context = to_canonical_pairs(stored_encryption_context);

    let body: HeaderBody = build_header_body(
        message_id,
        suite,
        &canonical_stored_encryption_context,
        encrypted_data_keys,
        frame_length,
        derived_data_keys.commitment_key.clone(),
    )?;

    let canonical_req_encryption_context = to_canonical_pairs(required_encryption_context_map);
    let mut serialized_req_encryption_context = Vec::new();
    write_empty_ec_or_write_aad(
        &mut serialized_req_encryption_context,
        &canonical_req_encryption_context,
    )?;

    let mut raw_header = Vec::new();
    write_header_body(&mut raw_header, &body)?;

    let header_auth = build_header_auth_tag(
        suite,
        &derived_data_keys.data_key,
        &raw_header,
        &serialized_req_encryption_context,
    )?;

    Ok(HeaderInfo {
        suite: suite.clone(),
        body,
        encryption_context: encryption_context.clone(),
        header_auth,
        raw_header,
    })
}

pub(crate) fn build_header_body(
    message_id: &MessageId,
    suite: &AlgorithmSuite,
    encryption_context: &ESDKCanonicalEncryptionContext,
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: u32,
    suite_data: Option<Vec<u8>>,
) -> Result<HeaderBody, Error> {
    match suite.commitment {
        aws_mpl_rs::suites::DerivationAlgorithm::Hkdf(h) => {
            if suite_data.is_none()
                || suite_data.as_ref().unwrap().len() != h.output_key_length as usize
            {
                return Err("Validation Error 1".into());
            }

            Ok(HeaderBody::V2Body(V2HeaderBody {
                algorithm_suite: suite.clone(),
                message_id: message_id.clone(),
                encryption_context: encryption_context.clone(),
                encrypted_data_keys: encrypted_data_keys.into(),
                content_type: ContentType::Framed,
                frame_length,
                suite_data: suite_data.unwrap(),
            }))
        }
        aws_mpl_rs::suites::DerivationAlgorithm::Identity => Err("Validation Error 2".into()),
        _ => Ok(HeaderBody::V1Body(V1HeaderBody {
            message_type: MessageType::TypeCustomerAed,
            algorithm_suite: suite.clone(),
            message_id: message_id.clone(),
            encryption_context: encryption_context.clone(),
            encrypted_data_keys: encrypted_data_keys.into(),
            content_type: ContentType::Framed,
            header_iv_length: u64::from(get_iv_length(suite)),
            frame_length,
        })),
    }
}

pub(crate) fn build_header_auth_tag(
    suite: &AlgorithmSuite,
    data_key: &[u8],
    raw_header: &[u8],
    serialized_req_encryption_context: &[u8],
) -> Result<HeaderAuth, Error> {
    let key_length = get_encrypt_key_length(suite);
    if data_key.len() != key_length as usize {
        return Err("Incorrect data key length".into());
    }

    let iv = vec![0; get_iv_length(suite) as usize];
    let mut auth_tag = Vec::new();
    aes_encrypt(
        get_encrypt(suite),
        &iv,
        data_key,
        &[],
        &[raw_header, serialized_req_encryption_context].concat(),
        &mut auth_tag,
    )?;

    Ok(HeaderAuth::AESMac {
        header_iv: iv,
        header_auth_tag: auth_tag,
    })
}

/*
 * Ensures that the suite data contained in the header of a message matches
 * the expected suite data
 */
pub(crate) fn validate_suite_data(
    suite: &AlgorithmSuite,
    header: &HeaderBody,
    expected_suite_data: &[u8],
) -> Result<(), Error> {
    if header.suite_data() != expected_suite_data {
        return Err("Commitment key does not match".into());
    }

    //= ../specification/data-format/message-header.md#algorithm-suite-data
    //= type=implication
    //# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
    //# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    if get_hkdf(&suite.commitment).output_key_length != expected_suite_data.len() as u32 {
        return Err("Commitment key is invalid".into());
    }

    Ok(())
}

pub(crate) fn read_and_decrypt_non_framed_message_body(
    r: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    //= ../specification/data-format/message-header.md#frame-length
    //= type=implication
    //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
    if header.body.frame_length() != 0 {
        return Err("Non-framed message contains non-zero frame length.".into());
    }
    let iv = read_vec(r, get_iv_length(&header.suite) as usize, raw)?;
    let enc_content = read_seq_u64_bounded(
        r,
        SAFE_MAX_ENCRYPT,
        "Frame exceeds AES-GCM cryptographic safety for a single key/iv.",
        raw,
    )?;
    let auth_tag = read_vec(r, get_tag_length(&header.suite) as usize, raw)?;
    let mut aad = Vec::new();
    body_aad(
        header.body.message_id(),
        //= ../specification/data-format/message-body-aad.md#body-aad-content
        //= type=implication
        //# - [Non-framed data](message-body.md#non-framed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
        BodyAADContent::SingleBlock,
        //= ../specification/data-format/message-body-aad.md#sequence-number
        //= type=implication
        //# For [non-framed data](message-body.md#non-framed-data), the value of this field MUST be `1`.
        NONFRAMED_SEQUENCE_NUMBER,
        //= ../specification/data-format/message-body-aad.md#content-length
        //= type=implication
        //# - For [non-framed data](message-body.md#non-framed-data), this value MUST equal the length, in bytes,
        //# of the plaintext data provided to the algorithm for encryption.
        enc_content.len() as u64,
        &mut aad,
    );

    let mut result: Vec<u8> = enc_content.clone();
    aws_mpl_primitives::aes_decrypt(
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

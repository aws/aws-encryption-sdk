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

pub(crate) fn encrypt_and_serialize(
    plaintext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    let frame_length = header.body.frame_length() as usize;
    let iv_len = get_iv_length(&header.suite) as usize;
    let auth_len = get_tag_length(&header.suite) as usize;
    let frame_len = frame_length + iv_len + auth_len + 4;
    let mut w = Vec::with_capacity(frame_len);
    write_bytes(&mut w, &header.raw_header)?;
    write_header_auth_tag(&mut w, &header.header_auth, &header.suite)?;
    write_bytes(out, &w)?;
    write_bytes(dw, &w)?;

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
        if sequence_number == ENDFRAME_SEQUENCE_NUMBER {
            return Err("too many frames".into());
        }
        iv_seq(sequence_number, &mut iv);

        body_aad2(
            header.body.message_id(),
            BodyAADContent::RegularFrame,
            sequence_number,
            frame_length as u64,
            &mut aad,
        );
        write_u32(&mut w, sequence_number)?;
        write_bytes(&mut w, &iv)?;
        aes_encrypt(alg, &iv, key, &plaintext_frame, &aad, &mut w)?;
        write_bytes(out, &w)?;
        write_bytes(dw, &w)?;

        sequence_number += 1;
    }
    // Final frame should not be empty, unless the whole plaintext was empty
    debug_assert!(in_size > 0 || sequence_number == START_SEQUENCE_NUMBER);
    iv_seq(sequence_number, &mut iv);
    w.clear();

    body_aad2(
        header.body.message_id(),
        BodyAADContent::FinalFrame,
        sequence_number,
        in_size as u64,
        &mut aad,
    );

    write_u32(&mut w, ENDFRAME_SEQUENCE_NUMBER)?;
    write_u32(&mut w, sequence_number)?;
    write_bytes(&mut w, &iv)?;
    write_u32(&mut w, in_size as u32)?;
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

pub(crate) fn validate_max_encrypted_data_keys(
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    edks: &[EncryptedDataKey],
) -> Result<(), Error> {
    if let Some(max) = max_encrypted_data_keys
        && edks.len() > max.get()
    {
        return Err("Encrypted data keys exceed maxEncryptedDataKeys".into());
    }

    Ok(())
}
/*
 * Generate a message id of appropriate length for the given algorithm suite.
 */
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
    let iv = read_vec(r, get_iv_length(&header.suite) as usize, raw)?;
    let enc_content = read_seq_u64_bounded(
        r,
        SAFE_MAX_ENCRYPT,
        "Frame exceeds AES-GCM cryptographic safety for a single key/iv.",
        raw,
    )?;
    let auth_tag = read_vec(r, get_tag_length(&header.suite) as usize, raw)?;
    let mut aad = Vec::new();
    body_aad2(
        header.body.message_id(),
        BodyAADContent::SingleBlock,
        NONFRAMED_SEQUENCE_NUMBER,
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

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
use crate::types::EncryptionContext;
use crate::types::{SafeRead, SafeWrite};

use aws_mpl_primitives::ecdsa_verify_context;
use aws_mpl_primitives::{EcdsaSignatureAlgorithm, aes_encrypt, generate_random_bytes};
use aws_mpl_rs::types::AlgorithmSuiteInfo;
use aws_mpl_rs::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef;

//= compliance/client-apis/encrypt.txt#2.4.6
//= type=implication
//# This
//# value MUST default to 4096 bytes.
pub(crate) const DEFAULT_FRAME_LENGTH: u32 = 4096;

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
    let alg = get_aes_alg(&header.suite);

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
    iv_seq(sequence_number, &mut iv);
    w.clear();

    body_aad2(
        header.body.message_id(),
        BodyAADContent::FinalFrame,
        sequence_number,
        //= compliance/client-apis/encrypt.txt#2.7.1
        //# o  For a final frame this MUST be the length of the remaining
        //# plaintext bytes which have not yet been encrypted, whose
        //# length MUST be equal to or less than the frame length.
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
    alg: &aws_mpl_rs::types::SignatureAlgorithm,
) -> Result<aws_mpl_rs::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm, Error> {
    match alg {
        aws_mpl_rs::types::SignatureAlgorithm::Ecdsa(x) => Ok(x.curve.unwrap()),
        _ => Err("UnsupportedAlgorithm".into()),
    }
}

pub(crate) const fn ecdsa_alg(
    alg: aws_mpl_rs::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
) -> EcdsaSignatureAlgorithm {
    match alg {
        aws_mpl_rs::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm::EcdsaP384 => {
            EcdsaSignatureAlgorithm::EcdsaP384
        }
        aws_mpl_rs::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm::EcdsaP256 => {
            EcdsaSignatureAlgorithm::EcdsaP256
        }
    }
}

pub(crate) fn verify_signature(
    r: &mut dyn SafeRead,
    context: aws_mpl_primitives::DigestContext,
    dec_mat: aws_mpl_rs::types::DecryptionMaterials,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    //= compliance/client-apis/decrypt.txt#2.7
    //= type=implication
    //# Otherwise this operation MUST NOT perform this
    //# step.
    if dec_mat.verification_key.is_none() {
        return Ok(());
    }

    //= compliance/client-apis/decrypt.txt#2.7.5
    //# If the algorithm suite has a signature algorithm, this operation MUST
    //# verify the message footer using the specified signature algorithm.

    //= compliance/client-apis/decrypt.txt#2.7
    //# ./framework/algorithm-
    //# suites.md#signature-algorithm), this operation MUST perform
    //# this step.

    //= compliance/client-apis/decrypt.txt#2.7.5
    //# After deserializing the body, this operation MUST deserialize the
    //# next encrypted message bytes as the message footer (../data-format/
    //# message-footer.md).

    let signature = read_seq_u16(r, raw)?;
    let ecdsa_params = get_ecdsa_alg(&dec_mat.algorithm_suite.unwrap().signature.unwrap())?;
    //= compliance/client-apis/decrypt.txt#2.7.5
    //# Once the message footer is deserialized, this operation MUST use the
    //# signature algorithm (../framework/algorithm-suites.md#signature-
    //# algorithm) from the algorithm suite (../framework/algorithm-
    //# suites.md) in the decryption materials to verify the encrypted
    //# message, with the following inputs:
    let valid = ecdsa_verify_context(
        ecdsa_alg(ecdsa_params),
        dec_mat.verification_key.unwrap().as_ref(),
        context,
        &signature,
    )?;

    if !valid {
        return Err("InvalidSignature".into());
    }
    Ok(())
}

pub(crate) fn validate_encryption_context(input: Option<&EncryptionContext>) -> Result<(), Error> {
    if let Some(ec) = input {
        for key in ec.keys() {
            if key.starts_with(RESERVED_ENCRYPTION_CONTEXT) {
                return Err(
                    "Encryption context keys cannot contain reserved prefix 'aws-crypto-'".into(),
                );
            }
        }
    }
    Ok(())
}

/*
 * Helper method for taking optional input keyrings/CMMs and returning a CMM,
 * either directly the one that was provided or a new default CMM from the
 * provided keyring.
 */
pub(crate) async fn create_cmm_from_input(
    mpl: &aws_mpl_rs::Client,
    input_cmm: Option<CryptographicMaterialsManagerRef>,
    input_keyring: Option<aws_mpl_rs::types::keyring::KeyringRef>,
) -> Result<CryptographicMaterialsManagerRef, Error> {
    match (input_cmm, input_keyring) {
        (Some(_cmm), Some(_keyring)) => Err("Cannot provide both a keyring and a CMM.".into()),
        (Some(cmm), _) => Ok(cmm),
        (None, None) => Err("Must provide either a keyring or a CMM".into()),
        (None, Some(keyring)) => {
            let cmm = mpl
                .create_default_cryptographic_materials_manager()
                .keyring(keyring)
                .send()
                .await?;
            Ok(cmm)
        }
    }
}

pub(crate) fn validate_max_encrypted_data_keys(
    max_encrypted_data_keys: Option<usize>,
    edks: &[aws_mpl_rs::types::EncryptedDataKey],
) -> Result<(), Error> {
    if let Some(max) = max_encrypted_data_keys
        && edks.len() > max
    {
        return Err("Encrypted data keys exceed maxEncryptedDataKeys".into());
    }

    Ok(())
}
/*
 * Generate a message id of appropriate length for the given algorithm suite.
 */
pub(crate) fn generate_message_id(suite: &AlgorithmSuiteInfo) -> Result<MessageId, Error> {
    let length = if suite.message_version.unwrap() == 1 {
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
    suite: &AlgorithmSuiteInfo,
    encryption_context: &EncryptionContext,
    required_encryption_context_keys: &[String],
    encrypted_data_keys: &ESDKEncryptedDataKeys,
    frame_length: u32,
    derived_data_keys: &key_derivation::ExpandedKeyMaterial,
) -> Result<HeaderInfo, Error> {
    //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-header
    //# - [AAD](../data-format/message-header.md#aad): MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //#  in the [encryption materials](../framework/structures.md#encryption-materials),
    //#  and this serialization MUST NOT contain any key value pairs listed in
    //#  the [encryption material's](../framework/structures.md#encryption-materials)
    //#  [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
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

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# Before encrypting input plaintext, this operation MUST serialize the
    //# message header body (../data-format/message-header.md).
    let mut raw_header = Vec::new();
    write_header_body(&mut raw_header, &body)?;

    //= compliance/client-apis/encrypt.txt#2.6.2
    //# After serializing the message header body, this operation MUST
    //# calculate an authentication tag (../data-format/message-
    //# header.md#authentication-tag) over the message header body.
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
    suite: &AlgorithmSuiteInfo,
    encryption_context: &ESDKCanonicalEncryptionContext,
    encrypted_data_keys: &ESDKEncryptedDataKeys,
    frame_length: u32,
    suite_data: Option<Vec<u8>>,
) -> Result<HeaderBody, Error> {
    //= compliance/client-apis/encrypt.txt#2.6.2
    //= type=implication
    //# If the algorithm suite has a commitment algorithm, this operation MUST
    //# include the suite data field in the header body.
    match suite.commitment.as_ref().unwrap() {
        aws_mpl_rs::types::DerivationAlgorithm::Hkdf(h) => {
            //= compliance/data-format/message-header.txt#2.5.2
            //= type=implication
            //# The length of the suite data field MUST be equal to
            //# the Algorithm Suite Data Length (../framework/algorithm-
            //# suites.md#algorithm-suite-data-length) value of the algorithm suite
            //# (../framework/algorithm-suites.md) specified by the Algorithm Suite
            //# ID (Section 2.5.1.5) field.
            if suite_data.is_none()
                || suite_data.as_ref().unwrap().len() != h.output_key_length.unwrap() as usize
            {
                return Err("Validation Error 1".into());
            }

            Ok(HeaderBody::V2Body(V2HeaderBody {
                algorithm_suite: suite.clone(),
                message_id: message_id.clone(),
                encryption_context: encryption_context.clone(),
                encrypted_data_keys: encrypted_data_keys.clone(),
                content_type: ContentType::Framed,
                frame_length,
                suite_data: suite_data.unwrap(),
            }))
        }
        aws_mpl_rs::types::DerivationAlgorithm::Identity(_i) => Err("Validation Error 2".into()),
        _ => Ok(HeaderBody::V1Body(V1HeaderBody {
            message_type: MessageType::TypeCustomerAed,
            algorithm_suite: suite.clone(),
            message_id: message_id.clone(),
            encryption_context: encryption_context.clone(),
            encrypted_data_keys: encrypted_data_keys.clone(),
            content_type: ContentType::Framed,
            header_iv_length: u64::from(get_iv_length(suite)),
            frame_length,
        })),
    }
}

pub(crate) fn build_header_auth_tag(
    suite: &AlgorithmSuiteInfo,
    data_key: &[u8],
    raw_header: &[u8],
    serialized_req_encryption_context: &[u8],
) -> Result<HeaderAuth, Error> {
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# The
    //# value of this MUST be the output of the authenticated encryption
    //# algorithm (../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the algorithm suite (../framework/algorithm-
    //# suites.md), with the following inputs:
    let key_length = get_encrypt_key_length(suite);
    if data_key.len() != key_length as usize {
        return Err("Incorrect data key length".into());
    }

    //= compliance/client-apis/encrypt.txt#2.6.2
    //#*  The IV has a value of 0.
    let iv = vec![0; get_iv_length(suite) as usize];
    let mut auth_tag = Vec::new();
    aes_encrypt(
        get_aes_alg(suite),
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

pub(crate) async fn get_encryption_materials(
    cmm: CryptographicMaterialsManagerRef,
    algorithm_suite_id: Option<aws_mpl_rs::types::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: i64,
    commitment_policy: aws_mpl_rs::types::EsdkCommitmentPolicy,
    mpl: &aws_mpl_rs::Client,
) -> Result<aws_mpl_rs::types::EncryptionMaterials, Error> {
    //= compliance/client-apis/encrypt.txt#2.6.1
    //# This operation MUST obtain this set of encryption
    //# materials (../framework/structures.md#encryption-materials) by
    //# calling Get Encryption Materials (../framework/cmm-interface.md#get-
    //# encryption-materials) on a CMM (../framework/cmm-interface.md).
    let output = cmm
        .get_encryption_materials()
        .encryption_context(encryption_context)
        .commitment_policy(aws_mpl_rs::types::CommitmentPolicy::Esdk(commitment_policy))
        .set_algorithm_suite_id(algorithm_suite_id)
        .max_plaintext_length(max_plaintext_length)
        .send()
        .await?;

    let materials = output.encryption_materials.unwrap();
    let return_materials = materials.clone();
    //= compliance/client-apis/encrypt.txt#2.6.1
    //# If this
    //# algorithm suite (../framework/algorithm-suites.md) is not supported
    //# by the commitment policy (client.md#commitment-policy) configured in
    //# the client (client.md) encrypt MUST yield an error.
    mpl.validate_commitment_policy_on_encrypt()
        .algorithm(
            materials
                .algorithm_suite
                .as_ref()
                .unwrap()
                .id
                .as_ref()
                .unwrap()
                .clone(),
        )
        .commitment_policy(aws_mpl_rs::types::CommitmentPolicy::Esdk(commitment_policy))
        .send()
        .await?;

    mpl.encryption_materials_has_plaintext_data_key()
        .set_algorithm_suite(materials.algorithm_suite.clone())
        .set_encryption_context(materials.encryption_context)
        .set_encrypted_data_keys(materials.encrypted_data_keys)
        .set_required_encryption_context_keys(materials.required_encryption_context_keys)
        .set_plaintext_data_key(materials.plaintext_data_key)
        .set_signing_key(materials.signing_key)
        .set_symmetric_signing_keys(materials.symmetric_signing_keys)
        .send()
        .await?;

    if !is_esdk_encryption_context(return_materials.encryption_context.as_ref().unwrap()) {
        return Err("CMM failed to return serializable encryption materials.".into());
    }
    if !is_esdk_encrypted_data_keys(return_materials.encrypted_data_keys.as_ref().unwrap()) {
        return Err("CMM failed to return serializable encrypted data keys.".into());
    }

    Ok(return_materials)
}

pub(crate) async fn get_decryption_materials(
    cmm: CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_rs::types::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_rs::types::EsdkCommitmentPolicy,
    mpl: &aws_mpl_rs::Client,
) -> Result<aws_mpl_rs::types::DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());
    //= compliance/client-apis/decrypt.txt#2.7.2
    //# ./framework/cmm-
    //# interface.md#decrypt-materials) operation MUST be constructed as
    //# follows:

    let output = cmm
        .decrypt_materials()
        //#*  Algorithm Suite ID: This is the parsed algorithm suite ID
        //#   (../data-format/message-header.md#algorithm-suite-id) from the
        //#   message header.
        .algorithm_suite_id(algorithm_suite_id)
        .commitment_policy(aws_mpl_rs::types::CommitmentPolicy::Esdk(commitment_policy))
        //#*  Encrypted Data Keys: This is the parsed encrypted data keys
        //#   (../data-format/message-header#encrypted-data-keys) from the
        //#   message header.
        .encrypted_data_keys(header_body.encrypted_data_keys().clone())
        //#*  Encryption Context: This is the parsed encryption context
        //#   (../data-format/message-header.md#aad) from the message header.
        .encryption_context(encryption_context)
        //#* Reproduced Encryption Context: This is the
        //# [input](#input) encryption context.
        .reproduced_encryption_context(reproduced_encryption_context.clone())
        .send()
        .await?;

    let materials = output.decryption_materials.unwrap();
    let return_materials = materials.clone();
    //= compliance/client-apis/decrypt.txt#2.7.2
    //# If the
    //# algorithm suite is not supported by the commitment policy
    //# (client.md#commitment-policy) configured in the client (client.md)
    //# decrypt MUST yield an error.
    mpl.validate_commitment_policy_on_decrypt()
        .algorithm(
            materials
                .algorithm_suite
                .as_ref()
                .unwrap()
                .id()
                .as_ref()
                .unwrap()
                .clone(),
        )
        .commitment_policy(aws_mpl_rs::types::CommitmentPolicy::Esdk(commitment_policy))
        .send()
        .await?;

    mpl.decryption_materials_with_plaintext_data_key()
        .set_algorithm_suite(materials.algorithm_suite.clone())
        .set_encryption_context(materials.encryption_context)
        .set_plaintext_data_key(materials.plaintext_data_key)
        .set_required_encryption_context_keys(materials.required_encryption_context_keys)
        .set_symmetric_signing_key(materials.symmetric_signing_key)
        .set_verification_key(materials.verification_key)
        .send()
        .await?;

    if !is_esdk_encryption_context(return_materials.encryption_context.as_ref().unwrap()) {
        return Err("CMM failed to return serializable encryption materials.".into());
    }
    Ok(return_materials)
}

/*
 * Ensures that the suite data contained in the header of a message matches
 * the expected suite data
 */
pub(crate) fn validate_suite_data(
    suite: &AlgorithmSuiteInfo,
    header: &HeaderBody,
    expected_suite_data: &[u8],
) -> Result<(), Error> {
    //= compliance/client-apis/decrypt.txt#2.7.2
    //# The derived commit key MUST equal the commit key stored in the message
    //# header.
    if header.suite_data() != expected_suite_data {
        return Err("Commitment key does not match".into());
    }

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# The length of the suite data field MUST be equal to
    //# the Algorithm Suite Data Length (../framework/algorithm-
    //# suites.md#algorithm-suite-data-length) value of the algorithm suite
    //# (../framework/algorithm-suites.md) specified by the Algorithm Suite
    //# ID (Section 2.5.1.5) field.
    #[allow(clippy::cast_possible_wrap)]
    if get_hkdf(suite.commitment.as_ref().unwrap()).output_key_length
        != Some(expected_suite_data.len() as i32)
    {
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
    //= compliance/client-apis/decrypt.txt#2.7.3
    //# The message header MUST be read and parsed as follows.

    //= compliance/client-apis/decrypt.txt#2.7.3
    //# The message body MUST be read and decrypted as follows.

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
        get_aes_alg(&header.suite),
        key,
        &enc_content,
        &auth_tag,
        &iv,
        &aad,
        result.as_mut(),
    )?;

    Ok(result)
}

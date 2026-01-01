// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// use crate::client::Client;
use crate::encrypt_decrypt;
use crate::error::Error;
use crate::key_derivation;
use crate::materials;
use crate::message_body;
use crate::serialize::header_types::ContentType;
use crate::serialize::serializable_types::{from_canonical_pairs, to_canonical_pairs};
use crate::serialize::serialize_functions::write_seq_u16;
use crate::serialize::*;
use crate::types::*;
use aws_mpl_primitives::*;
use aws_mpl_rs::commitment::EsdkCommitmentPolicy;

#[derive(Debug, PartialEq, Eq, Clone)]
enum ProtectionNeeded {
    /// Customer can see partial results, so multi-frame signed payloads forbidden
    Yes,
    /// Don't worry about multi-frame signed payloads
    No,
}
impl ProtectionNeeded {
    const fn yes(&self) -> bool {
        match self {
            Self::Yes => true,
            Self::No => false,
        }
    }

    // if overridden set to true, no safety needed
    const fn needs_protection(overridden: bool) -> Self {
        if overridden { Self::No } else { Self::Yes }
    }
}

// impl Client {
//     /// Decrypt slice into Vec
//     pub async fn encrypt(&self, input: &EncryptInput<'_>) -> Result<EncryptOutput, Error> {
//         encrypt(input).await
//     }
// }

/// Decrypt slice into Vec
pub async fn encrypt(input: &EncryptInput<'_>) -> Result<EncryptOutput, Error> {
    input.validate()?;

    let mut cursor: std::io::Cursor<&[u8]> = std::io::Cursor::new(input.plaintext);

    // calculate reasonable upper bound for ciphertext size, to minimize allocations.
    let frame_length_usize = input.frame_length.0.get() as usize;
    let frames = input.plaintext.len().div_ceil(frame_length_usize);
    let iv_len = 12_usize;
    let auth_len = 16_usize;
    let frame_len = frame_length_usize + iv_len + auth_len + 4;
    let header_overhead = 1024_usize;
    let total_size = frames * frame_len + header_overhead;

    let mut ciphertext: Vec<u8> = Vec::with_capacity(total_size);
    let out = internal_encrypt(
        &mut cursor,
        &mut ciphertext,
        Some(input.plaintext.len()),
        input.source.clone(),
        &input.encryption_context,
        input.algorithm_suite_id,
        input.frame_length,
        input.max_encrypted_data_keys,
        input.commitment_policy,
    )
    .await?;

    Ok(EncryptOutput {
        ciphertext,
        encryption_context: out.encryption_context,
        algorithm_suite_id: out.algorithm_suite_id,
    })
}

/// Encrypt dyn Read into dyn Write
pub async fn encrypt_stream(
    plaintext: &mut dyn SafeRead,
    ciphertext: &mut dyn SafeWrite,
    input: &EncryptStreamInput,
) -> Result<EncryptStreamOutput, Error> {
    input.validate()?;

    internal_encrypt(
        plaintext,
        ciphertext,
        input.data_size,
        input.source.clone(),
        &input.encryption_context,
        input.algorithm_suite_id,
        input.frame_length,
        input.max_encrypted_data_keys,
        input.commitment_policy,
    )
    .await
}

#[expect(clippy::too_many_arguments)]
async fn internal_encrypt(
    plaintext: &mut dyn SafeRead,
    ciphertext: &mut dyn SafeWrite,
    plaintext_len: Option<usize>,
    input_source: Option<MaterialSource>,
    encryption_context: &EncryptionContext,
    algorithm_suite_id: Option<aws_mpl_rs::suites::EsdkAlgorithmSuiteId>,
    frame_length: FrameLength,
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    commitment_policy: EsdkCommitmentPolicy,
) -> Result<EncryptStreamOutput, Error> {
    encrypt_decrypt::validate_encryption_context(encryption_context)?;

    let cmm = materials::create_cmm_from_input(input_source).await?;

    //= compliance/client-apis/encrypt.txt#2.4.5
    //# The algorithm suite (../framework/algorithm-suite.md) that SHOULD be
    //# used for encryption.
    let algorithm_suite_id = algorithm_suite_id.map(aws_mpl_rs::suites::AlgorithmSuiteId::Esdk);
    //= compliance/client-apis/encrypt.txt#2.6.1
    //# If an input algorithm suite (Section 2.4.5) is provided that is not
    //# supported by the commitment policy (client.md#commitment-policy)
    //# configured in the client (client.md) encrypt MUST yield an error.
    //
    //= compliance/client-apis/client.txt#2.4.2.1
    //# *  encrypt (encrypt.md) MUST only support algorithm suites that have
    //# a Key Commitment (../framework/algorithm-suites.md#algorithm-
    //# suites-encryption-key-derivation-settings) value of False
    //
    //= compliance/client-apis/client.txt#2.4.2.2
    //# *  encrypt (encrypt.md) MUST only support algorithm suites that have
    //# a Key Commitment (../framework/algorithm-suites.md#algorithm-
    //# suites-encryption-key-derivation-settings) value of True
    //
    //= compliance/client-apis/client.txt#2.4.2.3
    //# *  encrypt (encrypt.md) MUST only support algorithm suites that have
    //# a Key Commitment (../framework/algorithm-suites.md#algorithm-
    //# suites-encryption-key-derivation-settings) value of True
    if let Some(id) = algorithm_suite_id {
        let input = aws_mpl_rs::commitment::ValidateCommitmentPolicyOnEncryptInput::new(
            id,
            aws_mpl_rs::commitment::CommitmentPolicy::Esdk(commitment_policy),
        );
        aws_mpl_rs::commitment::validate_commitment_policy_on_encrypt(&input)?;
    }

    let materials = materials::get_encryption_materials(
        cmm,
        algorithm_suite_id,
        encryption_context.clone(),
        //= compliance/client-apis/encrypt.txt#2.6.1
        //# *  Max Plaintext Length: If the input plaintext (Section 2.4.1) has
        //# known length, this length MUST be used.
        plaintext_len,
        commitment_policy,
    )
    .await?;

    // if materials.algorithm_suite.as_ref().unwrap().id.is_none() {
    //     return Err(
    //         "Encryption materials contain incompatible algorithm suite for the AWS Encryption SDK."
    //             .into(),
    //     );
    // }

    //= compliance/client-apis/encrypt.txt#2.6.1
    //= type=implication
    //# If the number of
    //# encrypted data keys (../framework/structures.md#encrypted-data-keys)
    //# on the encryption materials (../framework/structures.md#encryption-
    //# materials) is greater than the maximum number of encrypted data keys
    //# (client.md#maximum-number-of-encrypted-data-keys) configured in the
    //# client (client.md) encrypt MUST yield an error.
    encrypt_decrypt::validate_max_encrypted_data_keys(
        max_encrypted_data_keys,
        &materials.encrypted_data_keys,
    )?;

    //= compliance/client-apis/encrypt.txt#2.6.1
    //# The algorithm suite (../framework/algorithm-suites.md) used in all
    //# aspects of this operation MUST be the algorithm suite in the
    //# encryption materials (../framework/structures.md#encryption-
    //# materials) returned from the Get Encryption Materials (../framework/
    //# cmm-interface.md#get-encryption-materials) call.
    let message_id = encrypt_decrypt::generate_message_id(&materials.algorithm_suite)?;

    // TODO Post-#619: Remove Net v4.0.0 references
    let derived_data_keys = key_derivation::derive_keys(
        &message_id,
        &materials.plaintext_data_key.unwrap().0, // TODO - can this be None?
        &materials.algorithm_suite,
        false,
    )?;

    let header = encrypt_decrypt::build_header_for_encrypt(
        &message_id,
        &materials.algorithm_suite,
        &materials.encryption_context,
        &materials.required_encryption_context_keys,
        &materials.encrypted_data_keys,
        frame_length.0.get(),
        &derived_data_keys,
    )?;
    let mut dw = DigestWriter::from_old_ecdsa(materials.algorithm_suite.signature)?;

    encrypt_decrypt::encrypt_and_serialize(
        plaintext,
        &header,
        &derived_data_keys.data_key,
        ciphertext,
        &mut dw,
    )?;
    let suite_id = get_esdk_id(header.suite.id)?;
    if let aws_mpl_rs::suites::SignatureAlgorithm::Ecdsa(_) = &header.suite.signature {
        let ecdsa_params = encrypt_decrypt::get_ecdsa_alg(header.suite.signature)?;
        let bytes = ecdsa_sign_digest(
            ecdsa_params,
            &materials.signing_key.unwrap().0,
            dw.context.unwrap(),
        )?;
        if bytes.len() >= u16::MAX.into() {
            return Err("Length of signature bytes is larger than the uint16 limit.".into());
        }
        write_seq_u16(ciphertext, bytes.as_ref())?;
    }

    Ok(EncryptStreamOutput {
        encryption_context: header.encryption_context,
        algorithm_suite_id: suite_id,
    })
}

fn get_esdk_id(
    id: aws_mpl_rs::suites::AlgorithmSuiteId,
) -> Result<aws_mpl_rs::suites::EsdkAlgorithmSuiteId, Error> {
    match id {
        aws_mpl_rs::suites::AlgorithmSuiteId::Esdk(x) => Ok(x),
        _ => Err("Unsupported algorithm suite".into()),
    }
}

/// Decrypt dyn Read into dyn Write
pub async fn decrypt_stream(
    ciphertext: &mut dyn SafeRead,
    plaintext: &mut dyn SafeWrite,
    input: &DecryptStreamInput,
) -> Result<DecryptStreamOutput, Error> {
    input.validate()?;

    internal_decrypt(
        ciphertext,
        plaintext,
        input.source.clone(),
        &input.encryption_context,
        input.net_v4_retry_policy,
        ProtectionNeeded::needs_protection(input.i_accept_the_danger),
        input.max_encrypted_data_keys,
        input.commitment_policy,
    )
    .await
}

/// Decrypt slice into Vec
pub async fn decrypt(input: &DecryptInput<'_>) -> Result<DecryptOutput, Error> {
    input.validate()?;

    //= compliance/client-apis/decrypt.txt#2.7.1
    //# Given encrypted message bytes, this operation MUST process those
    //# bytes sequentially, deserializing those bytes according to the
    //# message format (../data-format/message.md).

    let mut cursor: std::io::Cursor<&[u8]> = std::io::Cursor::new(input.ciphertext);
    let mut plaintext: Vec<u8> = Vec::with_capacity(input.ciphertext.len());
    let out = internal_decrypt(
        &mut cursor,
        &mut plaintext,
        input.source.clone(),
        &input.encryption_context,
        input.net_v4_retry_policy,
        ProtectionNeeded::No, // Customer cannot see any partial results
        input.max_encrypted_data_keys,
        input.commitment_policy,
    )
    .await?;

    if cursor.position() != input.ciphertext.len() as u64 {
        return Err("Data after message footer.".into());
    }

    Ok(DecryptOutput {
        plaintext,
        encryption_context: out.encryption_context,
        algorithm_suite_id: out.algorithm_suite_id,
    })
}

#[expect(clippy::too_many_arguments)]
async fn internal_decrypt(
    ciphertext: &mut dyn SafeRead,
    plaintext: &mut dyn SafeWrite,
    input_source: Option<MaterialSource>,
    encryption_context: &EncryptionContext,
    net_v4_retry_policy: NetV400RetryPolicy,
    safety_needed: ProtectionNeeded,
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    commitment_policy: EsdkCommitmentPolicy,
) -> Result<DecryptStreamOutput, Error> {
    let cmm = materials::create_cmm_from_input(input_source).await?;

    //= compliance/client-apis/decrypt.txt#2.5.1.1
    //= type=TODO
    //# To make diagnosing this mistake easier, implementations SHOULD detect
    //# the first two bytes of the Base64 encoding of any supported message
    //# versions (../data-format/message-header.md#version-1) and types
    //# (../data-format/message-header.md#type) and fail with a more specific
    //# error message.
    let mut raw_header = Vec::new();
    let header_body =
        header::read_header_body(ciphertext, max_encrypted_data_keys, &mut raw_header)?;

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# If the
    //# algorithm suite is not supported by the commitment policy
    //# (client.md#commitment-policy) configured in the client (client.md)
    //# decrypt MUST yield an error.
    aws_mpl_rs::commitment::validate_commitment_policy_on_decrypt(
        aws_mpl_rs::commitment::ValidateCommitmentPolicyOnDecryptInput::new(
            header_body.algorithm_suite().id,
            aws_mpl_rs::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    //= compliance/client-apis/decrypt.txt#2.5.2
    //# This CMM MUST obtain the decryption materials (../framework/
    //# structures.md#decryption-materials) required for decryption.

    //= compliance/client-apis/decrypt.txt#2.5.3
    //# This default CMM MUST obtain the decryption materials required for
    //# decryption.
    // TODO :: Consider removing "Default CMM MUST obtain" from spec.
    // It is redundant and hard to prove.

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# This operation MUST obtain this set of decryption materials
    //# (../framework/structures.md#decryption-materials), by calling Decrypt
    //# Materials (../framework/cmm-interface.md#decrypt-materials) on a CMM
    //# (../framework/cmm-interface.md).
    let dec_mat = materials::get_decryption_materials(
        cmm,
        header_body.algorithm_suite().id,
        &header_body,
        encryption_context,
        commitment_policy,
    )
    .await?;

    let suite = &dec_mat.algorithm_suite;

    if suite != header_body.algorithm_suite() {
        return Err(
            "Stored header algorithm suite does not match decryption algorithm suite.".into(),
        );
    }
    let mut dw = DigestWriter::from_old_ecdsa(suite.signature)?;
    serialize_functions::write_bytes(&mut dw, &raw_header)?;

    //= compliance/client-apis/decrypt.txt#2.4.2
    //# This operation MUST NOT release any unauthenticated plaintext or
    //# unauthenticated associated data.
    let header_auth = header_auth::read_header_auth_tag(ciphertext, suite, &mut dw)?;

    let derived_data_keys = key_derivation::derive_keys(
        header_body.message_id(),
        dec_mat.plaintext_data_key.as_ref().unwrap().as_bytes(),
        suite,
        false,
    )?;

    if !header::header_version_supports_commitment(suite, &header_body) {
        return Err("Invalid commitment values found in header body.".into());
    }
    if v2_header_body::has_hkdf(&suite.commitment) {
        encrypt_decrypt::validate_suite_data(
            suite,
            &header_body,
            derived_data_keys.commitment_key.as_ref().unwrap(),
        )?;
    }

    let header_encryption_context = from_canonical_pairs(header_body.encryption_context().clone());

    //#*  The encryption context to only authenticate MUST be the [encryption context]
    //#   (../framework/structures.md#encryption-context)
    //#   in the [decryption materials](../framework/structures.md#decryption-materials)
    //#   filtered to only contain key value pairs listed in
    //#   the [decryption material's](../framework/structures.md#decryption-materials)
    //#   [required encryption context keys]
    //#   (../framework/structures.md#required-encryption-context-keys-1)
    //#   serialized according to the [encryption context serialization specification]
    //#   (../framework/structures.md#serialization).
    let mut encryption_context_to_only_authenticate =
        build_encryption_context_to_only_authenticate(&dec_mat);

    let canonical_req_encryption_context =
        to_canonical_pairs(encryption_context_to_only_authenticate.clone());
    let mut serialized_req_encryption_context = Vec::new();
    encryption_context::write_empty_ec_or_write_aad(
        &mut serialized_req_encryption_context,
        &canonical_req_encryption_context,
    )?;

    let mut maybe_header_auth =
          //= compliance/client-apis/decrypt.txt#2.7.3
          //# Once a valid message header is deserialized and decryption materials
          //# are available, this operation MUST validate the message header body
          //# (../data-format/message-header.md#header-body) by using the
          //# authenticated encryption algorithm (../framework/algorithm-
          //# suites.md#encryption-algorithm) to decrypt with the following inputs:
    aes_decrypt(
        message_body::get_encrypt(suite),
        &derived_data_keys.data_key,
        &[],
        header_auth.header_auth_tag(),
        header_auth.header_iv(),
        &[&raw_header[..], &serialized_req_encryption_context[..]].concat(),
        &mut[]
    );

    // TODO Post-#619: Add to the ESDK Specification the following:
    // ESDK-NET v4.0.0 Header Auth Catch
    // This will catch the Header Auth failure,
    // The Retry MUST
    // calculate the HKDF without the Message ID in the info and
    // use EncryptionContext.WriteAAD to serialize the
    // the Canonical Required Encryption Context.

    if maybe_header_auth.is_err() && net_v4_retry_policy == NetV400RetryPolicy::AllowRetry {
        let derived_data_keys = key_derivation::derive_keys(
            header_body.message_id(),
            dec_mat.plaintext_data_key.as_ref().unwrap().as_bytes(),
            suite,
            true,
        )?;
        let mut serialized_req_encryption_context_v4 = Vec::new();
        encryption_context::write_aad(
            &mut serialized_req_encryption_context_v4,
            &canonical_req_encryption_context,
        )?;
        //= compliance/client-apis/decrypt.txt#2.7.3
        //# Once a valid message header is deserialized and decryption materials
        //# are available, this operation MUST validate the message header body
        //# (../data-format/message-header.md#header-body) by using the
        //# authenticated encryption algorithm (../framework/algorithm-
        //# suites.md#encryption-algorithm) to decrypt with the following inputs:
        maybe_header_auth = aes_decrypt(
            message_body::get_encrypt(suite),
            &derived_data_keys.data_key,
            &[],
            header_auth.header_auth_tag(),
            header_auth.header_iv(),
            &[&raw_header[..], &serialized_req_encryption_context_v4[..]].concat(),
            &mut [],
        );
    }
    //= compliance/client-apis/decrypt.txt#2.7.3
    //# If this tag verification fails, this operation MUST immediately halt
    //# and fail.
    maybe_header_auth?;
    let header = header::HeaderInfo {
        body: header_body,
        raw_header,
        encryption_context: header_encryption_context,
        suite: suite.clone(),
        header_auth,
    };

    let key = derived_data_keys.data_key;

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# Once the message header is successfully parsed, the next sequential
    //# bytes MUST be deserialized according to the message body spec
    //# (../data-format/message-body.md).

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# The content type (../data-format/message-header.md#content-type)
    //# field parsed from the message header above determines whether these
    //# bytes MUST be deserialized as framed data (../data-format/message-
    //# body.md#framed-data) or un-framed data (../data-format/message-
    //# body.md#un-framed-data).
    let last_frame = match header.body.content_type() {
        ContentType::NonFramed => {
            //= compliance/client-apis/decrypt.txt#2.7.4
            //# If this decryption fails, this operation MUST immediately halt and
            //# fail.
            encrypt_decrypt::read_and_decrypt_non_framed_message_body(
                ciphertext, &header, &key, &mut dw,
            )?
        }
        ContentType::Framed => {
            let fail_if_multi_frame = dec_mat.verification_key.is_some() && safety_needed.yes();

            //= compliance/client-apis/decrypt.txt#2.7.4
            //# If this decryption fails, this operation MUST immediately halt and
            //# fail.
            message_body::read_and_decrypt_framed_message_body(
                ciphertext,
                plaintext,
                &header,
                &key,
                &mut dw,
                fail_if_multi_frame,
            )?
        }
    };

    if dec_mat.verification_key.is_some() {
        //= compliance/client-apis/decrypt.txt#2.7.5
        //# If this verification is not successful, this operation MUST
        //# immediately halt and fail.
        let mut noop = NoopWriter;
        encrypt_decrypt::verify_signature(ciphertext, dw.context.unwrap(), dec_mat, &mut noop)?;
    }
    // now that we have verified the signature, we can write the last frame of data
    serialize_functions::write_bytes(plaintext, &last_frame)?;

    //= compliance/client-apis/decrypt.txt#2.7.1
    //# Until the header is verified (Section 2.7.3), this operation MUST NOT
    //# release any parsed information from the header.
    // Note that the header is verified above

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# This operation MUST NOT release any unauthenticated plaintext.

    //= compliance/client-apis/decrypt.txt#2.7
    //# If the input encrypted message is not being streamed (streaming.md)
    //# to this operation, all output MUST NOT be released until after these
    //# steps complete successfully.
    encryption_context_to_only_authenticate.extend(header.encryption_context);
    Ok(DecryptStreamOutput {
        encryption_context: encryption_context_to_only_authenticate,
        algorithm_suite_id: get_esdk_id(header.suite.id)?,
    })
}

// The encryption context to only authenticate MUST be
// the encryption context in the decryption materials filtered
// to only contain key value pairs listed
// in the decryption material's required encryption context keys.
// TODO Post-#619: Duvet this section
fn build_encryption_context_to_only_authenticate(
    dec_mat: &aws_mpl_rs::types::DecryptionMaterials,
) -> EncryptionContext {
    dec_mat
        .encryption_context
        .iter()
        .filter(|(k, _)| dec_mat.required_encryption_context_keys.contains(k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

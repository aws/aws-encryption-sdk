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
use aws_mpl_legacy::primitives::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
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
//= specification/client-apis/encrypt.md#plaintext
//# The plaintext to encrypt.
//# This MUST be a sequence of bytes.
pub async fn encrypt(input: &EncryptInput<'_>) -> Result<EncryptOutput, Error> {
    //= specification/client-apis/encrypt.md#input
    //# The following inputs to this behavior are REQUIRED:
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

    //= specification/client-apis/encrypt.md#output
    //# This behavior MUST output the following if the behavior is successful:
    //= specification/client-apis/encrypt.md#encrypted-message
    //# This MUST be a sequence of bytes
    //# and conform to the [message format specification](../data-format/message.md).
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
//= specification/client-apis/encrypt.md#encryption-context
//# See [encryption context](../framework/structures.md#encryption-context).
async fn internal_encrypt(
    plaintext: &mut dyn SafeRead,
    ciphertext: &mut dyn SafeWrite,
    plaintext_len: Option<usize>,
    input_source: Option<MaterialSource>,
    encryption_context: &EncryptionContext,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::EsdkAlgorithmSuiteId>,
    frame_length: FrameLength,
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    commitment_policy: EsdkCommitmentPolicy,
) -> Result<EncryptStreamOutput, Error> {
    //= specification/client-apis/encrypt.md#behavior
    //# This operation MUST perform all the above steps unless otherwise specified,
    //# and it MUST perform them in the above order.
    //= specification/client-apis/encrypt.md#encryption-context
    //# If the input encryption context contains any entries with a key beginning with this prefix,
    //# the encryption operation MUST fail.
    encrypt_decrypt::validate_encryption_context(encryption_context)?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The CMM used MUST be the input CMM, if supplied.
    let cmm = materials::create_cmm_from_input(input_source).await?;

    let algorithm_suite_id = algorithm_suite_id.map(aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk);
    //= specification/client-apis/encrypt.md#algorithm-suite
    //# The [algorithm suite](../framework/algorithm-suites.md) that SHOULD be used for encryption.
    if let Some(id) = algorithm_suite_id {
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# If an [input algorithm suite](#algorithm-suite) is provided
        //# that is not supported by the [commitment policy](client.md#commitment-policy)
        //# configured in the [client](client.md) encrypt MUST yield an error.
        let input = aws_mpl_legacy::commitment::ValidateCommitmentPolicyOnEncryptInput::new(
            id,
            aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy),
        );
        aws_mpl_legacy::commitment::validate_commitment_policy_on_encrypt(&input)?;
    }

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# This operation MUST obtain this set of [encryption materials](../framework/structures.md#encryption-materials)
    //# by calling [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) on a [CMM](../framework/cmm-interface.md).
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The call to [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials)
    //# on that CMM MUST be constructed as follows:
    let materials = materials::get_encryption_materials(
        cmm,
        algorithm_suite_id,
        encryption_context.clone(),
        plaintext_len,
        commitment_policy,
    )
    .await?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The [algorithm suite](../framework/algorithm-suites.md) used in all aspects of this operation
    //# MUST be the algorithm suite in the [encryption materials](../framework/structures.md#encryption-materials)
    //# returned from the [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) call.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys) on the [encryption materials](../framework/structures.md#encryption-materials)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# encrypt MUST yield an error.
    encrypt_decrypt::validate_max_encrypted_data_keys(
        max_encrypted_data_keys,
        &materials.encrypted_data_keys,
    )?;

    let message_id = encrypt_decrypt::generate_message_id(&materials.algorithm_suite)?;

    // TODO Post-#619: Remove Net v4.0.0 references
    let derived_data_keys = key_derivation::derive_keys(
        &message_id,
        &materials.plaintext_data_key.unwrap().0, // TODO - can this be None?
        &materials.algorithm_suite,
        false,
    )?;

    //= specification/client-apis/encrypt.md#construct-the-header
    //# Before encrypting input plaintext,
    //# this operation MUST serialize the [message header body](../data-format/message-header.md).
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

    //= specification/client-apis/encrypt.md#construct-the-body
    //# The encrypted message output by this operation MUST have a message body equal
    //# to the message body calculated in this step.
    encrypt_decrypt::encrypt_and_serialize(
        plaintext,
        &header,
        &derived_data_keys.data_key,
        ciphertext,
        &mut dw,
    )?;
    let suite_id = get_esdk_id(header.suite.id)?;
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# If the [algorithm suite](../framework/algorithm-suites.md) contains a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# this operation MUST calculate a signature over the message,
    //# and the output [encrypted message](#encrypted-message) MUST contain a [message footer](../data-format/message-footer.md).
    if let aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(_) = &header.suite.signature {
        let ecdsa_params = encrypt_decrypt::get_ecdsa_alg(header.suite.signature)?;
        //= specification/client-apis/encrypt.md#construct-the-signature
        //# To calculate a signature, this operation MUST use the [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
        //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following input:
        let bytes = ecdsa_sign_digest(
            ecdsa_params,
            &materials.signing_key.unwrap().0,
            //= specification/data-format/message-footer.md#signature
                //# This signature MUST be calculated over both the [message header](message-header.md) and the [message body](message-body.md),
            //# in the order of serialization.
            dw.context.unwrap(),
        )?;
        if bytes.len() >= u16::MAX.into() {
            return Err("Length of signature bytes is larger than the uint16 limit.".into());
        }
        //= specification/client-apis/encrypt.md#construct-the-signature
        //# This operation MUST then serialize a message footer with the following specifics:
        //= specification/client-apis/encrypt.md#construct-the-signature
        //# - [Signature Length](../data-format/message-footer.md#signature-length): MUST be the length of the
        //# output of the calculation above.
        //= specification/client-apis/encrypt.md#construct-the-signature
        //# - [Signature](../data-format/message-footer.md#signature): MUST be the output of the calculation above.
        //= specification/data-format/message.md#structure
        //# If the [message header](message-header.md) contains an [algorithm suite](../framework/algorithm-suites.md) in the
        //# [algorithm suite ID](message-header.md#algorithm-suite-id) field that contains a
        //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm), the message MUST also contain a
        //# [message footer](message-footer.md), serialized after the [message body](message-body.md).
        //= specification/client-apis/encrypt.md#construct-the-signature
        //# The encrypted message output by this operation MUST have a message footer equal
        //# to the message footer calculated in this step.
        write_seq_u16(ciphertext, bytes.as_ref())?;
    }

    //= specification/client-apis/encrypt.md#output
    //# This behavior MUST output the following if the behavior is successful:
    Ok(EncryptStreamOutput {
        encryption_context: header.encryption_context,
        algorithm_suite_id: suite_id,
    })
}

//= specification/data-format/message-header.md#algorithm-suite-id
//# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
fn get_esdk_id(
    id: aws_mpl_legacy::suites::AlgorithmSuiteId,
) -> Result<aws_mpl_legacy::suites::EsdkAlgorithmSuiteId, Error> {
    match id {
        aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk(x) => Ok(x),
        _ => Err("Unsupported algorithm suite".into()),
    }
}

/// Decrypt dyn Read into dyn Write
pub async fn decrypt_stream(
    ciphertext: &mut dyn SafeRead,
    plaintext: &mut dyn SafeWrite,
    input: &DecryptStreamInput,
) -> Result<DecryptStreamOutput, Error> {
    //= specification/client-apis/decrypt.md#input
    //# The client MUST require the following as inputs to this operation:
    //# - [Encrypted Message](#encrypted-message)
    //# The client MUST require exactly one of the following types of inputs:
    //# - [Cryptographic Materials Manager (CMM)](../framework/cmm-interface.md)
    //# - [Keyring](../framework/keyring-interface.md)
    //# The following inputs to this behavior MUST be OPTIONAL:
    //# - [Encryption Context](#encryption-context)
    input.validate()?;

    internal_decrypt(
        ciphertext,
        plaintext,
        input.source.clone(),
        &input.encryption_context,
        input.net_v4_retry_policy,
        ProtectionNeeded::needs_protection(input.allow_unsafe_unauthenticated_plaintext_read),
        input.max_encrypted_data_keys,
        input.commitment_policy,
    )
    .await
}

/// Decrypt slice into Vec
pub async fn decrypt(input: &DecryptInput<'_>) -> Result<DecryptOutput, Error> {
    //= specification/client-apis/decrypt.md#behavior
    //# If the input encrypted message is not being [streamed](streaming.md) to this operation,
    //# all output MUST NOT be released until after these steps complete successfully.
    //= specification/client-apis/decrypt.md#input
    //# The client MUST require the following as inputs to this operation:
    input.validate()?;
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

    //= specification/client-apis/decrypt.md#behavior
    //# - If this operation successfully completes the above steps
    //# but there are consumable bytes which are intended to be decrypted,
    //# this operation MUST fail.
    if cursor.position() != input.ciphertext.len() as u64 {
        return Err("Data after message footer.".into());
    }

    //= specification/client-apis/decrypt.md#output
    //# The client MUST return as output to this operation:
    //# - [Plaintext](#plaintext)
    //# - [Encryption Context](#encryption-context)
    //# - [Algorithm Suite](#algorithm-suite)
    Ok(DecryptOutput {
        plaintext,
        encryption_context: out.encryption_context,
        algorithm_suite_id: out.algorithm_suite_id,
        //= specification/client-apis/decrypt.md#output
        //= type=exception
        //= reason=Parsed header is not spec'ed out; this is a SHOULD, not a MUST
        //# The client SHOULD return as an output:
        //# - [Parsed Header](#parsed-header)
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
    //= specification/client-apis/decrypt.md#behavior
    //# This operation MUST perform all the above steps unless otherwise specified,
    //# and it MUST perform them in the above order.
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The CMM used MUST be the input CMM, if supplied.
    //# If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM](../framework/default-cmm.md)
    //# from the input [keyring](../framework/keyring-interface.md).
    let cmm = materials::create_cmm_from_input(input_source).await?;
    let mut raw_header = Vec::new();
    //= specification/client-apis/decrypt.md#parse-the-header
    //# Given encrypted message bytes, this operation MUST process those bytes sequentially,
    //# deserializing those bytes according to the [message format](../data-format/message.md).
    let header_body =
        header::read_header_body(ciphertext, max_encrypted_data_keys, &mut raw_header)?;
    //= specification/client-apis/decrypt.md#parse-the-header
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys)
    //# deserialized from the [message header](../data-format/message-header.md)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md),
    //# then as soon as that can be determined during deserializing
    //# decrypt MUST process no more bytes and yield an error.
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the parsed [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    aws_mpl_legacy::commitment::validate_commitment_policy_on_decrypt(
        aws_mpl_legacy::commitment::ValidateCommitmentPolicyOnDecryptInput::new(
            header_body.algorithm_suite().id,
            aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    let dec_mat = materials::get_decryption_materials(
        cmm,
        header_body.algorithm_suite().id,
        &header_body,
        encryption_context,
        commitment_policy,
    )
    .await?;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The algorithm suite used as input for all decryption described below is a algorithm suite
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The algorithm suite used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) associated with
    //# the returned decryption materials.
    let suite = &dec_mat.algorithm_suite;

    if suite != header_body.algorithm_suite() {
        return Err(
            "Stored header algorithm suite does not match decryption algorithm suite.".into(),
        );
    }
    let mut dw = DigestWriter::from_old_ecdsa(suite.signature)?;
    serialize_functions::write_bytes(&mut dw, &raw_header)?;

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
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the [algorithm suite](../framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) supports [key commitment](../framework/algorithm-suites.md#key-commitment)
    //# then the [commit key](../framework/algorithm-suites.md#commit-key) MUST be derived from the plaintext data key
    //# using the [commit key derivation](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    if v2_header_body::has_hkdf(&suite.commitment) {
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# The derived commit key MUST equal the commit key stored in the message header.
        encrypt_decrypt::validate_suite_data(
            suite,
            &header_body,
            derived_data_keys.commitment_key.as_ref().unwrap(),
        )?;
    }

    let header_encryption_context = from_canonical_pairs(header_body.encryption_context().clone());

    //= specification/client-apis/decrypt.md#verify-the-header
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [decryption materials](../framework/structures.md#decryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [decryption material's](../framework/structures.md#decryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys-1)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    let mut encryption_context_to_only_authenticate =
        build_encryption_context_to_only_authenticate(&dec_mat);

    let canonical_req_encryption_context =
        to_canonical_pairs(encryption_context_to_only_authenticate.clone());
    let mut serialized_req_encryption_context = Vec::new();
    encryption_context::write_empty_ec_or_write_aad(
        &mut serialized_req_encryption_context,
        &canonical_req_encryption_context,
    )?;

    //= specification/client-apis/decrypt.md#verify-the-header
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.
    //= specification/client-apis/decrypt.md#verify-the-header
    //# Once a valid message header is deserialized and decryption materials are available,
    //# this operation MUST validate the [message header body](../data-format/message-header.md#header-body)
    //# by using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# to decrypt with the following inputs:
    let mut maybe_header_auth = aes_decrypt(
        message_body::get_encrypt(suite),
        &derived_data_keys.data_key,
        &[],
        header_auth.header_auth_tag(),
        header_auth.header_iv(),
        &[&raw_header[..], &serialized_req_encryption_context[..]].concat(),
        &mut [],
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
    //= specification/client-apis/decrypt.md#verify-the-header
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    maybe_header_auth?;
    //= specification/client-apis/decrypt.md#verify-the-header
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    let header = header::HeaderInfo {
        body: header_body,
        raw_header,
        encryption_context: header_encryption_context,
        suite: suite.clone(),
        header_auth,
    };

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The data key used as input for all decryption described below is a data key derived from the plaintext data key
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    let key = derived_data_keys.data_key;

    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# Once the message header is successfully parsed, the next sequential bytes
    //# MUST be deserialized according to the [message body spec](../data-format/message-body.md).
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# The [content type](../data-format/message-header.md#content-type) field parsed from the
    //# message header above determines whether these bytes MUST be deserialized as
    //# [framed data](../data-format/message-body.md#framed-data) or
    //# [un-framed data](../data-format/message-body.md#un-framed-data).
    let last_frame = match header.body.content_type() {
        ContentType::NonFramed => encrypt_decrypt::read_and_decrypt_non_framed_message_body(
            ciphertext, &header, &key, &mut dw,
        )?,
        ContentType::Framed => {
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
                //# Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
            //# a final frame MUST NOT be released until [signature verification](#verify-the-signature)
            //# successfully completes.
            let fail_if_multi_frame = dec_mat.verification_key.is_some() && safety_needed.yes();
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

    //= specification/client-apis/decrypt.md#verify-the-signature
    //# If the algorithm suite has a signature algorithm,
    //# this operation MUST verify the message footer using the specified signature algorithm.
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# After deserializing the body, this operation MUST deserialize the next encrypted message bytes
    //# as the [message footer](../data-format/message-footer.md).
    if dec_mat.verification_key.is_some() {
        let mut noop = NoopWriter;
        //= specification/client-apis/decrypt.md#verify-the-signature
        //# Once the message footer is deserialized, this operation MUST use the
        //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
        //# from the [algorithm suite](../framework/algorithm-suites.md) in the decryption materials to
        //# verify the encrypted message, with the following inputs:
        //= specification/client-apis/decrypt.md#verify-the-signature
        //# If this verification is not successful, this operation MUST immediately halt and fail.
        encrypt_decrypt::verify_signature(ciphertext, dw.context.unwrap(), dec_mat, &mut noop)?;
    }
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
    //# a final frame MUST NOT be released until [signature verification](#verify-the-signature)
    //# successfully completes.
    // now that we have verified the signature, we can write the last frame of data
    serialize_functions::write_bytes(plaintext, &last_frame)?;

    encryption_context_to_only_authenticate.extend(header.encryption_context);
    //= specification/client-apis/decrypt.md#output
    //# The client MUST return as output to this operation:
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
    dec_mat: &aws_mpl_legacy::DecryptionMaterials,
) -> EncryptionContext {
    dec_mat
        .encryption_context
        .iter()
        .filter(|(k, _)| dec_mat.required_encryption_context_keys.contains(k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

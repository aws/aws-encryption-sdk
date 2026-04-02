// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Decrypt operation — maps to client-apis/decrypt.md

use crate::encrypt::get_esdk_id;
use crate::error::Error;
use crate::key_derivation;
use crate::materials;
use crate::message::header_types::ContentType;
use crate::message::serializable_types::{from_canonical_pairs, to_canonical_pairs};
use crate::message::*;
use crate::types::*;
use aws_mpl_legacy::primitives::*;
//= specification/client-apis/client.md#commitment-policy
//= type=implication
//# The AWS Encryption SDK MUST use the ESDK [commitment policies](../framework/commitment-policy.md) defined in the Material Providers Library.
//= specification/client-apis/client.md#initialization
//# If no [commitment policy](#commitment-policy) is provided the default MUST be [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../framework/algorithm-suites.md#require_encrypt_require_decrypt).
//= specification/client-apis/client.md#initialization
//= type=implication
//# Once a [commitment policy](#commitment-policy) has been set it SHOULD be immutable.
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

/// Decrypt dyn Read into dyn Write
pub async fn decrypt_stream(
    ciphertext: &mut dyn SafeRead,
    plaintext: &mut dyn SafeWrite,
    input: &DecryptStreamInput,
) -> Result<DecryptStreamOutput, Error> {
    input.validate()?;

    //= specification/client-apis/decrypt.md#security-considerations
    //# If this operation is [streaming](streaming.md) output to the caller
    //# and is decrypting messages created with an algorithm suite including a signature algorithm,
    //# any released plaintext MUST NOT be considered signed data until this operation finishes
    //# successfully.
    //= specification/client-apis/decrypt.md#behavior
    //# - The ESDK MUST provide a configuration option that causes the decryption operation
    //# to fail immediately after parsing the header if a signed algorithm suite is used.
    let safety = ProtectionNeeded::needs_protection(input.i_accept_the_danger);

    let result = internal_decrypt(
        ciphertext,
        plaintext,
        input.source.clone(),
        &input.encryption_context,
        input.net_v4_retry_policy,
        safety,
        input.max_encrypted_data_keys,
        input.commitment_policy,
    )
    .await;

    //= specification/client-apis/decrypt.md#security-considerations
    //# This means that callers that process such released plaintext MUST NOT consider any processing successful
    //# until this operation completes successfully.
    //= specification/client-apis/decrypt.md#security-considerations
    //# Additionally, if this operation fails, callers MUST discard the released plaintext and encryption context
    //# and MUST rollback any processing done due to the released plaintext or encryption context.
    result
}

//= specification/client-apis/client.md#decrypt
//# The AWS Encryption SDK Client MUST provide an [decrypt](./decrypt.md#input) function
//# that adheres to [decrypt](./decrypt.md).
/// Decrypt slice into Vec
pub async fn decrypt(input: &DecryptInput<'_>) -> Result<DecryptOutput, Error> {
    input.validate()?;
    let mut cursor: std::io::Cursor<&[u8]> = std::io::Cursor::new(input.ciphertext);
    let mut plaintext: Vec<u8> = Vec::with_capacity(input.ciphertext.len());
    //= specification/client-apis/decrypt.md#behavior
    //# If the input encrypted message is not being [streamed](streaming.md) to this operation,
    //# all output MUST NOT be released until after these steps complete successfully.
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

    Ok(DecryptOutput {
        //= specification/client-apis/decrypt.md#output
        //# - The output of the Decrypt operation MUST include a [Plaintext](#plaintext) value.
        plaintext,
        //= specification/client-apis/decrypt.md#output
        //# - The output of the Decrypt operation MUST include an [encryption context](#encryption-context) value.
        encryption_context: out.encryption_context,
        //= specification/client-apis/decrypt.md#output
        //# - The output of the Decrypt operation MUST include an [algorithm suite](#algorithm-suite) value.
        algorithm_suite_id: out.algorithm_suite_id,
        //= specification/client-apis/decrypt.md#output
        //= type=exception
        //= reason=Parsed header is not spec'ed out; this is a SHOULD, not a MUST
        //# - The output of the Decrypt operation SHOULD include a [Parsed Header](#parsed-header) value.
    })
}

/// Intermediate state passed between decrypt steps.
struct DecryptState {
    header: header::HeaderInfo,
    dec_mat: aws_mpl_legacy::DecryptionMaterials,
    derived_data_keys: key_derivation::ExpandedKeyMaterial,
    dw: DigestWriter,
    encryption_context_to_only_authenticate: EncryptionContext,
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
    //# - Output MUST NOT be released until otherwise indicated.
    //= specification/client-apis/decrypt.md#behavior
    //# - Decrypt operation Step 1 MUST be [Parse the header](#parse-the-header)
    //= specification/client-apis/decrypt.md#behavior
    //# - If all bytes have been provided and this operation
    //# is unable to complete the above steps with the consumable encrypted message bytes,
    //# this operation MUST halt and indicate a failure to the caller.
    let (header_body, raw_header) = step_parse_header(ciphertext, max_encrypted_data_keys)?;

    //= specification/client-apis/decrypt.md#behavior
    //# - Decrypt operation Step 2 MUST be [Get the decryption materials](#get-the-decryption-materials)
    let state = step_get_decryption_materials(
        ciphertext,
        &header_body,
        raw_header,
        input_source,
        encryption_context,
        commitment_policy,
    )
    .await?;

    //= specification/client-apis/decrypt.md#behavior
    //# - Decrypt operation Step 3 MUST be [Verify the header](#verify-the-header)
    //= specification/client-apis/decrypt.md#parse-the-header
    //# Until the [header is verified](#verify-the-header), this operation MUST NOT
    //# release any parsed information from the header.
    let mut state = step_verify_header(state, net_v4_retry_policy)?;

    //= specification/client-apis/decrypt.md#behavior
    //# - Decrypt operation Step 4 MUST be [Decrypt the message body](#decrypt-the-message-body)
    let last_frame =
        step_decrypt_body(ciphertext, plaintext, &mut state, &safety_needed)?;

    //= specification/client-apis/decrypt.md#behavior
    //# - Decrypt operation Step 5 MUST be [Verify the signature](#verify-the-signature)
    //= specification/client-apis/decrypt.md#behavior
    //# - If the message header contains an algorithm suite including a
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Decrypt operation MUST perform this step.
    step_verify_signature(ciphertext, &state)?;

    // now that we have verified the signature, we can write the last frame of data
    //= specification/client-apis/decrypt.md#authenticated-data
    //# This operation MUST NOT release any unauthenticated plaintext or unauthenticated associated data.
    serialize_functions::write_bytes(plaintext, &last_frame)?;

    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
    //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
    //# successfully completes.

    let mut ec = state.encryption_context_to_only_authenticate;
    ec.extend(state.header.encryption_context);
    Ok(DecryptStreamOutput {
        encryption_context: ec,
        //= specification/client-apis/decrypt.md#algorithm-suite
        //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
        algorithm_suite_id: get_esdk_id(state.header.suite.id)?,
    })
}

// Step 1: Parse the header
fn step_parse_header(
    ciphertext: &mut dyn SafeRead,
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
) -> Result<(header_types::HeaderBody, Vec<u8>), Error> {
    let mut raw_header = Vec::new();
    let header_body =
        header::read_header_body(ciphertext, max_encrypted_data_keys, &mut raw_header)?;

    Ok((header_body, raw_header))
}

// Step 2: Get the decryption materials
async fn step_get_decryption_materials(
    ciphertext: &mut dyn SafeRead,
    header_body: &header_types::HeaderBody,
    raw_header: Vec<u8>,
    input_source: Option<MaterialSource>,
    encryption_context: &EncryptionContext,
    commitment_policy: EsdkCommitmentPolicy,
) -> Result<DecryptState, Error> {
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The CMM used MUST be the input CMM, if supplied.
    //# If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM](../framework/default-cmm.md)
    //# from the input [keyring](../framework/keyring-interface.md).
    let cmm = materials::create_cmm_from_input(input_source).await?;

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

    let dec_mat = materials::get_decryption_materials(
        cmm,
        header_body.algorithm_suite().id,
        header_body,
        encryption_context,
        commitment_policy,
    )
    .await?;

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The algorithm suite used as input for all decryption described below MUST be the algorithm suite
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

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The data key used as input for all decryption described below MUST be a data key derived from the plaintext data key
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    let derived_data_keys = key_derivation::derive_keys(
        header_body.message_id(),
        dec_mat.plaintext_data_key.as_ref().unwrap().as_bytes(),
        suite,
        false,
    )?;

    if !header::header_version_supports_commitment(suite, header_body) {
        return Err("Invalid commitment values found in header body.".into());
    }
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the [algorithm suite](../framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) supports [key commitment](../framework/algorithm-suites.md#key-commitment)
    //# then the [commit key](../framework/algorithm-suites.md#commit-key) MUST be derived from the plaintext data key
    //# using the [commit key derivation](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    if v2_header_body::has_hkdf(&suite.commitment) {
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# The derived commit key MUST equal the commit key stored in the message header.
        header::validate_suite_data(
            suite,
            header_body,
            derived_data_keys.commitment_key.as_ref().unwrap(),
        )?;
    }

    let header_encryption_context = from_canonical_pairs(header_body.encryption_context().clone());
    let encryption_context_to_only_authenticate =
        build_encryption_context_to_only_authenticate(&dec_mat);

    let header = header::HeaderInfo {
        body: header_body.clone(),
        raw_header,
        encryption_context: header_encryption_context,
        suite: suite.clone(),
        header_auth,
    };

    Ok(DecryptState {
        header,
        dec_mat,
        derived_data_keys,
        dw,
        encryption_context_to_only_authenticate,
    })
}

// Step 3: Verify the header
fn step_verify_header(
    mut state: DecryptState,
    net_v4_retry_policy: NetV400RetryPolicy,
) -> Result<DecryptState, Error> {
    //= specification/client-apis/decrypt.md#verify-the-header
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [decryption materials](../framework/structures.md#decryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [decryption material's](../framework/structures.md#decryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys-1)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    let canonical_req_encryption_context =
        to_canonical_pairs(state.encryption_context_to_only_authenticate.clone());
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
    //= specification/client-apis/decrypt.md#verify-the-header
    //# - the cipherkey MUST be the derived data key
    //= specification/client-apis/decrypt.md#verify-the-header
    //# - the ciphertext MUST be an empty byte array
    //= specification/client-apis/decrypt.md#verify-the-header
    //# - the tag MUST be the value serialized in the message header's
    //# [authentication tag field](../data-format/message-header.md#authentication-tag)
    //= specification/client-apis/decrypt.md#verify-the-header
    //# - For message format version [1.0](../data-format/message-header.md#supported-versions)
    //# the IV MUST be the value serialized in the message header's [IV field](../data-format/message-header#iv).
    //= specification/client-apis/decrypt.md#verify-the-header
    //# For message format version [2.0](../data-format/message-header.md#supported-versions)
    //# the IV MUST be 0.
    let mut maybe_header_auth = aes_decrypt(
        body::get_encrypt(&state.header.suite),
        &state.derived_data_keys.data_key,
        &[],
        state.header.header_auth.header_auth_tag(),
        state.header.header_auth.header_iv(),
        &[&state.header.raw_header[..], &serialized_req_encryption_context[..]].concat(),
        &mut [],
    );

    // TODO Post-#619: Add to the ESDK Specification the following:
    // ESDK-NET v4.0.0 Header Auth Catch
    if maybe_header_auth.is_err() && net_v4_retry_policy == NetV400RetryPolicy::AllowRetry {
        let derived_data_keys = key_derivation::derive_keys(
            state.header.body.message_id(),
            state.dec_mat.plaintext_data_key.as_ref().unwrap().as_bytes(),
            &state.header.suite,
            true,
        )?;
        state.derived_data_keys = derived_data_keys;
        let mut serialized_req_encryption_context_v4 = Vec::new();
        encryption_context::write_aad(
            &mut serialized_req_encryption_context_v4,
            &canonical_req_encryption_context,
        )?;
        maybe_header_auth = aes_decrypt(
            body::get_encrypt(&state.header.suite),
            &state.derived_data_keys.data_key,
            &[],
            state.header.header_auth.header_auth_tag(),
            state.header.header_auth.header_iv(),
            &[&state.header.raw_header[..], &serialized_req_encryption_context_v4[..]].concat(),
            &mut [],
        );
    }
    //= specification/client-apis/decrypt.md#verify-the-header
    //# If this tag verification fails, this operation MUST immediately halt and fail.
    maybe_header_auth?;

    Ok(state)
}

// Step 4: Decrypt the message body
fn step_decrypt_body(
    ciphertext: &mut dyn SafeRead,
    plaintext: &mut dyn SafeWrite,
    state: &mut DecryptState,
    safety_needed: &ProtectionNeeded,
) -> Result<Vec<u8>, Error> {
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# Once the message header is successfully parsed, the next sequential bytes
    //# MUST be deserialized according to the [message body spec](../data-format/message-body.md).
    //= specification/client-apis/decrypt.md#decrypt-the-message-body
    //# The Decrypt operation MUST use the [content type](../data-format/message-header.md#content-type) field parsed from the
    //# message header to determine whether the operation will deserialize the message bytes as
    //# [framed data](../data-format/message-body.md#framed-data) or
    //# [un-framed data](../data-format/message-body.md#non-framed-data).
    let key = state.derived_data_keys.data_key.clone();
    let last_frame = match state.header.body.content_type() {
        ContentType::NonFramed => body::read_and_decrypt_non_framed_message_body(
            ciphertext, &state.header, &key, &mut state.dw,
        )?,
        ContentType::Framed => {
            //= specification/client-apis/decrypt.md#decrypt-the-message-body
            //# Any plaintext decrypted from [unframed data](../data-format/message-body.md#un-framed-data) or
            //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
            //# successfully completes.
            let fail_if_multi_frame = state.dec_mat.verification_key.is_some() && safety_needed.yes();
            body::read_and_decrypt_framed_message_body(
                ciphertext,
                plaintext,
                &state.header,
                &key,
                &mut state.dw,
                fail_if_multi_frame,
            )?
        }
    };
    Ok(last_frame)
}

// Step 5: Verify the signature
fn step_verify_signature(
    ciphertext: &mut dyn SafeRead,
    state: &DecryptState,
) -> Result<(), Error> {
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# If the algorithm suite has a signature algorithm,
    //# the Decrypt operation MUST verify the message footer using the specified signature algorithm.
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# After deserializing the body, the Decrypt operation MUST deserialize the next encrypted message bytes
    //# as the [message footer](../data-format/message-footer.md).
    if state.dec_mat.verification_key.is_some() {
        let mut noop = NoopWriter;
        //= specification/client-apis/decrypt.md#verify-the-signature
        //# Once the message footer is deserialized, the Decrypt operation MUST use the
        //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
        //# from the [algorithm suite](../framework/algorithm-suites.md) in the decryption materials to
        //# verify the encrypted message, with the following inputs:
        //= specification/client-apis/decrypt.md#verify-the-signature
        //# - The verification key MUST be the [verification key](../framework/structures.md#verification-key)
        //# in the decryption materials.
        //= specification/client-apis/decrypt.md#verify-the-signature
        //# - The input to verify MUST be the concatenation of the serialization of the
        //# [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md).
        //= specification/client-apis/decrypt.md#verify-the-signature
        //# If this verification is not successful, this operation MUST immediately halt and fail.
        verify_signature(
            ciphertext,
            state.dw.context.clone().unwrap(),
            state.dec_mat.clone(),
            &mut noop,
        )?;
    } else {
        //= specification/client-apis/decrypt.md#behavior
        //# - If the message header does not contain an algorithm suite including a signature algorithm,
        //# the Decrypt operation MUST NOT perform this step.
        return Ok(());
    }
    Ok(())
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

pub(crate) fn get_ecdsa_alg(
    alg: aws_mpl_legacy::suites::SignatureAlgorithm,
) -> Result<EcdsaSignatureAlgorithm, Error> {
    match alg {
        aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(x) => Ok(x),
        _ => Err("UnsupportedAlgorithm".into()),
    }
}

fn verify_signature(
    r: &mut dyn SafeRead,
    context: DigestContext,
    dec_mat: aws_mpl_legacy::DecryptionMaterials,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    if dec_mat.verification_key.is_none() {
        return Ok(());
    }

    //= specification/client-apis/decrypt.md#verify-the-signature
    //= type=implication
    //= reason=blocking read on the input stream implicitly waits for enough bytes
    //# If there are not enough consumable bytes to deserialize the message footer and
    //# the caller has not yet indicated an end to the encrypted message,
    //# the Decrypt operation MUST wait for enough bytes to become consumable or for the caller
    //# to indicate an end to the encrypted message.
    let signature = footer::read_footer(r, raw)?;
    let ecdsa_params = get_ecdsa_alg(dec_mat.algorithm_suite.signature)?;
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# If this verification is not successful, this operation MUST immediately halt and fail.
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

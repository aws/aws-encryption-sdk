// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Decrypt operation — deserializes the encrypted message header and body,
//! obtains decryption materials from a keyring/CMM, derives the data key,
//! and decrypts the plaintext (handling both framed and nonframed formats).

use crate::encrypt::get_esdk_id;
use crate::error::{Error, esdk_err, val_err};
use crate::key_derivation;
use crate::materials;
use crate::message::header_types::ContentType;
use crate::message::serializable_types::{from_canonical_pairs, to_canonical_pairs};
use crate::message::{
    DigestWriter, NoopWriter, body, encryption_context, footer, header, header_auth, header_types,
    serialize_functions, v2_header_body,
};
use crate::types::{
    DecryptInput, DecryptOutput, DecryptStreamInput, DecryptStreamOutput, EncryptionContext,
    MaterialSource, NetV400RetryPolicy, SafeRead, SafeWrite,
};
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::primitives::{
    DigestContext, EcdsaSignatureAlgorithm, aes_decrypt, ecdsa_verify_context,
};

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

//= spec/client-apis/decrypt.md#security-considerations
//= type=implication
//= reason=Ok is only returned after signature verification; streamed bytes are unverified until then
//# If this operation is [streaming](streaming.md) output to the caller
//# and is decrypting messages created with an algorithm suite including a signature algorithm,
//# any released plaintext MUST NOT be considered signed data until this operation finishes
//# successfully.
//
//= spec/client-apis/decrypt.md#verify-the-header
//= reason=Ok is returned only after step_verify_signature succeeds
//# However, if the streamed Decrypt operation is using an algorithm suite with a signature algorithm
//# all released output MUST NOT be considered signed data until
//# this operation successfully completes.
//
//= spec/client-apis/decrypt.md#security-considerations
//= type=implication
//= reason=Caller obligation; Ok signals completion
//# This means that callers that process such released plaintext MUST NOT consider any processing successful
//# until this operation completes successfully.
//
//= spec/client-apis/decrypt.md#security-considerations
//= reason=Caller obligation; Err signals discard
//# Additionally, if this operation fails, callers MUST discard the released plaintext and encryption context
//# and MUST rollback any processing done due to the released plaintext or encryption context.
/// Decrypt an encrypted message stream, writing plaintext to the output.
///
/// # Errors
/// Returns an error if input validation, header parsing, decryption, or signature verification fails.
pub async fn decrypt_stream(
    //= spec/client-apis/decrypt.md#encrypted-message
    //= type=implication
    //= reason=SafeRead processes bytes incrementally without buffering the full message
    //# This input MAY be [streamed](streaming.md) to this operation.
    //
    //= spec/client-apis/decrypt.md#encrypted-message
    //= type=implication
    //= reason=SafeRead processes incrementally; full message never held in memory
    //# If an implementation requires holding the entire encrypted message in memory in order to perform this operation,
    //# that implementation SHOULD NOT provide an API that allows the caller to stream the encrypted message.
    ciphertext: &mut dyn SafeRead,
    //= spec/client-apis/decrypt.md#plaintext
    //= type=implication
    //= reason=SafeWrite flushes each decrypted frame without buffering the full plaintext
    //# This operation MAY [stream](streaming.md) the plaintext as output.
    //
    //= spec/client-apis/decrypt.md#plaintext
    //= type=exception
    //= reason=Does not require holding input plaintext in memory
    //# If an implementation requires holding the entire encrypted message in memory in order to perform this operation,
    //# that implementation SHOULD NOT provide an API that allows the caller to stream the encrypted message.
    plaintext: &mut dyn SafeWrite,
    input: &DecryptStreamInput,
) -> Result<DecryptStreamOutput, Error> {
    input.validate()?;

    //= spec/client-apis/decrypt.md#behavior
    //= type=exception
    //= reason=unsafe_release_plaintext_before_verify gates multi-frame signed messages; single-frame is buffered until after verify
    //# - The ESDK MUST provide a configuration option that causes the decryption operation
    //# to fail immediately after parsing the header if a signed algorithm suite is used.
    let safety = ProtectionNeeded::needs_protection(input.unsafe_release_plaintext_before_verify);

    internal_decrypt(
        ciphertext,
        plaintext,
        input.source.clone(),
        &input.encryption_context,
        input.net_v4_retry_policy,
        safety,
        input.max_encrypted_data_keys,
        input.commitment_policy,
    )
    .await
}

//= spec/client-apis/client.md#decrypt
//# The AWS Encryption SDK Client MUST provide an [decrypt](./decrypt.md#input) function
//# that adheres to [decrypt](./decrypt.md).
/// Decrypt an encrypted message, returning the plaintext and metadata.
///
/// # Errors
/// Returns an error if input validation, header parsing, decryption, or signature verification fails.
pub async fn decrypt(input: &DecryptInput<'_>) -> Result<DecryptOutput, Error> {
    input.validate()?;
    let mut cursor = std::io::Cursor::new(input.ciphertext);

    //= spec/client-apis/decrypt.md#behavior
    //= type=implication
    //= reason=decrypt() returns Result; caller cannot access output until Ok
    //# If the input encrypted message is not being [streamed](streaming.md) to this operation,
    //# all output MUST NOT be released until after these steps complete successfully.
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

    let Ok(ciphertext_len) = u64::try_from(input.ciphertext.len()) else {
        return Err(esdk_err("Ciphertext length does not fit in u64"));
    };
    if cursor.position() != ciphertext_len {
        //= spec/client-apis/decrypt.md#behavior
        //# - If this operation successfully completes the above steps
        //# but there are consumable bytes which are intended to be decrypted,
        //# this operation MUST fail.
        return Err(esdk_err("Data after message footer"));
    }

    Ok(DecryptOutput {
        //= spec/client-apis/decrypt.md#output
        //= type=implication
        //= reason=DecryptOutput struct always contains plaintext field
        //# - Decrypt operation output MUST include a [Plaintext](#plaintext) value.
        plaintext,
        //= spec/client-apis/decrypt.md#output
        //# - Decrypt operation output MUST include an [encryption context](#encryption-context) value.
        //
        //= spec/client-apis/decrypt.md#encryption-context
        //= type=exception
        //= reason=The encryption context is returned directly as a field, not via a parsed header struct
        //# This output MAY be satisfied by outputting a [parsed header](#parsed-header) containing this value.
        encryption_context: out.encryption_context,
        //= spec/client-apis/decrypt.md#output
        //# - Decrypt operation output MUST include an [algorithm suite](#algorithm-suite) value.
        //
        //= spec/client-apis/decrypt.md#algorithm-suite
        //= type=exception
        //= reason=The algorithm suite is returned directly as a field, not via a parsed header struct
        //# This output MAY be satisfied by outputting a [parsed header](#parsed-header) containing this value.
        algorithm_suite_id: out.algorithm_suite_id,
        //= spec/client-apis/decrypt.md#output
        //= type=exception
        //= reason=Parsed header is not spec'ed out; this is a SHOULD, not a MUST
        //# - Decrypt operation output SHOULD include a [Parsed Header](#parsed-header) value.
    })
}

/// Intermediate state passed between decrypt steps.
struct DecryptState {
    header: header::HeaderInfo,
    dec_mat: aws_mpl_legacy::DecryptionMaterials,
    derived_data_keys: key_derivation::ExpandedKeyMaterial,
    sig_digest: DigestWriter,
    encryption_context_to_only_authenticate: EncryptionContext,
}

#[expect(clippy::too_many_arguments)]
//= spec/client-apis/decrypt.md#behavior
//= type=implication
//= reason=SafeWrite receives output only after per-frame AEAD; last frame held until after signature verify
//# - Output MUST NOT be released until otherwise indicated.
//
//= spec/client-apis/decrypt.md#behavior
//= reason=Every step uses ?; any failure returns Err to the caller
//# - If all bytes have been provided and this operation
//# is unable to complete the above steps with the consumable encrypted message bytes,
//# this operation MUST halt and indicate a failure to the caller.
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
    //= spec/client-apis/decrypt.md#behavior
    //= type=implication
    //# - Decrypt operation Step 1 MUST be [Parse the header](#parse-the-header)
    let (header_body, raw_header, sig_digest) =
        step_parse_header(ciphertext, max_encrypted_data_keys)?;

    //= spec/client-apis/decrypt.md#behavior
    //= type=implication
    //# - Decrypt operation Step 2 MUST be [Get the decryption materials](#get-the-decryption-materials)
    let state = step_get_decryption_materials(
        ciphertext,
        &header_body,
        raw_header,
        input_source,
        encryption_context,
        commitment_policy,
        sig_digest,
    )
    .await?;

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# decrypt MUST yield an error.
    let _esdk_id = get_esdk_id(state.header.suite.id)?;

    //= spec/client-apis/decrypt.md#behavior
    //= reason=verification_key presence must match suite's signature algorithm; mismatch means misbehaving CMM
    //# - If the message header contains an algorithm suite including a
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# the Decrypt operation MUST perform this step.
    let suite_has_signature = !matches!(
        state.header.suite.signature,
        aws_mpl_legacy::suites::SignatureAlgorithm::None
    );
    if suite_has_signature != state.dec_mat.verification_key.is_some() {
        return Err(val_err(
            "Decryption materials verification_key presence must match algorithm suite signature algorithm",
        ));
    }

    //= spec/client-apis/decrypt.md#v2-header-deserialization
    //= reason=Parsed header kept in local state until step_verify_header succeeds
    //# Until the [header is verified](#verify-the-header), this operation MUST NOT
    //# release any parsed information from the header.

    //= spec/client-apis/decrypt.md#behavior
    //= type=implication
    //# - Decrypt operation Step 3 MUST be [Verify the header](#verify-the-header)
    let mut state = step_verify_header(state, net_v4_retry_policy)?;

    //= spec/client-apis/decrypt.md#behavior
    //= type=implication
    //# - Decrypt operation Step 4 MUST be [Decrypt the message body](#decrypt-the-message-body)
    let last_frame = step_decrypt_body(ciphertext, plaintext, &mut state, &safety_needed)?;

    //= spec/client-apis/decrypt.md#behavior
    //= type=implication
    //# - Decrypt operation Step 5 MUST be [Verify the signature](#verify-the-signature)
    step_verify_signature(ciphertext, &state)?;

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //# Any plaintext decrypted from [nonframed data](../data-format/message-body.md#nonframed-data) or
    //# a final frame in a streamed Decrypt operation MUST NOT be released until [signature verification](#verify-the-signature)
    //# successfully completes.

    //= spec/client-apis/decrypt.md#authenticated-data
    //# This operation MUST NOT release any unauthenticated plaintext or unauthenticated associated data.
    serialize_functions::write_bytes(plaintext, &last_frame)?;

    //= spec/client-apis/decrypt.md#behavior
    //= type=exception
    //= reason=SafeRead has no "end of stream" signal; non-streaming wrapper checks cursor position instead
    //# - If this operation successfully completes the above steps
    //# but there are consumable bytes which are intended to be decrypted,
    //# this operation MUST fail.

    // Merge authenticate-only encryption context with header encryption context.
    // These sets are designed to be disjoint (authenticate-only keys are removed from the
    // header before serialization); on any overlap, prefer the authenticate-only value
    // because it was verified as AAD and is the source of truth.
    let mut ec = state.header.encryption_context;
    for (k, v) in state.encryption_context_to_only_authenticate {
        ec.insert(k, v);
    }

    //= spec/client-apis/streaming.md#outputs
    //= reason=All bytes written to SafeWrite before Ok is returned
    //# Operations MUST NOT indicate completion or success until an end to the output has been indicated.
    //
    //= spec/client-apis/decrypt.md#verify-the-header
    //= reason=Encryption context and suite released only after step_verify_header succeeded
    //# - A streamed Decrypt operation SHOULD release the parsed [encryption context](#encryption-context),
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id),
    //# and [other header information](#parsed-header)
    //# as soon as tag verification succeeds.
    Ok(DecryptStreamOutput {
        encryption_context: ec,
        //= spec/client-apis/decrypt.md#algorithm-suite
        //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
        algorithm_suite_id: get_esdk_id(state.header.suite.id)?,
    })
}

// Step 1: Parse the header
fn step_parse_header(
    ciphertext: &mut dyn SafeRead,
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
) -> Result<(header_types::HeaderBody, Vec<u8>, DigestWriter), Error> {
    let mut raw_header = Vec::new();

    //= spec/client-apis/decrypt.md#parse-the-header
    //= type=implication
    //= reason=SafeRead provides sequential byte access only; no seek or random access
    //# Given encrypted message bytes, this operation MUST process those bytes sequentially,
    //# deserializing those bytes according to the [message format](../data-format/message.md).
    //
    //= spec/client-apis/decrypt.md#parse-the-header
    //= type=implication
    //= reason=read_header_body returns Err on incomplete data or reads until header is complete
    //# This operation MUST attempt to deserialize all consumable encrypted message bytes until it has
    //# successfully deserialized a valid [message header](../data-format/message-header.md).
    let header_body =
        header::read_header_body(ciphertext, max_encrypted_data_keys, &mut raw_header)?;

    //= spec/client-apis/decrypt.md#verify-the-header
    //= reason=sig_digest is fed header bytes immediately; raw header not retained for later
    //# - The streamed Decrypt operation SHOULD input the serialized header to the signature algorithm as soon as it is deserialized,
    //# such that the serialized header isn't required to remain in memory to [verify the signature](#verify-the-signature).
    let mut sig_digest = DigestWriter::from_old_ecdsa(header_body.algorithm_suite().signature)?;
    serialize_functions::write_bytes(&mut sig_digest, &raw_header)?;

    Ok((header_body, raw_header, sig_digest))
}

// Step 2: Get the decryption materials
async fn step_get_decryption_materials(
    ciphertext: &mut dyn SafeRead,
    header_body: &header_types::HeaderBody,
    raw_header: Vec<u8>,
    input_source: Option<MaterialSource>,
    encryption_context: &EncryptionContext,
    commitment_policy: EsdkCommitmentPolicy,
    mut sig_digest: DigestWriter,
) -> Result<DecryptState, Error> {
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //# The CMM used MUST be the input CMM, if supplied.
    //
    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //# If a CMM is not supplied as the input, the decrypt operation MUST construct a [default CMM](../framework/default-cmm.md)
    //# from the input [keyring](../framework/keyring-interface.md).
    let cmm = materials::create_cmm_from_input(input_source).await?;

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
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

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //# The algorithm suite used as input for all decryption described below MUST be the algorithm suite
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).
    let suite = &dec_mat.algorithm_suite;

    if suite != header_body.algorithm_suite() {
        return Err(val_err(
            "Stored header algorithm suite does not match decryption algorithm suite",
        ));
    }

    let header_auth =
        //= spec/client-apis/decrypt.md#v1-header-deserialization
        //# - MUST deserialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
        //
        //= spec/client-apis/decrypt.md#v2-header-deserialization
        //# - MUST deserialize the [Authentication Tag](../data-format/message-header.md#authentication-tag).
        header_auth::read_header_auth_tag(ciphertext, suite, &mut sig_digest)?;

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //# The data key used as input for all decryption described below MUST be a data key derived from the plaintext data key
    //# included in the [decryption materials](../framework/structures.md#decryption-materials).

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //# The algorithm suite used to derive a data key from the plaintext data key MUST be
    //# the [key derivation algorithm](../framework/algorithm-suites.md#key-derivation-algorithm) included in the
    //# [algorithm suite](../framework/algorithm-suites.md) associated with
    //# the returned decryption materials.
    let derived_data_keys = key_derivation::derive_keys(
        header_body.message_id(),
        dec_mat
            .plaintext_data_key
            .as_ref()
            .ok_or_else(|| val_err("Decryption materials must contain a plaintext data key"))?
            .as_bytes(),
        suite,
        false,
    )?;

    if !header::header_version_supports_commitment(suite, header_body) {
        return Err(val_err("Invalid commitment values found in header body"));
    }

    //= spec/client-apis/decrypt.md#get-the-decryption-materials
    //# If the [algorithm suite](../framework/algorithm-suites.md#algorithm-suites-encryption-key-derivation-settings) supports [key commitment](../framework/algorithm-suites.md#key-commitment)
    //# then the [commit key](../framework/algorithm-suites.md#commit-key) MUST be derived from the plaintext data key
    //# using the [commit key derivation](../framework/algorithm-suites.md#algorithm-suites-commit-key-derivation-settings).
    if v2_header_body::has_hkdf(&suite.commitment) {
        //= spec/client-apis/decrypt.md#get-the-decryption-materials
        //# The derived commit key MUST equal the commit key stored in the message header.
        header::validate_suite_data(
            suite,
            header_body,
            derived_data_keys.commitment_key.as_ref().ok_or_else(|| {
                val_err("Derived key material must contain a commitment key for HKDF commitment")
            })?,
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
        sig_digest,
        encryption_context_to_only_authenticate,
    })
}

// Step 3: Verify the header
fn step_verify_header(
    mut state: DecryptState,
    net_v4_retry_policy: NetV400RetryPolicy,
) -> Result<DecryptState, Error> {
    //= spec/client-apis/decrypt.md#verify-the-header
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

    //= spec/client-apis/decrypt.md#verify-the-header
    //# Once a valid message header is deserialized and decryption materials are available,
    //# this operation MUST validate the [message header body](../data-format/message-header.md#header-body)
    //# by using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# to decrypt with the following inputs:
    let mut maybe_header_auth = aes_decrypt(
        body::get_alg_suite(&state.header.suite)?,
        //= spec/client-apis/decrypt.md#verify-the-header
        //# - the cipherkey MUST be the derived data key
        &state.derived_data_keys.data_key,
        //= spec/client-apis/decrypt.md#verify-the-header
        //# - the ciphertext MUST be an empty byte array
        &[],
        //= spec/client-apis/decrypt.md#verify-the-header
        //# - the tag MUST be the value serialized in the message header's
        //# [authentication tag field](../data-format/message-header.md#authentication-tag)
        state.header.header_auth.header_auth_tag(),
        //= spec/client-apis/decrypt.md#verify-the-header
        //# - For message format version [1.0](../data-format/message-header.md#supported-versions)
        //# the IV MUST be the value serialized in the message header's [IV field](../data-format/message-header.md#iv).
        //
        //= spec/client-apis/decrypt.md#verify-the-header
        //# For message format version [2.0](../data-format/message-header.md#supported-versions)
        //# the IV MUST be 0.
        state.header.header_auth.header_iv(),
        //= spec/client-apis/decrypt.md#verify-the-header
        //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
        //# and the serialization of encryption context to only authenticate.
        &[
            &state.header.raw_header[..],
            &serialized_req_encryption_context[..],
        ]
        .concat(),
        &mut [],
    );

    // ESDK-NET v4.0.0 Header Auth Catch
    if maybe_header_auth.is_err() && net_v4_retry_policy == NetV400RetryPolicy::AllowRetry {
        let derived_data_keys = key_derivation::derive_keys(
            state.header.body.message_id(),
            state
                .dec_mat
                .plaintext_data_key
                .as_ref()
                .ok_or_else(|| val_err("Decryption materials must contain a plaintext data key"))?
                .as_bytes(),
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
            body::get_alg_suite(&state.header.suite)?,
            &state.derived_data_keys.data_key,
            &[],
            state.header.header_auth.header_auth_tag(),
            state.header.header_auth.header_iv(),
            &[
                &state.header.raw_header[..],
                &serialized_req_encryption_context_v4[..],
            ]
            .concat(),
            &mut [],
        );
    }

    //= spec/client-apis/decrypt.md#verify-the-header
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
    let key = state.derived_data_keys.data_key.clone();

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //# Once the message header is successfully parsed, the next sequential bytes
    //# MUST be deserialized according to the [message body spec](../data-format/message-body.md).

    //= spec/client-apis/decrypt.md#decrypt-the-message-body
    //# The Decrypt operation MUST use the [content type](../data-format/message-header.md#content-type) field parsed from the
    //# message header to determine whether the operation will deserialize the message bytes as
    //# [framed data](../data-format/message-body.md#framed-data) or
    //# [nonframed data](../data-format/message-body.md#nonframed-data).
    let last_frame = match state.header.body.content_type() {
        ContentType::NonFramed => {
            //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
            //# Nonframed data deserialization MUST conform to the [Nonframed Data](../data-format/message-body.md#nonframed-data) specification.
            //
            //= spec/client-apis/decrypt.md#nonframed-message-body-decryption
            //= reason=read_and_decrypt_non_framed_message_body deserializes and decrypts per the nonframed data specification
            //# If a message has the [nonframed](../data-format/message-body.md#nonframed-data) content type,
            //# the Decrypt operation MUST deserialize the message body according to the
            //# [nonframed data specification](../data-format/message-body.md#nonframed-data)
            //# and decrypt it using the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
            //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
            body::read_and_decrypt_non_framed_message_body(
                ciphertext,
                &state.header,
                &key,
                &mut state.sig_digest,
            )?
        }
        ContentType::Framed => {
            let fail_if_multi_frame =
                state.dec_mat.verification_key.is_some() && safety_needed.yes();
            body::read_and_decrypt_framed_message_body(
                ciphertext,
                plaintext,
                &state.header,
                &key,
                &mut state.sig_digest,
                fail_if_multi_frame,
            )?
        }
    };
    Ok(last_frame)
}

// Step 5: Verify the signature
fn step_verify_signature(ciphertext: &mut dyn SafeRead, state: &DecryptState) -> Result<(), Error> {
    //= spec/client-apis/decrypt.md#verify-the-signature
    //# If the algorithm suite has a signature algorithm,
    //# the Decrypt operation MUST verify the message footer using the specified signature algorithm.
    //
    //= spec/client-apis/decrypt.md#verify-the-signature
    //# After deserializing the body, the Decrypt operation MUST deserialize the next encrypted message bytes
    //# as the [message footer](../data-format/message-footer.md).
    if state.dec_mat.verification_key.is_some() {
        let mut noop = NoopWriter;

        //= spec/client-apis/decrypt.md#verify-the-signature
        //# Once the message footer is deserialized, the Decrypt operation MUST use the
        //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
        //# from the [algorithm suite](../framework/algorithm-suites.md) in the decryption materials to
        //# verify the encrypted message, with the following inputs:
        verify_signature(
            ciphertext,
            //= spec/client-apis/decrypt.md#verify-the-signature
            //# - The input to verify MUST be the concatenation of the serialization of the
            //# [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md).
            state.sig_digest.context.clone().ok_or_else(|| {
                val_err("Signature digest context must be present for signature verification")
            })?,
            //= spec/client-apis/decrypt.md#verify-the-signature
            //# - The verification key MUST be the [verification key](../framework/structures.md#verification-key)
            //# in the decryption materials.
            state.dec_mat.clone(),
            &mut noop,
        )?;
    } else {
        //= spec/client-apis/decrypt.md#behavior
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
        _ => Err(val_err("Unsupported signature algorithm")),
    }
}

fn verify_signature(
    r: &mut dyn SafeRead,
    context: DigestContext,
    dec_mat: aws_mpl_legacy::DecryptionMaterials,
    sig_digest: &mut dyn SafeWrite,
) -> Result<(), Error> {
    if dec_mat.verification_key.is_none() {
        return Ok(());
    }

    //= spec/client-apis/decrypt.md#verify-the-signature
    //= reason=blocking read on the input stream implicitly waits for enough bytes
    //# If there are not enough consumable bytes to deserialize the message footer and
    //# the caller has not yet indicated an end to the encrypted message,
    //# the Decrypt operation MUST wait for enough bytes to become consumable or for the caller
    //# to indicate an end to the encrypted message.
    let signature = footer::read_footer(r, sig_digest)?;
    let ecdsa_params = get_ecdsa_alg(dec_mat.algorithm_suite.signature)?;

    let valid = ecdsa_verify_context(
        ecdsa_params,
        &dec_mat.verification_key
            .ok_or_else(|| val_err("Decryption materials must contain a verification key for signature verification"))?
            .0,
        context,
        &signature,
    )?;

    if !valid {
        //= spec/client-apis/decrypt.md#verify-the-signature
        //# If this verification is not successful, this operation MUST immediately halt and fail.
        return Err(esdk_err("Signature verification failed"));
    }
    Ok(())
}

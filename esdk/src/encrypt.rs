// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Encrypt operation — obtains encryption materials from a keyring/CMM,
//! derives a data key, encrypts the plaintext (framed or nonframed),
//! and serializes the result into an encrypted message.

use crate::error::Error;
use crate::error::{esdk_err, ser_err, val_err};
use crate::key_derivation;
use crate::materials;
use crate::message::encryption_context::write_empty_ec_or_write_aad;
use crate::message::header_types::{
    ContentType, HeaderAuth, HeaderBody, MessageId, MessageType, V1HeaderBody, V2HeaderBody,
};
use crate::message::serializable_types::{
    ESDKCanonicalEncryptionContext, get_encrypt_key_length, get_iv_length, to_canonical_pairs,
};
use crate::message::{DigestWriter, body, footer, header};
use crate::types::{
    EncryptInput, EncryptOutput, EncryptStreamInput, EncryptStreamOutput, EncryptionContext,
    FrameLength, MaterialSource, SafeRead, SafeWrite,
};
use aws_mpl_legacy::EncryptedDataKey;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;
use aws_mpl_legacy::primitives::{aes_encrypt, ecdsa_sign_digest};
use aws_mpl_legacy::suites::AlgorithmSuite;

/// Intermediate state produced by [`step_get_encryption_materials`] and consumed by subsequent steps.
struct EncryptionMaterialsResult {
    materials: aws_mpl_legacy::EncryptionMaterials,
    derived_data_keys: key_derivation::ExpandedKeyMaterial,
    message_id: MessageId,
}

/// This is the public-facing entry point for the ESDK encrypt method.
///
/// # Errors
///
/// Returns an error if:
/// - No keyring or CMM is provided (input validation fails)
/// - The encryption context contains keys with the reserved `aws-crypto-` prefix
/// - The algorithm suite is incompatible with the commitment policy
/// - Encryption materials cannot be obtained from the CMM
/// - The number of encrypted data keys exceeds the configured maximum
/// - Key derivation, header construction, body encryption, or signature generation fails
//= specification/client-apis/client.md#encrypt
//# The AWS Encryption SDK Client MUST provide an [encrypt](./encrypt.md#input) function
//# that adheres to [encrypt](./encrypt.md).
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
        //= specification/client-apis/encrypt.md#output
        //# - Encrypt operation output MUST include an [encrypted message](#encrypted-message) value.
        //
        //= specification/client-apis/encrypt.md#encrypted-message
        //# This MUST be a sequence of bytes
        //# and conform to the [message format specification](../data-format/message.md).
        ciphertext,
        //= specification/client-apis/encrypt.md#output
        //# - Encrypt operation output MUST include an [encryption context](#encryption-context) value.
        encryption_context: out.encryption_context,
        //= specification/client-apis/encrypt.md#output
        //# - Encrypt operation output MUST include an [algorithm suite](#algorithm-suite) value.
        //
        //= specification/client-apis/encrypt.md#algorithm-suite-1
        //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
        algorithm_suite_id: out.algorithm_suite_id,
    })
}

/// Encrypt a plaintext stream into a ciphertext stream.
///
/// # Errors
///
/// Returns an error if:
/// - No keyring or CMM is provided (input validation fails)
/// - The encryption context contains keys with the reserved `aws-crypto-` prefix
/// - The algorithm suite is incompatible with the commitment policy
/// - Encryption materials cannot be obtained from the CMM
/// - The number of encrypted data keys exceeds the configured maximum
/// - Key derivation, header construction, body encryption, or signature generation fails
//= specification/client-apis/streaming.md#overview
//# The AWS Encryption SDK MAY provide APIs that enable streamed [encryption](encrypt.md)
//# and [decryption](decrypt.md).
//
//= specification/client-apis/streaming.md#overview
//# APIs that support streaming of the encrypt or decrypt operation SHOULD allow customers
//# to be able to process arbitrarily large inputs with a finite amount of working memory.
//
//= specification/client-apis/encrypt.md#plaintext
//# This input MAY be [streamed](streaming.md) to this operation.
//
//= specification/client-apis/encrypt.md#encrypted-message
//= reason=SafeWrite accepts incremental writes, so each encrypted frame is flushed to the output as it's produced without buffering the full ciphertext
//# This operation MAY [stream](streaming.md) the encrypted message.
//
//= specification/client-apis/encrypt.md#plaintext
//# If an implementation requires holding the input entire plaintext in memory in order to perform this operation,
//# that implementation SHOULD NOT provide an API that allows this input to be streamed.
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
    // ciphertext is the output buffer for this function
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
    //= reason=every step below uses the ? operator, which halts and returns the error to the caller
    //# If any of these steps fails, this operation MUST halt and indicate a failure to the caller.
    //
    //= specification/client-apis/encrypt.md#encryption-context
    //# If the input encryption context contains any entries with a key beginning with `aws-crypto-`,
    //# the encryption operation MUST fail.
    validate_encryption_context(encryption_context)?;

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation Step 1 MUST be [Get the encryption materials](#get-the-encryption-materials)
    //
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# To construct the [encrypted message](#encrypted-message),
    //# some fields MUST be constructed using information obtained
    //# from a set of valid [encryption materials](../framework/structures.md#encryption-materials).
    let mat_result = step_get_encryption_materials(
        plaintext_len,
        input_source,
        encryption_context,
        algorithm_suite_id,
        max_encrypted_data_keys,
        commitment_policy,
    )
    .await?;

    let mut sig_digest =
        DigestWriter::from_old_ecdsa(mat_result.materials.algorithm_suite.signature)?;

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation step 2 MUST be [Construct the header](#construct-the-header)
    //
    //= specification/client-apis/encrypt.md#construct-the-header
    //# Before encrypting input plaintext,
    //# this operation MUST serialize the [message header body](../data-format/message-header.md).
    //
    //= specification/data-format/message.md#structure
    //# - The message MUST begin with [Message Header](message-header.md)
    let header = step_construct_header(
        &mat_result,
        &mat_result.materials.encryption_context,
        &mat_result.materials.required_encryption_context_keys,
        &mat_result.materials.encrypted_data_keys,
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# The frame length used in the procedures described below MUST be the input [frame length](#frame-length),
        //# if supplied.
        //
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# If no input frame length is supplied, the default frame length MUST be used.
        frame_length,
        ciphertext,
        &mut sig_digest,
    )?;

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation step 3 MUST be [Construct the body](#construct-the-body)
    //
    //= specification/data-format/message.md#structure
    //# - The [Message Body](message-body.md) MUST follow the Message Header
    step_construct_body(
        plaintext,
        &header,
        &mat_result.derived_data_keys.data_key,
        ciphertext,
        &mut sig_digest,
        plaintext_len,
    )?;

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation step 4 MUST be [Construct the signature](#construct-the-signature)
    if matches!(
        mat_result.materials.algorithm_suite.signature,
        aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(_)
    ) {
        //= specification/client-apis/encrypt.md#behavior
        //# - If the [encryption materials gathered](#get-the-encryption-materials) has a algorithm suite
        //# including a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
        //# the Encrypt operation MUST perform this step.
        step_construct_signature(
            &header,
            &mat_result.materials,
            //= specification/client-apis/encrypt.md#construct-the-signature
            //= reason=sig_digest (DigestWriter) was fed the header bytes in step 2 (write_header) and the body bytes in step 3 (encrypt_and_serialize_body)
            //# Note that the message header and message body MAY have already been input during previous steps.
            sig_digest,
            ciphertext,
        )?;
    } else {
        //= specification/client-apis/encrypt.md#behavior
        //# - If the materials do not have an algorithm suite including a signature algorithm,
        //# the Encrypt operation MUST NOT construct a signature.
    }

    let suite_id = get_esdk_id(header.suite.id)?;

    //= specification/client-apis/encrypt.md#behavior
    //= reason=The Ok return follows step_construct_signature; no post-write code appends bytes after the footer. The returned struct carries metadata only (encryption_context, algorithm_suite_id), not ciphertext bytes.
    //# Any data that is not specified within the [message format](../data-format/message.md)
    //# MUST NOT be added to the output message.
    //
    //= specification/client-apis/streaming.md#outputs
    //= reason=All bytes have been written to the SafeWrite before Ok is returned; success is only indicated after output is complete
    //# Operations MUST NOT indicate completion or success until an end to the output has been indicated.
    Ok(EncryptStreamOutput {
        encryption_context: header.encryption_context,
        algorithm_suite_id: suite_id,
    })
}

/// Step 1: [Get the encryption materials](specification/client-apis/encrypt.md#get-the-encryption-materials)
///
/// Validates the input algorithm suite against the commitment policy, obtains encryption
/// materials from the CMM, validates EDK count, generates a message ID, and derives keys.
async fn step_get_encryption_materials(
    plaintext_len: Option<usize>,
    input_source: Option<MaterialSource>,
    encryption_context: &EncryptionContext,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::EsdkAlgorithmSuiteId>,
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    commitment_policy: EsdkCommitmentPolicy,
) -> Result<EncryptionMaterialsResult, Error> {
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The CMM used MUST be the input CMM, if supplied.
    let cmm = materials::create_cmm_from_input(input_source).await?;

    //= specification/client-apis/encrypt.md#algorithm-suite
    //# The [algorithm suite](../framework/algorithm-suites.md) that MUST be used for encryption.
    //
    //= specification/client-apis/encrypt.md#algorithm-suite
    //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
    let algorithm_suite_id = algorithm_suite_id.map(aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk);
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
    //
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The call to [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials)
    //# on that CMM MUST be constructed as follows:
    let materials = materials::get_encryption_materials(
        cmm,
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - Algorithm Suite: If provided, this MUST be the [input algorithm suite](#algorithm-suite).
        algorithm_suite_id,
        encryption_context.clone(),
        plaintext_len,
        commitment_policy,
    )
    .await?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys) on the [encryption materials](../framework/structures.md#encryption-materials)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md) encrypt MUST yield an error.
    header::validate_max_encrypted_data_keys(
        max_encrypted_data_keys,
        &materials.encrypted_data_keys,
    )?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The [algorithm suite](../framework/algorithm-suites.md) used in all aspects of this operation
    //# MUST be the algorithm suite in the [encryption materials](../framework/structures.md#encryption-materials)
    //# returned from the [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) call.
    //
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=The code uses materials.algorithm_suite regardless of what was requested; the CMM may return a different suite
    //# Note that the algorithm suite in the retrieved encryption materials MAY be different
    //# from the [input algorithm suite](#algorithm-suite).
    let algorithm_suite = &materials.algorithm_suite;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=All EsdkAlgorithmSuiteId variants are ESDK-supported; the check guards against non-ESDK AlgorithmSuiteId variants returned by the CMM
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# encrypt MUST yield an error.
    let message_id = header::generate_message_id(&materials.algorithm_suite)?;

    let derived_data_keys = key_derivation::derive_keys(
        &message_id,
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# The data key used as input for all encryption described below MUST be a data key derived from the plaintext data key
        //# included in the [encryption materials](../framework/structures.md#encryption-materials).
        &materials
            .plaintext_data_key
            .as_ref()
            .ok_or_else(|| esdk_err(
                "Encryption materials must contain a plaintext data key",
            ))?
            .0,
        algorithm_suite,
        false,
    )?;

    Ok(EncryptionMaterialsResult {
        materials,
        derived_data_keys,
        message_id,
    })
}

/// Step 2: [Construct the header](specification/client-apis/encrypt.md#construct-the-header)
fn step_construct_header(
    mat_result: &EncryptionMaterialsResult,
    encryption_context: &EncryptionContext,
    required_encryption_context_keys: &[String],
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: FrameLength,
    ciphertext: &mut dyn SafeWrite,
    sig_digest: &mut DigestWriter,
) -> Result<header::HeaderInfo, Error> {
    //= specification/client-apis/encrypt.md#authentication-tag
    //= reason=build_header_for_encrypt builds the complete header (body + auth tag) before returning
    //# The serialized bytes MUST NOT be released until the entire message header has been serialized.
    let header = build_header_for_encrypt(
        &mat_result.message_id,
        &mat_result.materials.algorithm_suite,
        encryption_context,
        required_encryption_context_keys,
        encrypted_data_keys,
        frame_length.0.get(),
        &mat_result.derived_data_keys,
    )?;

    //= specification/client-apis/encrypt.md#authentication-tag
    //= reason=write_header writes the complete header (body + auth tag) to ciphertext via SafeWrite, which flushes immediately before body serialization begins
    //# If this operation is streaming the encrypted message and
    //# the entire message header has been serialized,
    //# the serialized message header MUST be released.
    header::write_header(&header, ciphertext, sig_digest)?;

    //= specification/client-apis/encrypt.md#authentication-tag
    //# The encrypted message output by the Encrypt operation MUST have a message header equal
    //# to the message header calculated in this step.
    //
    //= specification/client-apis/encrypt.md#authentication-tag
    //= reason=write_header above serializes this exact header directly to ciphertext; the output header is the header calculated here by construction, so inequality is structurally impossible
    //# If the message headers are not equal, the Encrypt operation MUST fail.
    Ok(header)
}

/// Step 3: [Construct the body](specification/client-apis/encrypt.md#construct-the-body)
fn step_construct_body(
    plaintext: &mut dyn SafeRead,
    header: &header::HeaderInfo,
    data_key: &[u8],
    ciphertext: &mut dyn SafeWrite,
    sig_digest: &mut DigestWriter,
    max_plaintext_length: Option<usize>,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#construct-the-body
    //# The encrypted message output by the Encrypt operation MUST have a message body equal
    //# to the message body calculated in this step.
    //
    //= specification/client-apis/encrypt.md#construct-the-body
    //= reason=The body is written directly to the output buffer by encrypt_and_serialize_body, making inequality structurally impossible
    //# If the message bodies are not equal, the Encrypt operation MUST fail.
    body::encrypt_and_serialize_body(
        plaintext,
        header,
        data_key,
        ciphertext,
        sig_digest,
        max_plaintext_length,
    )
}

/// Step 4: [Construct the signature](specification/client-apis/encrypt.md#construct-the-signature)
fn step_construct_signature(
    header: &header::HeaderInfo,
    materials: &aws_mpl_legacy::EncryptionMaterials,
    sig_digest: DigestWriter,
    ciphertext: &mut dyn SafeWrite,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# If the [algorithm suite](../framework/algorithm-suites.md) contains a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# this operation MUST calculate a signature over the message,
    //# and the output [encrypted message](#encrypted-message) MUST contain a [message footer](../data-format/message-footer.md).
    match &header.suite.signature {
        aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(_) => {
            let ecdsa_params = crate::decrypt::get_ecdsa_alg(header.suite.signature)?;

            //= specification/client-apis/encrypt.md#construct-the-signature
            //# To calculate a signature, this operation MUST use the [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
            //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following input:
            let signature_bytes = ecdsa_sign_digest(
                ecdsa_params,
                //= specification/client-apis/encrypt.md#construct-the-signature
                //# - the signature key MUST be the [signing key](../framework/structures.md#signing-key) in the [encryption materials](../framework/structures.md#encryption-materials)
                &materials.signing_key.as_ref()
                    .ok_or_else(|| esdk_err("Encryption materials must contain a signing key for signature algorithm"))?.0,
                //= specification/client-apis/encrypt.md#construct-the-signature
                //# - the input to sign MUST be the concatenation of the serialization of the [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md)
                sig_digest.context
                    .ok_or_else(|| esdk_err("Signature digest context must be present for signing"))?,
            )?;

            //= specification/client-apis/encrypt.md#construct-the-signature
            //# The encrypted message output by this operation MUST have a message footer equal
            //# to the message footer calculated in this step.
            //
            //= specification/data-format/message-footer.md#overview
            //# When an [algorithm suite](../framework/algorithm-suites.md) includes a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
            //# the [message](message.md) MUST contain a footer.
            footer::write_footer(
                //= specification/data-format/message-footer.md#signature
                //= type=implication
                //= reason=ciphertext is the concatenation of header and body
                //# This signature MUST be calculated over both the [message header](message-header.md) and the [message body](message-body.md),
                //# in the order of serialization.
                ciphertext,
                signature_bytes.as_ref(),
            )?;
        }

        //= specification/data-format/message.md#structure
        //# If the algorithm suite does not contain a signature algorithm, the message MUST NOT contain a message footer.
        aws_mpl_legacy::suites::SignatureAlgorithm::None => {}

        //= specification/data-format/message.md#structure
        //# If the [algorithm suite ID](message-header.md#algorithm-suite-id) is unrecognized or unsupported, or its [algorithm suite](../framework/algorithm-suites.md) definition cannot be used to determine whether a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm) is required, the operation MUST raise an error and MUST NOT treat any trailing bytes as a valid [message footer](message-footer.md).
        _ => {
            return Err(esdk_err(
                "Unrecognized signature algorithm in algorithm suite",
            ));
        }
    }

    //= specification/client-apis/encrypt.md#construct-the-signature
    //= reason=step_construct_signature writes directly to the output buffer; returning Ok(()) releases all serialized bytes
    //# Once the entire message footer has been serialized,
    //# this operation MUST release any previously unreleased serialized bytes from previous steps
    //# and MUST release the message footer.
    Ok(())
}

pub(crate) fn get_esdk_id(
    id: aws_mpl_legacy::suites::AlgorithmSuiteId,
) -> Result<aws_mpl_legacy::suites::EsdkAlgorithmSuiteId, Error> {
    match id {
        aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk(x) => Ok(x),
        other => Err(esdk_err(format!("Unsupported algorithm suite: {other:?}"))),
    }
}

const RESERVED_ENCRYPTION_CONTEXT: &str = "aws-crypto-";

fn validate_encryption_context(ec: &EncryptionContext) -> Result<(), Error> {
    for key in ec.keys() {
        if key.starts_with(RESERVED_ENCRYPTION_CONTEXT) {
            return Err(val_err(
                "Encryption context keys cannot contain reserved prefix 'aws-crypto-'",
            ));
        }
    }
    Ok(())
}

fn build_header_for_encrypt(
    message_id: &MessageId,
    suite: &AlgorithmSuite,
    encryption_context: &EncryptionContext,
    required_encryption_context_keys: &[String],
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: u32,
    derived_data_keys: &key_derivation::ExpandedKeyMaterial,
) -> Result<header::HeaderInfo, Error> {
    let mut stored_encryption_context = encryption_context.clone();

    //= specification/client-apis/encrypt.md#authentication-tag
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    let mut required_encryption_context_map: EncryptionContext = EncryptionContext::new();
    for key in required_encryption_context_keys {
        if let Some(val) = stored_encryption_context.remove(key) {
            required_encryption_context_map.insert(key.clone(), val);
        }
    }
    let canonical_stored_encryption_context = to_canonical_pairs(stored_encryption_context);

    let body: HeaderBody = build_header_body(
        message_id,
        suite,
        &canonical_stored_encryption_context,
        encrypted_data_keys,
        frame_length,
        derived_data_keys
            .commitment_key
            .as_deref()
            .cloned(),
    )?;

    let canonical_req_encryption_context = to_canonical_pairs(required_encryption_context_map);
    let mut serialized_req_encryption_context = Vec::new();
    write_empty_ec_or_write_aad(
        &mut serialized_req_encryption_context,
        &canonical_req_encryption_context,
    )?;

    let mut raw_header = Vec::new();
    header::write_header_body(&mut raw_header, &body)?;

    //= specification/client-apis/encrypt.md#authentication-tag
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.
    let header_auth = build_header_auth_tag(
        suite,
        &derived_data_keys.data_key,
        &raw_header,
        &serialized_req_encryption_context,
    )?;

    Ok(header::HeaderInfo {
        suite: suite.clone(),
        body,
        encryption_context: encryption_context.clone(),
        header_auth,
        raw_header,
    })
}

fn build_header_body(
    message_id: &MessageId,
    suite: &AlgorithmSuite,
    encryption_context: &ESDKCanonicalEncryptionContext,
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: u32,
    suite_data: Option<Vec<u8>>,
) -> Result<HeaderBody, Error> {
    //= specification/client-apis/encrypt.md#construct-the-header
    //# The [message format version](../data-format/message-header.md#supported-versions) MUST be the value associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites).
    match suite.commitment {
        aws_mpl_legacy::suites::DerivationAlgorithm::Hkdf(h) => {
            let Some(sd) = suite_data else {
                return ser_err("Suite data must be present for HKDF commitment");
            };
            if sd.len() != h.output_key_length as usize {
                return ser_err(
                    "Suite data length must match the commitment key output length for HKDF commitment",
                );
            }
            Ok(HeaderBody::V2Body(V2HeaderBody {
                algorithm_suite: suite.clone(),
                message_id: message_id.clone(),
                encryption_context: encryption_context.clone(),
                encrypted_data_keys: encrypted_data_keys.into(),
                //= specification/client-apis/encrypt.md#nonframed-message-body-encryption
                //# Implementations of the AWS Encryption SDK MUST NOT encrypt using the nonframed content type.
                content_type: ContentType::Framed,
                frame_length,
                suite_data: sd,
            }))
        }
        aws_mpl_legacy::suites::DerivationAlgorithm::Identity => {
            ser_err("Identity key derivation is not supported for V2 message format")
        }
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

fn build_header_auth_tag(
    suite: &AlgorithmSuite,
    data_key: &[u8],
    raw_header: &[u8],
    serialized_req_encryption_context: &[u8],
) -> Result<HeaderAuth, Error> {
    let key_length = get_encrypt_key_length(suite);
    if data_key.len() != key_length as usize {
        return ser_err(&format!(
            "Incorrect data key length: got {}, expected {}",
            data_key.len(),
            key_length
        ));
    }

    //= specification/client-apis/encrypt.md#authentication-tag
    //# - The IV MUST have a value of 0.
    let iv = vec![0; get_iv_length(suite) as usize];
    let mut auth_tag = Vec::new();

    //= specification/client-apis/encrypt.md#authentication-tag
    //# The value of this MUST be the output of the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    aes_encrypt(
        body::get_encrypt(suite)?,
        &iv,
        //= specification/client-apis/encrypt.md#authentication-tag
        //# - The cipherkey MUST be the derived data key
        data_key,
        //= specification/client-apis/encrypt.md#authentication-tag
        //# - The plaintext MUST be an empty byte array
        &[],
        //= specification/client-apis/encrypt.md#authentication-tag
        //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
        //# and the serialization of encryption context to only authenticate.
        &[raw_header, serialized_req_encryption_context].concat(),
        &mut auth_tag,
    )?;

    Ok(HeaderAuth::AESMac {
        header_iv: iv,
        header_auth_tag: auth_tag,
    })
}

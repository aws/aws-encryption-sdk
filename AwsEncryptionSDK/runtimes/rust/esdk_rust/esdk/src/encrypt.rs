// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Encrypt operation — maps to client-apis/encrypt.md

use crate::encrypt_decrypt;
use crate::error::Error;
use crate::key_derivation;
use crate::materials;
use crate::message::header_types::MessageId;
use crate::message::*;
use crate::types::*;
use aws_mpl_legacy::primitives::*;
use aws_mpl_legacy::commitment::EsdkCommitmentPolicy;

/// Intermediate state produced by [step_get_encryption_materials] and consumed by subsequent steps.
struct EncryptionMaterialsResult {
    materials: aws_mpl_legacy::EncryptionMaterials,
    derived_data_keys: key_derivation::ExpandedKeyMaterial,
    message_id: MessageId,
}

/// This is the public-facing entry point for the ESDK encrypt method.
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
        //# - The output of the Encrypt operation MUST include an [encrypted message](#encrypted-message) value.
        //= specification/client-apis/encrypt.md#encrypted-message
        //# This MUST be a sequence of bytes
        //# and conform to the [message format specification](../data-format/message.md).
        ciphertext,
        //= specification/client-apis/encrypt.md#output
        //# - The output of the Encrypt operation MUST include an [encryption context](#encryption-context) value.
        encryption_context: out.encryption_context,
        //= specification/client-apis/encrypt.md#output
        //# - The output of the Encrypt operation MUST include an [algorithm suite](#algorithm-suite) value.
        //= specification/client-apis/encrypt.md#algorithm-suite-1
        //# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
        algorithm_suite_id: out.algorithm_suite_id,
    })
}

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
    algorithm_suite_id: Option<aws_mpl_legacy::suites::EsdkAlgorithmSuiteId>,
    frame_length: FrameLength,
    //= specification/client-apis/client.md#initialization
    //# - On client initialization,
    //# the caller MUST have the option to provide a [maximum number of encrypted data keys](#maximum-number-of-encrypted-data-keys).
    /// ESDK Rust does not use a client interface but instead exposes static methods that take in this param.
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    //= specification/client-apis/client.md#initialization
    //# - On client initialization,
    //# the caller MUST have the option to provide a [commitment policy](#commitment-policy).
    /// ESDK Rust does not use a client interface but instead exposes static methods that take in this param.
    commitment_policy: EsdkCommitmentPolicy,
) -> Result<EncryptStreamOutput, Error> {
    //= specification/client-apis/encrypt.md#encryption-context
    //# If the input encryption context contains any entries with a key beginning with `aws-crypto-`,
    //# the encryption operation MUST fail.
    encrypt_decrypt::validate_encryption_context(encryption_context)?;

    //= specification/client-apis/encrypt.md#behavior
    //# If any of these steps fails, this operation MUST halt and indicate a failure to the caller.

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation Step 1 MUST be [Get the encryption materials](#get-the-encryption-materials)
    let mat_result = step_get_encryption_materials(
        plaintext_len,
        input_source,
        encryption_context,
        algorithm_suite_id,
        max_encrypted_data_keys,
        commitment_policy,
    )
    .await?;

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation step 2 MUST be [Construct the header](#construct-the-header)
    //= specification/client-apis/encrypt.md#construct-the-header
    //# Before encrypting input plaintext,
    //# this operation MUST serialize the [message header body](../data-format/message-header.md).
    let header = step_construct_header(
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# To construct the [encrypted message](#encrypted-message),
        //# some fields MUST be constructed using information obtained
        //# from a set of valid [encryption materials](../framework/structures.md#encryption-materials).
        &mat_result,
        &mat_result.materials.encryption_context,
        &mat_result.materials.required_encryption_context_keys,
        &mat_result.materials.encrypted_data_keys,
        frame_length,
    )?;

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation step 3 MUST be [Construct the body](#construct-the-body)
    let mut dw = DigestWriter::from_old_ecdsa(mat_result.materials.algorithm_suite.signature)?;
    step_construct_body(
        plaintext,
        &header,
        &mat_result.derived_data_keys.data_key,
        ciphertext,
        &mut dw,
    )?;

    //= specification/client-apis/encrypt.md#behavior
    //# - Encrypt operation step 4 MUST be [Construct the signature](#construct-the-signature)
    //= specification/client-apis/encrypt.md#behavior
    //# If the materials do not have an algorithm suite including a signature algorithm,
    //# the Encrypt operation MUST NOT construct a signature.
    if !matches!(mat_result.materials.algorithm_suite.signature, aws_mpl_legacy::suites::SignatureAlgorithm::None) {
        //# - If the [encryption materials gathered](#get-the-encryption-materials) has a algorithm suite
        //# including a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
        //# the Encrypt operation MUST perform this step.
        step_construct_signature(
            &header,
            &mat_result.materials,
            dw,
            //= specification/client-apis/encrypt.md#construct-the-signature
            //= type=implication
            //# Note that the message header and message body MAY have already been input during previous steps.
            ciphertext,
        )?;
    }

    //= specification/client-apis/encrypt.md#construct-the-signature
    //# Once the entire message footer has been serialized,
    //# this operation MUST release any previously unreleased serialized bytes from previous steps
    //# and MUST release the message footer.
    /// This is released via the ciphertext variable

    let suite_id = get_esdk_id(header.suite.id)?;
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
    let algorithm_suite_id = algorithm_suite_id.map(aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk);
    if let Some(id) = algorithm_suite_id {
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# If an [input algorithm suite](#algorithm-suite) is provided
        //# that is not supported by the [commitment policy](client.md#commitment-policy)
        //# configured in the [client](client.md) encrypt MUST yield an error.
        /// This is checked by the MPL-provided method
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
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - Algorithm Suite: If provided, this MUST be the [input algorithm suite](#algorithm-suite).
        algorithm_suite_id,
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - Encryption Context: If provided, this MUST be the [input encryption context](#encryption-context).
        encryption_context.clone(),
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - Max Plaintext Length: If the [input plaintext](#plaintext) has known length,
        //# this length MUST be used.
        plaintext_len,
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# - Commitment Policy: This MUST be the [commitment policy](client.md#commitment-policy) configured in the [client](client.md) exposing this encrypt function.
        commitment_policy,
    )
    .await?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If the number of [encrypted data keys](../framework/structures.md#encrypted-data-keys) on the [encryption materials](../framework/structures.md#encryption-materials)
    //# is greater than the [maximum number of encrypted data keys](client.md#maximum-number-of-encrypted-data-keys) configured in the [client](client.md)
    //# encrypt MUST yield an error.
    encrypt_decrypt::validate_max_encrypted_data_keys(
        max_encrypted_data_keys,
        &materials.encrypted_data_keys,
    )?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# The [algorithm suite](../framework/algorithm-suites.md) used in all aspects of this operation
    //# MUST be the algorithm suite in the [encryption materials](../framework/structures.md#encryption-materials)
    //# returned from the [Get Encryption Materials](../framework/cmm-interface.md#get-encryption-materials) call.
    let algorithm_suite = &materials.algorithm_suite;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# encrypt MUST yield an error.
    let message_id = encrypt_decrypt::generate_message_id(&materials.algorithm_suite)?;

    let derived_data_keys = key_derivation::derive_keys(
        &message_id,
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# The data key used as input for all encryption described below MUST be a data key derived from the plaintext data key
        //# included in the [encryption materials](../framework/structures.md#encryption-materials).
        &materials.plaintext_data_key.as_ref().unwrap().0, // TODO - can this be None?
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
    encrypted_data_keys: &[aws_mpl_legacy::EncryptedDataKey],
    frame_length: FrameLength,
) -> Result<header::HeaderInfo, Error> {
    encrypt_decrypt::build_header_for_encrypt(
        &mat_result.message_id,
        &mat_result.materials.algorithm_suite,
        encryption_context,
        required_encryption_context_keys,
        encrypted_data_keys,
        frame_length.0.get(),
        &mat_result.derived_data_keys,
    )
}

/// Step 3: [Construct the body](specification/client-apis/encrypt.md#construct-the-body)
fn step_construct_body(
    plaintext: &mut dyn SafeRead,
    header: &header::HeaderInfo,
    data_key: &[u8],
    ciphertext: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    encrypt_decrypt::encrypt_and_serialize(
        plaintext,
        header,
        data_key,
        ciphertext,
        dw,
    )
}

/// Step 4: [Construct the signature](specification/client-apis/encrypt.md#construct-the-signature)
fn step_construct_signature(
    header: &header::HeaderInfo,
    materials: &aws_mpl_legacy::EncryptionMaterials,
    dw: DigestWriter,
    ciphertext: &mut dyn SafeWrite,
) -> Result<(), Error> {
    //= specification/client-apis/encrypt.md#construct-the-signature
    //# If the [algorithm suite](../framework/algorithm-suites.md) contains a [signature algorithm](../framework/algorithm-suites.md#signature-algorithm),
    //# this operation MUST calculate a signature over the message,
    //# and the output [encrypted message](#encrypted-message) MUST contain a [message footer](../data-format/message-footer.md).
    //= specification/data-format/message.md#structure
    //# If the [message header](message-header.md) contains an [algorithm suite](../framework/algorithm-suites.md) in the
    //# [algorithm suite ID](message-header.md#algorithm-suite-id) field that contains a
    //# [signature algorithm](../framework/algorithm-suites.md#signature-algorithm), the message MUST also contain a
    //# [message footer](message-footer.md) serialized after the [message body](message-body.md).
    match &header.suite.signature {
        aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(_) => {
            let ecdsa_params = encrypt_decrypt::get_ecdsa_alg(header.suite.signature)?;
            //= specification/client-apis/encrypt.md#construct-the-signature
            //# To calculate a signature, this operation MUST use the [signature algorithm](../framework/algorithm-suites.md#signature-algorithm)
            //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following input:
            let signature_bytes = ecdsa_sign_digest(
                ecdsa_params,
                //= specification/client-apis/encrypt.md#construct-the-signature
                //# - the signature key MUST be the [signing key](../framework/structures.md#signing-key) in the [encryption materials](../framework/structures.md#encryption-materials)
                &materials.signing_key.as_ref().unwrap().0,
                //= specification/client-apis/encrypt.md#construct-the-signature
                //# - the input to sign MUST be the concatenation of the serialization of the [message header](../data-format/message-header.md) and [message body](../data-format/message-body.md)
                dw.context.unwrap(),
            )?;
            //= specification/client-apis/encrypt.md#construct-the-signature
            //# The encrypted message output by this operation MUST have a message footer equal
            //# to the message footer calculated in this step.
            footer::write_footer(
                //= specification/data-format/message-footer.md#signature
                //# This signature MUST be calculated over both the [message header](message-header.md) and the [message body](message-body.md),
                //# in the order of serialization.
                /// ciphertext here is the concatenation of header and body
                ciphertext,
                signature_bytes.as_ref()
            )?;
        }

        //= specification/data-format/message.md#structure
        //# If the algorithm suite does not contain a signature algorithm, the message MUST NOT  contain a message footer.
        aws_mpl_legacy::suites::SignatureAlgorithm::None => {}
    
        //= specification/data-format/message.md#structure
        //# If the algorithm suite contain an unrecognized signature algorithm, the operation MUST raise an error.
        _ => {
            return Err("Unrecognized signature algorithm in algorithm suite".into());
        }
    }
    Ok(())
}

//= specification/data-format/message-header.md#algorithm-suite-id
//# This algorithm suite MUST be [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum).
pub(crate) fn get_esdk_id(
    id: aws_mpl_legacy::suites::AlgorithmSuiteId,
) -> Result<aws_mpl_legacy::suites::EsdkAlgorithmSuiteId, Error> {
    match id {
        aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk(x) => Ok(x),
        _ => Err("Unsupported algorithm suite".into()),
    }
}

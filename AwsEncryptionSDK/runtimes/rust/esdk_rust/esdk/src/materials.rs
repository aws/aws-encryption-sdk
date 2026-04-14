// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Cryptographic materials management — CMM creation and materials retrieval.

use crate::legacy_compat::{convert_alg, convert_commit, convert_edks, from_legacy_dm, from_legacy_em};
use crate::message::header_types::HeaderBody;
use crate::message::serializable_types::{from_canonical_pairs, is_esdk_encrypted_data_keys, is_esdk_encryption_context};
use crate::error::{Error, val_err};
use crate::types::{EncryptionContext, MaterialSource, mpl};
use aws_mpl_legacy::DecryptionMaterials;
use aws_mpl_legacy::EncryptionMaterials;
use aws_mpl_legacy::cmm::DecryptMaterialsInput;
use aws_mpl_legacy::cmm::GetEncryptionMaterialsInput;

pub(crate) enum Cmm {
    Legacy(
        aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    ),
    Modern(aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef),
}

#[allow(clippy::unused_async)]
pub(crate) async fn create_cmm_from_input(
    input_source: Option<MaterialSource>,
) -> Result<Cmm, Error> {
    match input_source.ok_or_else(|| val_err("A Materials Source must be provided"))? {
        MaterialSource::LegacyCmm(cmm) => Ok(Cmm::Legacy(cmm)),
        MaterialSource::LegacyKeyring(keyring) => {
            let mpl = mpl();
            let cmm = mpl
                .create_default_cryptographic_materials_manager()
                .keyring(keyring)
                .send()
                .await?;
            Ok(Cmm::Legacy(cmm))
        }
        MaterialSource::Cmm(cmm) => Ok(Cmm::Modern(cmm)),
        MaterialSource::Keyring(keyring) => {
            //= specification/client-apis/encrypt.md#get-the-encryption-materials
            //# If instead the caller supplied a [keyring](../framework/keyring-interface.md),
            //# this behavior MUST use a [default CMM](../framework/default-cmm.md)
            //# constructed using the caller-supplied keyring as input.
            //= specification/client-apis/decrypt.md#keyring
            //# If the Keyring is provided as the input, the client MUST construct a [default CMM](../framework/default-cmm.md) that uses this keyring,
            //# to obtain the [decryption materials](../framework/structures.md#decryption-materials) that is required for decryption.
            //= specification/client-apis/decrypt.md#keyring
            //= reason=The default CMM constructed above will obtain decryption materials when decrypt_materials is called on it
            //# This default CMM constructed from the keyring MUST obtain the decryption materials required for decryption.
            let cmm = aws_mpl_legacy::cmm::create_default_cryptographic_materials_manager(keyring)?;
            Ok(Cmm::Modern(cmm))
        }
    }
}

pub(crate) async fn get_decryption_materials(
    cmm: Cmm,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let materials = match cmm {
        Cmm::Legacy(cmm) => {
            get_legacy_decryption_materials(
                cmm,
                algorithm_suite_id,
                header_body,
                reproduced_encryption_context,
                commitment_policy,
            )
            .await?
        }
        Cmm::Modern(cmm) => {
            get_modern_decryption_materials(
                cmm,
                algorithm_suite_id,
                header_body,
                reproduced_encryption_context,
                commitment_policy,
            )
            .await?
        }
    };
    if !is_esdk_encryption_context(&materials.encryption_context) {
        return Err(val_err("CMM failed to return serializable encryption materials"));
    }
    Ok(materials)
}

pub(crate) async fn get_encryption_materials(
    cmm: Cmm,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let materials = match cmm {
        Cmm::Legacy(cmm) => {
            get_legacy_encryption_materials(
                cmm,
                algorithm_suite_id,
                encryption_context,
                max_plaintext_length,
                commitment_policy,
            )
            .await?
        }
        Cmm::Modern(cmm) => {
            get_modern_encryption_materials(
                cmm,
                algorithm_suite_id,
                encryption_context,
                max_plaintext_length,
                commitment_policy,
            )
            .await?
        }
    };
    if !is_esdk_encryption_context(&materials.encryption_context) {
        return Err(val_err("CMM failed to return serializable encryption materials"));
    }
    if !is_esdk_encrypted_data_keys(&materials.encrypted_data_keys) {
        return Err(val_err("CMM failed to return serializable encrypted data keys"));
    }
    Ok(materials)
}

pub(crate) async fn get_modern_decryption_materials(
    cmm: aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());
    let mut input = DecryptMaterialsInput::default();
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Algorithm Suite ID: This MUST be the parsed
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# from the message header.
    input.algorithm_suite_id = algorithm_suite_id;
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Commitment Policy: This MUST be the commitment policy configured on the client.
    input.commitment_policy = aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy);
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header.md#encrypted-data-keys)
    //# from the message header.
    input.encrypted_data_keys = header_body.encrypted_data_keys().into();
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Encryption Context: This MUST be the parsed [encryption context](../data-format/message-header.md#aad)
    //# from the message header.
    input.encryption_context = encryption_context;
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
    input
        .reproduced_encryption_context
        .clone_from(reproduced_encryption_context);
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= specification/client-apis/decrypt.md#cryptographic-materials-manager
    //# This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
    let materials = cmm.decrypt_materials(&input).await?;
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# encrypt MUST yield an error.
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    aws_mpl_legacy::commitment::validate_commitment_policy_on_decrypt(
        aws_mpl_legacy::commitment::ValidateCommitmentPolicyOnDecryptInput::new(
            materials.algorithm_suite.id,
            aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    aws_mpl_legacy::materials::decryption_materials_with_plaintext_data_key(&materials)?;
    Ok(materials)
}

pub(crate) async fn get_legacy_decryption_materials(
    cmm: aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());

    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= specification/client-apis/decrypt.md#cryptographic-materials-manager
    //# This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    let output = cmm
        .decrypt_materials()
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Algorithm Suite ID: This MUST be the parsed
        //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
        //# from the message header.
        .algorithm_suite_id(convert_alg(algorithm_suite_id)?)
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Commitment Policy: This MUST be the commitment policy configured on the client.
        .commitment_policy(convert_commit(commitment_policy)?)
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header.md#encrypted-data-keys)
        //# from the message header.
        .encrypted_data_keys(convert_edks(header_body.encrypted_data_keys())?)
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Encryption Context: This MUST be the parsed [encryption context](../data-format/message-header.md#aad)
        //# from the message header.
        .encryption_context(encryption_context)
        //= specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
        .reproduced_encryption_context(reproduced_encryption_context.clone())
        .send()
        .await?;

    let materials = output.decryption_materials
        .ok_or_else(|| val_err("Legacy CMM did not return decryption materials"))?;
    let return_materials = materials.clone();
    let mpl = mpl();
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# encrypt MUST yield an error.
    //= specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    mpl.validate_commitment_policy_on_decrypt()
        .algorithm(
            materials
                .algorithm_suite
                .as_ref()
                .ok_or_else(|| val_err("Legacy decryption materials missing algorithm suite"))?
                .id()
                .as_ref()
                .ok_or_else(|| val_err("Legacy decryption materials algorithm suite missing id"))?
                .clone(),
        )
        .commitment_policy(convert_commit(commitment_policy)?)
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
    from_legacy_dm(return_materials)
}

pub(crate) async fn get_modern_encryption_materials(
    cmm: aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let mut input = GetEncryptionMaterialsInput::default();
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=algorithm_suite_id is Option; None means the field is not set on the input
    //# If no Algorithm Suite is provided, this field MUST NOT be included.
    input.algorithm_suite_id = algorithm_suite_id;
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# - Commitment Policy: This MUST be the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) exposing this encrypt function.
    input.commitment_policy = aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy);
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# - Encryption Context: If provided, this MUST be the [input encryption context](#encryption-context).
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=Encryption context is empty by default
    //# Otherwise, this MUST be an empty encryption context.
    input.encryption_context = encryption_context;
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# - Max Plaintext Length: If the [input plaintext](#plaintext) has known length,
    //# this length MUST be used.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=max_plaintext_length is Option; None means the field is not set on the input
    //# If no Plaintext Length Bound is provided, this field MUST NOT be included.
    input.max_plaintext_length = max_plaintext_length;
    let materials = cmm.get_encryption_materials(&input).await?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    aws_mpl_legacy::commitment::validate_commitment_policy_on_encrypt(
        &aws_mpl_legacy::commitment::ValidateCommitmentPolicyOnEncryptInput::new(
            materials.algorithm_suite.id,
            aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    aws_mpl_legacy::materials::encryption_materials_has_plaintext_data_key(&materials)?;
    Ok(materials)
}

pub(crate) async fn get_legacy_encryption_materials(
    cmm: aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let mpl = mpl();
    #[expect(
        clippy::cast_possible_wrap,
        reason = "max_plaintext_length is i64 in legacy mpl"
    )]
    let output = cmm
        .get_encryption_materials()
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# Otherwise, this MUST be an empty encryption context.
        .encryption_context(encryption_context)
        .commitment_policy(convert_commit(commitment_policy)?)
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //# If no Algorithm Suite is provided, this field MUST NOT be included.
        .set_algorithm_suite_id(algorithm_suite_id.map(convert_alg).transpose()?)
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //= reason=The caller resolves known length vs Plaintext Length Bound before calling; this receives the resolved value
        //# If the input [plaintext](#plaintext) has unknown length and a [Plaintext Length Bound](#plaintext-length-bound)
        //# was provided, this MUST be the [Plaintext Length Bound](#plaintext-length-bound).
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //= reason=max_plaintext_length is Option; .set_ with None means the field is not set
        //# If no Plaintext Length Bound is provided, this field MUST NOT be included.
        .set_max_plaintext_length(max_plaintext_length.map(|x| x as i64))
        .send()
        .await?;

    let materials = output.encryption_materials
        .ok_or_else(|| val_err("Legacy CMM did not return encryption materials"))?;
    let return_materials = materials.clone();
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    mpl.validate_commitment_policy_on_encrypt()
        .algorithm(
            materials
                .algorithm_suite
                .as_ref()
                .ok_or_else(|| val_err("Legacy encryption materials missing algorithm suite"))?
                .id
                .as_ref()
                .ok_or_else(|| val_err("Legacy encryption materials algorithm suite missing id"))?
                .clone(),
        )
        .commitment_policy(convert_commit(commitment_policy)?)
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

    from_legacy_em(return_materials)
}

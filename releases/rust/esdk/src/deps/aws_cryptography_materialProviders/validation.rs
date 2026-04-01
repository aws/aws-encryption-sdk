// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials(
    input: &crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DbeaconKeyIdentifier(
        &input.r#beacon_key_identifier,
    )?;
    validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DbeaconKey(&input.r#beacon_key)?;
    validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DhmacKeys(&input.r#hmac_keys)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DbeaconKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DbeaconKeyIdentifier(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "beacon_key_identifier",
                "beacon_key_identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials_DhmacKeys(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HHmacKeyMap(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials(
    input: &crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DbranchKeyVersion(
        &input.r#branch_key_version,
    )?;
    validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DbranchKey(&input.r#branch_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DbranchKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "branch_key",
                "branch_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DbranchKeyIdentifier(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "branch_key_identifier",
                "branch_key_identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DbranchKeyVersion(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "branch_key_version",
                "branch_key_version is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HEncryptionContext(
    input: &::std::collections::HashMap<::std::string::String, ::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for (inner_key, inner_val) in input.iter() {
        validate_aws_Pcryptography_PkeyStore_HEncryptionContext_Dkey(inner_key)?;
        validate_aws_Pcryptography_PkeyStore_HEncryptionContext_Dvalue(inner_val)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HEncryptionContext_Dkey(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HEncryptionContext_Dvalue(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HHmacKeyMap(
    input: &::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for (inner_key, inner_val) in input.iter() {
        validate_aws_Pcryptography_PkeyStore_HHmacKeyMap_Dkey(inner_key)?;
        validate_aws_Pcryptography_PkeyStore_HHmacKeyMap_Dvalue(inner_val)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HHmacKeyMap_Dkey(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HHmacKeyMap_Dvalue(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAccountIdList(
    input: &::std::vec::Vec<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HAccountIdList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAccountIdList_Dmember(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(
    input: &crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Esdk(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId_DESDK(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId::Dbe(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId_DDBE(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId_DDBE(
    input: &crate::deps::aws_cryptography_materialProviders::types::DbeAlgorithmSuiteId,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId_DESDK(
    input: &crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo(
    input: &crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Did(&input.r#id)?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DbinaryId(
        &input.r#binary_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DmessageVersion(
        &input.r#message_version,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dencrypt(&input.r#encrypt)?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dkdf(&input.r#kdf)?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dcommitment(
        &input.r#commitment,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dsignature(
        &input.r#signature,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DsymmetricSignature(
        &input.r#symmetric_signature,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DedkWrapping(
        &input.r#edk_wrapping,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_for_AwsCryptographicMaterialProviders_ValidAlgorithmSuiteInfo(
    input: &crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Did(&input.r#id)?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DbinaryId(
        &input.r#binary_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DmessageVersion(
        &input.r#message_version,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dencrypt(&input.r#encrypt)?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dkdf(&input.r#kdf)?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dcommitment(
        &input.r#commitment,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dsignature(
        &input.r#signature,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DsymmetricSignature(
        &input.r#symmetric_signature,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DedkWrapping(
        &input.r#edk_wrapping,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DbinaryId(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "binary_id",
                "binary_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dcommitment(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "commitment",
                "commitment is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DedkWrapping(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "edk_wrapping",
                "edk_wrapping is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEdkWrappingAlgorithm(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dencrypt(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encrypt",
                "encrypt is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncrypt(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Did(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "id",
                "id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dkdf(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kdf",
                "kdf is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DmessageVersion(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "message_version",
                "message_version is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_Dsignature(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signature",
                "signature is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HSignatureAlgorithm(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo_DsymmetricSignature(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "symmetric_signature",
                "symmetric_signature is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HSymmetricSignatureAlgorithm(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HBranchKeyIdSupplierReference(
    input: &crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCacheType(
    input: &crate::deps::aws_cryptography_materialProviders::types::CacheType,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::CacheType::Default(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCacheType_DDefault(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::CacheType::No(ref inner) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCacheType_DNo(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::CacheType::SingleThreaded(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCacheType_DSingleThreaded(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::CacheType::MultiThreaded(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCacheType_DMultiThreaded(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::CacheType::StormTracking(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCacheType_DStormTracking(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::CacheType::Shared(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCacheType_DShared(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCacheType_DDefault(
    input: &crate::deps::aws_cryptography_materialProviders::types::DefaultCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDefaultCache(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCacheType_DMultiThreaded(
    input: &crate::deps::aws_cryptography_materialProviders::types::MultiThreadedCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HMultiThreadedCache(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCacheType_DNo(
    input: &crate::deps::aws_cryptography_materialProviders::types::NoCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HNoCache(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCacheType_DShared(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsCacheReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCacheType_DSingleThreaded(
    input: &crate::deps::aws_cryptography_materialProviders::types::SingleThreadedCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HSingleThreadedCache(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCacheType_DStormTracking(
    input: &crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HClientSupplierReference(
    input: &crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy(
    input: &crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy::Esdk(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy_DESDK(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy::Dbe(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy_DDBE(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy_DDBE(
    input: &crate::deps::aws_cryptography_materialProviders::types::DbeCommitmentPolicy,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy_DESDK(
    input: &crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsDiscoveryKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsDiscoveryKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DdiscoveryFilter(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryKeyringInput_DkmsClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_client",
                "kms_client is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsDiscoveryMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_Dregions(
        &input.r#regions,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DclientSupplier(&input.r#client_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DgrantTokens(&input.r#grant_tokens)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsDiscoveryMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_discovery_multi_keyring::CreateAwsKmsDiscoveryMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_Dregions(
        &input.r#regions,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DclientSupplier(&input.r#client_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DgrantTokens(&input.r#grant_tokens)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DclientSupplier(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HClientSupplierReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DdiscoveryFilter(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsDiscoveryMultiKeyringInput_Dregions(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "regions",
                "regions is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HRegionList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsEcdhKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DKeyAgreementScheme(&input.r#key_agreement_scheme)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DcurveSpec(
        &input.r#curve_spec,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsEcdhKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_ecdh_keyring::CreateAwsKmsEcdhKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DKeyAgreementScheme(&input.r#key_agreement_scheme)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DcurveSpec(
        &input.r#curve_spec,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DcurveSpec(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "curve_spec",
                "curve_spec is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DKeyAgreementScheme(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_agreement_scheme",
                "key_agreement_scheme is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsEcdhStaticConfigurations(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsEcdhKeyringInput_DkmsClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_client",
                "kms_client is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DbranchKeyId(&input.r#branch_key_id)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DbranchKeyIdSupplier(&input.r#branch_key_id_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DkeyStore(
        &input.r#key_store,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DttlSeconds(&input.r#ttl_seconds)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_Dcache(
        &input.r#cache,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DpartitionId(&input.r#partition_id)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsHierarchicalKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DbranchKeyId(&input.r#branch_key_id)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DbranchKeyIdSupplier(&input.r#branch_key_id_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DkeyStore(
        &input.r#key_store,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DttlSeconds(&input.r#ttl_seconds)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_Dcache(
        &input.r#cache,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DpartitionId(&input.r#partition_id)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DbranchKeyId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DbranchKeyIdSupplier(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HBranchKeyIdSupplierReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_Dcache(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CacheType,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCacheType(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DkeyStore(
    input: &::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_store",
                "key_store is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKeyStoreReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DpartitionId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsHierarchicalKeyringInput_DttlSeconds(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ttl_seconds",
                "ttl_seconds is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "ttl_seconds",
        "ttl_seconds failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DkmsKeyId(
        &input.r#kms_key_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_keyring::CreateAwsKmsKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DkmsKeyId(
        &input.r#kms_key_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DkmsClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_client",
                "kms_client is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsKeyringInput_DkmsKeyId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_key_id",
                "kms_key_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkDiscoveryKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DgrantTokens(&input.r#grant_tokens)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_Dregion(
        &input.r#region,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkDiscoveryKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_keyring::CreateAwsKmsMrkDiscoveryKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DgrantTokens(&input.r#grant_tokens)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_Dregion(
        &input.r#region,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DdiscoveryFilter(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_DkmsClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_client",
                "kms_client is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryKeyringInput_Dregion(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "region",
                "region is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkDiscoveryMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_Dregions(&input.r#regions)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DclientSupplier(&input.r#client_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DgrantTokens(&input.r#grant_tokens)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkDiscoveryMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateAwsKmsMrkDiscoveryMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_Dregions(&input.r#regions)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DdiscoveryFilter(&input.r#discovery_filter)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DclientSupplier(&input.r#client_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DgrantTokens(&input.r#grant_tokens)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DclientSupplier(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HClientSupplierReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DdiscoveryFilter(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkDiscoveryMultiKeyringInput_Dregions(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "regions",
                "regions is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HRegionList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DkmsKeyId(
        &input.r#kms_key_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_keyring::CreateAwsKmsMrkKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DkmsKeyId(
        &input.r#kms_key_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DkmsClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_client",
                "kms_client is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkKeyringInput_DkmsKeyId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_key_id",
                "kms_key_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_Dgenerator(
        &input.r#generator,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DkmsKeyIds(
        &input.r#kms_key_ids,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DclientSupplier(&input.r#client_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_multi_keyring::CreateAwsKmsMrkMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_Dgenerator(
        &input.r#generator,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DkmsKeyIds(
        &input.r#kms_key_ids,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DclientSupplier(&input.r#client_supplier)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DclientSupplier(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HClientSupplierReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_Dgenerator(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMrkMultiKeyringInput_DkmsKeyIds(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsKeyIdList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_Dgenerator(
        &input.r#generator,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DkmsKeyIds(
        &input.r#kms_key_ids,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DclientSupplier(
        &input.r#client_supplier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_multi_keyring::CreateAwsKmsMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_Dgenerator(
        &input.r#generator,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DkmsKeyIds(
        &input.r#kms_key_ids,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DclientSupplier(
        &input.r#client_supplier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DclientSupplier(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HClientSupplierReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_Dgenerator(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsMultiKeyringInput_DkmsKeyIds(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsKeyIdList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsRsaKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DpublicKey(
        &input.r#public_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DkmsKeyId(
        &input.r#kms_key_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DencryptionAlgorithm(&input.r#encryption_algorithm)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_for_AwsCryptographicMaterialProviders_CreateAwsKmsRsaKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_rsa_keyring::CreateAwsKmsRsaKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DpublicKey(
        &input.r#public_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DkmsKeyId(
        &input.r#kms_key_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DencryptionAlgorithm(&input.r#encryption_algorithm)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DkmsClient(
        &input.r#kms_client,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DencryptionAlgorithm(
    input: &::std::option::Option<aws_sdk_kms::types::EncryptionAlgorithmSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_algorithm",
                "encryption_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DkmsClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DkmsKeyId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_key_id",
                "kms_key_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateAwsKmsRsaKeyringInput_DpublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateCryptographicMaterialsCacheInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheInput_Dcache(
        &input.r#cache,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheInput_for_AwsCryptographicMaterialProviders_CreateCryptographicMaterialsCache(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_cryptographic_materials_cache::CreateCryptographicMaterialsCacheInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheInput_Dcache(
        &input.r#cache,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheInput_Dcache(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CacheType,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "cache",
                "cache is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCacheType(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheOutput_DmaterialsCache(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheOutput_for_AwsCryptographicMaterialProviders_CreateCryptographicMaterialsCache(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheOutput_DmaterialsCache(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsCacheOutput_DmaterialsCache(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials_cache",
                "materials_cache is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsCacheReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsManagerOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsManagerOutput_DmaterialsManager(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsManagerOutput_for_AwsCryptographicMaterialProviders_CreateDefaultCryptographicMaterialsManager(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsManagerOutput_DmaterialsManager(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateCryptographicMaterialsManagerOutput_DmaterialsManager(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials_manager",
                "materials_manager is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsManagerReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultClientSupplierInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateDefaultClientSupplierInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultClientSupplierInput_for_AwsCryptographicMaterialProviders_CreateDefaultClientSupplier(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::CreateDefaultClientSupplierInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultClientSupplierOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultClientSupplierOutput_Dclient(
        &Some(input.clone()),
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultClientSupplierOutput_for_AwsCryptographicMaterialProviders_CreateDefaultClientSupplier(
    input: &crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultClientSupplierOutput_Dclient(
        &Some(input.clone()),
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultClientSupplierOutput_Dclient(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "client",
                "client is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HClientSupplierReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultCryptographicMaterialsManagerInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateDefaultCryptographicMaterialsManagerInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultCryptographicMaterialsManagerInput_Dkeyring(&input.r#keyring)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultCryptographicMaterialsManagerInput_for_AwsCryptographicMaterialProviders_CreateDefaultCryptographicMaterialsManager(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_default_cryptographic_materials_manager::CreateDefaultCryptographicMaterialsManagerInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultCryptographicMaterialsManagerInput_Dkeyring(&input.r#keyring)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateDefaultCryptographicMaterialsManagerInput_Dkeyring(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "keyring",
                "keyring is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKeyringReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsDiscoveryMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsEcdhKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkDiscoveryMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsDiscoveryKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsRsaKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkDiscoveryKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsHierarchicalKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateRawEcdhKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateRawRsaKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateRawAesKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsMrkKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_for_AwsCryptographicMaterialProviders_CreateAwsKmsKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateKeyringOutput_Dkeyring(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "keyring",
                "keyring is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKeyringReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput_Dgenerator(
        &input.r#generator,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput_DchildKeyrings(
        &input.r#child_keyrings,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput_for_AwsCryptographicMaterialProviders_CreateMultiKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_multi_keyring::CreateMultiKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput_Dgenerator(
        &input.r#generator,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput_DchildKeyrings(
        &input.r#child_keyrings,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput_DchildKeyrings(
    input: &::std::option::Option<
        ::std::vec::Vec<
            crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        >,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "child_keyrings",
                "child_keyrings is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKeyringList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateMultiKeyringInput_Dgenerator(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKeyringReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateRawAesKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DkeyNamespace(
        &input.r#key_namespace,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DkeyName(
        &input.r#key_name,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DwrappingKey(
        &input.r#wrapping_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DwrappingAlg(
        &input.r#wrapping_alg,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_for_AwsCryptographicMaterialProviders_CreateRawAesKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_raw_aes_keyring::CreateRawAesKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DkeyNamespace(
        &input.r#key_namespace,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DkeyName(
        &input.r#key_name,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DwrappingKey(
        &input.r#wrapping_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DwrappingAlg(
        &input.r#wrapping_alg,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DkeyName(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_name",
                "key_name is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DkeyNamespace(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_namespace",
                "key_namespace is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DwrappingAlg(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AesWrappingAlg,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "wrapping_alg",
                "wrapping_alg is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawAesKeyringInput_DwrappingKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "wrapping_key",
                "wrapping_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateRawEcdhKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput_DKeyAgreementScheme(
        &input.r#key_agreement_scheme,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput_DcurveSpec(
        &input.r#curve_spec,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput_for_AwsCryptographicMaterialProviders_CreateRawEcdhKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_raw_ecdh_keyring::CreateRawEcdhKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput_DKeyAgreementScheme(
        &input.r#key_agreement_scheme,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput_DcurveSpec(
        &input.r#curve_spec,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput_DcurveSpec(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "curve_spec",
                "curve_spec is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawEcdhKeyringInput_DKeyAgreementScheme(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_agreement_scheme",
                "key_agreement_scheme is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateRawRsaKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DkeyNamespace(
        &input.r#key_namespace,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DkeyName(
        &input.r#key_name,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DpaddingScheme(
        &input.r#padding_scheme,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DpublicKey(
        &input.r#public_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DprivateKey(
        &input.r#private_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_for_AwsCryptographicMaterialProviders_CreateRawRsaKeyring(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_raw_rsa_keyring::CreateRawRsaKeyringInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DkeyNamespace(
        &input.r#key_namespace,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DkeyName(
        &input.r#key_name,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DpaddingScheme(
        &input.r#padding_scheme,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DpublicKey(
        &input.r#public_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DprivateKey(
        &input.r#private_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DkeyName(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_name",
                "key_name is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DkeyNamespace(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_namespace",
                "key_namespace is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DpaddingScheme(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "padding_scheme",
                "padding_scheme is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DprivateKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRawRsaKeyringInput_DpublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::CreateRequiredEncryptionContextCmmInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_DunderlyingCMM(&input.r#underlying_cmm)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_Dkeyring(&input.r#keyring)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_for_AwsCryptographicMaterialProviders_CreateRequiredEncryptionContextCMM(
    input: &crate::deps::aws_cryptography_materialProviders::operation::create_required_encryption_context_cmm::CreateRequiredEncryptionContextCmmInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_DunderlyingCMM(&input.r#underlying_cmm)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_Dkeyring(&input.r#keyring)?;
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_Dkeyring(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKeyringReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_DrequiredEncryptionContextKeys(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "required_encryption_context_keys",
                "required_encryption_context_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMInput_DunderlyingCMM(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsManagerReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMOutput_DmaterialsManager(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMOutput_for_AwsCryptographicMaterialProviders_CreateRequiredEncryptionContextCMM(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMOutput_DmaterialsManager(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCreateRequiredEncryptionContextCMMOutput_DmaterialsManager(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials_manager",
                "materials_manager is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsManagerReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsCacheReference(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsManagerReference(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DalgorithmSuite(
        &input.r#algorithm_suite,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DplaintextDataKey(
        &input.r#plaintext_data_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DverificationKey(
        &input.r#verification_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DsymmetricSigningKey(
        &input.r#symmetric_signing_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_for_AwsCryptographicMaterialProviders_DecryptionMaterialsWithPlaintextDataKey(
    input: &crate::deps::aws_cryptography_materialProviders::operation::decryption_materials_with_plaintext_data_key::DecryptionMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DalgorithmSuite(
        &input.r#algorithm_suite,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DplaintextDataKey(
        &input.r#plaintext_data_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DverificationKey(
        &input.r#verification_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DsymmetricSigningKey(
        &input.r#symmetric_signing_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DalgorithmSuite(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "algorithm_suite",
                "algorithm_suite is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DplaintextDataKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DrequiredEncryptionContextKeys(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "required_encryption_context_keys",
                "required_encryption_context_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DsymmetricSigningKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials_DverificationKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DcommitmentPolicy(
        &input.r#commitment_policy,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DencryptedDataKeys(
        &input.r#encrypted_data_keys,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DreproducedEncryptionContext(&input.r#reproduced_encryption_context)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_for_CryptographicMaterialsManager_DecryptMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DcommitmentPolicy(
        &input.r#commitment_policy,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DencryptedDataKeys(
        &input.r#encrypted_data_keys,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DreproducedEncryptionContext(&input.r#reproduced_encryption_context)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DalgorithmSuiteId(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "algorithm_suite_id",
                "algorithm_suite_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DcommitmentPolicy(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "commitment_policy",
                "commitment_policy is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DencryptedDataKeys(
    input: &::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encrypted_data_keys",
                "encrypted_data_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKeyList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsInput_DreproducedEncryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::DecryptMaterialsOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsOutput_DdecryptionMaterials(
        &input.r#decryption_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsOutput_for_CryptographicMaterialsManager_DecryptMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsOutput_DdecryptionMaterials(
        &input.r#decryption_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDecryptMaterialsOutput_DdecryptionMaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "decryption_materials",
                "decryption_materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDefaultCache(
    input: &crate::deps::aws_cryptography_materialProviders::types::DefaultCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDefaultCache_DentryCapacity(
        &input.r#entry_capacity,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDefaultCache_DentryCapacity(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "entry_capacity",
                "entry_capacity is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "entry_capacity",
        "entry_capacity failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDeleteCacheEntryInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::DeleteCacheEntryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDeleteCacheEntryInput_Didentifier(
        &input.r#identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDeleteCacheEntryInput_for_CryptographicMaterialsCache_DeleteCacheEntry(
    input: &crate::deps::aws_cryptography_materialProviders::operation::delete_cache_entry::DeleteCacheEntryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDeleteCacheEntryInput_Didentifier(
        &input.r#identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDeleteCacheEntryInput_Didentifier(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "identifier",
                "identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm(
    input: &crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::Hkdf(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm_DHKDF(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::Identity(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm_DIDENTITY(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::None(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm_DNone(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm_DHKDF(
    input: &crate::deps::aws_cryptography_materialProviders::types::Hkdf,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HHKDF(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm_DIDENTITY(
    input: &crate::deps::aws_cryptography_materialProviders::types::Identity,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HIDENTITY(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm_DNone(
    input: &crate::deps::aws_cryptography_materialProviders::types::None,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HNone(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDIRECT__KEY__WRAPPING(
    input: &crate::deps::aws_cryptography_materialProviders::types::DirectKeyWrapping,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter(
    input: &crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter_DaccountIds(
        &input.r#account_ids,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter_Dpartition(&input.r#partition)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter_DaccountIds(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "account_ids",
                "account_ids is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAccountIdList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HDiscoveryFilter_Dpartition(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "partition",
                "partition is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HECDSA(
    input: &crate::deps::aws_cryptography_materialProviders::types::Ecdsa,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HECDSA_Dcurve(&input.r#curve)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HECDSA_Dcurve(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "curve",
                "curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEdkWrappingAlgorithm(
    input: &crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::DirectKeyWrapping(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HEdkWrappingAlgorithm_DDIRECT__KEY__WRAPPING(inner)?;
}
    if let crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::IntermediateKeyWrapping(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HEdkWrappingAlgorithm_DIntermediateKeyWrapping(inner)?;
}
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEdkWrappingAlgorithm_DDIRECT__KEY__WRAPPING(
    input: &crate::deps::aws_cryptography_materialProviders::types::DirectKeyWrapping,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDIRECT__KEY__WRAPPING(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEdkWrappingAlgorithm_DIntermediateKeyWrapping(
    input: &crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncrypt(
    input: &crate::deps::aws_cryptography_materialProviders::types::Encrypt,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::Encrypt::AesGcm(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HEncrypt_DAES__GCM(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncrypt_DAES__GCM(
    input: &crate::deps::aws_cryptography_primitives::types::AesGcm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAES__GCM(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey(
    input: &crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey_DkeyProviderId(
        &input.r#key_provider_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey_DkeyProviderInfo(
        &input.r#key_provider_info,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey_Dciphertext(
        &input.r#ciphertext,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey_Dciphertext(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ciphertext",
                "ciphertext is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey_DkeyProviderId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_provider_id",
                "key_provider_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey_DkeyProviderInfo(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_provider_info",
                "key_provider_info is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKeyList(
    input: &::std::vec::Vec<
        crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKeyList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKeyList_Dmember(
    input: &crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(
    input: &::std::collections::HashMap<::std::string::String, ::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for (inner_key, inner_val) in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext_Dkey(inner_key)?;
        validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext_Dvalue(inner_val)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext_Dkey(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext_Dvalue(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys(
    input: &::std::vec::Vec<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys_Dmember(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DalgorithmSuite(
        &input.r#algorithm_suite,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DencryptedDataKeys(
        &input.r#encrypted_data_keys,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DplaintextDataKey(
        &input.r#plaintext_data_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DsigningKey(
        &input.r#signing_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DsymmetricSigningKeys(
        &input.r#symmetric_signing_keys,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_for_AwsCryptographicMaterialProviders_EncryptionMaterialsHasPlaintextDataKey(
    input: &crate::deps::aws_cryptography_materialProviders::operation::encryption_materials_has_plaintext_data_key::EncryptionMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DalgorithmSuite(
        &input.r#algorithm_suite,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DencryptedDataKeys(
        &input.r#encrypted_data_keys,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DplaintextDataKey(
        &input.r#plaintext_data_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DsigningKey(
        &input.r#signing_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DsymmetricSigningKeys(
        &input.r#symmetric_signing_keys,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DalgorithmSuite(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteInfo,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "algorithm_suite",
                "algorithm_suite is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteInfo(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DencryptedDataKeys(
    input: &::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encrypted_data_keys",
                "encrypted_data_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKeyList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DplaintextDataKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DrequiredEncryptionContextKeys(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "required_encryption_context_keys",
                "required_encryption_context_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DsigningKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials_DsymmetricSigningKeys(
    input: &::std::option::Option<::std::vec::Vec<::aws_smithy_types::Blob>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HSymmetricSigningKeyList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEphemeralPrivateKeyToStaticPublicKeyInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HEphemeralPrivateKeyToStaticPublicKeyInput_DrecipientPublicKey(&input.r#recipient_public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HEphemeralPrivateKeyToStaticPublicKeyInput_DrecipientPublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "recipient_public_key",
                "recipient_public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetAlgorithmSuiteInfoInput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetAlgorithmSuiteInfoInput_DbinaryId(&Some(
        input.clone(),
    ))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetAlgorithmSuiteInfoInput_for_AwsCryptographicMaterialProviders_GetAlgorithmSuiteInfo(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_algorithm_suite_info::GetAlgorithmSuiteInfoInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetAlgorithmSuiteInfoInput_DbinaryId(
        &input.r#binary_id,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetAlgorithmSuiteInfoInput_DbinaryId(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "binary_id",
                "binary_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdInput_for_BranchKeyIdSupplier_GetBranchKeyId(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdInput_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::GetBranchKeyIdOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdOutput_DbranchKeyId(
        &input.r#branch_key_id,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdOutput_for_BranchKeyIdSupplier_GetBranchKeyId(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdOutput_DbranchKeyId(
        &input.r#branch_key_id,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetBranchKeyIdOutput_DbranchKeyId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "branch_key_id",
                "branch_key_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_Didentifier(
        &input.r#identifier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_for_CryptographicMaterialsCache_GetCacheEntry(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_Didentifier(
        &input.r#identifier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_DbytesUsed(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryInput_Didentifier(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "identifier",
                "identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_Dmaterials(
        &input.r#materials,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DcreationTime(
        &input.r#creation_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DexpiryTime(
        &input.r#expiry_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DmessagesUsed(
        &input.r#messages_used,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_for_CryptographicMaterialsCache_GetCacheEntry(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_cache_entry::GetCacheEntryOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_Dmaterials(
        &input.r#materials,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DcreationTime(
        &input.r#creation_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DexpiryTime(
        &input.r#expiry_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DmessagesUsed(
        &input.r#messages_used,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DbytesUsed(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "bytes_used",
                "bytes_used is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "bytes_used",
        "bytes_used failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DcreationTime(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "creation_time",
                "creation_time is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "creation_time",
        "creation_time failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DexpiryTime(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "expiry_time",
                "expiry_time is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "expiry_time",
        "expiry_time failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_Dmaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::Materials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials",
                "materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetCacheEntryOutput_DmessagesUsed(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "messages_used",
                "messages_used is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "messages_used",
        "messages_used failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetClientInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::GetClientInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetClientInput_Dregion(&input.r#region)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetClientInput_for_ClientSupplier_GetClient(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetClientInput_Dregion(&input.r#region)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetClientInput_Dregion(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "region",
                "region is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetClientOutput(
    input: &crate::deps::com_amazonaws_kms::client::Client,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetClientOutput_Dclient(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetClientOutput_for_ClientSupplier_GetClient(
    input: &crate::deps::com_amazonaws_kms::client::Client,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetClientOutput_Dclient(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetClientOutput_Dclient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "client",
                "client is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DcommitmentPolicy(
        &input.r#commitment_policy,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DmaxPlaintextLength(
        &input.r#max_plaintext_length,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_for_CryptographicMaterialsManager_GetEncryptionMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DcommitmentPolicy(
        &input.r#commitment_policy,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DmaxPlaintextLength(
        &input.r#max_plaintext_length,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DalgorithmSuiteId(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DcommitmentPolicy(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "commitment_policy",
                "commitment_policy is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DmaxPlaintextLength(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsInput_DrequiredEncryptionContextKeys(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::GetEncryptionMaterialsOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsOutput_DencryptionMaterials(&input.r#encryption_materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsOutput_for_CryptographicMaterialsManager_GetEncryptionMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsOutput_DencryptionMaterials(&input.r#encryption_materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGetEncryptionMaterialsOutput_DencryptionMaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_materials",
                "encryption_materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList(
    input: &::std::vec::Vec<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HGrantTokenList_Dmember(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HHKDF(
    input: &crate::deps::aws_cryptography_materialProviders::types::Hkdf,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HHKDF_Dhmac(&input.r#hmac)?;
    validate_aws_Pcryptography_PmaterialProviders_HHKDF_DsaltLength(&input.r#salt_length)?;
    validate_aws_Pcryptography_PmaterialProviders_HHKDF_DinputKeyLength(&input.r#input_key_length)?;
    validate_aws_Pcryptography_PmaterialProviders_HHKDF_DoutputKeyLength(
        &input.r#output_key_length,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HHKDF_Dhmac(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "hmac",
                "hmac is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HHKDF_DinputKeyLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "input_key_length",
                "input_key_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..=32).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "input_key_length",
        "input_key_length failed to satisfy constraint: Member must be between 1 and 32, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HHKDF_DoutputKeyLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "output_key_length",
                "output_key_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..=32).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "output_key_length",
        "output_key_length failed to satisfy constraint: Member must be between 1 and 32, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HHKDF_DsaltLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "salt_length",
                "salt_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "salt_length",
        "salt_length failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HIDENTITY(
    input: &crate::deps::aws_cryptography_materialProviders::types::Identity,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::InitializeDecryptionMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DalgorithmSuiteId(&input.r#algorithm_suite_id)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DencryptionContext(&input.r#encryption_context)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_for_AwsCryptographicMaterialProviders_InitializeDecryptionMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::operation::initialize_decryption_materials::InitializeDecryptionMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DalgorithmSuiteId(&input.r#algorithm_suite_id)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DencryptionContext(&input.r#encryption_context)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DalgorithmSuiteId(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "algorithm_suite_id",
                "algorithm_suite_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeDecryptionMaterialsInput_DrequiredEncryptionContextKeys(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "required_encryption_context_keys",
                "required_encryption_context_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::InitializeEncryptionMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DalgorithmSuiteId(&input.r#algorithm_suite_id)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DencryptionContext(&input.r#encryption_context)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DsigningKey(
        &input.r#signing_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DverificationKey(&input.r#verification_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_for_AwsCryptographicMaterialProviders_InitializeEncryptionMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::operation::initialize_encryption_materials::InitializeEncryptionMaterialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DalgorithmSuiteId(&input.r#algorithm_suite_id)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DencryptionContext(&input.r#encryption_context)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DrequiredEncryptionContextKeys(&input.r#required_encryption_context_keys)?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DsigningKey(
        &input.r#signing_key,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DverificationKey(&input.r#verification_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DalgorithmSuiteId(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "algorithm_suite_id",
                "algorithm_suite_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encryption_context",
                "encryption_context is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DrequiredEncryptionContextKeys(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "required_encryption_context_keys",
                "required_encryption_context_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionContextKeys(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DsigningKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HInitializeEncryptionMaterialsInput_DverificationKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping(
    input: &crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping_DkeyEncryptionKeyKdf(
        &input.r#key_encryption_key_kdf,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping_DmacKeyKdf(
        &input.r#mac_key_kdf,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping_DpdkEncryptAlgorithm(
        &input.r#pdk_encrypt_algorithm,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping_DkeyEncryptionKeyKdf(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_encryption_key_kdf",
                "key_encryption_key_kdf is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping_DmacKeyKdf(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "mac_key_kdf",
                "mac_key_kdf is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDerivationAlgorithm(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HIntermediateKeyWrapping_DpdkEncryptAlgorithm(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "pdk_encrypt_algorithm",
                "pdk_encrypt_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncrypt(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKeyringList(
    input: &::std::vec::Vec<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HKeyringList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKeyringList_Dmember(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HKeyringReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKeyringReference(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKeyStoreReference(
    input: &crate::deps::aws_cryptography_keyStore::client::Client,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsClientReference(
    input: &crate::deps::com_amazonaws_kms::client::Client,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsEcdhStaticConfigurations(
    input: &crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HKmsEcdhStaticConfigurations_DKmsPublicKeyDiscovery(inner)?;
}
    if let crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HKmsEcdhStaticConfigurations_DKmsPrivateKeyToStaticPublicKey(inner)?;
}
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsEcdhStaticConfigurations_DKmsPrivateKeyToStaticPublicKey(
    input: &crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsEcdhStaticConfigurations_DKmsPublicKeyDiscovery(
    input: &crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HKmsPublicKeyDiscoveryInput(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsKeyIdList(
    input: &::std::vec::Vec<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HKmsKeyIdList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsKeyIdList_Dmember(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::KmsPrivateKeyToStaticPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput_DsenderKmsIdentifier(&input.r#sender_kms_identifier)?;
    validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput_DsenderPublicKey(&input.r#sender_public_key)?;
    validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput_DrecipientPublicKey(&input.r#recipient_public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput_DrecipientPublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "recipient_public_key",
                "recipient_public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput_DsenderKmsIdentifier(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "sender_kms_identifier",
                "sender_kms_identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsPrivateKeyToStaticPublicKeyInput_DsenderPublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsPublicKeyDiscoveryInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::KmsPublicKeyDiscoveryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HKmsPublicKeyDiscoveryInput_DrecipientKmsIdentifier(&input.r#recipient_kms_identifier)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKmsPublicKeyDiscoveryInput_DrecipientKmsIdentifier(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "recipient_kms_identifier",
                "recipient_kms_identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMaterialProvidersConfig(
    input: &crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMaterials(
    input: &crate::deps::aws_cryptography_materialProviders::types::Materials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::Materials::Encryption(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HMaterials_DEncryption(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::Materials::Decryption(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HMaterials_DDecryption(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::Materials::BranchKey(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HMaterials_DBranchKey(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::Materials::BeaconKey(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HMaterials_DBeaconKey(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMaterials_DBeaconKey(
    input: &crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMaterials_DBranchKey(
    input: &crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMaterials_DDecryption(
    input: &crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMaterials_DEncryption(
    input: &crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMultiThreadedCache(
    input: &crate::deps::aws_cryptography_materialProviders::types::MultiThreadedCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HMultiThreadedCache_DentryCapacity(
        &input.r#entry_capacity,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HMultiThreadedCache_DentryPruningTailSize(
        &input.r#entry_pruning_tail_size,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMultiThreadedCache_DentryCapacity(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "entry_capacity",
                "entry_capacity is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "entry_capacity",
        "entry_capacity failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HMultiThreadedCache_DentryPruningTailSize(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "entry_pruning_tail_size",
        "entry_pruning_tail_size failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HNoCache(
    input: &crate::deps::aws_cryptography_materialProviders::types::NoCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HNone(
    input: &crate::deps::aws_cryptography_materialProviders::types::None,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::OnDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput_Dmaterials(&input.r#materials)?;
    validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput_DencryptedDataKeys(
        &input.r#encrypted_data_keys,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput_for_Keyring_OnDecrypt(
    input: &crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput_Dmaterials(&input.r#materials)?;
    validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput_DencryptedDataKeys(
        &input.r#encrypted_data_keys,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput_DencryptedDataKeys(
    input: &::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "encrypted_data_keys",
                "encrypted_data_keys is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptedDataKeyList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnDecryptInput_Dmaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials",
                "materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnDecryptOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::OnDecryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnDecryptOutput_Dmaterials(&input.r#materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnDecryptOutput_for_Keyring_OnDecrypt(
    input: &crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnDecryptOutput_Dmaterials(&input.r#materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnDecryptOutput_Dmaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials",
                "materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnEncryptInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::OnEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnEncryptInput_Dmaterials(&input.r#materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnEncryptInput_for_Keyring_OnEncrypt(
    input: &crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnEncryptInput_Dmaterials(&input.r#materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnEncryptInput_Dmaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials",
                "materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnEncryptOutput(
    input: &crate::deps::aws_cryptography_materialProviders::types::OnEncryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnEncryptOutput_Dmaterials(&input.r#materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnEncryptOutput_for_Keyring_OnEncrypt(
    input: &crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HOnEncryptOutput_Dmaterials(&input.r#materials)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HOnEncryptOutput_Dmaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials",
                "materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPublicKeyDiscoveryInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::PublicKeyDiscoveryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HPublicKeyDiscoveryInput_DrecipientStaticPrivateKey(&input.r#recipient_static_private_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPublicKeyDiscoveryInput_DrecipientStaticPrivateKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "recipient_static_private_key",
                "recipient_static_private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::PutCacheEntryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_Didentifier(
        &input.r#identifier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_Dmaterials(
        &input.r#materials,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DcreationTime(
        &input.r#creation_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DexpiryTime(
        &input.r#expiry_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DmessagesUsed(
        &input.r#messages_used,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_for_CryptographicMaterialsCache_PutCacheEntry(
    input: &crate::deps::aws_cryptography_materialProviders::operation::put_cache_entry::PutCacheEntryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_Didentifier(
        &input.r#identifier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_Dmaterials(
        &input.r#materials,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DcreationTime(
        &input.r#creation_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DexpiryTime(
        &input.r#expiry_time,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DmessagesUsed(
        &input.r#messages_used,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DbytesUsed(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "bytes_used",
        "bytes_used failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DcreationTime(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "creation_time",
                "creation_time is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "creation_time",
        "creation_time failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DexpiryTime(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "expiry_time",
                "expiry_time is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "expiry_time",
        "expiry_time failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_Didentifier(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "identifier",
                "identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_Dmaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::Materials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "materials",
                "materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HPutCacheEntryInput_DmessagesUsed(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "messages_used",
        "messages_used failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations(
    input: &crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::PublicKeyDiscovery(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations_DPublicKeyDiscovery(inner)?;
}
    if let crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations_DRawPrivateKeyToStaticPublicKey(inner)?;
}
    if let crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations_DEphemeralPrivateKeyToStaticPublicKey(inner)?;
}
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations_DEphemeralPrivateKeyToStaticPublicKey(
    input: &crate::deps::aws_cryptography_materialProviders::types::EphemeralPrivateKeyToStaticPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HEphemeralPrivateKeyToStaticPublicKeyInput(
        input,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations_DPublicKeyDiscovery(
    input: &crate::deps::aws_cryptography_materialProviders::types::PublicKeyDiscoveryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HPublicKeyDiscoveryInput(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRawEcdhStaticConfigurations_DRawPrivateKeyToStaticPublicKey(
    input: &crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HRawPrivateKeyToStaticPublicKeyInput(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRawPrivateKeyToStaticPublicKeyInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::RawPrivateKeyToStaticPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HRawPrivateKeyToStaticPublicKeyInput_DsenderStaticPrivateKey(&input.r#sender_static_private_key)?;
    validate_aws_Pcryptography_PmaterialProviders_HRawPrivateKeyToStaticPublicKeyInput_DrecipientPublicKey(&input.r#recipient_public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRawPrivateKeyToStaticPublicKeyInput_DrecipientPublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "recipient_public_key",
                "recipient_public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRawPrivateKeyToStaticPublicKeyInput_DsenderStaticPrivateKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "sender_static_private_key",
                "sender_static_private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRegionList(
    input: &::std::vec::Vec<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HRegionList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HRegionList_Dmember(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSignatureAlgorithm(
    input: &crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::Ecdsa(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HSignatureAlgorithm_DECDSA(inner)?;
    }
    if let crate::deps::aws_cryptography_materialProviders::types::SignatureAlgorithm::None(
        ref inner,
    ) = &input
    {
        validate_aws_Pcryptography_PmaterialProviders_HSignatureAlgorithm_DNone(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSignatureAlgorithm_DECDSA(
    input: &crate::deps::aws_cryptography_materialProviders::types::Ecdsa,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HECDSA(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSignatureAlgorithm_DNone(
    input: &crate::deps::aws_cryptography_materialProviders::types::None,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HNone(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSingleThreadedCache(
    input: &crate::deps::aws_cryptography_materialProviders::types::SingleThreadedCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HSingleThreadedCache_DentryCapacity(
        &input.r#entry_capacity,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HSingleThreadedCache_DentryPruningTailSize(
        &input.r#entry_pruning_tail_size,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSingleThreadedCache_DentryCapacity(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "entry_capacity",
                "entry_capacity is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "entry_capacity",
        "entry_capacity failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSingleThreadedCache_DentryPruningTailSize(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "entry_pruning_tail_size",
        "entry_pruning_tail_size failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache(
    input: &crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DentryCapacity(
        &input.r#entry_capacity,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DentryPruningTailSize(
        &input.r#entry_pruning_tail_size,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DgracePeriod(
        &input.r#grace_period,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DgraceInterval(
        &input.r#grace_interval,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DfanOut(&input.r#fan_out)?;
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DinFlightTTL(
        &input.r#in_flight_ttl,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DsleepMilli(
        &input.r#sleep_milli,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DtimeUnits(
        &input.r#time_units,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DentryCapacity(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "entry_capacity",
                "entry_capacity is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "entry_capacity",
        "entry_capacity failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DentryPruningTailSize(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "entry_pruning_tail_size",
        "entry_pruning_tail_size failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DfanOut(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "fan_out",
                "fan_out is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "fan_out",
                "fan_out failed to satisfy constraint: Member must be greater than or equal to 1",
            ),
        );
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DgraceInterval(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "grace_interval",
                "grace_interval is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "grace_interval",
        "grace_interval failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DgracePeriod(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "grace_period",
                "grace_period is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "grace_period",
        "grace_period failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DinFlightTTL(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "in_flight_ttl",
                "in_flight_ttl is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "in_flight_ttl",
        "in_flight_ttl failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DsleepMilli(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "sleep_milli",
                "sleep_milli is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "sleep_milli",
        "sleep_milli failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HStormTrackingCache_DtimeUnits(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::TimeUnits,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSymmetricSignatureAlgorithm(
    input: &crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm::Hmac(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HSymmetricSignatureAlgorithm_DHMAC(inner)?;
}
    if let crate::deps::aws_cryptography_materialProviders::types::SymmetricSignatureAlgorithm::None(ref inner) = &input {
  validate_aws_Pcryptography_PmaterialProviders_HSymmetricSignatureAlgorithm_DNone(inner)?;
}
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSymmetricSignatureAlgorithm_DHMAC(
    input: &crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSymmetricSignatureAlgorithm_DNone(
    input: &crate::deps::aws_cryptography_materialProviders::types::None,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HNone(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSymmetricSigningKeyList(
    input: &::std::vec::Vec<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PmaterialProviders_HSymmetricSigningKeyList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HSymmetricSigningKeyList_Dmember(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::UpdateUsageMetadataInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput_Didentifier(
        &input.r#identifier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput_for_CryptographicMaterialsCache_UpdateUsageMetadata(
    input: &crate::deps::aws_cryptography_materialProviders::operation::update_usage_metadata::UpdateUsageMetadataInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput_Didentifier(
        &input.r#identifier,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput_DbytesUsed(
        &input.r#bytes_used,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput_DbytesUsed(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "bytes_used",
                "bytes_used is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "bytes_used",
        "bytes_used failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HUpdateUsageMetadataInput_Didentifier(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "identifier",
                "identifier is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput_Dalgorithm(&input.r#algorithm)?;
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput_DcommitmentPolicy(&input.r#commitment_policy)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput_for_AwsCryptographicMaterialProviders_ValidateCommitmentPolicyOnDecrypt(
    input: &crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_decrypt::ValidateCommitmentPolicyOnDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput_Dalgorithm(&input.r#algorithm)?;
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput_DcommitmentPolicy(&input.r#commitment_policy)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput_Dalgorithm(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "algorithm",
                "algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnDecryptInput_DcommitmentPolicy(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "commitment_policy",
                "commitment_policy is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::ValidateCommitmentPolicyOnEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_Dalgorithm(&input.r#algorithm)?;
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_DcommitmentPolicy(&input.r#commitment_policy)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_for_AwsCryptographicMaterialProviders_ValidateCommitmentPolicyOnEncrypt(
    input: &crate::deps::aws_cryptography_materialProviders::operation::validate_commitment_policy_on_encrypt::ValidateCommitmentPolicyOnEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_Dalgorithm(&input.r#algorithm)?;
    validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_DcommitmentPolicy(&input.r#commitment_policy)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_Dalgorithm(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::AlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "algorithm",
                "algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HAlgorithmSuiteId(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidateCommitmentPolicyOnEncryptInput_DcommitmentPolicy(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::CommitmentPolicy,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "commitment_policy",
                "commitment_policy is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCommitmentPolicy(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::ValidDecryptionMaterialsTransitionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_Dstart(
        &input.r#start,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_Dstop(
        &input.r#stop,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_for_AwsCryptographicMaterialProviders_ValidDecryptionMaterialsTransition(
    input: &crate::deps::aws_cryptography_materialProviders::operation::valid_decryption_materials_transition::ValidDecryptionMaterialsTransitionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_Dstart(
        &input.r#start,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_Dstop(
        &input.r#stop,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_Dstart(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "start",
                "start is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidDecryptionMaterialsTransitionInput_Dstop(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "stop",
                "stop is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HDecryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput(
    input: &crate::deps::aws_cryptography_materialProviders::types::ValidEncryptionMaterialsTransitionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput_Dstart(
        &input.r#start,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput_Dstop(
        &input.r#stop,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput_for_AwsCryptographicMaterialProviders_ValidEncryptionMaterialsTransition(
    input: &crate::deps::aws_cryptography_materialProviders::operation::valid_encryption_materials_transition::ValidEncryptionMaterialsTransitionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput_Dstart(
        &input.r#start,
    )?;
    validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput_Dstop(
        &input.r#stop,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput_Dstart(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "start",
                "start is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HValidEncryptionMaterialsTransitionInput_Dstop(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EncryptionMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "stop",
                "stop is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HEncryptionMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM(
    input: &crate::deps::aws_cryptography_primitives::types::AesGcm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAES__GCM_DkeyLength(&input.r#key_length)?;
    validate_aws_Pcryptography_Pprimitives_HAES__GCM_DtagLength(&input.r#tag_length)?;
    validate_aws_Pcryptography_Pprimitives_HAES__GCM_DivLength(&input.r#iv_length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM_DivLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "iv_length",
                "iv_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..=255).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "iv_length",
        "iv_length failed to satisfy constraint: Member must be between 0 and 255, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM_DkeyLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_length",
                "key_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..=32).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "key_length",
        "key_length failed to satisfy constraint: Member must be between 1 and 32, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM_DtagLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "tag_length",
                "tag_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..=32).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "tag_length",
        "tag_length failed to satisfy constraint: Member must be between 0 and 32, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_smithy_Papi_HUnit(
    input: &(),
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_smithy_Papi_HUnit_for_KeyStore_GetKeyStoreInfo(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::Unit,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}

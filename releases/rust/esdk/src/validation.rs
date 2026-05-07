// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HAwsEncryptionSdkConfig(
    input: &crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HAwsEncryptionSdkConfig_DcommitmentPolicy(
        &input.r#commitment_policy,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HAwsEncryptionSdkConfig_DmaxEncryptedDataKeys(
        &input.r#max_encrypted_data_keys,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HAwsEncryptionSdkConfig_DnetV4__0__0__RetryPolicy(
        &input.r#net_v4_0_0_retry_policy,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HAwsEncryptionSdkConfig_DcommitmentPolicy(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HAwsEncryptionSdkConfig_DmaxEncryptedDataKeys(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "max_encrypted_data_keys",
        "max_encrypted_data_keys failed to satisfy constraint: Member must be greater than or equal to 1",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HAwsEncryptionSdkConfig_DnetV4__0__0__RetryPolicy(
    input: &::std::option::Option<crate::types::NetV400RetryPolicy>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptInput(
    input: &crate::types::DecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_Dciphertext(&input.r#ciphertext)?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_DmaterialsManager(
        &input.r#materials_manager,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_Dkeyring(&input.r#keyring)?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_for_AwsEncryptionSdk_Decrypt(
    input: &crate::operation::decrypt::DecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_Dciphertext(&input.r#ciphertext)?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_DmaterialsManager(
        &input.r#materials_manager,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_Dkeyring(&input.r#keyring)?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_Dciphertext(
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
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_DencryptionContext(
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
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_Dkeyring(
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
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptInput_DmaterialsManager(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsManagerReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput(
    input: &crate::types::DecryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_Dplaintext(&input.r#plaintext)?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_for_AwsEncryptionSdk_Decrypt(
    input: &crate::operation::decrypt::DecryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_Dplaintext(&input.r#plaintext)?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_DalgorithmSuiteId(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId,
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

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_DencryptionContext(
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
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HDecryptOutput_Dplaintext(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "plaintext",
                "plaintext is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput(
    input: &crate::types::EncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_Dplaintext(&input.r#plaintext)?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DmaterialsManager(
        &input.r#materials_manager,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_Dkeyring(&input.r#keyring)?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DframeLength(&input.r#frame_length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_for_AwsEncryptionSdk_Encrypt(
    input: &crate::operation::encrypt::EncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_Dplaintext(&input.r#plaintext)?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DmaterialsManager(
        &input.r#materials_manager,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_Dkeyring(&input.r#keyring)?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DframeLength(&input.r#frame_length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DalgorithmSuiteId(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DencryptionContext(
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
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DframeLength(
    input: &::std::option::Option<::std::primitive::i64>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..=4294967296).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "frame_length",
        "frame_length failed to satisfy constraint: Member must be between 1 and 4294967296, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_Dkeyring(
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
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_DmaterialsManager(
    input: &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsManagerReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptInput_Dplaintext(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "plaintext",
                "plaintext is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if input.as_ref().len() > 134_217_728 {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
            "plaintext",
            format!("plaintext size {} bytes exceeds the maximum allowed size of 128 MB", input.as_ref().len()),
        ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput(
    input: &crate::types::EncryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_Dciphertext(&input.r#ciphertext)?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_for_AwsEncryptionSdk_Encrypt(
    input: &crate::operation::encrypt::EncryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_Dciphertext(&input.r#ciphertext)?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_DalgorithmSuiteId(
        &input.r#algorithm_suite_id,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_DalgorithmSuiteId(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::EsdkAlgorithmSuiteId,
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

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_Dciphertext(
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
pub(crate) fn validate_aws_Pcryptography_PencryptionSdk_HEncryptOutput_DencryptionContext(
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
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HCryptographicMaterialsManagerReference(
    input: &crate::deps::aws_cryptography_materialProviders::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
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
pub(crate) fn validate_aws_Pcryptography_PmaterialProviders_HKeyringReference(
    input: &crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}

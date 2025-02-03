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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyInput(
    input: &crate::deps::aws_cryptography_keyStore::types::CreateKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_for_KeyStore_CreateKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::create_key::CreateKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_DencryptionContext(
        &input.r#encryption_context,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_DbranchKeyIdentifier(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyInput_DencryptionContext(
    input: &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HEncryptionContext(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyOutput(
    input: &crate::deps::aws_cryptography_keyStore::types::CreateKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HCreateKeyOutput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyOutput_for_KeyStore_CreateKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::create_key::CreateKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HCreateKeyOutput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyOutput_DbranchKeyIdentifier(
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreInput(
    input: &crate::deps::aws_cryptography_keyStore::types::CreateKeyStoreInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreInput_for_KeyStore_CreateKeyStore(
    input: &crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreOutput(
    input: &crate::deps::aws_cryptography_keyStore::types::CreateKeyStoreOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreOutput_DtableArn(&input.r#table_arn)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreOutput_for_KeyStore_CreateKeyStore(
    input: &crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreOutput_DtableArn(&input.r#table_arn)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HCreateKeyStoreOutput_DtableArn(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "table_arn",
                "table_arn is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..=1024).contains(
        &input
            .chars()
            .map(::std::primitive::char::len_utf16)
            .fold(0usize, ::std::ops::Add::add),
    ) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "table_arn",
        "table_arn failed to satisfy constraint: Member must have length between 1 and 1024, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HDdbClientReference(
    input: &crate::deps::com_amazonaws_dynamodb::client::Client,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HDiscovery(
    input: &crate::deps::aws_cryptography_keyStore::types::Discovery,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyInput(
    input: &crate::deps::aws_cryptography_keyStore::types::GetActiveBranchKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyInput_for_KeyStore_GetActiveBranchKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_active_branch_key::GetActiveBranchKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyInput_DbranchKeyIdentifier(
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyOutput(
    input: &crate::deps::aws_cryptography_keyStore::types::GetActiveBranchKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyOutput_DbranchKeyMaterials(
        &input.r#branch_key_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyOutput_for_KeyStore_GetActiveBranchKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_active_branch_key::GetActiveBranchKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyOutput_DbranchKeyMaterials(
        &input.r#branch_key_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetActiveBranchKeyOutput_DbranchKeyMaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "branch_key_materials",
                "branch_key_materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyInput(
    input: &crate::deps::aws_cryptography_keyStore::types::GetBeaconKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyInput_for_KeyStore_GetBeaconKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyInput_DbranchKeyIdentifier(
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyOutput(
    input: &crate::deps::aws_cryptography_keyStore::types::GetBeaconKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyOutput_DbeaconKeyMaterials(
        &input.r#beacon_key_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyOutput_for_KeyStore_GetBeaconKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyOutput_DbeaconKeyMaterials(
        &input.r#beacon_key_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBeaconKeyOutput_DbeaconKeyMaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "beacon_key_materials",
                "beacon_key_materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HBeaconKeyMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput(
    input: &crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput_DbranchKeyVersion(
        &input.r#branch_key_version,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput_for_KeyStore_GetBranchKeyVersion(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput_DbranchKeyVersion(
        &input.r#branch_key_version,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput_DbranchKeyIdentifier(
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionInput_DbranchKeyVersion(
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionOutput(
    input: &crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionOutput_DbranchKeyMaterials(
        &input.r#branch_key_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionOutput_for_KeyStore_GetBranchKeyVersion(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_branch_key_version::GetBranchKeyVersionOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionOutput_DbranchKeyMaterials(
        &input.r#branch_key_materials,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetBranchKeyVersionOutput_DbranchKeyMaterials(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "branch_key_materials",
                "branch_key_materials is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HBranchKeyMaterials(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput(
    input: &crate::deps::aws_cryptography_keyStore::types::GetKeyStoreInfoOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkeyStoreId(&input.r#key_store_id)?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkeyStoreName(
        &input.r#key_store_name,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DlogicalKeyStoreName(
        &input.r#logical_key_store_name,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkmsConfiguration(
        &input.r#kms_configuration,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_for_KeyStore_GetKeyStoreInfo(
    input: &crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfoOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkeyStoreId(&input.r#key_store_id)?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkeyStoreName(
        &input.r#key_store_name,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DlogicalKeyStoreName(
        &input.r#logical_key_store_name,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DgrantTokens(
        &input.r#grant_tokens,
    )?;
    validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkmsConfiguration(
        &input.r#kms_configuration,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "grant_tokens",
                "grant_tokens is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkeyStoreId(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_store_id",
                "key_store_id is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkeyStoreName(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_store_name",
                "key_store_name is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(3..=255).contains(
        &input
            .chars()
            .map(::std::primitive::char::len_utf16)
            .fold(0usize, ::std::ops::Add::add),
    ) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "key_store_name",
        "key_store_name failed to satisfy constraint: Member must have length between 3 and 255, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DkmsConfiguration(
    input: &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::KmsConfiguration>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_configuration",
                "kms_configuration is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HKMSConfiguration(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGetKeyStoreInfoOutput_DlogicalKeyStoreName(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "logical_key_store_name",
                "logical_key_store_name is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGrantTokenList(
    input: &::std::vec::Vec<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    for inner in input.iter() {
        validate_aws_Pcryptography_PkeyStore_HGrantTokenList_Dmember(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HGrantTokenList_Dmember(
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig(
    input: &crate::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DddbTableName(&input.r#ddb_table_name)?;
    validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DkmsConfiguration(
        &input.r#kms_configuration,
    )?;
    validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DlogicalKeyStoreName(
        &input.r#logical_key_store_name,
    )?;
    validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_Did(&input.r#id)?;
    validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DgrantTokens(&input.r#grant_tokens)?;
    validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DddbClient(&input.r#ddb_client)?;
    validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DkmsClient(&input.r#kms_client)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DddbClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_dynamodb::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HDdbClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DddbTableName(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ddb_table_name",
                "ddb_table_name is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(3..=255).contains(
        &input
            .chars()
            .map(::std::primitive::char::len_utf16)
            .fold(0usize, ::std::ops::Add::add),
    ) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "ddb_table_name",
        "ddb_table_name failed to satisfy constraint: Member must have length between 3 and 255, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DgrantTokens(
    input: &::std::option::Option<::std::vec::Vec<::std::string::String>>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HGrantTokenList(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_Did(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DkmsClient(
    input: &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HKmsClientReference(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DkmsConfiguration(
    input: &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::KmsConfiguration>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "kms_configuration",
                "kms_configuration is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_PkeyStore_HKMSConfiguration(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKeyStoreConfig_DlogicalKeyStoreName(
    input: &::std::option::Option<::std::string::String>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "logical_key_store_name",
                "logical_key_store_name is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKmsClientReference(
    input: &crate::deps::com_amazonaws_kms::client::Client,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKMSConfiguration(
    input: &crate::deps::aws_cryptography_keyStore::types::KmsConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if let crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsKeyArn(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_DkmsKeyArn(inner)?;
    }
    if let crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsMrKeyArn(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_DkmsMRKeyArn(inner)?;
    }
    if let crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::Discovery(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_Ddiscovery(inner)?;
    }
    if let crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::MrDiscovery(ref inner) =
        &input
    {
        validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_DmrDiscovery(inner)?;
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_Ddiscovery(
    input: &crate::deps::aws_cryptography_keyStore::types::Discovery,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HDiscovery(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_DkmsKeyArn(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if !(1..=2048).contains(
        &input
            .chars()
            .map(::std::primitive::char::len_utf16)
            .fold(0usize, ::std::ops::Add::add),
    ) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "kms_key_arn",
        "kms_key_arn failed to satisfy constraint: Member must have length between 1 and 2048, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_DkmsMRKeyArn(
    input: &::std::string::String,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if !(1..=2048).contains(
        &input
            .chars()
            .map(::std::primitive::char::len_utf16)
            .fold(0usize, ::std::ops::Add::add),
    ) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "kms_mr_key_arn",
        "kms_mr_key_arn failed to satisfy constraint: Member must have length between 1 and 2048, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HKMSConfiguration_DmrDiscovery(
    input: &crate::deps::aws_cryptography_keyStore::types::MrDiscovery,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HMRDiscovery(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HMRDiscovery(
    input: &crate::deps::aws_cryptography_keyStore::types::MrDiscovery,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HMRDiscovery_Dregion(&input.r#region)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HMRDiscovery_Dregion(
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

    if !(1..=32).contains(
        &input
            .chars()
            .map(::std::primitive::char::len_utf16)
            .fold(0usize, ::std::ops::Add::add),
    ) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "region",
        "region failed to satisfy constraint: Member must have length between 1 and 32, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HVersionKeyInput(
    input: &crate::deps::aws_cryptography_keyStore::types::VersionKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HVersionKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HVersionKeyInput_for_KeyStore_VersionKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_PkeyStore_HVersionKeyInput_DbranchKeyIdentifier(
        &input.r#branch_key_identifier,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HVersionKeyInput_DbranchKeyIdentifier(
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
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HVersionKeyOutput(
    input: &crate::deps::aws_cryptography_keyStore::types::VersionKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_PkeyStore_HVersionKeyOutput_for_KeyStore_VersionKey(
    input: &crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
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

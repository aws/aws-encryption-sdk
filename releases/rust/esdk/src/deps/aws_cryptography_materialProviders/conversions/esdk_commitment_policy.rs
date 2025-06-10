// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy::FORBID_ENCRYPT_ALLOW_DECRYPT {},
crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy::REQUIRE_ENCRYPT_ALLOW_DECRYPT {},
crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy::REQUIRE_ENCRYPT_REQUIRE_DECRYPT {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy,
) -> crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy::FORBID_ENCRYPT_ALLOW_DECRYPT {} => crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy::REQUIRE_ENCRYPT_ALLOW_DECRYPT {} => crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::ESDKCommitmentPolicy::REQUIRE_ENCRYPT_REQUIRE_DECRYPT {} => crate::deps::aws_cryptography_materialProviders::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
    }
}

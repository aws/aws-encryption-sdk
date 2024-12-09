// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::AwsEncryptionSdkConfig,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::AwsEncryptionSdkConfig,
    >,
) -> crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig {
    plain_from_dafny(&*dafny_value)
}


#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig,
) -> crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::AwsEncryptionSdkConfig {
    crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::AwsEncryptionSdkConfig::AwsEncryptionSdkConfig {
        commitmentPolicy: ::std::rc::Rc::new(match &value.commitment_policy {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::esdk_commitment_policy::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 maxEncryptedDataKeys: crate::standard_library_conversions::olong_to_dafny(&value.max_encrypted_data_keys),
 netV4_0_0_RetryPolicy: ::std::rc::Rc::new(match &value.net_v4_0_0_retry_policy {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::conversions::net_v4_0_0_retry_policy::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::AwsEncryptionSdkConfig,
) -> crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig {
    match dafny_value {
        crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::AwsEncryptionSdkConfig::AwsEncryptionSdkConfig {..} =>
            crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig::builder()
                .set_commitment_policy(match &**dafny_value.commitmentPolicy() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::aws_cryptography_materialProviders::conversions::esdk_commitment_policy::from_dafny(value)
    ),
    _ => None,
}
)
 .set_max_encrypted_data_keys(crate::standard_library_conversions::olong_from_dafny(dafny_value.maxEncryptedDataKeys().clone()))
 .set_net_v4_0_0_retry_policy(match &**dafny_value.netV4_0_0_RetryPolicy() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::conversions::net_v4_0_0_retry_policy::from_dafny(value)
    ),
    _ => None,
}
)
                .build()
                .unwrap()
    }
}

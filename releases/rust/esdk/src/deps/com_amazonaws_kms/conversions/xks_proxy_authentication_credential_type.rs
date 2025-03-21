// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::XksProxyAuthenticationCredentialType,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyAuthenticationCredentialType>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyAuthenticationCredentialType::XksProxyAuthenticationCredentialType {
        AccessKeyId: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.access_key_id),
 RawSecretAccessKey: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.raw_secret_access_key),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::XksProxyAuthenticationCredentialType,
    >,
) -> aws_sdk_kms::types::XksProxyAuthenticationCredentialType {
    aws_sdk_kms::types::XksProxyAuthenticationCredentialType::builder()
        .set_access_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.AccessKeyId(),
            ),
        ))
        .set_raw_secret_access_key(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.RawSecretAccessKey(),
            ),
        ))
        .build()
        .unwrap()
}

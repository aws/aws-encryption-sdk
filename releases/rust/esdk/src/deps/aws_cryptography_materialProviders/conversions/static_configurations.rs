// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StaticConfigurations,
>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::AwsKmsEcdh(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StaticConfigurations::AWS_KMS_ECDH {
        AWS_KMS_ECDH: crate::deps::aws_cryptography_materialProviders::conversions::kms_ecdh_static_configurations::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::RawEcdh(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StaticConfigurations::RAW_ECDH {
        RAW_ECDH: crate::deps::aws_cryptography_materialProviders::conversions::raw_ecdh_static_configurations::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StaticConfigurations,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StaticConfigurations::AWS_KMS_ECDH {
    AWS_KMS_ECDH: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::AwsKmsEcdh(crate::deps::aws_cryptography_materialProviders::conversions::kms_ecdh_static_configurations::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StaticConfigurations::RAW_ECDH {
    RAW_ECDH: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::StaticConfigurations::RawEcdh(crate::deps::aws_cryptography_materialProviders::conversions::raw_ecdh_static_configurations::from_dafny(x.clone())
),
    }
}

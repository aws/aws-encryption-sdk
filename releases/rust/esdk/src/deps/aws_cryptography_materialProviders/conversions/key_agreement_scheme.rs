// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::KeyAgreementScheme,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KeyAgreementScheme,
>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::KeyAgreementScheme::StaticConfiguration(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KeyAgreementScheme::StaticConfiguration {
        StaticConfiguration: crate::deps::aws_cryptography_materialProviders::conversions::static_configurations::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KeyAgreementScheme,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::KeyAgreementScheme {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KeyAgreementScheme::StaticConfiguration {
    StaticConfiguration: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::KeyAgreementScheme::StaticConfiguration(crate::deps::aws_cryptography_materialProviders::conversions::static_configurations::from_dafny(x.clone())
),
    }
}

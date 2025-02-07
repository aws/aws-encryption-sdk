// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::Encrypt,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Encrypt,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::Encrypt::AesGcm(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Encrypt::AES_GCM {
        AES_GCM: crate::deps::aws_cryptography_primitives::conversions::aes_gcm::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Encrypt,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::Encrypt {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Encrypt::AES_GCM {
    AES_GCM: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::Encrypt::AesGcm(crate::deps::aws_cryptography_primitives::conversions::aes_gcm::from_dafny(x.clone())
),
    }
}

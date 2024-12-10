// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm,
> {
    ::std::rc::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::Hkdf(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm::HKDF {
        HKDF: crate::deps::aws_cryptography_materialProviders::conversions::hkdf::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::Identity(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm::IDENTITY {
        IDENTITY: crate::deps::aws_cryptography_materialProviders::conversions::identity::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::None(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm::None {
        _None: crate::deps::aws_cryptography_materialProviders::conversions::none::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm {
    match &::std::rc::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm::HKDF {
    HKDF: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::Hkdf(crate::deps::aws_cryptography_materialProviders::conversions::hkdf::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm::IDENTITY {
    IDENTITY: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::Identity(crate::deps::aws_cryptography_materialProviders::conversions::identity::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::DerivationAlgorithm::None {
    _None: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm::None(crate::deps::aws_cryptography_materialProviders::conversions::none::from_dafny(x.clone())
),
    }
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EdkWrappingAlgorithm,
>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::DirectKeyWrapping(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EdkWrappingAlgorithm::DIRECT_KEY_WRAPPING {
        DIRECT_KEY_WRAPPING: crate::deps::aws_cryptography_materialProviders::conversions::direct_key_wrapping::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::IntermediateKeyWrapping(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EdkWrappingAlgorithm::IntermediateKeyWrapping {
        IntermediateKeyWrapping: crate::deps::aws_cryptography_materialProviders::conversions::intermediate_key_wrapping::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EdkWrappingAlgorithm,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EdkWrappingAlgorithm::DIRECT_KEY_WRAPPING {
    DIRECT_KEY_WRAPPING: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::DirectKeyWrapping(crate::deps::aws_cryptography_materialProviders::conversions::direct_key_wrapping::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::EdkWrappingAlgorithm::IntermediateKeyWrapping {
    IntermediateKeyWrapping: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::EdkWrappingAlgorithm::IntermediateKeyWrapping(crate::deps::aws_cryptography_materialProviders::conversions::intermediate_key_wrapping::from_dafny(x.clone())
),
    }
}

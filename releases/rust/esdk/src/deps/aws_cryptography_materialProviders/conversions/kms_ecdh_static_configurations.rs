// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KmsEcdhStaticConfigurations,
>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery {
        KmsPublicKeyDiscovery: crate::deps::aws_cryptography_materialProviders::conversions::kms_public_key_discovery_input::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey {
        KmsPrivateKeyToStaticPublicKey: crate::deps::aws_cryptography_materialProviders::conversions::kms_private_key_to_static_public_key_input::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KmsEcdhStaticConfigurations,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery {
    KmsPublicKeyDiscovery: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPublicKeyDiscovery(crate::deps::aws_cryptography_materialProviders::conversions::kms_public_key_discovery_input::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey {
    KmsPrivateKeyToStaticPublicKey: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::KmsEcdhStaticConfigurations::KmsPrivateKeyToStaticPublicKey(crate::deps::aws_cryptography_materialProviders::conversions::kms_private_key_to_static_public_key_input::from_dafny(x.clone())
),
    }
}

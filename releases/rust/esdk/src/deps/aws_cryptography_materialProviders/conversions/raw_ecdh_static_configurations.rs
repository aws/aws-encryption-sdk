// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations,
>{
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::PublicKeyDiscovery(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations::PublicKeyDiscovery {
        PublicKeyDiscovery: crate::deps::aws_cryptography_materialProviders::conversions::public_key_discovery_input::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey {
        RawPrivateKeyToStaticPublicKey: crate::deps::aws_cryptography_materialProviders::conversions::raw_private_key_to_static_public_key_input::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey {
        EphemeralPrivateKeyToStaticPublicKey: crate::deps::aws_cryptography_materialProviders::conversions::ephemeral_private_key_to_static_public_key_input::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations::PublicKeyDiscovery {
    PublicKeyDiscovery: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::PublicKeyDiscovery(crate::deps::aws_cryptography_materialProviders::conversions::public_key_discovery_input::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey {
    RawPrivateKeyToStaticPublicKey: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::RawPrivateKeyToStaticPublicKey(crate::deps::aws_cryptography_materialProviders::conversions::raw_private_key_to_static_public_key_input::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey {
    EphemeralPrivateKeyToStaticPublicKey: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::RawEcdhStaticConfigurations::EphemeralPrivateKeyToStaticPublicKey(crate::deps::aws_cryptography_materialProviders::conversions::ephemeral_private_key_to_static_public_key_input::from_dafny(x.clone())
),
    }
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::Materials,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::Materials::Encryption(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::Encryption {
        Encryption: crate::deps::aws_cryptography_materialProviders::conversions::encryption_materials::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::Materials::Decryption(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::Decryption {
        Decryption: crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::Materials::BranchKey(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::BranchKey {
        BranchKey: crate::deps::aws_cryptography_keyStore::conversions::branch_key_materials::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::Materials::BeaconKey(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::BeaconKey {
        BeaconKey: crate::deps::aws_cryptography_keyStore::conversions::beacon_key_materials::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::Materials {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::Encryption {
    Encryption: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::Materials::Encryption(crate::deps::aws_cryptography_materialProviders::conversions::encryption_materials::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::Decryption {
    Decryption: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::Materials::Decryption(crate::deps::aws_cryptography_materialProviders::conversions::decryption_materials::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::BranchKey {
    BranchKey: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::Materials::BranchKey(crate::deps::aws_cryptography_keyStore::conversions::branch_key_materials::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::Materials::BeaconKey {
    BeaconKey: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::Materials::BeaconKey(crate::deps::aws_cryptography_keyStore::conversions::beacon_key_materials::from_dafny(x.clone())
),
    }
}

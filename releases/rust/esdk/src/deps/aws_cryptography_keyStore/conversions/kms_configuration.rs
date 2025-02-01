// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_keyStore::types::KmsConfiguration,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsKeyArn(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsKeyArn {
        kmsKeyArn: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x),
    },
crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsMrKeyArn(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsMRKeyArn {
        kmsMRKeyArn: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x),
    },
crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::Discovery(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::discovery {
        discovery: crate::deps::aws_cryptography_keyStore::conversions::discovery::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::MrDiscovery(x) =>
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::mrDiscovery {
        mrDiscovery: crate::deps::aws_cryptography_keyStore::conversions::mr_discovery::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration,
    >,
) -> crate::deps::aws_cryptography_keyStore::types::KmsConfiguration {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsKeyArn {
    kmsKeyArn: x @ _,
} => crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsKeyArn(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(x)),
crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::kmsMRKeyArn {
    kmsMRKeyArn: x @ _,
} => crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsMrKeyArn(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(x)),
crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::discovery {
    discovery: x @ _,
} => crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::Discovery(crate::deps::aws_cryptography_keyStore::conversions::discovery::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::keystore::internaldafny::types::KMSConfiguration::mrDiscovery {
    mrDiscovery: x @ _,
} => crate::deps::aws_cryptography_keyStore::types::KmsConfiguration::MrDiscovery(crate::deps::aws_cryptography_keyStore::conversions::mr_discovery::from_dafny(x.clone())
),
    }
}

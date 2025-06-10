// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::types::PaddingScheme,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::Pkcs1 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::PKCS1 {},
crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha1Mgf1 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA1_MGF1 {},
crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha256Mgf1 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA256_MGF1 {},
crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha384Mgf1 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA384_MGF1 {},
crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha512Mgf1 => crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA512_MGF1 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme,
) -> crate::deps::aws_cryptography_materialProviders::types::PaddingScheme {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::PKCS1 {} => crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::Pkcs1,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA1_MGF1 {} => crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha1Mgf1,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA256_MGF1 {} => crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha256Mgf1,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA384_MGF1 {} => crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha384Mgf1,
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::PaddingScheme::OAEP_SHA512_MGF1 {} => crate::deps::aws_cryptography_materialProviders::types::PaddingScheme::OaepSha512Mgf1,
    }
}

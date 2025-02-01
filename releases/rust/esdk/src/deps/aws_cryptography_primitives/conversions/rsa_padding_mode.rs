// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::types::RsaPaddingMode,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::Pkcs1 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::PKCS1 {},
crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha1 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA1 {},
crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha256 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA256 {},
crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha384 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA384 {},
crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha512 => crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA512 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode,
) -> crate::deps::aws_cryptography_primitives::types::RsaPaddingMode {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::PKCS1 {} => crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::Pkcs1,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA1 {} => crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha1,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA256 {} => crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha256,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA384 {} => crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha384,
crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode::OAEP_SHA512 {} => crate::deps::aws_cryptography_primitives::types::RsaPaddingMode::OaepSha512,
    }
}

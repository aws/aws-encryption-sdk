// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: aws_sdk_kms::types::KeyEncryptionMechanism,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyEncryptionMechanism>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_kms::types::KeyEncryptionMechanism::RsaesOaepSha256 => crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyEncryptionMechanism::RSAES_OAEP_SHA_256 {},
        _ => panic!("Unknown enum variant: {}", value),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyEncryptionMechanism,
) -> aws_sdk_kms::types::KeyEncryptionMechanism {
    match dafny_value {
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyEncryptionMechanism::RSAES_OAEP_SHA_256 {} => aws_sdk_kms::types::KeyEncryptionMechanism::RsaesOaepSha256,
    }
}

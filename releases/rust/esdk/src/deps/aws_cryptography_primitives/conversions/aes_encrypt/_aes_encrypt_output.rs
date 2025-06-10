// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput::AESEncryptOutput {
        cipherText: crate::standard_library_conversions::blob_to_dafny(&value.cipher_text.unwrap()),
 authTag: crate::standard_library_conversions::blob_to_dafny(&value.auth_tag.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput {
    crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput::builder()
        .set_cipher_text(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.cipherText().clone(),
        )))
        .set_auth_tag(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.authTag().clone(),
        )))
        .build()
        .unwrap()
}

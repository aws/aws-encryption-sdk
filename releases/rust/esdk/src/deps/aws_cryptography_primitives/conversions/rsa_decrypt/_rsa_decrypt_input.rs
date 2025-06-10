// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSADecryptInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSADecryptInput::RSADecryptInput {
        padding: crate::deps::aws_cryptography_primitives::conversions::rsa_padding_mode::to_dafny(value.padding.clone().unwrap()),
 privateKey: crate::standard_library_conversions::blob_to_dafny(&value.private_key.unwrap()),
 cipherText: crate::standard_library_conversions::blob_to_dafny(&value.cipher_text.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSADecryptInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptInput {
    crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptInput::builder()
        .set_padding(Some(
            crate::deps::aws_cryptography_primitives::conversions::rsa_padding_mode::from_dafny(
                dafny_value.padding(),
            ),
        ))
        .set_private_key(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.privateKey().clone(),
        )))
        .set_cipher_text(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.cipherText().clone(),
        )))
        .build()
        .unwrap()
}

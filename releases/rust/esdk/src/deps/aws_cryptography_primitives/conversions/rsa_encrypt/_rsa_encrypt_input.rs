// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput::RSAEncryptInput {
        padding: crate::deps::aws_cryptography_primitives::conversions::rsa_padding_mode::to_dafny(value.padding.clone().unwrap()),
 publicKey: crate::standard_library_conversions::blob_to_dafny(&value.public_key.unwrap()),
 plaintext: crate::standard_library_conversions::blob_to_dafny(&value.plaintext.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAEncryptInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptInput {
    crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptInput::builder()
        .set_padding(Some(
            crate::deps::aws_cryptography_primitives::conversions::rsa_padding_mode::from_dafny(
                dafny_value.padding(),
            ),
        ))
        .set_public_key(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.publicKey().clone(),
        )))
        .set_plaintext(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.plaintext().clone(),
        )))
        .build()
        .unwrap()
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptInput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptInput::AESEncryptInput {
        encAlg: crate::deps::aws_cryptography_primitives::conversions::aes_gcm::to_dafny(&value.enc_alg.clone().unwrap())
,
 iv: crate::standard_library_conversions::blob_to_dafny(&value.iv.unwrap()),
 key: crate::standard_library_conversions::blob_to_dafny(&value.key.unwrap()),
 msg: crate::standard_library_conversions::blob_to_dafny(&value.msg.unwrap()),
 aad: crate::standard_library_conversions::blob_to_dafny(&value.aad.unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::AESEncryptInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptInput {
    crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptInput::builder()
        .set_enc_alg(Some(
            crate::deps::aws_cryptography_primitives::conversions::aes_gcm::from_dafny(
                dafny_value.encAlg().clone(),
            ),
        ))
        .set_iv(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.iv().clone(),
        )))
        .set_key(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.key().clone(),
        )))
        .set_msg(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.msg().clone(),
        )))
        .set_aad(Some(crate::standard_library_conversions::blob_from_dafny(
            dafny_value.aad().clone(),
        )))
        .build()
        .unwrap()
}

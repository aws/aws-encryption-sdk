// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::derive_shared_secret::DeriveSharedSecretOutput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::DeriveSharedSecretOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::DeriveSharedSecretOutput::DeriveSharedSecretOutput {
        sharedSecret: crate::standard_library_conversions::blob_to_dafny(&value.shared_secret.unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DeriveSharedSecretOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::derive_shared_secret::DeriveSharedSecretOutput {
    crate::deps::aws_cryptography_primitives::operation::derive_shared_secret::DeriveSharedSecretOutput::builder()
        .set_shared_secret(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.sharedSecret().clone())))
        .build()
        .unwrap()
}

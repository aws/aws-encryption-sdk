// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::GenerateRandomBytesInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRandomBytesInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRandomBytesInput::GenerateRandomBytesInput {
        length: value.length.clone().unwrap(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::GenerateRandomBytesInput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::GenerateRandomBytesInput{
    crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::GenerateRandomBytesInput::builder()
        .set_length(Some( dafny_value.length() .clone() ))
        .build()
        .unwrap()
}

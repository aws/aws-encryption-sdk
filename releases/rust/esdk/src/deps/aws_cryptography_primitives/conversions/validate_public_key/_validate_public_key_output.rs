// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::operation::validate_public_key::ValidatePublicKeyOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::ValidatePublicKeyOutput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::ValidatePublicKeyOutput::ValidatePublicKeyOutput {
        success: value.success.clone().unwrap(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::ValidatePublicKeyOutput,
    >,
) -> crate::deps::aws_cryptography_primitives::operation::validate_public_key::ValidatePublicKeyOutput
{
    crate::deps::aws_cryptography_primitives::operation::validate_public_key::ValidatePublicKeyOutput::builder()
        .set_success(Some( dafny_value.success() .clone() ))
        .build()
        .unwrap()
}

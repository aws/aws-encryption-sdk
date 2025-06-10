// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::CreateDefaultClientSupplierInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateDefaultClientSupplierInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateDefaultClientSupplierInput::CreateDefaultClientSupplierInput {

    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateDefaultClientSupplierInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::CreateDefaultClientSupplierInput{
    crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::CreateDefaultClientSupplierInput::builder()

        .build()
        .unwrap()
}

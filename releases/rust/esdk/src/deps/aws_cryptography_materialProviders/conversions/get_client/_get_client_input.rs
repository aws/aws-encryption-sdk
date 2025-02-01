// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetClientInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetClientInput::GetClientInput {
        region: crate::standard_library_conversions::ostring_to_dafny(&value.region) .Extract(),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::GetClientInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientInput {
    crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientInput::builder(
    )
    .set_region(Some(
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
            dafny_value.region(),
        ),
    ))
    .build()
    .unwrap()
}

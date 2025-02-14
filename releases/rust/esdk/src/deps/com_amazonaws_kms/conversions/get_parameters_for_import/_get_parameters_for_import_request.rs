// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::get_parameters_for_import::GetParametersForImportInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportRequest,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportRequest::GetParametersForImportRequest {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id) .Extract(),
 WrappingAlgorithm: crate::deps::com_amazonaws_kms::conversions::algorithm_spec::to_dafny(value.wrapping_algorithm.clone().unwrap()),
 WrappingKeySpec: crate::deps::com_amazonaws_kms::conversions::wrapping_key_spec::to_dafny(value.wrapping_key_spec.clone().unwrap()),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportRequest,
    >,
) -> aws_sdk_kms::operation::get_parameters_for_import::GetParametersForImportInput {
    aws_sdk_kms::operation::get_parameters_for_import::GetParametersForImportInput::builder()
        .set_key_id(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.KeyId(),
            ),
        ))
        .set_wrapping_algorithm(Some(
            crate::deps::com_amazonaws_kms::conversions::algorithm_spec::from_dafny(
                dafny_value.WrappingAlgorithm(),
            ),
        ))
        .set_wrapping_key_spec(Some(
            crate::deps::com_amazonaws_kms::conversions::wrapping_key_spec::from_dafny(
                dafny_value.WrappingKeySpec(),
            ),
        ))
        .build()
        .unwrap()
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::get_parameters_for_import::GetParametersForImportOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportResponse::GetParametersForImportResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 ImportToken: crate::standard_library_conversions::oblob_to_dafny(&value.import_token),
 PublicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key),
 ParametersValidTo: crate::standard_library_conversions::otimestamp_to_dafny(&value.parameters_valid_to),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportResponse,
    >,
) -> aws_sdk_kms::operation::get_parameters_for_import::GetParametersForImportOutput {
    aws_sdk_kms::operation::get_parameters_for_import::GetParametersForImportOutput::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_import_token(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.ImportToken().clone(),
        ))
        .set_public_key(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.PublicKey().clone(),
        ))
        .set_parameters_valid_to(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.ParametersValidTo().clone(),
        ))
        .build()
}

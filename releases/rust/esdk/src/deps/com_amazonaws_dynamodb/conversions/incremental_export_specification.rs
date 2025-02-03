// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::IncrementalExportSpecification,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IncrementalExportSpecification>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IncrementalExportSpecification::IncrementalExportSpecification {
        ExportFromTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.export_from_time),
 ExportToTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.export_to_time),
 ExportViewType: ::dafny_runtime::Rc::new(match &value.export_view_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::export_view_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IncrementalExportSpecification,
    >,
) -> aws_sdk_dynamodb::types::IncrementalExportSpecification {
    aws_sdk_dynamodb::types::IncrementalExportSpecification::builder()
        .set_export_from_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.ExportFromTime().clone(),
        ))
        .set_export_to_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.ExportToTime().clone(),
        ))
        .set_export_view_type(match &**dafny_value.ExportViewType() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::export_view_type::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .build()
}

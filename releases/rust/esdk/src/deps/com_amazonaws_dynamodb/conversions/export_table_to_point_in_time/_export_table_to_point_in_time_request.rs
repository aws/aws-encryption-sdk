// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportTableToPointInTimeInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportTableToPointInTimeInput::ExportTableToPointInTimeInput {
        TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn) .Extract(),
 ExportTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.export_time),
 ClientToken: crate::standard_library_conversions::ostring_to_dafny(&value.client_token),
 S3Bucket: crate::standard_library_conversions::ostring_to_dafny(&value.s3_bucket) .Extract(),
 S3BucketOwner: crate::standard_library_conversions::ostring_to_dafny(&value.s3_bucket_owner),
 S3Prefix: crate::standard_library_conversions::ostring_to_dafny(&value.s3_prefix),
 S3SseAlgorithm: ::dafny_runtime::Rc::new(match &value.s3_sse_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::s3_sse_algorithm::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 S3SseKmsKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.s3_sse_kms_key_id),
 ExportFormat: ::dafny_runtime::Rc::new(match &value.export_format {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::export_format::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ExportType: ::dafny_runtime::Rc::new(match &value.export_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::export_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 IncrementalExportSpecification: ::dafny_runtime::Rc::new(match &value.incremental_export_specification {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::incremental_export_specification::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportTableToPointInTimeInput,
    >,
) -> aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput {
    aws_sdk_dynamodb::operation::export_table_to_point_in_time::ExportTableToPointInTimeInput::builder()
          .set_table_arn(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.TableArn()) ))
 .set_export_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.ExportTime().clone()))
 .set_client_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ClientToken().clone()))
 .set_s3_bucket(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.S3Bucket()) ))
 .set_s3_bucket_owner(crate::standard_library_conversions::ostring_from_dafny(dafny_value.S3BucketOwner().clone()))
 .set_s3_prefix(crate::standard_library_conversions::ostring_from_dafny(dafny_value.S3Prefix().clone()))
 .set_s3_sse_algorithm(match &**dafny_value.S3SseAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::s3_sse_algorithm::from_dafny(value)
    ),
    _ => None,
}
)
 .set_s3_sse_kms_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.S3SseKmsKeyId().clone()))
 .set_export_format(match &**dafny_value.ExportFormat() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::export_format::from_dafny(value)
    ),
    _ => None,
}
)
 .set_export_type(match &**dafny_value.ExportType() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::export_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_incremental_export_specification(match (*dafny_value.IncrementalExportSpecification()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::incremental_export_specification::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}

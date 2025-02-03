// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ExportDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportDescription::ExportDescription {
        ExportArn: crate::standard_library_conversions::ostring_to_dafny(&value.export_arn),
 ExportStatus: ::dafny_runtime::Rc::new(match &value.export_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::export_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 StartTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.start_time),
 EndTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.end_time),
 ExportManifest: crate::standard_library_conversions::ostring_to_dafny(&value.export_manifest),
 TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 TableId: crate::standard_library_conversions::ostring_to_dafny(&value.table_id),
 ExportTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.export_time),
 ClientToken: crate::standard_library_conversions::ostring_to_dafny(&value.client_token),
 S3Bucket: crate::standard_library_conversions::ostring_to_dafny(&value.s3_bucket),
 S3BucketOwner: crate::standard_library_conversions::ostring_to_dafny(&value.s3_bucket_owner),
 S3Prefix: crate::standard_library_conversions::ostring_to_dafny(&value.s3_prefix),
 S3SseAlgorithm: ::dafny_runtime::Rc::new(match &value.s3_sse_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::s3_sse_algorithm::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 S3SseKmsKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.s3_sse_kms_key_id),
 FailureCode: crate::standard_library_conversions::ostring_to_dafny(&value.failure_code),
 FailureMessage: crate::standard_library_conversions::ostring_to_dafny(&value.failure_message),
 ExportFormat: ::dafny_runtime::Rc::new(match &value.export_format {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::export_format::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 BilledSizeBytes: crate::standard_library_conversions::olong_to_dafny(&value.billed_size_bytes),
 ItemCount: crate::standard_library_conversions::olong_to_dafny(&value.item_count),
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
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExportDescription,
    >,
) -> aws_sdk_dynamodb::types::ExportDescription {
    aws_sdk_dynamodb::types::ExportDescription::builder()
          .set_export_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ExportArn().clone()))
 .set_export_status(match &**dafny_value.ExportStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::export_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_start_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.StartTime().clone()))
 .set_end_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.EndTime().clone()))
 .set_export_manifest(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ExportManifest().clone()))
 .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableArn().clone()))
 .set_table_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableId().clone()))
 .set_export_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.ExportTime().clone()))
 .set_client_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ClientToken().clone()))
 .set_s3_bucket(crate::standard_library_conversions::ostring_from_dafny(dafny_value.S3Bucket().clone()))
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
 .set_failure_code(crate::standard_library_conversions::ostring_from_dafny(dafny_value.FailureCode().clone()))
 .set_failure_message(crate::standard_library_conversions::ostring_from_dafny(dafny_value.FailureMessage().clone()))
 .set_export_format(match &**dafny_value.ExportFormat() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::export_format::from_dafny(value)
    ),
    _ => None,
}
)
 .set_billed_size_bytes(crate::standard_library_conversions::olong_from_dafny(dafny_value.BilledSizeBytes().clone()))
 .set_item_count(crate::standard_library_conversions::olong_from_dafny(dafny_value.ItemCount().clone()))
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
}

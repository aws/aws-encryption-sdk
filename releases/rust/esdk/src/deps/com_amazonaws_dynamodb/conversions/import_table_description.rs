// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ImportTableDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableDescription::ImportTableDescription {
        ImportArn: crate::standard_library_conversions::ostring_to_dafny(&value.import_arn),
 ImportStatus: ::dafny_runtime::Rc::new(match &value.import_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::import_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 TableId: crate::standard_library_conversions::ostring_to_dafny(&value.table_id),
 ClientToken: crate::standard_library_conversions::ostring_to_dafny(&value.client_token),
 S3BucketSource: ::dafny_runtime::Rc::new(match &value.s3_bucket_source {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::s3_bucket_source::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ErrorCount: crate::standard_library_conversions::olong_to_dafny(&Some(value.error_count)),
 CloudWatchLogGroupArn: crate::standard_library_conversions::ostring_to_dafny(&value.cloud_watch_log_group_arn),
 InputFormat: ::dafny_runtime::Rc::new(match &value.input_format {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::input_format::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 InputFormatOptions: ::dafny_runtime::Rc::new(match &value.input_format_options {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::input_format_options::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 InputCompressionType: ::dafny_runtime::Rc::new(match &value.input_compression_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::input_compression_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TableCreationParameters: ::dafny_runtime::Rc::new(match &value.table_creation_parameters {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::table_creation_parameters::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 StartTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.start_time),
 EndTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.end_time),
 ProcessedSizeBytes: crate::standard_library_conversions::olong_to_dafny(&value.processed_size_bytes),
 ProcessedItemCount: crate::standard_library_conversions::olong_to_dafny(&Some(value.processed_item_count)),
 ImportedItemCount: crate::standard_library_conversions::olong_to_dafny(&Some(value.imported_item_count)),
 FailureCode: crate::standard_library_conversions::ostring_to_dafny(&value.failure_code),
 FailureMessage: crate::standard_library_conversions::ostring_to_dafny(&value.failure_message),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableDescription,
    >,
) -> aws_sdk_dynamodb::types::ImportTableDescription {
    aws_sdk_dynamodb::types::ImportTableDescription::builder()
          .set_import_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ImportArn().clone()))
 .set_import_status(match &**dafny_value.ImportStatus() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::import_status::from_dafny(value)
    ),
    _ => None,
}
)
 .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableArn().clone()))
 .set_table_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableId().clone()))
 .set_client_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ClientToken().clone()))
 .set_s3_bucket_source(match (*dafny_value.S3BucketSource()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::s3_bucket_source::from_dafny(value.clone())),
    _ => None,
}
)
 .set_error_count(crate::standard_library_conversions::olong_from_dafny(dafny_value.ErrorCount().clone()))
 .set_cloud_watch_log_group_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CloudWatchLogGroupArn().clone()))
 .set_input_format(match &**dafny_value.InputFormat() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::input_format::from_dafny(value)
    ),
    _ => None,
}
)
 .set_input_format_options(match (*dafny_value.InputFormatOptions()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::input_format_options::from_dafny(value.clone())),
    _ => None,
}
)
 .set_input_compression_type(match &**dafny_value.InputCompressionType() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::input_compression_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_table_creation_parameters(match (*dafny_value.TableCreationParameters()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::table_creation_parameters::from_dafny(value.clone())),
    _ => None,
}
)
 .set_start_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.StartTime().clone()))
 .set_end_time(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.EndTime().clone()))
 .set_processed_size_bytes(crate::standard_library_conversions::olong_from_dafny(dafny_value.ProcessedSizeBytes().clone()))
 .set_processed_item_count(crate::standard_library_conversions::olong_from_dafny(dafny_value.ProcessedItemCount().clone()))
 .set_imported_item_count(crate::standard_library_conversions::olong_from_dafny(dafny_value.ImportedItemCount().clone()))
 .set_failure_code(crate::standard_library_conversions::ostring_from_dafny(dafny_value.FailureCode().clone()))
 .set_failure_message(crate::standard_library_conversions::ostring_from_dafny(dafny_value.FailureMessage().clone()))
          .build()
}

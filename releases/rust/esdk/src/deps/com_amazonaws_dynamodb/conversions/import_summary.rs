// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ImportSummary,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportSummary>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportSummary::ImportSummary {
        ImportArn: crate::standard_library_conversions::ostring_to_dafny(&value.import_arn),
 ImportStatus: ::dafny_runtime::Rc::new(match &value.import_status {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::import_status::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TableArn: crate::standard_library_conversions::ostring_to_dafny(&value.table_arn),
 S3BucketSource: ::dafny_runtime::Rc::new(match &value.s3_bucket_source {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::s3_bucket_source::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CloudWatchLogGroupArn: crate::standard_library_conversions::ostring_to_dafny(&value.cloud_watch_log_group_arn),
 InputFormat: ::dafny_runtime::Rc::new(match &value.input_format {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::input_format::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 StartTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.start_time),
 EndTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.end_time),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportSummary,
    >,
) -> aws_sdk_dynamodb::types::ImportSummary {
    aws_sdk_dynamodb::types::ImportSummary::builder()
        .set_import_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.ImportArn().clone(),
        ))
        .set_import_status(match &**dafny_value.ImportStatus() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::import_status::from_dafny(value),
            ),
            _ => None,
        })
        .set_table_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.TableArn().clone(),
        ))
        .set_s3_bucket_source(match (*dafny_value.S3BucketSource()).as_ref() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::s3_bucket_source::from_dafny(
                    value.clone(),
                ),
            ),
            _ => None,
        })
        .set_cloud_watch_log_group_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.CloudWatchLogGroupArn().clone(),
        ))
        .set_input_format(match &**dafny_value.InputFormat() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_dynamodb::conversions::input_format::from_dafny(value),
            ),
            _ => None,
        })
        .set_start_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.StartTime().clone(),
        ))
        .set_end_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.EndTime().clone(),
        ))
        .build()
}

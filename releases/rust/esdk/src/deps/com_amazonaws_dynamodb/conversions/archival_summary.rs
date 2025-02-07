// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ArchivalSummary,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ArchivalSummary>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ArchivalSummary::ArchivalSummary {
        ArchivalDateTime: crate::standard_library_conversions::otimestamp_to_dafny(&value.archival_date_time),
 ArchivalReason: crate::standard_library_conversions::ostring_to_dafny(&value.archival_reason),
 ArchivalBackupArn: crate::standard_library_conversions::ostring_to_dafny(&value.archival_backup_arn),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ArchivalSummary,
    >,
) -> aws_sdk_dynamodb::types::ArchivalSummary {
    aws_sdk_dynamodb::types::ArchivalSummary::builder()
        .set_archival_date_time(crate::standard_library_conversions::otimestamp_from_dafny(
            dafny_value.ArchivalDateTime().clone(),
        ))
        .set_archival_reason(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.ArchivalReason().clone(),
        ))
        .set_archival_backup_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.ArchivalBackupArn().clone(),
        ))
        .build()
}

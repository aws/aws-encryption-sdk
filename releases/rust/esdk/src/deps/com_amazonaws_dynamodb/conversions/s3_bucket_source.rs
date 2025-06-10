// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::S3BucketSource,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3BucketSource>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3BucketSource::S3BucketSource {
        S3BucketOwner: crate::standard_library_conversions::ostring_to_dafny(&value.s3_bucket_owner),
 S3Bucket: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.s3_bucket),
 S3KeyPrefix: crate::standard_library_conversions::ostring_to_dafny(&value.s3_key_prefix),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::S3BucketSource,
    >,
) -> aws_sdk_dynamodb::types::S3BucketSource {
    aws_sdk_dynamodb::types::S3BucketSource::builder()
        .set_s3_bucket_owner(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.S3BucketOwner().clone(),
        ))
        .set_s3_bucket(Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                dafny_value.S3Bucket(),
            ),
        ))
        .set_s3_key_prefix(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.S3KeyPrefix().clone(),
        ))
        .build()
        .unwrap()
}

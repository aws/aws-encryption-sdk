// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::MultiRegionKey,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKey,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKey::MultiRegionKey {
        Arn: crate::standard_library_conversions::ostring_to_dafny(&value.arn),
 Region: crate::standard_library_conversions::ostring_to_dafny(&value.region),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKey,
    >,
) -> aws_sdk_kms::types::MultiRegionKey {
    aws_sdk_kms::types::MultiRegionKey::builder()
        .set_arn(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.Arn().clone(),
        ))
        .set_region(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.Region().clone(),
        ))
        .build()
}

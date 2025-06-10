// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::Replica,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Replica,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Replica::Replica {
        RegionName: crate::standard_library_conversions::ostring_to_dafny(&value.region_name),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Replica,
    >,
) -> aws_sdk_dynamodb::types::Replica {
    aws_sdk_dynamodb::types::Replica::builder()
        .set_region_name(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.RegionName().clone(),
        ))
        .build()
}

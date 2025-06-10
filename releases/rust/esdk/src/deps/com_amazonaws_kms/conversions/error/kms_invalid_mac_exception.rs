// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: aws_sdk_kms::types::error::KmsInvalidMacException,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error::KMSInvalidMacException {
      message: crate::standard_library_conversions::ostring_to_dafny(&value.message),
    }
  )
}

// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::FailureException,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::FailureException>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::FailureException::FailureException {
        ExceptionName: crate::standard_library_conversions::ostring_to_dafny(&value.exception_name),
 ExceptionDescription: crate::standard_library_conversions::ostring_to_dafny(&value.exception_description),
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::FailureException,
    >,
) -> aws_sdk_dynamodb::types::FailureException {
    aws_sdk_dynamodb::types::FailureException::builder()
          .set_exception_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ExceptionName().clone()))
 .set_exception_description(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ExceptionDescription().clone()))
          .build()

}

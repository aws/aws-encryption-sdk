// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::BatchStatementError,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementError>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementError::BatchStatementError {
        Code: ::dafny_runtime::Rc::new(match &value.code {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::batch_statement_error_code_enum::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 Message: crate::standard_library_conversions::ostring_to_dafny(&value.message),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::BatchStatementError,
    >,
) -> aws_sdk_dynamodb::types::BatchStatementError {
    aws_sdk_dynamodb::types::BatchStatementError::builder()
          .set_code(match &**dafny_value.Code() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::batch_statement_error_code_enum::from_dafny(value)
    ),
    _ => None,
}
)
 .set_message(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Message().clone()))
          .build()
}
